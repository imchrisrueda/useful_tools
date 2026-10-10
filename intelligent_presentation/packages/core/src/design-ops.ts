import { DesignManifest, DesignManifestSchema, Result, okResult, errorResult } from './models.js';

// --- Shared base tokens to eliminate repetition across themes ---

const BASE_TYPOGRAPHY = {
  headingFont: 'system-ui, -apple-system, sans-serif',
  bodyFont: 'system-ui, -apple-system, sans-serif',
  codeFont: 'monospace'
};

const BASE_FONT: DesignManifest['fonts'][number] = {
  family: 'Inter',
  assetRef: 'system-ui',
  license: 'OFL-1.1'
};

const BASE_LAYOUTS = [
  'portada',
  'seccion',
  'texto',
  'dos-columnas',
  'figura',
  'grafico',
  'conclusion'
];

const BASE_CAPABILITIES: DesignManifest['capabilities'] = ['web', 'pdf'];

interface ThemeConfig {
  id: string;
  family: string;
  colors: Record<string, string>;
  spacing: { safeMargin: string; gap: string };
  extraLayouts?: string[];
  extraComponents?: string[];
}

function createTheme(config: ThemeConfig): DesignManifest {
  return {
    id: config.id,
    version: '1.0.0',
    family: config.family,
    tokens: {
      colors: config.colors,
      typography: BASE_TYPOGRAPHY,
      spacing: config.spacing
    },
    layouts: [...BASE_LAYOUTS, ...(config.extraLayouts ?? [])],
    components: ['title', 'quote', 'figure-caption', 'chart', ...(config.extraComponents ?? ['message-box'])],
    fonts: [BASE_FONT],
    capabilities: BASE_CAPABILITIES,
    license: 'MIT'
  };
}

// 1. Scientific Minimal (MVP)
export const SCIENTIFIC_MINIMAL_THEME: DesignManifest = createTheme({
  id: 'scientific-minimal',
  family: 'Scientific Minimal',
  colors: {
    background: '#ffffff',
    surface: '#f8fafc',
    text: '#0f172a',
    mutedText: '#475569',
    accent: '#2563eb',
    border: '#e2e8f0'
  },
  spacing: { safeMargin: '48px', gap: '24px' }
});

// 2. Tech Keynote (MVP)
export const TECH_KEYNOTE_THEME: DesignManifest = createTheme({
  id: 'tech-keynote',
  family: 'Tech Keynote',
  colors: {
    background: '#090d16',
    surface: '#111827',
    text: '#f9fafb',
    mutedText: '#9ca3af',
    accent: '#38bdf8',
    border: '#1f2937'
  },
  spacing: { safeMargin: '48px', gap: '24px' }
});

// 3. Data Storytelling (Fase 2)
export const DATA_STORYTELLING_THEME: DesignManifest = createTheme({
  id: 'data-storytelling',
  family: 'Data Storytelling',
  colors: {
    background: '#fafaf9',
    surface: '#f5f5f4',
    text: '#1c1917',
    mutedText: '#57534e',
    accent: '#ea580c', // Orange focal accent for data callouts
    border: '#e7e5e4'
  },
  spacing: { safeMargin: '48px', gap: '20px' },
  extraLayouts: ['comparativa-metricas', 'destacado-dato'],
  extraComponents: ['metric-callout', 'message-box']
});

// 4. Interactive Workshop (Fase 2)
export const INTERACTIVE_WORKSHOP_THEME: DesignManifest = createTheme({
  id: 'interactive-workshop',
  family: 'Interactive Workshop',
  colors: {
    background: '#f8fafc',
    surface: '#ffffff',
    text: '#0f172a',
    mutedText: '#334155',
    accent: '#8b5cf6', // Violet accent for laboratory exercises and interactive components
    border: '#cbd5e1'
  },
  spacing: { safeMargin: '40px', gap: '20px' },
  extraLayouts: ['laboratorio-interactivo', 'ejercicio-guiado'],
  extraComponents: ['interactive-slider', 'step-tracker']
});

// 5. Executive Professional (Fase 2)
export const EXECUTIVE_PROFESSIONAL_THEME: DesignManifest = createTheme({
  id: 'executive-professional',
  family: 'Executive Professional',
  colors: {
    background: '#ffffff',
    surface: '#f1f5f9',
    text: '#0f172a',
    mutedText: '#334155',
    accent: '#0369a1', // Deep corporate navy blue
    border: '#cbd5e1'
  },
  spacing: { safeMargin: '56px', gap: '28px' },
  extraLayouts: ['resumen-ejecutivo', 'matriz-decisiones'],
  extraComponents: ['kpi-summary', 'executive-bullet']
});

