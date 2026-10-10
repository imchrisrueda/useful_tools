# Tareas de evaluación — T-001 a T-012

**Cuándo leer:** al elegir/ejecutar una tarea de fase 0. Referencias comunes: [spec](spec.md), [plan](plan.md), [checklist](checklist.md). Cada tarea enlaza el contexto adicional mínimo.

Estado: T-001–006 disponen de documentos candidatos de esta entrega y están pendientes de aceptación. T-007–012 no ejecutadas. Dependencias significan tareas aceptadas, no solo archivos presentes.

## T-001 — Requisitos y cobertura (P0)

- Responsable: coordinador SDD. Depende: ninguna.
- Inputs/contexto: [brief](../../idea.md), preferencias confirmadas del README.
- Alcance/entrega: [requisitos](../../docs/PRODUCT_REQUIREMENTS.md) y [trazabilidad](../TRACEABILITY.md); no editar idea.md.
- Aceptación: EV-001; casos A–F, restricciones, documentos obligatorios y exclusiones mapeados con IDs.
- Comprobación/evidencia: revisión de cobertura y contradicciones con verificador; registro de faltantes/resolución.

## T-002 — Constitución y método SDD (P0)

- Responsable: coordinador SDD. Depende: T-001.
- Contexto: [constitución](../CONSTITUTION.md), [instrucciones](../../AGENTS.md).
- Alcance/entrega: plantillas por capacidad, IDs, definición de preparado/terminado y lectura selectiva.
- Aceptación: una tarea puede rastrearse a requisito/aceptación sin releer todo; excepciones explícitas.
- Comprobación/evidencia: recorrer un encargo de ejemplo y verificar documentos mínimos, gates y propietario por tema.

## T-003 — Evaluación tecnológica y licencias (P0)

- Responsable: investigador. Depende: T-001.
- Contexto: [evaluación](../../docs/TECHNOLOGY_EVALUATION.md), [riesgos](../../docs/RISKS.md).
- Alcance/entrega: completar candidatos relevantes con documentación oficial, release/tag, LICENSE, dependencias y mantenimiento; matriz justificada.
- Aceptación: diferenciar documentado/probado/hipótesis; ninguna adopción obligatoria sin licencia/versiones comprobadas.
- Comprobación/evidencia: referencias primarias por capacidad y registro de fecha; candidatos descartados con razón. No instalar candidatos para investigación documental.

## T-004 — Arquitectura, contratos y estado (P0)

- Responsable: arquitecto. Depende: T-002, T-003.
- Contexto: [arquitectura](../../docs/ARCHITECTURE.md), [modelos](../../docs/DATA_MODELS.md), [ciclo](../../docs/PROJECT_LIFECYCLE.md).
- Alcance/entrega: cuatro paquetes, contratos once operaciones, diseño de estado/revisión/seguridad e interfaces de extensión.
- Aceptación: EV-002; fuente de verdad única por dato, sin ciclos entre módulos ni ejecución IA oculta.
- Comprobación/evidencia: revisión de entradas/salidas/precondiciones y escenarios de actualización/conflicto/export obsoleto.

## T-005 — Corpus y protocolos PoC (P0)

- Responsable: investigador y verificador. Depende: T-004.
- Contexto: [índice PoC](../../docs/POC_PLAN.md); abrir solo protocolo asignado.
- Alcance/entrega: siete protocolos y corpus de cinco slides con texto/notas, figura, tabla y CSV sintético; añadir casos avanzados para PoC 4.
- Aceptación: EV-003; inputs con procedencia/licencia y procedimiento/métricas/éxito/consecuencias completos. Para fuentes científicas usar referencias públicas verificadas, sin placeholders presentados como citas reales.
- Comprobación/evidencia: un segundo agente puede ejecutar el protocolo con sus instrucciones/locks; no afirmar que ha sido ejecutado.

## T-006 — Documentación y consistencia (P0)

- Responsable: verificador documental. Depende: T-001–T-005.
- Contexto: [índice](../../docs/INDEX.md), [decisiones](../../docs/DECISIONS.md), [roadmap](../../docs/ROADMAP.md).
- Alcance/entrega: documentos obligatorios completos y enlazados, TASKS y aceptación preparada.
- Aceptación: EV-001–003; documentos propietarios sin duplicar interfaces/requisitos; lectura por rol/tarea.
- Comprobación/evidencia: enlaces/IDs válidos, cobertura del brief, dependencias sin ciclos y decisiones con estado; informar dimensiones de investigación pendientes.

