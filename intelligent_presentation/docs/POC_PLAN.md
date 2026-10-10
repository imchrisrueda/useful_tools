# Plan de pruebas de concepto

**Propósito:** localizar protocolos independientes. **Cuándo leer:** al elegir o coordinar un experimento. Referencias: [roadmap](ROADMAP.md), [riesgos](RISKS.md), [evaluación](TECHNOLOGY_EVALUATION.md).

**Estado de todas las PoC: no ejecutadas.** Las dependencias y comandos se fijan antes de la ejecución, no se instalan con esta documentación.

| PoC | Protocolo | Puerta / decisión |
|---|---|---|
| 1 | [Slidev y agentes](pocs/01-slidev-agents.md) | Fase 0; ADR-005/006/009. |
| 2 | [Manim](pocs/02-manim.md) | Antes de fase 3; ADR-013. |
| 3 | [Interactividad](pocs/03-interactivity.md) | Antes de fase 3; ADR-014. |
| 4 | [PPTX editable](pocs/04-editable-pptx.md) | Antes de fase 4; ADR-015. |
| 5 | [Validación](pocs/05-validation.md) | Fase 0; ADR-012. |
| 6 | [Plantillas](pocs/06-templates.md) | Fase 0; ADR-006/010. |
| 7 | [Presenton](pocs/07-presenton.md) | Incorporación opcional; ADR-016. |

## Corpus y entorno comunes

Cada experimento usa un directorio propio dentro del componente y un fixture público pequeño. Preparar cinco slides base: portada, texto con notas/cita aportada, figura con caption, tabla y gráfico sobre CSV sintético etiquetado. El corpus de exportación añade fórmula, SVG, vídeo y efectos CSS documentados. Mantener versión/hash de fuente y recursos.

Usar Node/npm/Chromium/fuentes fijados; Python únicamente para PoC 2, Docker/servicio únicamente para PoC 7. Configuración global y documentos privados reales quedan fuera del experimento. La autorización de ejecución debe abarcar los proveedores/datos usados por agentes.

## Registro de resultado

Cada run guarda manifiesto: fecha, responsable, commit/tag/lockfile, OS, herramientas, hardware, entradas y hashes, pasos/comandos exactos, duración, tamaños, capturas, comprobaciones y fallos. Resultado: success/failed/blocked; hipótesis revisada y efecto en ADR.

Fixtures/scripts nuevos se incorporan al autorizar ejecución; las fuentes de resultados públicos deben ser redistribuibles. Evidencia temporal en `.agents/`; resumen y artefactos seleccionados en documentación pública o fixtures de tests. Un screenshot sin inputs/versiones no es experimento reproducible.

Las PoC 1/5/6 pueden compartir corpus y ejecutarse independientemente después de preparar entorno. Antes de MVP se necesita cerrar las tres. Los resultados de PoC posteriores no se presuponen por usar el mismo navegador.
