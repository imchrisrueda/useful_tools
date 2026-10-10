import test from 'node:test';
import assert from 'node:assert/strict';
import {
  THEME_CATALOG,
  suggestDesigns,
  calculateThemeScore
} from './design-ops.js';

test('Phase 2 Catalog: all 5 design families registered with valid manifests and layouts', () => {
  const families = [
    'scientific-minimal',
    'tech-keynote',
    'data-storytelling',
    'interactive-workshop',
    'executive-professional'
  ];

  for (const f of families) {
    const theme = THEME_CATALOG[f];
    assert.ok(theme, `Theme "${f}" must exist in catalog`);
    assert.equal(theme.id, f);
    assert.ok(theme.layouts.length >= 7, `${f} must have at least 7 layouts`);
    assert.ok(theme.components.length >= 5, `${f} must have at least 5 reusable components`);
    assert.ok(theme.capabilities.includes('web'));
    assert.ok(theme.capabilities.includes('pdf'));
  }
});

test('Phase 2 Recommendation: suggests top 3 comparable proposals based on audience and data density', () => {
  // Scientific audience recommendation
  const scientificRes = suggestDesigns({
    projectId: 'p-01',
    revision: 'rev-01',
    storyboardRevision: 'rev-01',
    formats: ['web', 'pdf'],
    requestedCount: 3,
    criteria: {
      audience: 'cientifica',
      presentationTone: 'riguroso',
      dataDensity: 'alta'
    }
  });

  assert.equal(scientificRes.ok, true);
  if (scientificRes.ok) {
    assert.equal(scientificRes.value.proposals.length, 3);
    // Scientific Minimal should be ranked first
    assert.equal(scientificRes.value.proposals[0].designId, 'scientific-minimal');
    assert.ok(scientificRes.value.proposals[0].score > 80);
    // Each proposal provides identical comparison preview set
    for (const p of scientificRes.value.proposals) {
      assert.equal(p.previews.length, 3);
      assert.ok(p.previews.some((prev) => prev.includes('-cover.png')));
      assert.ok(p.previews.some((prev) => prev.includes('-dense.png')));
      assert.ok(p.previews.some((prev) => prev.includes('-chart.png')));
    }
  }

  // Executive audience recommendation
  const execRes = suggestDesigns({
    projectId: 'p-02',
    revision: 'rev-02',
    storyboardRevision: 'rev-02',
    formats: ['web', 'pdf'],
    requestedCount: 3,
    criteria: {
      audience: 'ejecutiva',
      presentationTone: 'institucional'
    }
  });

  assert.equal(execRes.ok, true);
  if (execRes.ok) {
    assert.equal(execRes.value.proposals[0].designId, 'executive-professional');
  }

  // Interactive workshop recommendation
  const workshopRes = suggestDesigns({
    projectId: 'p-03',
    revision: 'rev-03',
    storyboardRevision: 'rev-03',
    formats: ['web', 'pdf'],
    requestedCount: 3,
    criteria: {
      audience: 'estudiantes',
      presentationTone: 'didactico'
    }
  });

  assert.equal(workshopRes.ok, true);
  if (workshopRes.ok) {
    assert.equal(workshopRes.value.proposals[0].designId, 'interactive-workshop');
  }
});
