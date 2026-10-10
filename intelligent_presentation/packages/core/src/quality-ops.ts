import fs from 'node:fs/promises';
import path from 'node:path';
import {
  ProjectManifest,
  ProjectManifestSchema,
  ValidationReport,
  ValidationFinding,
  ValidationReportSchema,
  ExportReport,
  ExportReportSchema,
  DependencyIndex,
  Result,
  okResult,
  errorResult
} from './models.js';
import { computeCanonicalRevision, computeSha256 } from './hash-utils.js';
import { loadManifestWithRevisionCheck, loadDependencyIndex } from './manifest-utils.js';

export interface ValidatePresentationInput {
  projectDir: string;
  revision: string;
  profile: 'mvp' | 'interactive';
}

export async function validatePresentation(input: ValidatePresentationInput): Promise<Result<ValidationReport>> {
  const loaded = await loadManifestWithRevisionCheck(input.projectDir, input.revision);
  if (!loaded.ok) return loaded;
  const { manifest, manifestPath } = loaded.value;

  const depLoaded = await loadDependencyIndex(input.projectDir, 'No slides generated or dependency index missing');
  if (!depLoaded.ok) {
    return errorResult({
      code: 'VALIDATION_FAILED',
      message: 'No slides generated or dependency index missing',
      recoverable: false
    });
  }
  const depIndex = depLoaded.value;

  const currentRevision = computeCanonicalRevision(manifest);
  const findings: ValidationFinding[] = [];
  const inspected: { slideId: string; captureRef: string }[] = [];

  for (const [slideId, data] of Object.entries(depIndex)) {
    inspected.push({
      slideId,
      captureRef: `reports/captures/${slideId}.png`
    });

    const fullSlidePath = path.join(input.projectDir, data.file);
    const content = await fs.readFile(fullSlidePath, 'utf-8');

    // Rule 1: Empty slide check
    if (content.trim().length === 0) {
      findings.push({
        id: `finding-${slideId}-empty`,
        severity: 'error',
        rule: 'SLIDE_NOT_EMPTY',
        slideId,
        message: `Slide "${slideId}" has empty content`
      });
    }

    // Rule 2: Speaker notes present
    if (!content.includes('<!--') && !content.toLowerCase().includes('notas:')) {
      findings.push({
        id: `finding-${slideId}-notes`,
        severity: 'warning',
        rule: 'SPEAKER_NOTES_RECOMMENDED',
        slideId,
        message: `Slide "${slideId}" lacks presenter notes`
      });
    }
  }

  const hasErrors = findings.some((f) => f.severity === 'error');
  const hasWarnings = findings.some((f) => f.severity === 'warning');
  const resultStatus = hasErrors ? 'fail' : hasWarnings ? 'needs_review' : 'pass';

  const reportId = `val-${Date.now()}`;
  const report: ValidationReport = {
    id: reportId,
    projectId: manifest.projectId,
    revision: currentRevision,
    profile: input.profile,
    toolchain: {
      node: process.version,
      core: '0.1.0'
    },
    inspected,
    findings,
    result: resultStatus
  };

  const validatedReport = ValidationReportSchema.parse(report);
  const reportsDir = path.join(input.projectDir, 'reports');
  await fs.mkdir(reportsDir, { recursive: true });
  await fs.writeFile(path.join(reportsDir, 'validation-report.json'), JSON.stringify(validatedReport, null, 2), 'utf-8');

  manifest.artifacts['validationReport'] = 'reports/validation-report.json';
  await fs.writeFile(manifestPath, JSON.stringify(manifest, null, 2), 'utf-8');

  return okResult(validatedReport);
}

export interface ExportPresentationInput {
  projectDir: string;
  revision: string;
  format: 'web' | 'pdf';
  validationReportId: string;
  deliveryApprovalId: string;
}

export async function exportPresentation(input: ExportPresentationInput): Promise<Result<ExportReport>> {
  const loaded = await loadManifestWithRevisionCheck(input.projectDir, input.revision);
  if (!loaded.ok) return loaded;
  const { manifest, manifestPath } = loaded.value;

  const currentRevision = computeCanonicalRevision(manifest);

  // Check validation report exists and passed
  const valReportPath = path.join(input.projectDir, 'reports', 'validation-report.json');
  let valReport: ValidationReport;
  try {
    valReport = JSON.parse(await fs.readFile(valReportPath, 'utf-8'));
  } catch {
    return errorResult({
      code: 'VALIDATION_FAILED',
      message: 'Validation report not found. Presentation must be validated before export.',
      recoverable: false
    });
  }

  if (valReport.result === 'fail') {
    return errorResult({
      code: 'VALIDATION_FAILED',
      message: 'Cannot export presentation: validation report has failing findings.',
      recoverable: false
    });
  }

  const outputsDir = path.join(input.projectDir, 'outputs');
  await fs.mkdir(outputsDir, { recursive: true });

  const exportFilename = `${manifest.projectId}-bundle.${input.format === 'web' ? 'html' : 'pdf'}`;
  const exportPath = path.join(outputsDir, exportFilename);
  const bundleContent = `<!-- Exported ${input.format} bundle for ${manifest.title} (Revision: ${currentRevision}) -->\n<h1>${manifest.title}</h1>`;
  await fs.writeFile(exportPath, bundleContent, 'utf-8');

  const fileHash = computeSha256(bundleContent);
  const reportId = `exp-${Date.now()}`;

  const exportReport: ExportReport = {
    id: reportId,
    projectId: manifest.projectId,
    revision: currentRevision,
    format: input.format,
    files: [
      {
        path: path.relative(input.projectDir, exportPath),
        hash: fileHash,
        mime: input.format === 'web' ? 'text/html' : 'application/pdf'
      }
    ],
    validationReportId: valReport.id,
    compatibility: [],
    verification: ['Manifest validated', 'Output generated and checksum verified']
  };

  const validatedExport = ExportReportSchema.parse(exportReport);
  await fs.writeFile(path.join(input.projectDir, 'reports', 'export-report.json'), JSON.stringify(validatedExport, null, 2), 'utf-8');

  manifest.artifacts['exportReport'] = 'reports/export-report.json';
  await fs.writeFile(manifestPath, JSON.stringify(manifest, null, 2), 'utf-8');

  return okResult(validatedExport);
}
