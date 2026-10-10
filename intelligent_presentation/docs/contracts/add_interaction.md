# add_interaction

**Propósito:** añadir controles y visualización dinámica. **Cuándo leer:** al preparar fase 3. Fase: 3; no soportada por MVP. Referencias: [PoC 3](../pocs/03-interactivity.md), [modelos](../DATA_MODELS.md).

## Entrada y salida

```ts
type Input = { projectId: string; expectedRevision: string; slideId: string;
  interaction: { componentId: string; datasetId?: string;
    parameters: Record<string, unknown>; testStates: string[];
    exportStateId: string; altText: string } };
type Output = { componentRef: string; stateManifestRef: string;
  staticFallbackRef: string; revision: string };
```

Componente y parámetros pertenecen al registro confiable. No cargar componentes/scripts desde documentos ni URLs arbitrarias. Schema por componente se fija tras PoC 3.

## Precondiciones y dependencias

Vue/Plotly iniciales, dataset autorizado, parámetros acotados y revisión actual; aprobación si altera contenido/significado. Código del componente revisado.

## Efectos

Añade componente, estados de prueba y snapshot estático; invalida render/validación de consumidores. La interacción no transmite datos ni requiere red externa por defecto.

## Errores y validación

CAPABILITY_UNSUPPORTED en MVP; INVALID_INPUT por límites/estado inexistente; SOURCE_UNAUTHORIZED; APPROVAL_REQUIRED; REVISION_CONFLICT; DEPENDENCY_MISSING. Verificar teclado, mínimo/intermedio/máximo/reset, resultado numérico conocido, alt y ausencia de errores. Exportación utiliza estado declarado y reporta pérdida de controles.
