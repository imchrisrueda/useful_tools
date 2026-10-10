# Trazabilidad de planificación y pruebas

**Propósito:** localizar cobertura sin duplicar definiciones. **Cuándo leer:** al revisar impacto o aceptación. Propietarios: [FR/NFR](../docs/PRODUCT_REQUIREMENTS.md), [AC](001-mvp/spec.md), [EV](000-evaluation/spec.md).

## Requisitos MVP

| Requisitos | Aceptación | Tareas | Tests previstos |
|---|---|---|---|
| FR-001/002 | AC-001/002 | T-015, T-023/024 | TEST-003/011 |
| FR-003 | AC-002/006/007 | T-019, T-023 | TEST-007/011 |
| FR-004 | AC-001/002 | T-018, T-023/024 | TEST-006/011 |
| FR-005 | AC-004–007 | T-021, T-024 | TEST-009 |
| FR-006 | AC-008/010 | T-020 | TEST-008 |
| FR-007 | AC-009/012 | T-022, T-024 | TEST-010 |
| FR-008 | AC-003 | T-016/017 | TEST-004/005 |
| FR-009 | AC-002/010 | T-013–015, T-020/021 | TEST-001–003/008/009 |
| FR-010 | AC-001/011 | T-008/009, T-023/025 | TEST-011/012 |

## Propiedades transversales

| Requisitos | Aceptación | Tareas | Evidencia prevista |
|---|---|---|---|
| NFR-001 | AC-001/009/012 | T-007, T-024/025 | Ambiente Windows y CI. |
| NFR-002 | AC-003/009/012 | T-017/022/024 | TEST-005/010. |
| NFR-003/004 | AC-011/012 | T-014/015/022 | TEST-002/012. |
| NFR-005 | AC-002/003/008 | T-015–017, T-020/024 | Invariantes y revisión científica. |
| NFR-006/007 | AC-004–007 | T-010/019/021 | TEST-007/009 y revisión humana. |
| NFR-008 | AC-008/010 | T-020 | TEST-008 y medición de caché. |
| NFR-009 | AC-010/011 | T-013/015/025 | TEST-001/003 y revisión de dependencias. |
| NFR-010 | AC-009/011 | T-023–025 | Core sin SDK IA; E2E sin credenciales. |
| NFR-011/012 | AC-012 | T-003/019/022 | Inventario y ExportReport. |

## Pruebas: estado inicial de todas = not_run

| ID | Escenario | Evidencia esperada |
|---|---|---|
| TEST-001 | Schemas/modelos compatibles e incompatibles. | Fixtures, aserciones y tipos/JSON Schema coherentes. |
| TEST-002 | Proyectos/rutas/lock/staging. | Pruebas de no sobrescritura, escapes y fallo atómico. |
| TEST-003 | Estados, revisiones y aprobaciones. | Transiciones/errores e invalidación real. |
| TEST-004 | CSV válido e inválido. | Diagnóstico por columna/fila y valores esperados. |
| TEST-005 | SVG reproducible y datos/unidades. | Valores/captions, hashes y resultado normalizado. |
| TEST-006 | Slidev, IDs, notas y recursos. | Fuentes/render después de reinicio. |
| TEST-007 | Temas/layouts equivalentes. | Capturas completas e invariantes de contenido. |
| TEST-008 | Edición y caché incremental. | Fuentes ajenas idénticas, consumidores invalidados y cache hits. |
| TEST-009 | Defectos, cobertura y accesibilidad. | Findings/capturas antes/después y revisión manual. |
| TEST-010 | Web/PDF y reconstrucción. | Apertura, contenido, manifest y outputs normalizados. |
| TEST-011 | Flujo con ambos agentes. | Rúbrica, inputs/versiones, gates y entrega. |
| TEST-012 | Permisos, privacidad y secretos. | Casos negativos y bundle/log sin contenido no autorizado. |

No existen tests implementados todavía. Estados futuros: passed, failed, blocked o not_run con fecha, revisión, entorno y enlace a evidencia.

## Preparación y extensiones

EV-001–003 → T-001–006 y revisión documental. EV-004 → T-007/008. EV-005 → T-009. EV-006 → T-010/011. EV-007 → T-012. Las PoC tienen criterios propios en su protocolo, no se confunden con tests de producto.

| Requisitos futuros | Fase / protocolo | Spec a preparar al entrar en fase |
|---|---|---|
| FR-011 | 2 / PoC 6 ampliada | Recomendación y propuestas comparables. |
| FR-012 | 3 / PoC 2 | Animación/control y alternativa estática. |
| FR-013 | 3 / PoC 3 | Interacción, estados y accesibilidad. |
| FR-014/015 | 4 / PoC 4 | Exportación por objeto e importación independiente. |
| FR-016 | 3–5 / experimento de datos | Lectores/análisis reproducible según necesidad. |
| FR-017 | 5 / PoC 7 si aplica | Automatización basada en métricas, proveedor opcional. |

## Cobertura del brief

Secciones 1–4 → requisitos/casos; 5/17 → evaluación/decisiones; 6/15 → arquitectura/componentes; 7/16 → modelos/contratos; 8 → diseño; 9 → agentes; 10 → ciclo; 11/12 → requisitos/calidad/riesgos; 13 → siete PoC; 14 → roadmap/tareas; 18 → índice documental; 19/20 → constitución/instrucciones/estado; 21 → README, arquitectura y backlog. Los diagramas pedidos están en arquitectura, componentes, ciclo, roadmap y evaluación de exportación; se amplían con evidencia si cambia el diseño.
