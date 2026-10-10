# validate_presentation

**Propósito:** producir evidencia de calidad sobre una revisión concreta. **Cuándo leer:** al implementar validators/review. Fase: MVP, ampliada por estados en fase 3. Referencias: [calidad](../QUALITY_STRATEGY.md), [modelos](../DATA_MODELS.md).

## Entrada y salida

```ts
type Input = { projectId: string; revision: string;
  profile: 'mvp' | 'interactive';
  humanReview?: { slideId: string; stateId: string;
    decision: 'accepted' | 'changes_requested'; reason: string }[] };
type Output = { reportRef: string; captureRefs: string[];
  result: 'pass' | 'fail' | 'needs_review' };
```

ValidationReport identifica hashes/toolchain y todos los slide/state inspeccionados. Un reporte con findings es un resultado de validación, no necesariamente error operativo; result fail debe impedir exportación de entrega.

## Precondiciones y dependencias

Fuentes válidas, recursos presentes, runtime/navegador/fuentes fijados, permisos de render. Perfil disponible. La revisión humana proviene de decisión recibida, no fabricada por un modelo.

## Efectos

Genera capturas y reporte; no modifica contenido ni repara automáticamente. Resultados ligados a la revisión dejan de ser actuales si cambian dependencias durante/después de ejecución.

## Errores y validación

DEPENDENCY_MISSING; RENDER_FAILED; REVISION_CONFLICT si el proyecto cambia; INVALID_INPUT por perfil/cobertura. findings deterministas y subjetivos separados; warning no resuelta produce needs_review, error/blocker fail. Comprobar cobertura 100 %, ausencia de capturas de portada usadas como sustituto, bounds, recursos, datos/alt y accesibilidad aplicable.
