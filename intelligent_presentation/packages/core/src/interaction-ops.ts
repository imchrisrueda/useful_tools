import fs from 'node:fs/promises';
import path from 'node:path';
import {
  Result,
  okResult,
  errorResult
} from './models.js';
import {
  loadManifestWithRevisionCheck,
  loadDependencyIndex,
  invalidateReportsAndSave,
  saveDependencyIndex
} from './manifest-utils.js';

export interface InteractiveComponentSpec {
  id: string;
  name: string;
  allowedParameters: Record<string, { type: 'number' | 'string' | 'boolean'; min?: number; max?: number; default: unknown }>;
  supportedEvents: string[];
}

export const REGISTERED_INTERACTIVE_COMPONENTS: Record<string, InteractiveComponentSpec> = {
  'slider-linear-response': {
    id: 'slider-linear-response',
    name: 'Dynamic Linear Slope Curve (y = a * x)',
    allowedParameters: {
      a: { type: 'number', min: 0, max: 2, default: 1 },
      xMin: { type: 'number', default: 0 },
      xMax: { type: 'number', default: 10 },
      points: { type: 'number', default: 11 }
    },
    supportedEvents: ['param_changed', 'reset', 'keyboard_adjust']
  },
  'dose-response-curve': {
    id: 'dose-response-curve',
    name: 'Sigmoidal Dose-Response Curve (Hill Equation)',
    allowedParameters: {
      ec50: { type: 'number', min: 0.1, max: 100, default: 10 },
      hillSlope: { type: 'number', min: 0.5, max: 4.0, default: 1.0 },
      top: { type: 'number', default: 100 },
      bottom: { type: 'number', default: 0 }
    },
    supportedEvents: ['param_changed', 'reset']
  }
};

export interface AddInteractionInput {
  projectDir: string;
  expectedRevision: string;
  slideId: string;
  interaction: {
    componentId: string;
    datasetId?: string;
    parameters: Record<string, unknown>;
    testStates: string[];
    exportStateId: string;
    altText: string;
  };
}

export interface AddInteractionOutput {
  componentRef: string;
  stateManifestRef: string;
  staticFallbackRef: string;
  revision: string;
}

