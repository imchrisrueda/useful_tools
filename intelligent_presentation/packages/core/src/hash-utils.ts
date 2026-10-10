import crypto from 'node:crypto';

export function computeSha256(content: string | Buffer): string {
  return crypto.createHash('sha256').update(content).digest('hex');
}

export function computeCanonicalRevision(data: unknown): string {
  // If data is a ProjectManifest-like object, strip non-content artifacts (reports, cache, approvals)
  if (data && typeof data === 'object' && 'artifacts' in data && (data as any).artifacts) {
    const record = { ...(data as any) };
    const artifactsCopy = { ...record.artifacts };
    for (const key of Object.keys(artifactsCopy)) {
      if (key.startsWith('approval:') || key === 'validationReport' || key === 'exportReport') {
        delete artifactsCopy[key];
      }
    }
    record.artifacts = artifactsCopy;
    return computeSha256(canonicalJsonStringify(record));
  }

  const jsonString = canonicalJsonStringify(data);
  return computeSha256(jsonString);
}

function canonicalJsonStringify(obj: unknown): string {
  if (obj === null || typeof obj !== 'object') {
    return JSON.stringify(obj);
  }

  if (Array.isArray(obj)) {
    return '[' + obj.map((item) => canonicalJsonStringify(item)).join(',') + ']';
  }

  const record = obj as Record<string, unknown>;
  const sortedKeys = Object.keys(record).sort();
  const entries = sortedKeys.map((key) => {
    return JSON.stringify(key) + ':' + canonicalJsonStringify(record[key]);
  });

  return '{' + entries.join(',') + '}';
}
