import fs from 'node:fs/promises';
import path from 'node:path';
import {
  ProjectManifest,
  ProjectManifestSchema,
  Result,
  okResult,
  errorResult
} from './models.js';
import { assertPathConfinement, isValidProjectName } from './fs-utils.js';
import { computeCanonicalRevision } from './hash-utils.js';

export interface CreateProjectInput {
  name: string;
  root: string;
  config: {
    language: string;
    aspectRatio: '16/9';
    engineId: 'slidev';
    title?: string;
  };
}

export interface CreateProjectOutput {
  projectId: string;
  projectDir: string;
  manifestPath: string;
  revision: string;
}

export async function createProject(input: CreateProjectInput): Promise<Result<CreateProjectOutput>> {
  // 1. Validations
  if (!isValidProjectName(input.name)) {
    return errorResult({
      code: 'INVALID_INPUT',
      message: `Invalid project name "${input.name}". Must be non-empty and cannot contain path separators or illegal characters.`,
      recoverable: false
    });
  }

  let projectDir: string;
  try {
    projectDir = assertPathConfinement(input.root, input.name);
  } catch (err: unknown) {
    return errorResult({
      code: 'PATH_DENIED',
      message: (err as Error).message,
      recoverable: false
    });
  }

  // Check if destination exists
  try {
    await fs.access(projectDir);
    return errorResult({
      code: 'INVALID_INPUT',
      message: `Destination directory already exists: ${projectDir}`,
      recoverable: false
    });
  } catch {
    // Expected: directory does not exist yet
  }

  const projectId = `proj-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
  const stagingDir = path.resolve(input.root, `.staging-${projectId}`);

  try {
    // 2. Create staging directory structure
    await fs.mkdir(stagingDir, { recursive: true });
    await fs.mkdir(path.join(stagingDir, 'slides'), { recursive: true });
    await fs.mkdir(path.join(stagingDir, 'sources'), { recursive: true });
    await fs.mkdir(path.join(stagingDir, 'data'), { recursive: true });
    await fs.mkdir(path.join(stagingDir, 'assets'), { recursive: true });
    await fs.mkdir(path.join(stagingDir, 'reports'), { recursive: true });
    await fs.mkdir(path.join(stagingDir, 'outputs'), { recursive: true });

    // 3. Build ProjectManifest
    const manifestData: ProjectManifest = {
      schemaVersion: '1.0',
      projectId,
      title: input.config.title ?? input.name,
      language: input.config.language,
      aspectRatio: input.config.aspectRatio,
      engine: {
        id: input.config.engineId,
        version: '53.0.0'
      },
      artifacts: {
        manifest: 'project.json',
        slidesDir: 'slides',
        dataDir: 'data',
        assetsDir: 'assets',
        reportsDir: 'reports',
        outputsDir: 'outputs'
      },
      toolchain: {
        node: process.version,
        core: '0.1.0'
      }
    };

    // Validate with Zod
    const validatedManifest = ProjectManifestSchema.parse(manifestData);

    // Compute canonical revision
    const revision = computeCanonicalRevision(validatedManifest);

    // Write manifest to staging
    const stagingManifestPath = path.join(stagingDir, 'project.json');
    await fs.writeFile(stagingManifestPath, JSON.stringify(validatedManifest, null, 2), 'utf-8');

    // 4. Atomic rename staging -> projectDir
    await fs.rename(stagingDir, projectDir);

    const manifestPath = path.join(projectDir, 'project.json');

    return okResult({
      projectId,
      projectDir,
      manifestPath,
      revision
    });
  } catch (err: unknown) {
    // Clean up staging directory on error
    try {
      await fs.rm(stagingDir, { recursive: true, force: true });
    } catch {
      // Ignore cleanup error
    }

    return errorResult({
      code: 'INVALID_INPUT',
      message: `Failed to create project: ${(err as Error).message}`,
      recoverable: false
    });
  }
}
