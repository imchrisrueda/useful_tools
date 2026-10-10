import { z } from 'zod';

export const IdSchema = z.string().min(1, 'Id must not be empty');
export type Id = z.infer<typeof IdSchema>;

export const Sha256Schema = z.string().regex(/^[a-f0-9]{64}$/i, 'Must be a 64-character SHA-256 hash');
export type Sha256 = z.infer<typeof Sha256Schema>;

export const IsoDateTimeSchema = z.string().datetime({ message: 'Must be an ISO 8601 UTC date string' });
export type IsoDateTime = z.infer<typeof IsoDateTimeSchema>;

export const FormatSchema = z.enum([
  'web',
  'pdf',
  'pptx',
  'pptx-editable',
  'png',
  'video'
]);
export type Format = z.infer<typeof FormatSchema>;

export const OperationErrorCodeSchema = z.enum([
  'INVALID_INPUT',
  'PATH_DENIED',
  'DEPENDENCY_MISSING',
  'REVISION_CONFLICT',
  'APPROVAL_REQUIRED',
  'CAPABILITY_UNSUPPORTED',
  'RENDER_FAILED',
  'VALIDATION_FAILED',
  'SOURCE_UNAUTHORIZED'
]);
export type OperationErrorCode = z.infer<typeof OperationErrorCodeSchema>;

export const OperationErrorSchema = z.object({
  code: OperationErrorCodeSchema,
  message: z.string().min(1),
  recoverable: z.boolean(),
  details: z.record(z.unknown()).optional()
});
export type OperationError = z.infer<typeof OperationErrorSchema>;

export type Result<T> =
  | { ok: true; value: T; revision?: string; warnings: string[] }
  | { ok: false; error: OperationError };

export function okResult<T>(value: T, revision?: string, warnings: string[] = []): Result<T> {
  return { ok: true, value, revision, warnings };
}

export function errorResult<T = never>(error: OperationError): Result<T> {
  return { ok: false, error };
}

export const ProjectManifestSchema = z.object({
  schemaVersion: z.string().regex(/^\d+\.\d+$/),
  projectId: IdSchema,
  title: z.string().min(1),
  language: z.string().min(2),
  aspectRatio: z.string().default('16/9'),
  engine: z.object({
    id: z.string().min(1),
    version: z.string().min(1)
  }),
  designRef: IdSchema.optional(),
  artifacts: z.record(z.string()),
  toolchain: z.record(z.string())
});
export type ProjectManifest = z.infer<typeof ProjectManifestSchema>;

export const RequirementsSchema = z.object({
  schemaVersion: z.string().regex(/^\d+\.\d+$/),
  projectId: IdSchema,
  objective: z.string().min(1),
  audience: z.string().min(1),
  durationMinutes: z.number().positive(),
  requiredTopics: z.array(z.string()),
  outputs: z.array(FormatSchema),
  constraints: z.array(z.string()),
  unresolvedQuestions: z.array(z.string())
});
export type Requirements = z.infer<typeof RequirementsSchema>;

export const StoryboardSlideSchema = z.object({
  slideId: IdSchema,
  purpose: z.string().min(1),
  message: z.string().min(1),
  sourceIds: z.array(IdSchema),
  assetIds: z.array(IdSchema),
  estimatedSeconds: z.number().nonnegative()
});
export type StoryboardSlide = z.infer<typeof StoryboardSlideSchema>;

export const StoryboardSchema = z.object({
  schemaVersion: z.string().regex(/^\d+\.\d+$/),
  projectId: IdSchema,
  requirementsRevision: z.string().min(1),
  slides: z.array(StoryboardSlideSchema),
  unresolvedItems: z.array(z.string())
});
export type Storyboard = z.infer<typeof StoryboardSchema>;

export const SourceRecordSchema = z.object({
  id: IdSchema,
  kind: z.enum(['document', 'dataset', 'url', 'asset']),
  location: z.string().min(1),
  hash: Sha256Schema.optional(),
  provenance: z.string().min(1),
  license: z.string().min(1),
  privacy: z.enum(['public', 'private', 'restricted']),
  uses: z.array(z.object({
    slideId: IdSchema,
    locator: z.string().optional()
  }))
});
export type SourceRecord = z.infer<typeof SourceRecordSchema>;