## T-007 — Runtime y diagnóstico local (P0)

- Responsable: implementador entorno. Depende: T-004, T-005; G-0 revisada antes de instalaciones.
- Contexto: [arquitectura](../../docs/ARCHITECTURE.md), [evaluación](../../docs/TECHNOLOGY_EVALUATION.md).
- Alcance/entrega: scripts locales, Node 24 con checksum, npm/lock, motor/navegador/fonts y diagnóstico en directorio de experimentos; no desarrollar aún core.
- Aceptación: EV-004; ambiente reproducible y sin modificar herramientas/config global. No instalar dependencias de PoC futuras.
- Comprobación/evidencia: versiones, checksum, arranque/render corpus, permisos y comandos exactos en Windows. Dividir bootstrap/diagnóstico si requieren cambios independientes.

## T-008 — Skills/perfiles y descubrimiento (P0)

- Responsable: integración de agentes. Depende: T-002, T-007.
- Contexto: [agentes](../../docs/AGENT_SYSTEM.md), [PoC 1](../../docs/pocs/01-slidev-agents.md).
- Alcance/entrega: fuentes en skills/agent-configs y preparación local por copia con hashes; probar descubrimiento por superficie real.
- Aceptación: Codex y AGY leen procedimiento asignado con contexto mínimo; copias desactualizadas se detectan; config global intacta.
- Comprobación/evidencia: versiones/superficie/rutas y resultado de descubrimiento, sin secretos. Si CLI ausente, documentar vía app/IDE o bloqueo de esa dimensión.

## T-009 — Ejecutar PoC 1 (P0)

- Responsable: investigador agentes. Depende: T-005, T-008.
- Contexto: [protocolo 1](../../docs/pocs/01-slidev-agents.md).
- Alcance/entrega: runs separados por agente y CLI/MCP, fuentes/capturas y resumen de resultado.
- Aceptación: EV-005; vía portable pasa con ambos; MCP evaluado por separado sin falsear soporte.
- Comprobación/evidencia: hashes, operaciones, notas/orden y render completo; actualizar ADR-005/006/009 según evidencia.

## T-010 — Ejecutar PoC 5 (P0)

- Responsable: verificador calidad. Depende: T-005, T-007.
- Contexto: [protocolo 5](../../docs/pocs/05-validation.md), [calidad](../../docs/QUALITY_STRATEGY.md).
- Alcance/entrega: fixtures sano/defectuosos y detector experimental aislado; evidencias antes/después.
- Aceptación: EV-006; tres clases detectadas, cobertura completa, reparación preserva significado y reporte obsoleto reconocido.
- Comprobación/evidencia: findings localizados, falsos positivos y calibración registrada; actualizar ADR-012.

## T-011 — Ejecutar PoC 6 (P0)

- Responsable: especialista visual. Depende: T-005, T-007.
- Contexto: [protocolo 6](../../docs/pocs/06-templates.md), [diseño](../../docs/DESIGN_SYSTEM.md).
- Alcance/entrega: dos temas experimentales, manifest/capturas y comparación del corpus idéntico.
- Aceptación: EV-006; texto/notas/datos invariantes, layouts completos legibles y sin duplicar contenido.
- Comprobación/evidencia: hashes y revisión completa de capturas; actualizar ADR-006/010.

## T-012 — Cerrar decisiones MVP (P0)

- Responsable: coordinador y verificador independiente. Depende: T-006, T-009–T-011.
- Contexto: [decisiones](../../docs/DECISIONS.md), [riesgos](../../docs/RISKS.md), [MVP spec](../001-mvp/spec.md).
- Alcance/entrega: consolidación de resultados y propuesta concreta de arquitectura/contratos para aprobación humana.
- Aceptación: EV-007; PoC críticas pasan, riesgos críticos resueltos y aprobación explícita antes de core. Si falla, revisar alternativa y tareas dependientes.
- Comprobación/evidencia: checklist de fase con enlaces a runs y decisión humana; no cerrar por agotamiento de tiempo o existencia de documentos.
