import { DesignManifest, DesignManifestSchema, Result, okResult, errorResult } from './models.js';

// 1. Scientific Minimal (MVP)
export const SCIENTIFIC_MINIMAL_THEME: DesignManifest = {
  id: 'scientific-minimal',
  version: '1.0.0',
  family: 'Scientific Minimal',
  tokens: {
    colors: {
      background: '#ffffff',
      surface: '#f8fafc',
      text: '#0f172a',
      mutedText: '#475569',
      accent: '#2563eb',
      border: '#e2e8f0'
    },
    typography: {
      headingFont: 'system-ui, -apple-system, sans-serif',
      bodyFont: 'system-ui, -apple-system, sans-serif',
      codeFont: 'monospace'
    },
    spacing: {
      safeMargin: '48px',
      gap: '24px'
    }
  },
  layouts: [
    'portada',
    'seccion',
    'texto',
    'dos-columnas',
    'figura',
    'grafico',
    'conclusion'
  ],
  components: ['title', 'quote', 'figure-caption', 'chart', 'message-box'],
  fonts: [
    {
      family: 'Inter',
      assetRef: 'system-ui',
      license: 'OFL-1.1'
    }
  ],
  capabilities: ['web', 'pdf'],
  license: 'MIT'
};

// 2. Tech Keynote (MVP)
export const TECH_KEYNOTE_THEME: DesignManifest = {
  id: 'tech-keynote',
  version: '1.0.0',
  family: 'Tech Keynote',
  tokens: {
    colors: {
      background: '#090d16',
      surface: '#111827',
      text: '#f9fafb',
      mutedText: '#9ca3af',
      accent: '#38bdf8',
      border: '#1f2937'
    },
    typography: {
      headingFont: 'system-ui, -apple-system, sans-serif',
      bodyFont: 'system-ui, -apple-system, sans-serif',
      codeFont: 'monospace'
    },
    spacing: {
      safeMargin: '48px',
      gap: '24px'
    }
  },
  layouts: [
    'portada',
    'seccion',
    'texto',
    'dos-columnas',
    'figura',
    'grafico',
    'conclusion'
  ],
  components: ['title', 'quote', 'figure-caption', 'chart', 'message-box'],
  fonts: [
    {
      family: 'Inter',
      assetRef: 'system-ui',
      license: 'OFL-1.1'
    }
  ],
  capabilities: ['web', 'pdf'],
  license: 'MIT'
};

// 3. Data Storytelling (Fase 2)
export const DATA_STORYTELLING_THEME: DesignManifest = {
  id: 'data-storytelling',
  version: '1.0.0',
  family: 'Data Storytelling',
  tokens: {
    colors: {
      background: '#fafaf9',
      surface: '#f5f5f4',
      text: '#1c1917',
      mutedText: '#57534e',
      accent: '#ea580c', // Orange focal accent for data callouts
      border: '#e7e5e4'
    },
    typography: {
      headingFont: 'system-ui, -apple-system, sans-serif',
      bodyFont: 'system-ui, -apple-system, sans-serif',
      codeFont: 'monospace'
    },
    spacing: {
      safeMargin: '48px',
      gap: '20px'
    }
  },
  layouts: [
    'portada',
    'seccion',
    'texto',
    'dos-columnas',
    'figura',
    'grafico',
    'conclusion',
    'comparativa-metricas',
    'destacado-dato'
  ],
  components: ['title', 'quote', 'figure-caption', 'chart', 'metric-callout', 'message-box'],
  fonts: [
    {
      family: 'Inter',
      assetRef: 'system-ui',
      license: 'OFL-1.1'
    }
  ],
  capabilities: ['web', 'pdf'],
  license: 'MIT'
};

// 4. Interactive Workshop (Fase 2)
export const INTERACTIVE_WORKSHOP_THEME: DesignManifest = {
  id: 'interactive-workshop',
  version: '1.0.0',
  family: 'Interactive Workshop',
  tokens: {
    colors: {
      background: '#f8fafc',
      surface: '#ffffff',
      text: '#0f172a',
      mutedText: '#334155',
      accent: '#8b5cf6', // Violet accent for laboratory exercises and interactive components
      border: '#cbd5e1'
    },
    typography: {
      headingFont: 'system-ui, -apple-system, sans-serif',
      bodyFont: 'system-ui, -apple-system, sans-serif',
      codeFont: 'monospace'
    },
    spacing: {
      safeMargin: '40px',
      gap: '20px'
    }
  },
  layouts: [
    'portada',
    'seccion',
    'texto',
    'dos-columnas',
    'figura',
    'grafico',
    'conclusion',
    'laboratorio-interactivo',
    'ejercicio-guiado'
  ],
  components: ['title', 'quote', 'figure-caption', 'chart', 'interactive-slider', 'step-tracker'],
  fonts: [
    {
      family: 'Inter',
      assetRef: 'system-ui',
      license: 'OFL-1.1'
    }
  ],
  capabilities: ['web', 'pdf'],
  license: 'MIT'
};

