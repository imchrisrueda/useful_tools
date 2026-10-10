# Backlog y entrada por tarea

Estado actual: **Fase 1 (MVP), Fase 2 (Diseño) y Fase 3 (Animación e Interactividad) completadas y verificadas**.
21 pruebas automatizadas pasando al 100% en Node y Python. Presentación piloto generada y validada en `backpropagation-neural-network/`.

| Prioridad / fase | Tareas | Estado | Documento propietario |
|---|---|---|---|
| P0: preparación documental | T-001–T-006 | Completado | [Evaluación](specs/000-evaluation/tasks.md) |
| P0: entorno y evidencia arquitectónica | T-007–T-012 | Completado | [Evaluación](specs/000-evaluation/tasks.md) |
| P1: implementación MVP | T-013–T-025 | Completado | [MVP](specs/001-mvp/tasks.md) |
| P2: Fase 2 (5 Familias de Diseño & Recom.) | Implementado | Completado | [Roadmap](docs/ROADMAP.md) / [Diseño](docs/DESIGN_SYSTEM.md) |
| P3: Fase 3 (Animaciones Manim & Interactividad) | Implementado | Completado | [Roadmap](docs/ROADMAP.md) / [Contratos](docs/contracts/add_animation.md) |
| P4: Fase 4 (Exportación Avanzada PPTX) | Pendiente | En backlog | [Roadmap](docs/ROADMAP.md) |
| P5: Fase 5 (Automatización & Optimización) | Pendiente | En backlog | [Roadmap](docs/ROADMAP.md) |

## Cómo usar el backlog

Elige la primera tarea cuya preparación y dependencias estén satisfechas. Lee el bloque de tarea, su spec y el contexto enlazado; no todo el directorio docs. Cada tarea produce un cambio revisable y evidencia. Si un bloque necesita más de un cambio independiente, subdividir como T-xxx.a/b sin cambiar su requisito padre.

T-001–T-006 tienen documentos candidatos en esta entrega y requieren revisión para aceptarse. T-007–T-025 permanecen pendientes. La entrega actual no autoriza instalar dependencias ni implementar el sistema fuera de una tarea posterior solicitada.

T-016 (datos) y T-018 (motor) pueden avanzar en paralelo después de T-013–T-015, con contratos estabilizados y propiedad de archivos. Esquemas/lockfile permanecen bajo un propietario. Especialistas de revisión/investigación pueden trabajar en paralelo sin editar fuentes compartidas.

## Acceso directo a cada encargo

Los enlaces apuntan al bloque concreto; extrae ese bloque y abre únicamente su contexto. Las dependencias detalladas pertenecen a la tarea enlazada.

| Tarea | Objetivo |
|---|---|
| [T-001](specs/000-evaluation/tasks.md#t-001--requisitos-y-cobertura-p0) | Requisitos y cobertura |
| [T-002](specs/000-evaluation/tasks.md#t-002--constitución-y-método-sdd-p0) | Constitución SDD |
| [T-003](specs/000-evaluation/tasks.md#t-003--evaluación-tecnológica-y-licencias-p0) | Tecnología y licencias |
| [T-004](specs/000-evaluation/tasks.md#t-004--arquitectura-contratos-y-estado-p0) | Arquitectura y contratos |
| [T-005](specs/000-evaluation/tasks.md#t-005--corpus-y-protocolos-poc-p0) | Protocolos y corpus |
| [T-006](specs/000-evaluation/tasks.md#t-006--documentación-y-consistencia-p0) | Coherencia documental |
| [T-007](specs/000-evaluation/tasks.md#t-007--runtime-y-diagnóstico-local-p0) | Runtime local |
| [T-008](specs/000-evaluation/tasks.md#t-008--skillsperfiles-y-descubrimiento-p0) | Skills y perfiles |
| [T-009](specs/000-evaluation/tasks.md#t-009--ejecutar-poc-1-p0) | PoC de agentes |
| [T-010](specs/000-evaluation/tasks.md#t-010--ejecutar-poc-5-p0) | PoC de validación |
| [T-011](specs/000-evaluation/tasks.md#t-011--ejecutar-poc-6-p0) | PoC de temas |
| [T-012](specs/000-evaluation/tasks.md#t-012--cerrar-decisiones-mvp-p0) | Cierre de arquitectura |
| [T-013](specs/001-mvp/tasks.md#t-013--schemas-e-interfaces-p1) | Esquemas e interfaces |
| [T-014](specs/001-mvp/tasks.md#t-014--proyectos-y-escritura-segura-p1) | Persistencia segura |
| [T-015](specs/001-mvp/tasks.md#t-015--artefactos-estado-y-aprobaciones-p1) | Estado y aprobaciones |
| [T-016](specs/001-mvp/tasks.md#t-016--csv-y-procedencia-p1) | CSV y procedencia |
| [T-017](specs/001-mvp/tasks.md#t-017--gráficos-svg-reproducibles-p1) | Gráficos SVG |
| [T-018](specs/001-mvp/tasks.md#t-018--integración-slidev-y-preview-p1) | Motor y preview |
| [T-019](specs/001-mvp/tasks.md#t-019--temas-y-layouts-mvp-p1) | Temas y layouts |
| [T-020](specs/001-mvp/tasks.md#t-020--edición-e-invalidación-p1) | Edición incremental |
| [T-021](specs/001-mvp/tasks.md#t-021--validación-y-capturas-completas-p1) | Validación completa |
| [T-022](specs/001-mvp/tasks.md#t-022--exportación-y-entrega-p1) | Web/PDF y entrega |
| [T-023](specs/001-mvp/tasks.md#t-023--skills-de-producto-sobre-cli-p1) | Skills de uso |
| [T-024](specs/001-mvp/tasks.md#t-024--ejemplo-e2e-y-guía-windows-p1) | Ejemplo y E2E |
| [T-025](specs/001-mvp/tasks.md#t-025--ci-y-cierre-del-mvp-p1) | CI y aceptación |

## Evidencia y estado

Estados: pending, ready, running, blocked, done. Registrar motivo de blocked y evidencia de done. Planes y traspasos internos van en `.agents/`; referencias duraderas y estado público relevante se enlazan desde la tarea. Mapa de requisitos/checks en [trazabilidad](specs/TRACEABILITY.md).
