# export_presentation

**Propósito:** exportar una revisión aprobada y entregar evidencia. **Cuándo leer:** al implementar export/delivery. Fase: web/PDF en MVP; PPTX en fase 4. Referencias: [calidad](../QUALITY_STRATEGY.md), [modelos](../DATA_MODELS.md), [PoC 4](../pocs/04-editable-pptx.md).

## Entrada y salida

```ts
type Input = { projectId: string; revision: string; format: string;
  validationReportId: string; deliveryApprovalId: string;
  includeAuthorizedSources: boolean };
type Output = { exportReportRef: string;
  files: { path: string; hash: string; mime: string }[] };
```

Devuelve Result<Output>; ExportReport declara degradaciones y comprobaciones. Exportación de borrador para inspección puede realizarse como derivado interno, pero este contrato de entrega exige aprobación.

## Precondiciones y dependencias

Reporte pass sobre revisión/dependencias actuales, aprobación humana final y formatos disponibles. Engine/exportador y permisos locales. Redistribución de fuentes/assets requiere licencia y autorización; conservar localmente no autoriza publicar.

## Efectos

Genera web/PDF en outputs y bundle de fuentes autorizado con manifiesto, versiones y guía. Excluye secretos, caché innecesaria y datos no autorizados para distribución. No publica ni envía archivos a terceros.

## Errores y validación

APPROVAL_REQUIRED; VALIDATION_FAILED por reporte fail/needs_review; REVISION_CONFLICT por reporte obsoleto; CAPABILITY_UNSUPPORTED; DEPENDENCY_MISSING; RENDER_FAILED; PATH_DENIED; SOURCE_UNAUTHORIZED. Verificar número de páginas/estados conforme política, apertura de web/PDF, recursos locales, contenido/notas según formato, manifest y pérdidas. Un archivo existente no prueba que exportación sea correcta.