// Catalog built once from validated constants (Zod validation runs at module init)
export const THEME_CATALOG: Record<string, DesignManifest> = Object.fromEntries(
  [
    SCIENTIFIC_MINIMAL_THEME,
    TECH_KEYNOTE_THEME,
    DATA_STORYTELLING_THEME,
    INTERACTIVE_WORKSHOP_THEME,
    EXECUTIVE_PROFESSIONAL_THEME
  ].map((theme) => [theme.id, DesignManifestSchema.parse(theme)])
);

export interface RecommendationCriteria {
  audience?: 'cientifica' | 'tecnica' | 'ejecutiva' | 'estudiantes' | 'general';
  dataDensity?: 'baja' | 'media' | 'alta';
  presentationTone?: 'riguroso' | 'innovador' | 'institucional' | 'didactico';
}

export interface DesignProposal {
  id: string;
  designId: string;
  designVersion: string;
  family: string;
  overrides: Record<string, unknown>;
  score: number;
  rationale: string;
  previews: string[];
  limitations: string[];
}

export interface SuggestDesignsInput {
  projectId: string;
  revision: string;
  storyboardRevision: string;
  formats: ('web' | 'pdf')[];
  requestedCount: number;
  criteria?: RecommendationCriteria;
}

const THEME_SCORING_RULES: Record<string, (c: RecommendationCriteria) => number> = {
  'scientific-minimal': (c) =>
    (c.audience === 'cientifica' ? 35 : 0) +
    (c.presentationTone === 'riguroso' ? 20 : 0) +
    (c.dataDensity === 'alta' ? 15 : 0),
  'tech-keynote': (c) =>
    (c.audience === 'tecnica' || c.audience === 'general' ? 35 : 0) +
    (c.presentationTone === 'innovador' ? 25 : 0),
  'data-storytelling': (c) =>
    (c.dataDensity === 'alta' || c.dataDensity === 'media' ? 35 : 0) +
    (c.presentationTone === 'innovador' || c.presentationTone === 'riguroso' ? 15 : 0),
  'interactive-workshop': (c) =>
    (c.audience === 'estudiantes' || c.audience === 'tecnica' ? 35 : 0) +
    (c.presentationTone === 'didactico' ? 25 : 0),
  'executive-professional': (c) =>
    (c.audience === 'ejecutiva' ? 40 : 0) +
    (c.presentationTone === 'institucional' ? 25 : 0) +
    (c.dataDensity === 'baja' ? 15 : 0)
};

const THEME_RATIONALES: Record<string, string> = {
  'scientific-minimal': 'Rigor metodológico, figuras de alto contraste sobre fondo blanco y máxima legibilidad.',
  'tech-keynote': 'Diseño oscuro contemporáneo de alto contraste para presentaciones técnicas y divulgación.',
  'data-storytelling': 'Optimizado para destacar hallazgos numéricos, comparativas de métricas y gráficos protagonistas.',
  'interactive-workshop': 'Enfoque didáctico para formación técnica con soporte de controles visibles y laboratorios guiados.',
  'executive-professional': 'Elegancia corporativa estructurada para síntesis estratégica y comités directivos.'
};

export function calculateThemeScore(themeId: string, criteria?: RecommendationCriteria): number {
  if (!criteria) return 50; // Neutral baseline
  const scoreFn = THEME_SCORING_RULES[themeId];
  return 50 + (scoreFn ? scoreFn(criteria) : 0);
}

export function suggestDesigns(input: SuggestDesignsInput): Result<{ proposals: DesignProposal[] }> {
  const scoredThemes: { id: string; theme: DesignManifest; score: number }[] = [];

  for (const [id, theme] of Object.entries(THEME_CATALOG)) {
    // Check format compatibility
    const isCompatible = input.formats.every((fmt) => theme.capabilities.includes(fmt));
    if (!isCompatible) continue;

    const score = calculateThemeScore(id, input.criteria);
    scoredThemes.push({ id, theme, score });
  }

  if (scoredThemes.length === 0) {
    return errorResult({
      code: 'CAPABILITY_UNSUPPORTED',
      message: 'No available themes support all requested formats',
      recoverable: false
    });
  }

  // Sort descending by score
  scoredThemes.sort((a, b) => b.score - a.score);

  const proposals: DesignProposal[] = scoredThemes.slice(0, input.requestedCount).map(({ id, theme, score }) => ({
    id: `prop-${id}`,
    designId: theme.id,
    designVersion: theme.version,
    family: theme.family,
    overrides: {},
    score,
    rationale: THEME_RATIONALES[id] ?? '',
    previews: [
      `templates/previews/${id}-cover.png`,
      `templates/previews/${id}-dense.png`,
      `templates/previews/${id}-chart.png`
    ],
    limitations: ['Animaciones Manim pesadas se presentan como póster estático en PDF']
  }));

  return okResult({ proposals });
}
