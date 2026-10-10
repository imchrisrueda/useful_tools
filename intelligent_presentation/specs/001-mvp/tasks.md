# Tareas MVP — T-013 a T-025

**Cuándo leer:** al elegir/ejecutar una tarea MVP. Contexto común: [spec](spec.md), [plan](plan.md), [checklist](checklist.md). Estado de todas las tareas: pendientes; se necesita T-012 aceptada. Los tests que siguen son trabajo futuro, no resultados de esta entrega.

## T-013 — Schemas e interfaces (P1)

- Responsable: implementador core. Depende: T-012.
- Contexto: [modelos](../../docs/DATA_MODELS.md); abrir contrato asignado.
- Alcance/entrega: paquetes/schemas iniciales, Zod/tipos/JSON Schema y contrato de motor/CLI; fixtures válidos/inválidos. Propiedad central de tipos/lockfile.
- Aceptación: FR-009/010, NFR-009; contratos documentados coinciden con ejecutables, sin divergencia TS/JSON.
- Comprobación/evidencia: TEST-001, validación de fixtures y typecheck; dividir por modelos/contratos si son varios cambios independientes.

## T-014 — Proyectos y escritura segura (P1)

- Responsable: core. Depende: T-013.
- Contexto: [crear](../../docs/contracts/create_project.md), [ciclo](../../docs/PROJECT_LIFECYCLE.md).
- Alcance/entrega: creación/lectura, path confinement, staging, lock y revisión; sin sobrescritura de destinos ajenos.
- Aceptación: AC-011; lectura posterior válida y último estado intacto al fallar.
- Comprobación/evidencia: TEST-002 y TEST-012; destino ocupado, escape, enlaces externos, fallo de commit y concurrencia.

## T-015 — Artefactos, estado y aprobaciones (P1)

- Responsable: core/CLI. Depende: T-014.
- Contexto: [ciclo](../../docs/PROJECT_LIFECYCLE.md), contratos [requisitos](../../docs/contracts/analyze_request.md), [storyboard](../../docs/contracts/create_storyboard.md), [diseño](../../docs/contracts/select_design.md).
- Alcance/entrega: aplicar artifacts propuestos, status, revisión esperada y ApprovalRecord. No cliente IA oculto.
- Aceptación: AC-002/010/011; preguntas críticas bloquean avance, approvals provienen del usuario y cambian con inputs.
- Comprobación/evidencia: TEST-003, fallos de schema/revisión/aprobación y transiciones permitidas; dividir operaciones en cambios propios.

## T-016 — CSV y procedencia (P1)

