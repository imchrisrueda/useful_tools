# update_slide

**Propósito:** modificar una slide sin reescribir fuentes ajenas. **Cuándo leer:** al implementar edición incremental. Fase: MVP. Referencias: [ciclo](../PROJECT_LIFECYCLE.md), [modelos](../DATA_MODELS.md).

## Entrada y salida

```ts
type Input = { projectId: string; expectedRevision: string; slideId: string;
  change: { markdown?: string; resourceIds?: string[];
    position?: number }; semanticChange: boolean;
  approval?: ApprovalRecord };
type Output = { slideId: string; revision: string;
  invalidatedArtifacts: string[]; preservedSlideIds: string[] };
```

Se propone contenido completo del archivo acotado, no un patch textual ambiguo. position usa orden renderizado actual y conserva ID. El análisis de impacto verifica semanticChange; el campo no es permiso para eludir aprobación.

## Precondiciones y dependencias

ID conocido, snapshot actual, bloqueo y parser de motor. Fuentes nuevas autorizadas; aprobación aplicable si cambia significado/narrativa/resultados.

## Efectos

Escribe solo slide/orden y metadatos derivados afectados; invalida capturas, aprobación final e informes dependientes. Hashes/bytes de otras fuentes se conservan. Puede renovar bundle/PDF global sin reescribir otras slides.

## Errores y validación

INVALID_INPUT por slide inexistente/posición inválida; REVISION_CONFLICT; APPROVAL_REQUIRED; SOURCE_UNAUTHORIZED; PATH_DENIED. Probar edición, reordenación, conflicto, reparación de composición y cambio de datos compartidos. Verificar preservación de valores/unidades/citas cuando el cambio se declara visual.
