import test from 'node:test';
import assert from 'node:assert/strict';
import os from 'node:os';
import path from 'node:path';
import fs from 'node:fs/promises';
import { createProject } from './project-ops.js';
import { generateSlides } from './slide-ops.js';
import { addInteraction, REGISTERED_INTERACTIVE_COMPONENTS } from './interaction-ops.js';

test('Phase 3 addInteraction successfully adds interactive component, test states, and static export fallback', async () => {
  const tempDir = await fs.mkdtemp(path.join(os.tmpdir(), 'ips-inter-'));
  try {
    const projRes = await createProject({
      name: 'inter-demo',
      root: tempDir,
      config: {
        language: 'es',
        aspectRatio: '16/9',
        engineId: 'slidev',
        title: 'Interactive Demo'
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
          slideId: 'slide-plot',
          markdown: '# Gráfico Interactivo\n\n<slider-linear-response />\n\n<!-- notas: Manipulación en vivo -->',
          resourceIds: []
        }
      ]
    });
    assert.equal(slidesRes.ok, true);
    if (!slidesRes.ok) return;

    const rev1 = slidesRes.value.revision;

    // 1. Successful interaction addition
    const interRes = await addInteraction({
      projectDir,
      expectedRevision: rev1,
      slideId: 'slide-plot',
      interaction: {
        componentId: 'slider-linear-response',
        parameters: { a: 1.5, xMin: 0, xMax: 10 },
        testStates: ['a=0', 'a=1', 'a=1.5', 'a=2'],
        exportStateId: 'a=1',
        altText: 'Curva lineal con slider de pendiente'
      }
    });

    assert.equal(interRes.ok, true);
    if (!interRes.ok) return;

    assert.ok(interRes.value.componentRef.endsWith('.vue'));
    assert.ok(interRes.value.stateManifestRef.endsWith('.json'));
    assert.ok(interRes.value.staticFallbackRef.endsWith('.svg'));

    // Check files on disk
    await fs.access(path.join(projectDir, interRes.value.componentRef));
    await fs.access(path.join(projectDir, interRes.value.stateManifestRef));
    await fs.access(path.join(projectDir, interRes.value.staticFallbackRef));

    // 2. Reject unregistered component
    const badCompRes = await addInteraction({
      projectDir,
      expectedRevision: interRes.value.revision,
      slideId: 'slide-plot',
      interaction: {
        componentId: 'untrusted-script-runner',
        parameters: {},
        testStates: ['s0'],
        exportStateId: 's0',
        altText: 'hack'
      }
    });
    assert.equal(badCompRes.ok, false);
    if (!badCompRes.ok) {
      assert.equal(badCompRes.error.code, 'INVALID_INPUT');
      assert.match(badCompRes.error.message, /not in the trusted component registry/);
    }

    // 3. Reject parameter out of bounds
    const outOfBoundsRes = await addInteraction({
      projectDir,
      expectedRevision: interRes.value.revision,
      slideId: 'slide-plot',
      interaction: {
        componentId: 'slider-linear-response',
        parameters: { a: 5.0 }, // max is 2
        testStates: ['s0'],
        exportStateId: 's0',
        altText: 'invalid'
      }
    });
    assert.equal(outOfBoundsRes.ok, false);
    if (!outOfBoundsRes.ok) {
      assert.equal(outOfBoundsRes.error.code, 'INVALID_INPUT');
      assert.match(outOfBoundsRes.error.message, /exceeds maximum allowed/);
    }
  } finally {
    await fs.rm(tempDir, { recursive: true, force: true });
  }
});