// 5. Executive Professional (Fase 2)
export const EXECUTIVE_PROFESSIONAL_THEME: DesignManifest = {
  id: 'executive-professional',
  version: '1.0.0',
  family: 'Executive Professional',
  tokens: {
    colors: {
      background: '#ffffff',
      surface: '#f1f5f9',
      text: '#0f172a',
      mutedText: '#334155',
      accent: '#0369a1', // Deep corporate navy blue
      border: '#cbd5e1'
    },
    typography: {
      headingFont: 'system-ui, -apple-system, sans-serif',
      bodyFont: 'system-ui, -apple-system, sans-serif',
      codeFont: 'monospace'
    },
    spacing: {
      safeMargin: '56px',
      gap: '28px'
    }
  },
  layouts: [
    'portada',
    'seccion',
    'texto',
    'dos-columnas',
    'figura',
    'grafico',
    'conclusion',
    'resumen-ejecutivo',
    'matriz-decisiones'
  ],
  components: ['title', 'quote', 'figure-caption', 'chart', 'kpi-summary', 'executive-bullet'],
  fonts: [
    {
      family: 'Inter',
      assetRef: 'system-ui',
      license: 'OFL-1.1'
    }
  ],
  capabilities: ['web', 'pdf'],
  license: 'MIT'
};

export const THEME_CATALOG: Record<string, DesignManifest> = {
  'scientific-minimal': DesignManifestSchema.parse(SCIENTIFIC_MINIMAL_THEME),
  'tech-keynote': DesignManifestSchema.parse(TECH_KEYNOTE_THEME),
  'data-storytelling': DesignManifestSchema.parse(DATA_STORYTELLING_THEME),
  'interactive-workshop': DesignManifestSchema.parse(INTERACTIVE_WORKSHOP_THEME),
  'executive-professional': DesignManifestSchema.parse(EXECUTIVE_PROFESSIONAL_THEME)
};

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

export function calculateThemeScore(themeId: string, criteria?: RecommendationCriteria): number {
  if (!criteria) return 50; // Neutral baseline
  let score = 50;

  const { audience, dataDensity, presentationTone } = criteria;

  if (themeId === 'scientific-minimal') {
    if (audience === 'cientifica') score += 35;
    if (presentationTone === 'riguroso') score += 20;
    if (dataDensity === 'alta') score += 15;
  } else if (themeId === 'tech-keynote') {
    if (audience === 'tecnica' || audience === 'general') score += 35;
    if (presentationTone === 'innovador') score += 25;
  } else if (themeId === 'data-storytelling') {
    if (dataDensity === 'alta' || dataDensity === 'media') score += 35;
    if (presentationTone === 'innovador' || presentationTone === 'riguroso') score += 15;
  } else if (themeId === 'interactive-workshop') {
    if (audience === 'estudiantes' || audience === 'tecnica') score += 35;
    if (presentationTone === 'didactico') score += 25;
  } else if (themeId === 'executive-professional') {
    if (audience === 'ejecutiva') score += 40;
    if (presentationTone === 'institucional') score += 25;
    if (dataDensity === 'baja') score += 15;
  }

  return score;
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

  const proposals: DesignProposal[] = scoredThemes.slice(0, input.requestedCount).map(({ id, theme, score }) => {
    let rationale = '';
    switch (id) {
      case 'scientific-minimal':
        rationale = 'Rigor metodológico, figuras de alto contraste sobre fondo blanco y máxima legibilidad.';
        break;
      case 'tech-keynote':
        rationale = 'Diseño oscuro contemporáneo de alto contraste para presentaciones técnicas y divulgación.';
        break;
      case 'data-storytelling':
        rationale = 'Optimizado para destacar hallazgos numéricos, comparativas de métricas y gráficos protagonistas.';
        break;
      case 'interactive-workshop':
        rationale = 'Enfoque didáctico para formación técnica con soporte de controles visibles y laboratorios guiados.';
        break;
      case 'executive-professional':
        rationale = 'Elegancia corporativa estructurada para síntesis estratégica y comités directivos.';
        break;
    }

    return {
      id: `prop-${id}`,
      designId: theme.id,
      designVersion: theme.version,
      family: theme.family,
      overrides: {},
      score,
      rationale,
      previews: [
        `templates/previews/${id}-cover.png`,
        `templates/previews/${id}-dense.png`,
        `templates/previews/${id}-chart.png`
      ],
      limitations: ['Animaciones Manim pesadas se presentan como póster estático en PDF']
    };
  });

  return okResult({ proposals });
}
