# analyze_request

**Propósito:** convertir solicitud en requisitos explícitos. **Cuándo leer:** al diseñar la skill de creación/aplicación de requisitos. Fase: MVP. Referencias: [modelos](../DATA_MODELS.md), [requisitos](../PRODUCT_REQUIREMENTS.md).

## Entrada y salida

```ts
type Input = { projectId: string; expectedRevision: string;
  request: string; authorizedSourceIds: string[];
  proposal: Requirements };
type Output = { requirementsRef: string; revision: string;
  unresolvedQuestions: string[] };
```

Requirements se define en DATA_MODELS. El agente interpreta el texto y entrega proposal; core valida/aplica. La operación no realiza llamadas IA ocultas.

## Precondiciones y dependencias

Proyecto accesible, fuentes autorizadas y revisión esperada actual; skill de solicitud y schemas. Las preguntas críticas pendientes permiten guardar borrador pero bloquean la aprobación/avance a generación.

## Efectos

Conserva solicitud, requirements y sus fuentes. Actualiza revisión/etapa e invalida storyboard/diseño/entrega dependientes si cambian requisitos. No registra aprobación humana automáticamente.

## Errores y validación

INVALID_INPUT para objetivo/audiencia/duración/outputs inválidos; SOURCE_UNAUTHORIZED para fuente fuera de autorización; REVISION_CONFLICT para snapshot obsoleto; CAPABILITY_UNSUPPORTED para formato obligatorio ausente. Distinguir faltantes, datos aportados y supuestos. Verificar que propuesta cubre la solicitud y no inventa requisitos científicos; revisar humano antes de aprobar.
