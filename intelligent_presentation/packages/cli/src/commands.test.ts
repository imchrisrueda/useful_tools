import test from 'node:test';
import assert from 'node:assert/strict';
import { runDoctor, getCapabilities } from './commands.js';

test('runDoctor returns structured environment checks', () => {
  const report = runDoctor();
  assert.equal(typeof report.os, 'string');
  assert.equal(typeof report.timestamp, 'string');
  assert.ok(Array.isArray(report.checks));
  assert.ok(report.checks.some((c) => c.name === 'Node.js runtime'));
  assert.ok(report.checks.some((c) => c.name === 'npm CLI'));
});

test('getCapabilities returns supported operations and limits', () => {
  const caps = getCapabilities();
  assert.equal(caps.engine, 'slidev');
  assert.ok(caps.operations.includes('create_project'));
  assert.ok(caps.operations.includes('analyze_request'));
  assert.ok(caps.formats.includes('web'));
  assert.ok(caps.formats.includes('pdf'));
});
