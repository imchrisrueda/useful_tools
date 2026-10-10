import fs from 'node:fs/promises';
import path from 'node:path';
import {
  Requirements,
  RequirementsSchema,
  Storyboard,
  StoryboardSchema,
  ApprovalRecord,
  ApprovalRecordSchema,
  ProjectManifest,
  ProjectManifestSchema,
  Result,
  okResult,
  errorResult
} from './models.js';
import { computeCanonicalRevision } from './hash-utils.js';

export interface AnalyzeRequestInput {
  projectDir: string;
  expectedRevision: string;
  request: string;
  authorizedSourceIds: string[];
  proposal: Requirements;
}

export interface AnalyzeRequestOutput {
  requirementsRef: string;
  revision: string;
  unresolvedQuestions: string[];
}

export async function applyRequirements(input: AnalyzeRequestInput): Promise<Result<AnalyzeRequestOutput>> {
  const manifestPath = path.join(input.projectDir, 'project.json');
  let manifest: ProjectManifest;

  try {
    const raw = await fs.readFile(manifestPath, 'utf-8');
    manifest = ProjectManifestSchema.parse(JSON.parse(raw));
  } catch (err: unknown) {
    return errorResult({
      code: 'INVALID_INPUT',
      message: `Failed to load project manifest: ${(err as Error).message}`,
      recoverable: false
    });
  }

  // Check expected revision
  const currentRevision = computeCanonicalRevision(manifest);
  if (currentRevision !== input.expectedRevision) {
    return errorResult({
      code: 'REVISION_CONFLICT',
      message: `Revision conflict: expected ${input.expectedRevision}, but current is ${currentRevision}`,
      recoverable: false
    });
  }

  // Validate proposal schema
  let validatedProposal: Requirements;
  try {
    validatedProposal = RequirementsSchema.parse(input.proposal);
  } catch (err: unknown) {
    return errorResult({
      code: 'INVALID_INPUT',
      message: `Invalid requirements proposal: ${(err as Error).message}`,
      recoverable: false
    });
  }

  // Write requirements artifact
  const reqFilename = 'requirements.json';
  const reqPath = path.join(input.projectDir, reqFilename);
  await fs.writeFile(reqPath, JSON.stringify(validatedProposal, null, 2), 'utf-8');

  // Update manifest artifact map and recompute revision
  manifest.artifacts['requirements'] = reqFilename;
  const newRevision = computeCanonicalRevision(manifest);
  await fs.writeFile(manifestPath, JSON.stringify(manifest, null, 2), 'utf-8');

  return okResult({
    requirementsRef: reqFilename,
    revision: newRevision,
    unresolvedQuestions: validatedProposal.unresolvedQuestions
  });
}

export interface ApplyStoryboardInput {
  projectDir: string;
  expectedRevision: string;
  requirementsRevision: string;
  sourceIds: string[];
  proposal: Storyboard;
}

export interface ApplyStoryboardOutput {
  storyboardRef: string;
  revision: string;
  unresolvedItems: string[];
}

