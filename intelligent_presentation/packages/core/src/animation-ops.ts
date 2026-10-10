import fs from 'node:fs/promises';
import path from 'node:path';
import {
  Result,
  okResult
} from './models.js';
import {
  loadManifestWithRevisionCheck,
  loadDependencyIndex,
  invalidateReportsAndSave,
  saveDependencyIndex
} from './manifest-utils.js';

export interface AddAnimationInput {
  projectDir: string;
  expectedRevision: string;
  slideId: string;
  animation: {
    provider: string;
    sceneRef: string;
    parameters: Record<string, unknown>;
    segmentIds: string[];
    staticStateId: string;
    altText: string;
  };
}

export interface AddAnimationOutput {
  resourceIds: string[];
  controlManifestRef: string;
  staticFallbackRef: string;
  revision: string;
}

export async function addAnimation(input: AddAnimationInput): Promise<Result<AddAnimationOutput>> {
  const loaded = await loadManifestWithRevisionCheck(input.projectDir, input.expectedRevision);
  if (!loaded.ok) return loaded;
  const { manifest, manifestPath } = loaded.value;

  const depLoaded = await loadDependencyIndex(input.projectDir, 'Slides must be generated before adding animations');
  if (!depLoaded.ok) return depLoaded;
  const depIndex = depLoaded.value;

  if (!depIndex[input.slideId]) {
    return { ok: false, error: {
      code: 'INVALID_INPUT',
      message: `Slide ID "${input.slideId}" not found in project`,
      recoverable: false
    }};
  }

  const assetsDir = path.join(input.projectDir, 'assets');
  await fs.mkdir(assetsDir, { recursive: true });

  const controlManifestRef = `assets/animation-${input.slideId}-manifest.json`;
  const staticFallbackRef = `assets/animation-${input.slideId}-fallback.svg`;

  // Write control manifest for Slidev interactive player
  const manifestData = {
    slideId: input.slideId,
    provider: input.animation.provider,
    sceneRef: input.animation.sceneRef,
    parameters: input.animation.parameters,
    segmentIds: input.animation.segmentIds,
    staticStateId: input.animation.staticStateId,
    altText: input.animation.altText,
    controls: {
      stepForward: 'Right / Click',
      stepBackward: 'Left',
      playPause: 'Space',
      reset: 'R'
    }
  };

  await fs.writeFile(path.join(input.projectDir, controlManifestRef), JSON.stringify(manifestData, null, 2), 'utf-8');

  // Write fallback poster for PDF export
  const fallbackSvg = `<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 800 450" width="800" height="450">
  <rect width="100%" height="100%" fill="#090d16" />
  <text x="400" y="200" font-family="sans-serif" font-size="22" font-weight="bold" fill="#38bdf8" text-anchor="middle">Animación Científica (${input.animation.provider})</text>
  <text x="400" y="250" font-family="sans-serif" font-size="14" fill="#9ca3af" text-anchor="middle">${input.animation.altText}</text>
</svg>`;
  await fs.writeFile(path.join(input.projectDir, staticFallbackRef), fallbackSvg, 'utf-8');

  // Register resources in slide
  const animResourceId = `anim-${input.slideId}`;
  if (!depIndex[input.slideId].resources.includes(animResourceId)) {
    depIndex[input.slideId].resources.push(animResourceId);
  }
  await saveDependencyIndex(input.projectDir, depIndex);

  const newRevision = await invalidateReportsAndSave(manifest, manifestPath);

  return okResult({
    resourceIds: [animResourceId],
    controlManifestRef,
    staticFallbackRef,
    revision: newRevision
  });
}
