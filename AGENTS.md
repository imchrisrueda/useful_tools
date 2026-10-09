# Guía de agentes para useful_tools

Este repositorio reúne herramientas, skills y procedimientos para desarrollo con IA.
Las instrucciones generales se aplican a todo el repositorio; cada componente puede
ampliarlas con su propio `AGENTS.md`.

## Organización

- `herdr_usage/`: fork de `herdr-agent-usage`. Antes de editarlo o interactuar con Herdr,
  lee [herdr_usage/AGENTS.md](herdr_usage/AGENTS.md) y aplica sus restricciones y gates.
- `skills/`: skills reutilizables, cada una en `skills/<nombre>/SKILL.md`.
- `procedures/`: procedimientos documentados con requisitos, pasos y verificación.
- `.github/`: workflows y plantillas compartidas. Los workflows deben declarar
  el directorio del componente al que se aplican.

## Forma de trabajo

1. Define el alcance e inspecciona `git status` y el diff antes de editar.
   Los cambios ajenos pertenecen al usuario.
2. Mantén las dependencias, fuentes, tests, assets y documentación de cada herramienta
   en su directorio. Ejecuta sus comandos desde ese directorio.
3. Aplica cambios pequeños y verificables. Distingue hechos, inferencias e incógnitas.
4. Usa el toolchain fijado por el componente y artefactos locales; no instales
   tooling global sin necesidad ni alteres configuraciones reales para verificar cambios.
5. Ejecuta los gates del componente afectado y comunica cualquier comprobación bloqueada.
6. Guarda decisiones y planes internos de trabajos con varios objetivos en `.agents/`,
   ignorado por Git. La documentación pública describe el contenido y comportamiento entregados.
7. Conserva la atribución y licencia de los componentes derivados de otros proyectos.
