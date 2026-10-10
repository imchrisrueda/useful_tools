# Riesgos, incertidumbres y mitigaciones

**Propósito:** concentrar bloqueos y evidencia necesaria. **Cuándo leer:** al preparar tareas o revisar una puerta. Referencias: [decisiones](DECISIONS.md), [PoC](POC_PLAN.md), [calidad](QUALITY_STRATEGY.md).

Probabilidad/impacto son valoraciones iniciales cualitativas, no mediciones. Todos los riesgos experimentales están abiertos.

| ID | Riesgo | Probabilidad / impacto | Mitigación y evidencia | Responsable |
|---|---|---|---|---|
| R-001 | Docs/main no corresponden a release o agente instalado. | Alta / alto | Fijar release; doctor; PoC 1 por plataforma. | Investigador |
| R-002 | Node local incompatible. | Confirmado en inspección / alto | Runtime 24 LTS local con checksum y diagnóstico. | Implementador entorno |
| R-003 | Markdown/Vue o Python importado ejecuta código no confiable. | Media / crítico | Importar como datos, rutas confinadas, ejecución autorizada. | Coordinador/core |
| R-004 | Datos privados enviados o secrets en logs/bundle. | Media / crítico | Autorización por destino; clasificación; saneamiento y tests negativos. | Coordinador/core |
| R-005 | Reparación visual altera resultados, unidades o referencias. | Media / crítico | Invariantes antes/después y revisión científica. | Especialista científico |
| R-006 | Validación geométrica genera falsos positivos/negativos. | Alta / alto | Corpus defectuoso y sano; calibración PoC 5; revisión humana. | Verificador |
| R-007 | PPTX editable pierde objetos, notas, wrapping o fuentes. | Alta / alto | PoC 4 por objeto, OOXML y apertura PowerPoint; reporte de pérdidas. | Exportación |
| R-008 | Animación/interacción se pierde en exportación estática. | Confirmado conceptualmente / alto | Estado de exportación explícito, poster/alt y compatibilidad. | Motor/calidad |
| R-009 | Manim/Qt/LaTeX y costes de render dificultan Windows. | Media / alto | Python separado; PoC 2; alternativa WSL2/Docker autorizada. | Animación |
| R-010 | Caché o aprobaciones usan revisión obsoleta. | Media / alto | Hash de dependencias y tests de invalidación. | Core |
| R-011 | Agentes sobrescriben fuentes simultáneamente. | Media / alto | Propiedad de archivos, worktrees, revisión esperada y lock. | Coordinador |
| R-012 | Fuente de verdad duplicada al añadir IR/PPTX. | Media / alto | IR derivada acotada; contrato y migración explícitos. | Arquitecto |
| R-013 | Licencia o atribución de assets/dependencias incompleta. | Media / alto | Inventario por release, fuente y licencia; gate de entrega. | Investigador |
| R-014 | Presenton añade servicio, coste o transmisión no deseados. | Media / alto | Fuera por defecto; PoC 7 local y revisión de red. | Investigador |
| R-015 | Sobrecarga de documentos o multiagentes sin valor. | Media / medio | Lectura selectiva; propietario por tema; roles bajo demanda. | Coordinador SDD |
| R-016 | Capturas distintas por fuentes/OS/navegador. | Alta / medio | Ambiente fijado, baseline por entorno y checks semánticos. | Calidad |
| R-017 | No hay PowerPoint disponible para verificar compatibilidad real. | Desconocida / alto en fase 4 | Resultado blocked en esa dimensión; OOXML/LibreOffice no sustituyen PowerPoint. | Verificador exportación |

## Conflictos de requisitos

Editabilidad y fidelidad visual pueden competir; interactividad web no se conserva íntegra en PDF/PPTX; independencia del proveedor no equivale a generación offline completa; conservar fuentes no autoriza redistribuir todos los datos privados. Los informes y aprobaciones deben explicar estas consecuencias.

## Seguimiento

Al ejecutar una tarea actualizar estado abierto/mitigado/aceptado, evidencia, fecha y residual. Ningún riesgo crítico se cierra por una afirmación del agente. PoC fallida devuelve el ADR a revisión; no se implementa una extensión dependiente hasta resolverla.
