# Índice de lectura selectiva

**Propósito:** localizar el contexto necesario para una tarea. **Cuándo leer:** al comenzar un trabajo sin un encargo preciso o cuando falte información. Este índice no es una lectura obligatoria adicional si la tarea ya enlaza su contexto.

## Rutas de entrada

| Objetivo | Leer primero | Consultar después si aplica |
|---|---|---|
| Elegir trabajo | [Backlog](../TASKS.md) | Especificación de la fase asignada |
| Aclarar alcance | [Requisitos](PRODUCT_REQUIREMENTS.md) | [Decisiones](DECISIONS.md) |
| Diseñar módulos | [Arquitectura](ARCHITECTURE.md) | [Componentes](COMPONENTS.md), [modelos](DATA_MODELS.md) |
| Cambiar una operación | [Contratos](DATA_MODELS.md#contratos) | [Ciclo](PROJECT_LIFECYCLE.md) |
| Integrar Codex o AGY | [Agentes](AGENT_SYSTEM.md) | [PoC 1](pocs/01-slidev-agents.md) |
| Diseñar diapositivas | [Diseño](DESIGN_SYSTEM.md) | [Calidad](QUALITY_STRATEGY.md) |
| Procesar datos | [Modelos](DATA_MODELS.md) | [Requisitos](PRODUCT_REQUIREMENTS.md), [calidad](QUALITY_STRATEGY.md) |
| Evaluar una herramienta | [Evaluación](TECHNOLOGY_EVALUATION.md) | Una [PoC](POC_PLAN.md) y su decisión |
| Revisar/exportar | [Calidad](QUALITY_STRATEGY.md) | [Validación](contracts/validate_presentation.md), [exportación](contracts/export_presentation.md) |
| Coordinar fases | [Roadmap](ROADMAP.md) | [Riesgos](RISKS.md), [trazabilidad](../specs/TRACEABILITY.md) |

## Especificaciones ejecutables por fase

- [Constitución SDD](../specs/CONSTITUTION.md): reglas comunes del desarrollo.
- [Trazabilidad](../specs/TRACEABILITY.md): mapa entre requisitos, aceptación, tareas y pruebas previstas.
- Evaluación: [spec](../specs/000-evaluation/spec.md), [plan](../specs/000-evaluation/plan.md), [tareas](../specs/000-evaluation/tasks.md), [checklist](../specs/000-evaluation/checklist.md).
- MVP: [spec](../specs/001-mvp/spec.md), [plan](../specs/001-mvp/plan.md), [tareas](../specs/001-mvp/tasks.md), [checklist](../specs/001-mvp/checklist.md).

## Propiedad documental

Los requisitos pertenecen a PRODUCT_REQUIREMENTS; las interfaces a DATA_MODELS y contracts; el estado a PROJECT_LIFECYCLE; los pasos experimentales a cada PoC; las tareas a su fase. Otros documentos enlazan esas definiciones. README resume la propuesta sin convertirse en otra especificación.

Las fuentes oficiales se concentran en TECHNOLOGY_EVALUATION. Los resultados futuros se registrarán junto a la PoC y se resumirán en DECISIONS; actualmente todas las PoC están sin ejecutar.
