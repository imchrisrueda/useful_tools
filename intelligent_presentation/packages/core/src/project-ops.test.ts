import test from 'node:test';
import assert from 'node:assert/strict';
import os from 'node:os';
import path from 'node:path';
import fs from 'node:fs/promises';
import { createProject } from './project-ops.js';
import { assertPathConfinement, isValidProjectName } from './fs-utils.js';
import { computeCanonicalRevision } from './hash-utils.js';

test('assertPathConfinement detects directory traversal attempts', () => {
  const base = path.resolve('/test/base');
  assert.equal(assertPathConfinement(base, 'child'), path.resolve(base, 'child'));
  assert.equal(assertPathConfinement(base, 'nested/dir'), path.resolve(base, 'nested/dir'));

  assert.throws(() => {
    assertPathConfinement(base, '../escape');
  }, /Path escape detected/);

  assert.throws(() => {
    assertPathConfinement(base, 'child/../../escape');
  }, /Path escape detected/);
});

test('isValidProjectName validates legal directory names', () => {
  assert.equal(isValidProjectName('my-presentation'), true);
  assert.equal(isValidProjectName('presentation_2026'), true);
  assert.equal(isValidProjectName(''), false);
  assert.equal(isValidProjectName('..'), false);
  assert.equal(isValidProjectName('.'), false);
  assert.equal(isValidProjectName('test/project'), false);
  assert.equal(isValidProjectName('test\\project'), false);
  assert.equal(isValidProjectName('bad:name'), false);
});

test('computeCanonicalRevision is deterministic across key ordering', () => {
  const obj1 = { b: 2, a: 1, nested: { y: 20, x: 10 } };
  const obj2 = { a: 1, b: 2, nested: { x: 10, y: 20 } };

  const rev1 = computeCanonicalRevision(obj1);
  const rev2 = computeCanonicalRevision(obj2);

  assert.equal(rev1, rev2);
  assert.equal(typeof rev1, 'string');
  assert.equal(rev1.length, 64);
});

test('createProject successfully initializes a new project with staging and manifest', async () => {
  const tempDir = await fs.mkdtemp(path.join(os.tmpdir(), 'ips-test-'));
  try {
    const res = await createProject({
      name: 'demo-presentation',
      root: tempDir,
      config: {
        language: 'es',
        aspectRatio: '16/9',
        engineId: 'slidev',
        title: 'Demo Presentation'
      }
    });

    assert.equal(res.ok, true);
    if (res.ok) {
      assert.ok(res.value.projectId.startsWith('proj-'));
      assert.equal(typeof res.value.revision, 'string');
      assert.equal(res.value.revision.length, 64);

      // Verify files created
      const manifestRaw = await fs.readFile(res.value.manifestPath, 'utf-8');
      const manifest = JSON.parse(manifestRaw);
      assert.equal(manifest.title, 'Demo Presentation');
      assert.equal(manifest.language, 'es');
      assert.equal(manifest.engine.id, 'slidev');

      // Verify folders created
      await fs.access(path.join(res.value.projectDir, 'slides'));
      await fs.access(path.join(res.value.projectDir, 'data'));
      await fs.access(path.join(res.value.projectDir, 'assets'));
      await fs.access(path.join(res.value.projectDir, 'reports'));
      await fs.access(path.join(res.value.projectDir, 'outputs'));
    }
  } finally {
    await fs.rm(tempDir, { recursive: true, force: true });
  }
});

test('createProject rejects occupied destination directory', async () => {
  const tempDir = await fs.mkdtemp(path.join(os.tmpdir(), 'ips-test-'));
  try {
    const occupiedDir = path.join(tempDir, 'already-exists');
    await fs.mkdir(occupiedDir);

    const res = await createProject({
      name: 'already-exists',
      root: tempDir,
      config: {
        language: 'es',
        aspectRatio: '16/9',
        engineId: 'slidev'
      }
    });

    assert.equal(res.ok, false);
    if (!res.ok) {
      assert.equal(res.error.code, 'INVALID_INPUT');
      assert.match(res.error.message, /already exists/);
    }
  } finally {
    await fs.rm(tempDir, { recursive: true, force: true });
  }
});

test('createProject rejects path traversal attempt in project name', async () => {
  const tempDir = await fs.mkdtemp(path.join(os.tmpdir(), 'ips-test-'));
  try {
    const res = await createProject({
      name: '../escape-project',
      root: tempDir,
      config: {
        language: 'es',
        aspectRatio: '16/9',
        engineId: 'slidev'
      }
    });

    assert.equal(res.ok, false);
    if (!res.ok) {
      assert.equal(res.error.code, 'INVALID_INPUT');
    }
  } finally {
    await fs.rm(tempDir, { recursive: true, force: true });
  }
});
