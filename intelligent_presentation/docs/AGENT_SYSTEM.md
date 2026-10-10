# Sistema de agentes y herramientas

**Propósito:** definir colaboración y portabilidad. **Cuándo leer:** al encargar trabajo o integrar Codex/AGY. Referencias: [instrucciones](../AGENTS.md), [CLI](DATA_MODELS.md), [PoC 1](pocs/01-slidev-agents.md).

## Roles de desarrollo

| Rol | Responsabilidad | Entrega / permiso |
|---|---|---|
| Coordinador SDD | Alcance, contratos, dependencias, decisiones e integración. | Especificaciones coherentes y revisión de entregas. |
| Investigador | Documentación, licencia, experimento autorizado. | Fuentes y límites; lectura por defecto. |
| Implementador | Una tarea preparada y archivos asignados. | Cambio pequeño y evidencia pertinente. |
| Especialista visual | Temas/componentes y composición. | Recursos y capturas; escritura en alcance definido. |
| Verificador independiente | Aceptación contrastada con ejecución real. | Defectos y reporte; no modifica producto por defecto. |
| Especialista científico | Datos, unidades, figuras y referencias. | Procedencia e invariantes; no inventa resultados. |

Son roles, no seis procesos permanentes. Un coordinador y hasta dos especialistas concurrentes inicialmente. Solo paralelizar trabajos independientes, investigaciones o revisión; modelos compartidos/lockfile tienen un propietario. Usar ramas/worktrees para implementación simultánea; el coordinador integra y resuelve conflictos.

Durante creación de presentaciones, un agente principal utiliza skills y herramientas. Delegar contenido científico/revisión visual cuando aporte ventaja y exista autorización aplicable. Los cálculos, checks y exportaciones siguen a cargo de herramientas deterministas.

## Contrato de encargo

Entrada: taskId, requirementIds, acceptanceIds, revisión base, objetivo, archivos autorizados, documentos mínimos, contratos, entradas y checks. Salida: resumen, artefactos, archivos modificados, revisión, checks ejecutados con evidencia, bloqueos, límites y siguiente paso. No transmite secretos ni todo el contexto del proyecto por defecto.

El verificador conoce requisito/rúbrica, no acepta una afirmación de éxito sin artefactos. La historia interna va a `.agents/`; requisitos y decisiones públicas permanecen versionados. Delegación no permite eludir permisos ni cambiar la especificación.

## Skills previstas

| Skill | Entrada | Salida |
|---|---|---|
| ips-specify | Necesidad y fuentes autorizadas. | Spec, preguntas, aceptación e IDs. |
| ips-plan | Spec preparada y evidencia técnica. | Plan, contratos y dependencias. |
| ips-implement-task | Tarea preparada y revisión base. | Cambio acotado y comprobación. |
| ips-verify | Artefacto y criterios de aceptación. | Reporte independiente. |
| ips-create-presentation | Solicitud, recursos y autorización. | Requisitos/storyboard, diseño y proyecto validado. |
| ips-edit-presentation | Revisión y cambio solicitado. | Cambio por ID, impacto e informe. |
| ips-review-export | Proyecto y formatos. | Capturas, resolución de defectos y entrega aprobada. |

Skills de animación/análisis avanzado llegan con sus fases. Cada SKILL.md tendrá nombre, descripción de activación, entradas, salidas, pasos, errores y finalización; instrucciones como primera opción y scripts solo para trabajo determinista.

## Fuentes y preparación local

Conservar SKILL.md versionados en `skills/` y fuentes de perfiles en `agent-configs/codex/` y `agent-configs/agy/`. Preparación genera copias locales en `.agents/skills/` y perfiles en ubicaciones descubiertas por la versión instalada. Registrar hashes; `doctor` detecta desactualización y ofrece regenerar copias sin sobrescribir cambios locales ajenos.

`.agents/` está ignorado por Git en el repositorio; no colocar allí la única copia de una skill reutilizable. No depender de symlinks con permisos especiales Windows. Usar perfiles del componente sin modificar configuración global. Las rutas/modelos/opciones exactos se verifican contra la superficie instalada; no asumir que CLI, IDE y app comparten configuración.

## Integración

Codex: AGENTS.md del componente, skills locales y perfiles adaptados a su versión; detectar CLI por separado de la aplicación. AGY: reglas AGENTS.md y skills compatibles, perfiles específicos comprobados con CLI 1.3.2 o la versión acordada. Fuentes oficiales de configuración están en [evaluación](TECHNOLOGY_EVALUATION.md).

CLI IPS ofrece JSON, diagnóstico y catálogo de capacidades; está disponible como vía portable aunque no exista MCP. MCP nativo Slidev es opcional y limitado a proyecto confiable. Puede dirigirse a slides por número: resolver ID estable → posición actual antes de mutar, comprobar revisión e invalidar reportes después. No exponer endpoint en interfaces públicas.

No se crea MCP propio en MVP. La conexión/configuración debe ser local y explícita. No se usa el modo de saltarse permisos de una plataforma para lograr reproducibilidad.

## Flujo de uso

Solicitud → requisitos/storyboard → aprobación → selección de tema → aprobación → generación → capturas/checks → correcciones acotadas → revisión humana → exportación → comprobación/entrega. Las reglas de recuperación y cambios posteriores pertenecen a [PROJECT_LIFECYCLE](PROJECT_LIFECYCLE.md).

## Evaluación de agentes

Ejecutar el mismo corpus y rúbrica con ambos agentes y registrar versión, modelo configurado, herramientas, iteraciones, duración y defectos. No fijar un proveedor/modelo obligatorio. Para reducir contexto, el encargo enlaza documentos propietarios y contrato puntual; nunca exige releer toda la planificación. El E2E determinista no invoca agentes externos ni requiere cuentas.
