import test from 'node:test';
import assert from 'node:assert/strict';
import os from 'node:os';
import path from 'node:path';
import fs from 'node:fs/promises';
import { createProject } from './project-ops.js';
import { applyRequirements, applyStoryboard, recordApproval } from './state-ops.js';
import { parseScientificCsv } from './csv-ops.js';
import { renderSvgChart } from './chart-ops.js';

test('T-015 lifecycle: apply requirements, storyboard, and record approval', async () => {
  const tempDir = await fs.mkdtemp(path.join(os.tmpdir(), 'ips-lifecycle-'));
  try {
    const projRes = await createProject({
      name: 'lifecycle-demo',
      root: tempDir,
      config: {
        language: 'es',
        aspectRatio: '16/9',
        engineId: 'slidev',
        title: 'Lifecycle Demo'
      }
    });

    assert.equal(projRes.ok, true);
    if (!projRes.ok) return;

    const { projectDir, revision: rev0 } = projRes.value;

    // 1. Apply requirements
    const reqRes = await applyRequirements({
      projectDir,
      expectedRevision: rev0,
      request: 'Presentar avance científico',
      authorizedSourceIds: ['src-paper-1'],
      proposal: {
        schemaVersion: '1.0',
        projectId: projRes.value.projectId,
        objective: 'Divulgación científica',
        audience: 'Comité evaluador',
        durationMinutes: 15,
        requiredTopics: ['Introducción', 'Resultados', 'Conclusión'],
        outputs: ['web', 'pdf'],
        constraints: ['15 slides'],
        unresolvedQuestions: []
      }
    });

    assert.equal(reqRes.ok, true);
    if (!reqRes.ok) return;
    const rev1 = reqRes.value.revision;
    assert.notEqual(rev1, rev0);

    // 2. Conflict test on outdated expectedRevision
    const conflictRes = await applyRequirements({
      projectDir,
      expectedRevision: rev0,
      request: 'Otro cambio',
      authorizedSourceIds: [],
      proposal: {
        schemaVersion: '1.0',
        projectId: projRes.value.projectId,
        objective: 'Otro',
        audience: 'Otro',
        durationMinutes: 10,
        requiredTopics: [],
        outputs: ['web'],
        constraints: [],
        unresolvedQuestions: []
      }
    });
    assert.equal(conflictRes.ok, false);
    if (!conflictRes.ok) {
      assert.equal(conflictRes.error.code, 'REVISION_CONFLICT');
    }

    // 3. Apply storyboard
    const sbRes = await applyStoryboard({
      projectDir,
      expectedRevision: rev1,
      requirementsRevision: rev1,
      sourceIds: ['src-paper-1'],
      proposal: {
        schemaVersion: '1.0',
        projectId: projRes.value.projectId,
        requirementsRevision: rev1,
        slides: [
          {
            slideId: 'slide-intro',
            purpose: 'Introducción del estudio',
            message: 'Contexto y motivación científica',
            sourceIds: ['src-paper-1'],
            assetIds: [],
            estimatedSeconds: 60
          }
        ],
        unresolvedItems: []
      }
    });

    assert.equal(sbRes.ok, true);
    if (!sbRes.ok) return;
    const rev2 = sbRes.value.revision;

    // 4. Record human approval
    const appRes = await recordApproval({
      projectDir,
      expectedRevision: rev2,
      approval: {
        id: 'app-001',
        projectId: projRes.value.projectId,
        stage: 'storyboard',
        revision: rev2,
        decision: 'approved',
        actor: 'Dra. Investigadora',
        timestamp: new Date().toISOString(),
        scope: 'storyboard complete'
      }
    });

    assert.equal(appRes.ok, true);
  } finally {
    await fs.rm(tempDir, { recursive: true, force: true });
  }
});

test('T-016 CSV: parses valid CSV and rejects irregular/corrupted rows (TEST-004)', () => {
  const validCsv = `category,measurement,p_value
GroupA,14.5,0.01
GroupB,22.8,0.002
GroupC,35.1,0.0001`;

  const parsed = parseScientificCsv(validCsv);
  assert.equal(parsed.ok, true);
  if (parsed.ok) {
    assert.deepEqual(parsed.value.headers, ['category', 'measurement', 'p_value']);
    assert.equal(parsed.value.rows.length, 3);
    assert.equal(parsed.value.rows[0].measurement, '14.5');
  }

  // Reject duplicate headers
  const dupHeader = `cat,val,val\nA,1,2`;
  const dupRes = parseScientificCsv(dupHeader);
  assert.equal(dupRes.ok, false);
  if (!dupRes.ok) assert.equal(dupRes.error.code, 'INVALID_INPUT');

  // Reject irregular column count
  const irregular = `cat,val\nA,1\nB,2,extra`;
  const irregRes = parseScientificCsv(irregular);
  assert.equal(irregRes.ok, false);
  if (!irregRes.ok) assert.equal(irregRes.error.code, 'INVALID_INPUT');

  // Reject NaN / empty cell
  const nanCell = `cat,val\nA,NaN`;
  const nanRes = parseScientificCsv(nanCell);
  assert.equal(nanRes.ok, false);
  if (!nanRes.ok) assert.equal(nanRes.error.code, 'INVALID_INPUT');
});

test('T-017 Chart: renders reproducible SVG and includes data/spec hashes (TEST-005)', () => {
  const csv = `group,score\nControl,12.5\nTreatment,28.4`;
  const parsedCsv = parseScientificCsv(csv);
  assert.equal(parsedCsv.ok, true);
  if (!parsedCsv.ok) return;

  const chartRes = renderSvgChart({
    chartSpec: {
      id: 'fig-1',
      datasetId: 'ds-01',
      columns: { x: 'group', y: 'score' },
      units: { score: 'µg/L' },
      type: 'bar',
      transform: { kind: 'identity' },
      missingPolicy: 'reject',
      caption: 'Eficacia de tratamiento',
      altText: 'Gráfico de barras comparando Control y Treatment'
    },
    dataset: parsedCsv.value
  });

  assert.equal(chartRes.ok, true);
  if (chartRes.ok) {
    assert.ok(chartRes.value.svgContent.includes('<svg'));
    assert.ok(chartRes.value.svgContent.includes('Eficacia de tratamiento'));
    assert.ok(chartRes.value.svgContent.includes('µg/L'));
    assert.equal(typeof chartRes.value.dataHash, 'string');
    assert.equal(chartRes.value.dataHash.length, 64);
    assert.equal(chartRes.value.specHash.length, 64);
  }
});
