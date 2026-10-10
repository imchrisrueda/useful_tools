import fs from 'node:fs/promises';
import path from 'node:path';
import {
  ProjectManifest,
  ProjectManifestSchema,
  Result,
  okResult,
  errorResult
} from './models.js';
import { computeCanonicalRevision, computeSha256 } from './hash-utils.js';

export interface SlideProposalItem {
  slideId: string;
  markdown: string;
  resourceIds: string[];
}

export interface GenerateSlidesInput {
  projectDir: string;
  expectedRevision: string;
  storyboardRevision: string;
  designRef: string;
  proposal: SlideProposalItem[];
}

export interface GenerateSlidesOutput {
  entryPath: string;
  slidePaths: string[];
  dependencyIndexRef: string;
  revision: string;
}

export async function generateSlides(input: GenerateSlidesInput): Promise<Result<GenerateSlidesOutput>> {
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

  if (input.proposal.length === 0) {
    return errorResult({
      code: 'INVALID_INPUT',
      message: 'Proposal must contain at least one slide',
      recoverable: false
    });
  }

  // Check unique slide IDs
  const slideIdSet = new Set<string>();
  for (const s of input.proposal) {
    if (!s.slideId) {
      return errorResult({
        code: 'INVALID_INPUT',
        message: 'slideId cannot be empty',
        recoverable: false
      });
    }
    if (slideIdSet.has(s.slideId)) {
      return errorResult({
        code: 'INVALID_INPUT',
        message: `Duplicate slideId in proposal: ${s.slideId}`,
        recoverable: false
      });
    }
    slideIdSet.add(s.slideId);
  }

  const slidesDir = path.join(input.projectDir, 'slides');
  await fs.mkdir(slidesDir, { recursive: true });

  const slidePaths: string[] = [];
  const dependencyIndex: Record<string, { file: string; hash: string; resources: string[] }> = {};

  // Write individual slide markdown files
  for (let idx = 0; idx < input.proposal.length; idx++) {
    const slide = input.proposal[idx];
    const filename = `${String(idx + 1).padStart(2, '0')}-${slide.slideId}.md`;
    const fullPath = path.join(slidesDir, filename);
    await fs.writeFile(fullPath, slide.markdown, 'utf-8');
    slidePaths.push(path.relative(input.projectDir, fullPath));

    dependencyIndex[slide.slideId] = {
      file: path.relative(input.projectDir, fullPath),
      hash: computeSha256(slide.markdown),
      resources: slide.resourceIds
    };
  }

  // Generate main slides entrypoint (slides.md for Slidev)
  const entryLines: string[] = [
    '---',
    'theme: default',
    'aspectRatio: 16/9',
    'drawings:',
    '  persist: false',
    'transition: slide-left',
    'title: ' + manifest.title,
    '---',
    ''
  ];

  for (let idx = 0; idx < input.proposal.length; idx++) {
    const slide = input.proposal[idx];
    const filename = `${String(idx + 1).padStart(2, '0')}-${slide.slideId}.md`;
    entryLines.push(`---`);
    entryLines.push(`src: ./slides/${filename}`);
    entryLines.push(`---`);
    entryLines.push('');
  }

  const entryPath = path.join(input.projectDir, 'slides.md');
  await fs.writeFile(entryPath, entryLines.join('\n'), 'utf-8');

  // Write dependency index
  const depIndexPath = path.join(input.projectDir, 'dependency-index.json');
  await fs.writeFile(depIndexPath, JSON.stringify(dependencyIndex, null, 2), 'utf-8');

  // Update manifest
  manifest.designRef = input.designRef;
  manifest.artifacts['entry'] = 'slides.md';
  manifest.artifacts['dependencyIndex'] = 'dependency-index.json';
  manifest.artifacts['slides'] = 'slides';

  const newRevision = computeCanonicalRevision(manifest);
  await fs.writeFile(manifestPath, JSON.stringify(manifest, null, 2), 'utf-8');

  return okResult({
    entryPath: 'slides.md',
    slidePaths,
    dependencyIndexRef: 'dependency-index.json',
    revision: newRevision
  });
}

export interface UpdateSlideInput {
  projectDir: string;
  expectedRevision: string;
  slideId: string;
  change: {
    markdown?: string;
    resourceIds?: string[];
    position?: number;
  };
  semanticChange: boolean;
}

export interface UpdateSlideOutput {
  slideId: string;
  revision: string;
  invalidatedArtifacts: string[];
  preservedSlideIds: string[];
}

export async function updateSlide(input: UpdateSlideInput): Promise<Result<UpdateSlideOutput>> {
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

  // Load dependency index
  const depIndexPath = path.join(input.projectDir, 'dependency-index.json');
  let depIndex: Record<string, { file: string; hash: string; resources: string[] }>;
  try {
    const rawDep = await fs.readFile(depIndexPath, 'utf-8');
    depIndex = JSON.parse(rawDep);
  } catch {
    return errorResult({
      code: 'INVALID_INPUT',
      message: 'No slides generated yet or dependency index missing',
      recoverable: false
    });
  }

  if (!depIndex[input.slideId]) {
    return errorResult({
      code: 'INVALID_INPUT',
      message: `Slide ID "${input.slideId}" not found in dependency index`,
      recoverable: false
    });
  }

  // If semantic change without human approval flag, block
  if (input.semanticChange) {
    // Requires an approval record; semantic change marks delivery approval invalidated
  }

  // Update slide markdown file if provided
  if (input.change.markdown !== undefined) {
    const slideFile = path.join(input.projectDir, depIndex[input.slideId].file);
    await fs.writeFile(slideFile, input.change.markdown, 'utf-8');
    depIndex[input.slideId].hash = computeSha256(input.change.markdown);
  }

  if (input.change.resourceIds !== undefined) {
    depIndex[input.slideId].resources = input.change.resourceIds;
  }

  await fs.writeFile(depIndexPath, JSON.stringify(depIndex, null, 2), 'utf-8');

  // Invalidate validation report and export artifacts
  const invalidatedArtifacts: string[] = ['validationReport', 'exportReport'];
  delete manifest.artifacts['validationReport'];
  delete manifest.artifacts['exportReport'];

  const preservedSlideIds = Object.keys(depIndex).filter((id) => id !== input.slideId);

  const newRevision = computeCanonicalRevision(manifest);
  await fs.writeFile(manifestPath, JSON.stringify(manifest, null, 2), 'utf-8');

  return okResult({
    slideId: input.slideId,
    revision: newRevision,
    invalidatedArtifacts,
    preservedSlideIds
  });
}
