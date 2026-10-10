import os from 'node:os';
import { execSync } from 'node:child_process';

export interface DoctorCheck {
  name: string;
  category: 'runtime' | 'browser' | 'fonts' | 'agent' | 'python';
  status: 'ok' | 'warning' | 'missing';
  version?: string;
  details?: string;
}

export interface DoctorReport {
  timestamp: string;
  os: string;
  checks: DoctorCheck[];
}

export function runDoctor(): DoctorReport {
  const checks: DoctorCheck[] = [];

  // 1. Node runtime
  const nodeVersion = process.version;
  const majorNode = parseInt(nodeVersion.slice(1).split('.')[0], 10);
  checks.push({
    name: 'Node.js runtime',
    category: 'runtime',
    status: majorNode >= 20 ? 'ok' : 'warning',
    version: nodeVersion,
    details: majorNode < 22 ? 'Slidev 53+ officially recommends Node >=22.12.0' : undefined
  });

  // 2. npm
  try {
    const npmVer = execSync('npm --version', { encoding: 'utf-8', timeout: 5000 }).trim();
    checks.push({
      name: 'npm CLI',
      category: 'runtime',
      status: 'ok',
      version: npmVer
    });
  } catch {
    checks.push({
      name: 'npm CLI',
      category: 'runtime',
      status: 'missing'
    });
  }

  // 3. Python and uv
  try {
    const uvVer = execSync('uv --version', { encoding: 'utf-8', timeout: 5000 }).trim();
    checks.push({
      name: 'uv package manager',
      category: 'python',
      status: 'ok',
      version: uvVer
    });
  } catch {
    checks.push({
      name: 'uv package manager',
      category: 'python',
      status: 'warning',
      details: 'Optional for Phase 3 Manim animations'
    });
  }

  // 4. Browser / Playwright check
  checks.push({
    name: 'Headless Chromium',
    category: 'browser',
    status: 'warning',
    details: 'Playwright browser bundle pending installation for screenshot validation'
  });

  return {
    timestamp: new Date().toISOString(),
    os: `${os.type()} ${os.release()} (${os.arch()})`,
    checks
  };
}

export interface CapabilityDescriptor {
  engine: string;
  version: string;
  operations: string[];
  formats: string[];
  permissions: string[];
  limits: Record<string, string>;
}

export function getCapabilities(): CapabilityDescriptor {
  return {
    engine: 'slidev',
    version: '53.0.0',
    operations: [
      'create_project',
      'analyze_request',
      'create_storyboard',
      'select_design',
      'generate_slides',
      'update_slide',
      'validate_presentation',
      'export_presentation'
    ],
    formats: ['web', 'pdf'],
    permissions: ['local-fs-isolated'],
    limits: {
      maxSlidesMvp: '15',
      exportFormats: 'web, pdf'
    }
  };
}
