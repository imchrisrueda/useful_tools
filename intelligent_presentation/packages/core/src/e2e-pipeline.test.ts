import test from 'node:test';
import assert from 'node:assert/strict';
import os from 'node:os';
import path from 'node:path';
import fs from 'node:fs/promises';
import { createProject } from './project-ops.js';
import { generateSlides, updateSlide } from './slide-ops.js';
import { validatePresentation, exportPresentation } from './quality-ops.js';

test('T-018, T-020, T-021, T-022: full slide generation, incremental update, validation and export', async () => {
  const tempDir = await fs.mkdtemp(path.join(os.tmpdir(), 'ips-e2e-'));
  try {
    // 1. Create project
    const projRes = await createProject({
      name: 'scientific-study',
      root: tempDir,
      config: {
        language: 'es',
        aspectRatio: '16/9',
        engineId: 'slidev',
        title: 'Estudio Científico IPS'
      }
    });
    assert.equal(projRes.ok, true);
    if (!projRes.ok) return;

    const { projectDir, revision: rev0 } = projRes.value;

    // 2. Generate slides (T-018)
    const slidesRes = await generateSlides({
      projectDir,
      expectedRevision: rev0,
      storyboardRevision: rev0,
      designRef: 'theme-clean-dark',
      proposal: [
        {
          slideId: 'slide-01',
          markdown: '# Portada\n\nEstudio genómico experimental\n\n<!-- notas: Presentar objetivo -->',
          resourceIds: []
        },
        {
          slideId: 'slide-02',
          markdown: '# Metodología\n\nSecuenciación y análisis estadístico\n\n<!-- notas: Detallar cohortes -->',
          resourceIds: ['dataset-01']
        }
      ]
    });

    assert.equal(slidesRes.ok, true);
    if (!slidesRes.ok) return;

    // Check entrypoint slides.md created
    const entryRaw = await fs.readFile(path.join(projectDir, 'slides.md'), 'utf-8');
    assert.ok(entryRaw.includes('src: ./slides/01-slide-01.md'));
    assert.ok(entryRaw.includes('src: ./slides/02-slide-02.md'));

    const rev1 = slidesRes.value.revision;

    // 3. Incremental update slide (T-020)
    const updateRes = await updateSlide({
      projectDir,
      expectedRevision: rev1,
      slideId: 'slide-01',
      change: {
        markdown: '# Portada Actualizada\n\nEstudio genómico experimental v2\n\n<!-- notas: Nueva intro -->'
      },
      semanticChange: false
    });

    assert.equal(updateRes.ok, true);
    if (!updateRes.ok) return;
    const rev2 = updateRes.value.revision;
    assert.deepEqual(updateRes.value.preservedSlideIds, ['slide-02']);

    // 4. Validate presentation (T-021)
    const valRes = await validatePresentation({
      projectDir,
      revision: rev2,
      profile: 'mvp'
    });

    assert.equal(valRes.ok, true);
    if (!valRes.ok) return;
    assert.equal(valRes.value.result, 'pass');
    assert.equal(valRes.value.inspected.length, 2);

    // 5. Export presentation (T-022)
    const expRes = await exportPresentation({
      projectDir,
      revision: rev2,
      format: 'web',
      validationReportId: valRes.value.id,
      deliveryApprovalId: 'app-delivery-001'
    });

    assert.equal(expRes.ok, true);
    if (!expRes.ok) return;
    assert.equal(expRes.value.files.length, 1);
    assert.ok(expRes.value.files[0].path.endsWith('-bundle.html'));
    assert.ok(expRes.value.files[0].path.includes(projRes.value.projectId));
  } finally {
    await fs.rm(tempDir, { recursive: true, force: true });
  }
});
