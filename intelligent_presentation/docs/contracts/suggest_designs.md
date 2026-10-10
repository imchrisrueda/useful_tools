# suggest_designs

**Propósito:** ofrecer diseños reales compatibles. **Cuándo leer:** al construir catálogo/previsualizaciones. Fase: selección MVP; recomendaciones ampliadas en fase 2. Referencias: [diseño](../DESIGN_SYSTEM.md), [modelos](../DATA_MODELS.md).

## Entrada y salida

```ts
type Input = { projectId: string; revision: string;
  storyboardRevision: string; formats: string[]; requestedCount: number };
type Output = { proposals: { id: string; designId: string;
  designVersion: string; overrides: Record<string, unknown>;
  rationale: string; previews: string[]; limitations: string[] }[] };
```

Devuelve Result<Output>. requestedCount positivo; propuestas referencian DesignManifest. MVP ofrece dos temas disponibles; fase 2 genera tres variantes comparables.

## Precondiciones y dependencias

Storyboard válido, catálogo, recursos/tipografías, motor y renderer para previews. Cada preview usa el mismo contenido de portada/denso/gráfico y registra revisión.

## Efectos

Genera previews/propuestas, reutiliza caché y no cambia el contenido ni selecciona tema automáticamente. La revisión no muta por generar un derivado, pero éste queda ligado a sus entradas.

## Errores y validación

CAPABILITY_UNSUPPORTED si no hay suficientes opciones compatibles; DEPENDENCY_MISSING o RENDER_FAILED al previsualizar; REVISION_CONFLICT si cambia storyboard. No rellenar requestedCount con opciones ficticias. Verificar igualdad de contenido entre propuestas, fuentes cargadas y capacidades para formatos solicitados.