export async function applyStoryboard(input: ApplyStoryboardInput): Promise<Result<ApplyStoryboardOutput>> {
  const manifestPath = path.join(input.projectDir, 'project.json');
  let manifest: ProjectManifest;

  try {
    const raw = await fs.readFile(manifestPath, 'utf-8');
    manifest = ProjectManifestSchema.parse(JSON.parse(raw));
  } catch (err: unknown) {
    return errorResult({
      code: 'INVALID_INPUT',
      message: `Failed to load project manifest: ${(err as Error).message}`,
      recoverable: false
    });
  }

  const currentRevision = computeCanonicalRevision(manifest);
  if (currentRevision !== input.expectedRevision) {
    return errorResult({
      code: 'REVISION_CONFLICT',
      message: `Revision conflict: expected ${input.expectedRevision}, but current is ${currentRevision}`,
      recoverable: false
    });
  }

  // Validate proposal schema
  let validatedStoryboard: Storyboard;
  try {
    validatedStoryboard = StoryboardSchema.parse(input.proposal);
  } catch (err: unknown) {
    return errorResult({
      code: 'INVALID_INPUT',
      message: `Invalid storyboard proposal: ${(err as Error).message}`,
      recoverable: false
    });
  }

  // Check unique slide IDs
  const slideIds = new Set<string>();
  for (const slide of validatedStoryboard.slides) {
    if (slideIds.has(slide.slideId)) {
      return errorResult({
        code: 'INVALID_INPUT',
        message: `Duplicate slideId in storyboard: ${slide.slideId}`,
        recoverable: false
      });
    }
    slideIds.add(slide.slideId);

    // Validate that slide sourceIds are authorized
    for (const srcId of slide.sourceIds) {
      if (!input.sourceIds.includes(srcId)) {
        return errorResult({
          code: 'SOURCE_UNAUTHORIZED',
          message: `Source ID "${srcId}" on slide "${slide.slideId}" is not in authorized sources list.`,
          recoverable: false
        });
      }
    }
  }

  // Write storyboard artifact
  const storyboardFilename = 'storyboard.json';
  const sbPath = path.join(input.projectDir, storyboardFilename);
  await fs.writeFile(sbPath, JSON.stringify(validatedStoryboard, null, 2), 'utf-8');

  manifest.artifacts['storyboard'] = storyboardFilename;
  const newRevision = computeCanonicalRevision(manifest);
  await fs.writeFile(manifestPath, JSON.stringify(manifest, null, 2), 'utf-8');

  return okResult({
    storyboardRef: storyboardFilename,
    revision: newRevision,
    unresolvedItems: validatedStoryboard.unresolvedItems
  });
}

export interface RecordApprovalInput {
  projectDir: string;
  expectedRevision: string;
  approval: ApprovalRecord;
}

export interface RecordApprovalOutput {
  approvalRef: string;
  revision: string;
}

export async function recordApproval(input: RecordApprovalInput): Promise<Result<RecordApprovalOutput>> {
  const manifestPath = path.join(input.projectDir, 'project.json');
  let manifest: ProjectManifest;

  try {
    const raw = await fs.readFile(manifestPath, 'utf-8');
    manifest = ProjectManifestSchema.parse(JSON.parse(raw));
  } catch (err: unknown) {
    return errorResult({
      code: 'INVALID_INPUT',
      message: `Failed to load project manifest: ${(err as Error).message}`,
      recoverable: false
    });
  }

  const currentRevision = computeCanonicalRevision(manifest);
  if (currentRevision !== input.expectedRevision) {
    return errorResult({
      code: 'REVISION_CONFLICT',
      message: `Revision conflict: expected ${input.expectedRevision}, but current is ${currentRevision}`,
      recoverable: false
    });
  }

  let validatedApproval: ApprovalRecord;
  try {
    validatedApproval = ApprovalRecordSchema.parse(input.approval);
  } catch (err: unknown) {
    return errorResult({
      code: 'APPROVAL_REQUIRED',
      message: `Invalid approval record: ${(err as Error).message}`,
      recoverable: false
    });
  }

  const approvalsDir = path.join(input.projectDir, 'reports', 'approvals');
  await fs.mkdir(approvalsDir, { recursive: true });

  const approvalFilename = `${validatedApproval.stage}-${validatedApproval.id}.json`;
  const approvalPath = path.join(approvalsDir, approvalFilename);
  await fs.writeFile(approvalPath, JSON.stringify(validatedApproval, null, 2), 'utf-8');

  manifest.artifacts[`approval:${validatedApproval.stage}`] = path.relative(input.projectDir, approvalPath);
  const newRevision = computeCanonicalRevision(manifest);
  await fs.writeFile(manifestPath, JSON.stringify(manifest, null, 2), 'utf-8');

  return okResult({
    approvalRef: path.relative(input.projectDir, approvalPath),
    revision: newRevision
  });
}