export async function addInteraction(input: AddInteractionInput): Promise<Result<AddInteractionOutput>> {
  const loaded = await loadManifestWithRevisionCheck(input.projectDir, input.expectedRevision);
  if (!loaded.ok) return loaded;
  const { manifest, manifestPath } = loaded.value;

  // Validate component registration
  const registeredSpec = REGISTERED_INTERACTIVE_COMPONENTS[input.interaction.componentId];
  if (!registeredSpec) {
    return errorResult({
      code: 'INVALID_INPUT',
      message: `Interactive component "${input.interaction.componentId}" is not in the trusted component registry`,
      recoverable: false
    });
  }

  // Validate parameters against allowed limits
  for (const [paramName, paramRule] of Object.entries(registeredSpec.allowedParameters)) {
    const val = input.interaction.parameters[paramName];
    if (val !== undefined) {
      if (typeof val !== paramRule.type) {
        return errorResult({
          code: 'INVALID_INPUT',
          message: `Parameter "${paramName}" must be of type ${paramRule.type}`,
          recoverable: false
        });
      }
      if (paramRule.type === 'number') {
        const numVal = val as number;
        if (paramRule.min !== undefined && numVal < paramRule.min) {
          return errorResult({
            code: 'INVALID_INPUT',
            message: `Parameter "${paramName}" (${numVal}) below minimum allowed (${paramRule.min})`,
            recoverable: false
          });
        }
        if (paramRule.max !== undefined && numVal > paramRule.max) {
          return errorResult({
            code: 'INVALID_INPUT',
            message: `Parameter "${paramName}" (${numVal}) exceeds maximum allowed (${paramRule.max})`,
            recoverable: false
          });
        }
      }
    }
  }

  // Check testStates contains exportStateId
  if (!input.interaction.testStates.includes(input.interaction.exportStateId)) {
    return errorResult({
      code: 'INVALID_INPUT',
      message: `exportStateId "${input.interaction.exportStateId}" must be one of the defined testStates: [${input.interaction.testStates.join(', ')}]`,
      recoverable: false
    });
  }

  const depLoaded = await loadDependencyIndex(input.projectDir, 'Slides must be generated before adding interactions');
  if (!depLoaded.ok) return depLoaded;
  const depIndex = depLoaded.value;

  if (!depIndex[input.slideId]) {
    return errorResult({
      code: 'INVALID_INPUT',
      message: `Slide ID "${input.slideId}" not found in project`,
      recoverable: false
    });
  }

  const componentsDir = path.join(input.projectDir, 'components');
  const assetsDir = path.join(input.projectDir, 'assets');
  await fs.mkdir(componentsDir, { recursive: true });
  await fs.mkdir(assetsDir, { recursive: true });

  const componentFilename = `${input.interaction.componentId}.vue`;
  const componentRef = `components/${componentFilename}`;
  const stateManifestRef = `assets/interaction-${input.slideId}-states.json`;
  const staticFallbackRef = `assets/interaction-${input.slideId}-fallback.svg`;

  // Write Vue Component implementation
  const vueContent = `<script setup lang="ts">
import { ref, computed } from 'vue';

const props = defineProps<{
  initialA?: number;
}>();

const a = ref(props.initialA ?? 1);
const points = computed(() => {
  const pts = [];
  for (let x = 0; x <= 10; x++) {
    pts.push({ x, y: a.value * x });
  }
  return pts;
});

function reset() {
  a.value = 1;
}
</script>

<template>
  <div class="interactive-plot-container border border-slate-700 rounded-lg p-4 bg-slate-900 text-white">
    <div class="flex items-center gap-4 mb-4">
      <label for="slope-slider" class="text-sm font-semibold">Parámetro a: {{ a.toFixed(2) }}</label>
      <input
        id="slope-slider"
        type="range"
        min="0"
        max="2"
        step="0.1"
        v-model.number="a"
        class="slider w-48"
        aria-label="Ajustar pendiente de la curva"
      />
      <button @click="reset" class="px-3 py-1 bg-blue-600 hover:bg-blue-700 rounded text-xs">Reset (a = 1)</button>
    </div>
    <div class="curve-display text-xs text-slate-300">
      y = {{ a.toFixed(2) }} * x
    </div>
  </div>
</template>
`;
  await fs.writeFile(path.join(input.projectDir, componentRef), vueContent, 'utf-8');

  // Write State Manifest
  const stateManifest = {
    slideId: input.slideId,
    componentId: input.interaction.componentId,
    datasetId: input.interaction.datasetId,
    parameters: input.interaction.parameters,
    testStates: input.interaction.testStates,
    exportStateId: input.interaction.exportStateId,
    altText: input.interaction.altText,
    interactionPolicy: {
      resetOnExplicitReset: true,
      preserveOnSlideNavigation: true,
      keyboardAdjustable: true
    }
  };
  await fs.writeFile(path.join(input.projectDir, stateManifestRef), JSON.stringify(stateManifest, null, 2), 'utf-8');

  // Write static fallback SVG for PDF export (exportStateId)
  const fallbackSvg = `<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 800 450" width="800" height="450">
  <rect width="100%" height="100%" fill="#090d16" />
  <title>Snapshot de componente interactivo (${input.interaction.exportStateId})</title>
  <desc>${input.interaction.altText}</desc>
  <text x="400" y="60" font-family="sans-serif" font-size="20" font-weight="bold" fill="#38bdf8" text-anchor="middle">${registeredSpec.name}</text>
  <text x="400" y="90" font-family="sans-serif" font-size="13" fill="#94a3b8" text-anchor="middle">Estado fijado para exportación estática: ${input.interaction.exportStateId}</text>
  <!-- Curva fija y = 1 * x -->
  <line x1="80" y1="380" x2="720" y2="380" stroke="#475569" stroke-width="2" />
  <line x1="80" y1="380" x2="80" y2="120" stroke="#475569" stroke-width="2" />
  <line x1="80" y1="380" x2="680" y2="140" stroke="#3b82f6" stroke-width="3" stroke-linecap="round" />
  <text x="400" y="420" font-family="sans-serif" font-size="12" fill="#64748b" text-anchor="middle">${input.interaction.altText}</text>
</svg>`;
  await fs.writeFile(path.join(input.projectDir, staticFallbackRef), fallbackSvg, 'utf-8');

  // Register resource in slide dependency index
  const interResourceId = `inter-${input.slideId}`;
  if (!depIndex[input.slideId].resources.includes(interResourceId)) {
    depIndex[input.slideId].resources.push(interResourceId);
  }
  await saveDependencyIndex(input.projectDir, depIndex);

  const newRevision = await invalidateReportsAndSave(manifest, manifestPath);

  return okResult({
    componentRef,
    stateManifestRef,
    staticFallbackRef,
    revision: newRevision
  });
}