- Responsable: especialista datos/core. Depende: T-013, T-014.
- Contexto: [datos científicos](../../docs/DATA_MODELS.md#datos-científicos-del-mvp), [requisitos](../../docs/PRODUCT_REQUIREMENTS.md).
- Alcance/entrega: lector CSV de dialecto declarado, SourceRecord/hash, selección de columnas/unidades y diagnóstico por fila/celda.
- Aceptación: AC-003/011; no imputar/agregar ni cambiar unidades implícitamente; fuentes privadas clasificadas.
- Comprobación/evidencia: TEST-004 con datos válidos, cabeceras duplicadas, filas irregulares, NaN/vacíos/columnas ausentes; corpus etiquetado.

## T-017 — Gráficos SVG reproducibles (P1)

- Responsable: datos/visual. Depende: T-016.
- Contexto: [ChartSpec](../../docs/DATA_MODELS.md), [diseño](../../docs/DESIGN_SYSTEM.md).
- Alcance/entrega: barras/líneas desde valores validados, captions/alt y manifiesto de procedencia; recursos independientes del layout.
- Aceptación: AC-003; valores/unidades fieles, parámetros y versión conservados; sin resultados IA.
- Comprobación/evidencia: TEST-005, comparar valores y etiquetas con fixture conocido, reproducir SVG normalizado y cambiar dataset para invalidar consumidores.

## T-018 — Integración Slidev y preview (P1)

- Responsable: engine-slidev. Depende: T-015.
- Contexto: [generar](../../docs/contracts/generate_slides.md), [arquitectura](../../docs/ARCHITECTURE.md), evidencia PoC 1.
- Alcance/entrega: aplicación Markdown por slide, orden/inclusiones, mapping ID→posición, resources y preview loopback; contrato de renderer.
- Aceptación: AC-002/004/011; fuentes/notas persistentes y documentos importados no ejecutados.
- Comprobación/evidencia: TEST-006, reinicio desde fuentes, IDs/notas/orden y recursos; errores de dependencias explícitos.

## T-019 — Temas y layouts MVP (P1)

- Responsable: visual/engine. Depende: T-018, T-011.
- Contexto: [diseño](../../docs/DESIGN_SYSTEM.md), [propuestas](../../docs/contracts/suggest_designs.md).
- Alcance/entrega: catálogo de dos temas, siete layouts, tokens y fonts con licencia; selección humana y previews.
- Aceptación: AC-002/006/007; mismo contenido en ambos temas y no ofrecer capacidades ficticias.
- Comprobación/evidencia: TEST-007, corpus completo y invariantes; revisar contraste y fuentes locales. Cambios por tema/layout separables.

## T-020 — Edición e invalidación (P1)

- Responsable: core/engine con propiedad asignada. Depende: T-017–T-019.
- Contexto: [actualizar](../../docs/contracts/update_slide.md), [ciclo](../../docs/PROJECT_LIFECYCLE.md).
- Alcance/entrega: edición por ID, conflicto/revisión, mapping y caché dependiente de datos/assets/tema/toolchain.
- Aceptación: AC-008/010; otras fuentes idénticas, consumidores compartidos invalidados y aprobación final obsoleta.
- Comprobación/evidencia: TEST-008, comparar hashes/bytes de fuentes ajenas, cache hits y errores de revisión; cambio semántico requiere aprobación.

## T-021 — Validación y capturas completas (P1)

- Responsable: quality. Depende: T-010, T-018; integración final con T-019/020.
- Contexto: [validar](../../docs/contracts/validate_presentation.md), [calidad](../../docs/QUALITY_STRATEGY.md).
- Alcance/entrega: cobertura por estado, capture, findings estructurales/visual/a11y, revisión manual y reporte actual.
- Aceptación: AC-004–007/010; reportar elementos no evaluables y bloquear error/blocker.
- Comprobación/evidencia: TEST-009, corpus sano/defectuoso, fuentes/listos, bounds, recursos, contraste, teclado y obsolescencia. Evitar aprobación automática de warnings subjetivas.

## T-022 — Exportación y entrega (P1)

- Responsable: CLI/engine. Depende: T-021 y recursos T-017/019.
- Contexto: [exportar](../../docs/contracts/export_presentation.md), [calidad](../../docs/QUALITY_STRATEGY.md).
- Alcance/entrega: web/PDF, ExportReport, fuentes autorizadas, toolchain y guía de reconstrucción; sin publicación automática.
- Aceptación: AC-009/012; outputs coherentes y aprobación/report actual obligatorios; secrets/datos no autorizados excluidos.
- Comprobación/evidencia: TEST-010/012, abrir outputs, comprobar páginas/estados/recursos y reconstruir sin modelo. Reportar pérdidas, incluso estáticas.

## T-023 — Skills de producto sobre CLI (P1)

- Responsable: integración de agentes. Depende: T-008, T-020–T-022.
- Contexto: [agentes](../../docs/AGENT_SYSTEM.md), [ciclo](../../docs/PROJECT_LIFECYCLE.md).
- Alcance/entrega: ips-create/edit/review-export, copias/perfiles y catálogo; flujo semántico común con CLI/MCP opcional.
- Aceptación: AC-001/002/011; prompts activan skill correcta, gates humanos explícitos y contexto reducido.
- Comprobación/evidencia: TEST-011, creación/edición/revisión con ambos agentes, autorizaciones y límites declarados; no modificar config global.

## T-024 — Ejemplo, E2E y guía Windows (P1)

- Responsable: verificador/científico. Depende: T-023.
- Contexto: [spec](spec.md), [calidad](../../docs/QUALITY_STRATEGY.md).
- Alcance/entrega: ejemplo científico público de 15 slides con notas, fuentes verificadas y CSV sintético; pruebas completas y procedimiento Windows.
- Aceptación: AC-001–012; E2E determinista sin IA y evaluación separada de agentes; narrativa/rigor revisados.
- Comprobación/evidencia: TEST-001–012 aplicables; bundles/input versions/reportes y reproducción en ambiente limpio. Ningún dato sintético presentado como experimento real.

## T-025 — CI y cierre del MVP (P1)

- Responsable: coordinador/verificador. Depende: T-024.
- Contexto: [checklist](checklist.md), [calidad](../../docs/QUALITY_STRATEGY.md), instrucciones del repositorio padre.
- Alcance/entrega: workflow limitado al componente con working-directory explícito, Windows principal/Linux secundaria y revisión de aceptación. Cambios .github del padre requieren permisos aplicables.
- Aceptación: AC-001–012 completos, sin fallos ocultos; métricas/reporte/limitaciones y aprobación final.
- Comprobación/evidencia: runs de CI real o bloqueo explícito, revisión independiente y decisión humana; no simular éxito CI con checks locales.
