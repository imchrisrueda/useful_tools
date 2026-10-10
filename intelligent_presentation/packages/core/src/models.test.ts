import test from 'node:test';
import assert from 'node:assert/strict';
import {
  ProjectManifestSchema,
  RequirementsSchema,
  ChartSpecSchema,
  okResult,
  errorResult
} from './models.js';

test('ProjectManifestSchema accepts valid project manifest', () => {
  const validManifest = {
    schemaVersion: '1.0',
    projectId: 'proj-001',
    title: 'IPS Presentation',
    language: 'es',
    aspectRatio: '16/9',
    engine: {
      id: 'slidev',
      version: '53.0.0'
    },
    artifacts: {
      requirements: 'requirements.json',
      storyboard: 'storyboard.json'
    },
    toolchain: {
      node: '20.11.0'
    }
  };

  const parsed = ProjectManifestSchema.parse(validManifest);
  assert.equal(parsed.projectId, 'proj-001');
  assert.equal(parsed.engine.id, 'slidev');
});

test('RequirementsSchema rejects non-positive duration', () => {
  const invalidReqs = {
    schemaVersion: '1.0',
    projectId: 'proj-001',
    objective: 'Test',
    audience: 'Audience',
    durationMinutes: 0,
    requiredTopics: ['topic'],
    outputs: ['web'],
    constraints: [],
    unresolvedQuestions: []
  };

  assert.throws(() => {
    RequirementsSchema.parse(invalidReqs);
  });
});

test('ChartSpecSchema enforces identity transform and reject policy', () => {
  const validChart = {
    id: 'chart-1',
    datasetId: 'dataset-1',
    columns: { x: 'category', y: 'value' },
    units: { value: 'mg/ml' },
    type: 'bar',
    transform: { kind: 'identity' },
    missingPolicy: 'reject',
    caption: 'Figure 1: Measurement summary',
    altText: 'Bar chart showing measurements per category'
  };

  const parsed = ChartSpecSchema.parse(validChart);
  assert.equal(parsed.missingPolicy, 'reject');
});

test('Result helpers generate canonical envelopes', () => {
  const success = okResult({ count: 42 }, 'rev-123', ['Minor notice']);
  assert.equal(success.ok, true);
  if (success.ok) {
    assert.equal(success.value.count, 42);
    assert.equal(success.revision, 'rev-123');
    assert.deepEqual(success.warnings, ['Minor notice']);
  }

  const fail = errorResult({
    code: 'INVALID_INPUT',
    message: 'Value out of bounds',
    recoverable: false
  });
  assert.equal(fail.ok, false);
  if (!fail.ok) {
    assert.equal(fail.error.code, 'INVALID_INPUT');
  }
});
