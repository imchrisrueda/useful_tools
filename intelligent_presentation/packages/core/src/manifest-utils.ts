import fs from 'node:fs/promises';
import path from 'node:path';
import {
  ProjectManifest,
  ProjectManifestSchema,
  DependencyIndex,
  Result,
  errorResult
} from './models.js';
import { computeCanonicalRevision } from './hash-utils.js';

/**
 * Loads project.json, parses it with Zod, and verifies the revision matches.
 * Consolidates the ~15-line pattern repeated across 8 call sites.
 */
export async function loadManifestWithRevisionCheck(
  projectDir: string,
  expectedRevision: string
): Promise<Result<{ manifest: ProjectManifest; manifestPath: string }>> {
  const manifestPath = path.join(projectDir, 'project.json');
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
  if (currentRevision !== expectedRevision) {
    return errorResult({
      code: 'REVISION_CONFLICT',
      message: `Revision conflict: expected ${expectedRevision}, but current is ${currentRevision}`,
      recoverable: false
    });
  }

  return { ok: true, value: { manifest, manifestPath }, warnings: [] };
}

/**
 * Loads dependency-index.json from the project directory.
 * Consolidates the ~10-line pattern repeated across 5 call sites.
 */
export async function loadDependencyIndex(
  projectDir: string,
  errorMessage = 'No slides generated yet or dependency index missing'
): Promise<Result<DependencyIndex>> {
  const depIndexPath = path.join(projectDir, 'dependency-index.json');
  try {
    const raw = await fs.readFile(depIndexPath, 'utf-8');
    return { ok: true, value: JSON.parse(raw) as DependencyIndex, warnings: [] };
  } catch {
    return errorResult({
      code: 'INVALID_INPUT',
      message: errorMessage,
      recoverable: false
    });
  }
}

/**
 * Invalidates validation/export report artifacts, recomputes revision, and saves manifest.
 * Consolidates the pattern repeated across slide-ops, animation-ops, interaction-ops.
 */
export async function invalidateReportsAndSave(
  manifest: ProjectManifest,
  manifestPath: string
): Promise<string> {
  delete manifest.artifacts['validationReport'];
  delete manifest.artifacts['exportReport'];
  const newRevision = computeCanonicalRevision(manifest);
  await fs.writeFile(manifestPath, JSON.stringify(manifest, null, 2), 'utf-8');
  return newRevision;
}

/**
 * Saves the dependency index back to disk.
 */
export async function saveDependencyIndex(
  projectDir: string,
  depIndex: DependencyIndex
): Promise<void> {
  const depIndexPath = path.join(projectDir, 'dependency-index.json');
  await fs.writeFile(depIndexPath, JSON.stringify(depIndex, null, 2), 'utf-8');
}
