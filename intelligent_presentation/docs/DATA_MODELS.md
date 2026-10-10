# Modelos de datos e interfaces

**Propósito:** definir datos comunes y enlazar contratos. **Cuándo leer:** al implementar validación, persistencia u operaciones. Referencias: [arquitectura](ARCHITECTURE.md), [ciclo](PROJECT_LIFECYCLE.md).

## Convenciones preliminares

Esquemas ejecutables en Zod con tipos TS y JSON Schema derivados. Estos modelos son especificaciones, no archivos de esquema ya implementados. `Id` es una cadena estable no vacía; `Revision` es el hash del snapshot pertinente; fechas ISO 8601 UTC; hashes SHA-256; rutas de proyecto relativas y resueltas dentro de su raíz. URLs solo en campos de procedencia/red autorizada.

El snapshot de contenido incluye fuentes, configuración y hashes de dependencias; excluye logs, caché, reportes y registros de aprobación para evitar referencias circulares. Las aprobaciones de etapa enlazan los hashes de sus entradas pertinentes; la aprobación final y validación enlazan el snapshot completo. T-013 formaliza su serialización canónica antes de persistir revisiones.

`schemaVersion` usa major.minor. Un lector rechaza major incompatible; campos adicionales solo se admiten si el contrato permite extensión. Migraciones preservan backup y requieren operación explícita. Versiones de catálogo y herramientas también se registran.

| Modelo | Campos obligatorios y tipos principales |
|---|---|
| ProjectManifest | schemaVersion:string, projectId:Id, title:string, language:string, aspectRatio:string, engine:{id,version}, designRef?:Id, artifacts:map de rutas, toolchain:map de versiones. |
| Requirements | schemaVersion, projectId, objective:string, audience:string, durationMinutes:number>0, requiredTopics:string[], outputs:Format[], constraints:string[], unresolvedQuestions:string[]. |
| Storyboard | schemaVersion, projectId, requirementsRevision, slides:{slideId,purpose,message,sourceIds:Id[],assetIds:Id[],estimatedSeconds:number>=0}[], unresolvedItems:string[]. |
| SourceRecord | id, kind:document/dataset/url/asset, location:string, hash?:SHA256, provenance:string, license:string, privacy:public/private/restricted, uses:{slideId,locator?:string}[]. |
| ChartSpec | id, datasetId, columns:{x,y}, units:map, type:bar/line, transform:{kind:identity}, missingPolicy:reject, caption:string, altText:string. |
| DesignManifest | id, version, family, tokens:map, layouts:string[], components:string[], fonts:{family,assetRef,license}[], capabilities:Format[], license:string. |
| ValidationReport | id, projectId, revision, profile, toolchain, inspected:{slideId,stateId,captureRef}[], findings:{id,severity,rule,slideId?,stateId?,evidenceRef,message}[], result:pass/fail/needs_review. |
| ExportReport | id, projectId, revision, format, files:{path,hash,mime}[], validationReportId, compatibility:{elementId?,loss,reason}[], verification:string[]. |
| ApprovalRecord | id, projectId, stage, revision, decision:approved/changes_requested, actor:string, timestamp, scope:string. |
| ProjectState | projectId, revision, stage, status, artifactRevisions:map, approvals:Id[], lastError?:OperationError. |

Format inicial: web/pdf. Formatos futuros: pptx/pptx-editable/png/video según capacidades. Metadatos de notes/layout pertenecen al Markdown de cada slide; el índice derivado aporta ID, posición, ruta y dependencias sin otro texto definitivo.

## Datos científicos del MVP

CSV UTF-8, separador coma y cabecera explícita; cabeceras únicas, sin filas con distinto número de columnas. ChartSpec selecciona x e y: x categórica para barras o numérica para líneas; y numérica finita. Columnas requeridas ausentes, valores vacíos o no numéricos fallan con fila/columna. El MVP no imputa, agrega ni cambia unidades automáticamente. Otros dialectos/análisis se añaden por contrato futuro.

Figuras incluyen data hash, ChartSpec hash, versión de generador y referencias a unidades. Referencias bibliográficas aportadas conservan identificador y localizador; una cita no verificada queda marcada y no se convierte en evidencia científica por parecer plausible.

## Resultado común

```ts
type OperationError = {
  code: 'INVALID_INPUT' | 'PATH_DENIED' | 'DEPENDENCY_MISSING'
    | 'REVISION_CONFLICT' | 'APPROVAL_REQUIRED' | 'CAPABILITY_UNSUPPORTED'
    | 'RENDER_FAILED' | 'VALIDATION_FAILED' | 'SOURCE_UNAUTHORIZED';
  message: string;
  recoverable: boolean;
  details?: Record<string, unknown>; // datos de diagnóstico sin secretos
};
type Result<T> = { ok: true; value: T; revision?: string; warnings: string[] }
  | { ok: false; error: OperationError };
```

Los contratos usan este envelope. Las operaciones mutantes declaran revisión esperada, escritura y aprobación aplicable. El lock y la política de recuperación se definen en PROJECT_LIFECYCLE. Salida CLI: código 0 en éxito, no cero en error y JSON coherente si se solicita. Advertencias no convierten un fallo en éxito.

## Contratos

| Operación | Definición propietaria |
|---|---|
| create_project | [Crear proyecto](contracts/create_project.md) |
| analyze_request | [Analizar solicitud](contracts/analyze_request.md) |
| create_storyboard | [Crear storyboard](contracts/create_storyboard.md) |
| suggest_designs | [Proponer diseños](contracts/suggest_designs.md) |
| select_design | [Seleccionar diseño](contracts/select_design.md) |
| generate_slides | [Generar diapositivas](contracts/generate_slides.md) |
| add_animation | [Añadir animación](contracts/add_animation.md) |
| add_interaction | [Añadir interacción](contracts/add_interaction.md) |
| update_slide | [Actualizar diapositiva](contracts/update_slide.md) |
| validate_presentation | [Validar presentación](contracts/validate_presentation.md) |
| export_presentation | [Exportar presentación](contracts/export_presentation.md) |

## CLI y catálogo de capacidades

`doctor`: diagnostica runtime, navegador, fuentes, agente y dependencias opcionales. `capabilities`: enumera motor, versión, operación, formatos, permisos, dependencias, límites y check de verificación. `create`: inicializa proyecto. `apply`: valida/aplica artefacto o cambio según contrato. `preview`: inicia motor local. `validate`, `export` y `status`: ejecutan/consultan operaciones respectivas.

El contrato de motor ofrece descriptor, disponibilidad, preview/render/export y reporte de compatibilidad. Su implementación exacta se fija en T-013; ninguna operación del MVP ejecuta un proveedor IA oculto. Un proveedor opcional no disponible devuelve CAPABILITY_UNSUPPORTED o DEPENDENCY_MISSING antes de modificar el proyecto.
