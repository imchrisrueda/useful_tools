#!/usr/bin/env node
import { runDoctor, getCapabilities } from './commands.js';
import { createProject } from '@ips/core';

async function main() {
  const args = process.argv.slice(2);
  const command = args[0] || 'doctor';

  switch (command) {
    case 'doctor': {
      const report = runDoctor();
      console.log('--- IPS Doctor Report ---');
      console.log(`OS: ${report.os}`);
      console.log(`Timestamp: ${report.timestamp}`);
      console.log('\nChecks:');
      for (const check of report.checks) {
        const badge = check.status === 'ok' ? '[OK]' : check.status === 'warning' ? '[WARN]' : '[FAIL]';
        console.log(`  ${badge} ${check.name} ${check.version ? `(${check.version})` : ''}`);
        if (check.details) {
          console.log(`        -> ${check.details}`);
        }
      }
      process.exit(0);
      break;
    }

    case 'capabilities': {
      const caps = getCapabilities();
      console.log(JSON.stringify(caps, null, 2));
      process.exit(0);
      break;
    }

    case 'create': {
      const name = args[1];
      if (!name) {
        console.error('Error: specify a project name. Usage: ips create <name>');
        process.exit(1);
      }
      const res = await createProject({
        name,
        root: process.cwd(),
        config: {
          language: 'es',
          aspectRatio: '16/9',
          engineId: 'slidev'
        }
      });
      if (res.ok) {
        console.log(`Project successfully created at ${res.value.projectDir}`);
        console.log(`Manifest: ${res.value.manifestPath}`);
        console.log(`Revision: ${res.value.revision}`);
        process.exit(0);
      } else {
        console.error(`Error (${res.error.code}): ${res.error.message}`);
        process.exit(1);
      }
      break;
    }

    default: {
      console.log(`Unknown command: ${command}`);
      console.log('Available commands: doctor, capabilities, create <name>');
      process.exit(1);
    }
  }
}

main().catch((err) => {
  console.error('Fatal CLI error:', err);
  process.exit(1);
});
