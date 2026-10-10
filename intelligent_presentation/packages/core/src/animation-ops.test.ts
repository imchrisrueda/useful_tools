import test from 'node:test';
import assert from 'node:assert/strict';
import os from 'node:os';
import path from 'node:path';
import fs from 'node:fs/promises';
import { createProject } from './project-ops.js';
import { generateSlides } from './slide-ops.js';
import { addAnimation } from './animation-ops.js';

test('Phase 3 addAnimation applies multi-segment animation and static poster fallback', async () => {
  const tempDir = await fs.mkdtemp(path.join(os.tmpdir(), 'ips-anim-'));
  try {
    const projRes = await createProject({
      name: 'anim-demo',
      root: tempDir,
      config: {
        language: 'es',
        aspectRatio: '16/9',
        engineId: 'slidev',
        title: 'Anim Demo'
      }
    });

    assert.equal(projRes.ok, true);
    if (!projRes.ok) return;

    const { projectDir, revision: rev0 } = projRes.value;

    const slidesRes = await generateSlides({
      projectDir,
      expectedRevision: rev0,
      storyboardRevision: rev0,
      designRef: 'scientific-minimal',
      proposal: [
        {
          slideId: 'slide-dyn',
          markdown: '# Dinámica Molecular\n\n<!-- notas: Explicación de simulación -->',
          resourceIds: []
        }
      ]
    });
    assert.equal(slidesRes.ok, true);
    if (!slidesRes.ok) return;

    const rev1 = slidesRes.value.revision;

    // Add animation
    const animRes = await addAnimation({
      projectDir,
      expectedRevision: rev1,
      slideId: 'slide-dyn',
      animation: {
        provider: 'ips-scientific-animator',
        sceneRef: 'python/animation_generator.py:AnimationGenerator',
        parameters: { steps: 3, temperature_K: 300 },
        segmentIds: ['step-1-basal', 'step-2-activation', 'step-3-response'],
        staticStateId: 'step-3-response',
        altText: 'Transición molecular en 3 estados cinéticos'
      }
    });

    assert.equal(animRes.ok, true);
    if (!animRes.ok) return;

    assert.ok(animRes.value.controlManifestRef.includes('manifest.json'));
    assert.ok(animRes.value.staticFallbackRef.includes('fallback.svg'));

    // Verify files created on disk
    await fs.access(path.join(projectDir, animRes.value.controlManifestRef));
    await fs.access(path.join(projectDir, animRes.value.staticFallbackRef));
  } finally {
    await fs.rm(tempDir, { recursive: true, force: true });
  }
});