export const ChartSpecSchema = z.object({
  id: IdSchema,
  datasetId: IdSchema,
  columns: z.object({
    x: z.string().min(1),
    y: z.string().min(1)
  }),
  units: z.record(z.string()),
  type: z.enum(['bar', 'line']),
  transform: z.object({
    kind: z.literal('identity')
  }),
  missingPolicy: z.literal('reject'),
  caption: z.string().min(1),
  altText: z.string().min(1)
});
export type ChartSpec = z.infer<typeof ChartSpecSchema>;

export const FontDescriptorSchema = z.object({
  family: z.string().min(1),
  assetRef: z.string().min(1),
  license: z.string().min(1)
});
export type FontDescriptor = z.infer<typeof FontDescriptorSchema>;

export const DesignManifestSchema = z.object({
  id: IdSchema,
  version: z.string().min(1),
  family: z.string().min(1),
  tokens: z.record(z.unknown()),
  layouts: z.array(z.string()),
  components: z.array(z.string()),
  fonts: z.array(FontDescriptorSchema),
  capabilities: z.array(FormatSchema),
  license: z.string().min(1)
});
export type DesignManifest = z.infer<typeof DesignManifestSchema>;

export const FindingSeveritySchema = z.enum(['info', 'warning', 'error']);
export type FindingSeverity = z.infer<typeof FindingSeveritySchema>;

export const ValidationFindingSchema = z.object({
  id: IdSchema,
  severity: FindingSeveritySchema,
  rule: z.string().min(1),
  slideId: IdSchema.optional(),
  stateId: IdSchema.optional(),
  evidenceRef: z.string().optional(),
  message: z.string().min(1)
});
export type ValidationFinding = z.infer<typeof ValidationFindingSchema>;

export const ValidationReportSchema = z.object({
  id: IdSchema,
  projectId: IdSchema,
  revision: z.string().min(1),
  profile: z.string().min(1),
  toolchain: z.record(z.string()),
  inspected: z.array(z.object({
    slideId: IdSchema,
    stateId: IdSchema.optional(),
    captureRef: z.string().min(1)
  })),
  findings: z.array(ValidationFindingSchema),
  result: z.enum(['pass', 'fail', 'needs_review'])
});
export type ValidationReport = z.infer<typeof ValidationReportSchema>;

export const ExportReportSchema = z.object({
  id: IdSchema,
  projectId: IdSchema,
  revision: z.string().min(1),
  format: FormatSchema,
  files: z.array(z.object({
    path: z.string().min(1),
    hash: Sha256Schema,
    mime: z.string().min(1)
  })),
  validationReportId: IdSchema,
  compatibility: z.array(z.object({
    elementId: IdSchema.optional(),
    loss: z.string().min(1),
    reason: z.string().min(1)
  })),
  verification: z.array(z.string())
});
export type ExportReport = z.infer<typeof ExportReportSchema>;

export const ApprovalStageSchema = z.enum([
  'requirements',
  'storyboard',
  'design',
  'slides',
  'export'
]);
export type ApprovalStage = z.infer<typeof ApprovalStageSchema>;

export const ApprovalRecordSchema = z.object({
  id: IdSchema,
  projectId: IdSchema,
  stage: ApprovalStageSchema,
  revision: z.string().min(1),
  decision: z.enum(['approved', 'changes_requested']),
  actor: z.string().min(1),
  timestamp: IsoDateTimeSchema,
  scope: z.string().min(1)
});
export type ApprovalRecord = z.infer<typeof ApprovalRecordSchema>;

export const ProjectStateSchema = z.object({
  projectId: IdSchema,
  revision: z.string().min(1),
  stage: z.string().min(1),
  status: z.enum(['draft', 'in_review', 'approved', 'rejected', 'failed']),
  artifactRevisions: z.record(z.string()),
  approvals: z.array(IdSchema),
  lastError: OperationErrorSchema.optional()
});
export type ProjectState = z.infer<typeof ProjectStateSchema>;

export interface DependencyIndexEntry {
  file: string;
  hash: string;
  resources: string[];
}

export type DependencyIndex = Record<string, DependencyIndexEntry>;
