import path from 'node:path';

export function assertPathConfinement(baseDir: string, relativeOrSubPath: string): string {
  const normalizedBase = path.resolve(baseDir);
  const target = path.resolve(normalizedBase, relativeOrSubPath);

  // Check if target starts with normalizedBase directory prefix
  const relative = path.relative(normalizedBase, target);
  if (relative.startsWith('..') || path.isAbsolute(relative)) {
    throw new Error(`Path escape detected: "${relativeOrSubPath}" escapes base directory "${baseDir}"`);
  }

  return target;
}

export function isValidProjectName(name: string): boolean {
  if (!name || typeof name !== 'string') return false;
  const trimmed = name.trim();
  if (trimmed.length === 0 || trimmed.length > 100) return false;
  // Disallow path traversal, directory separators, and dangerous characters
  if (/[\\/:\*\?"<>\|]/.test(trimmed)) return false;
  if (trimmed === '.' || trimmed === '..') return false;
  return true;
}
