# Registro de decisiones

**Propósito:** evitar decisiones implícitas y distinguir preferencia de evidencia. **Cuándo leer:** al cambiar arquitectura o resolver una alternativa. Referencias: [evaluación](TECHNOLOGY_EVALUATION.md), [riesgos](RISKS.md), [PoC](POC_PLAN.md).

## Estados

`Confirmada` es una preferencia explícita del usuario; `recomendada` es una elección de planificación; `pendiente de PoC` exige evidencia antes de cerrar arquitectura. Ninguna PoC ha sido ejecutada. La autorización para documentar el plan no convierte sus hipótesis en resultados.

| ID | Decisión | Estado | Motivo / alternativa | Evidencia requerida |
|---|---|---|---|---|
| ADR-001 | SDD portable sin Spec Kit obligatorio. | Confirmada | Artefactos comunes entre agentes; Spec Kit puede evaluarse como herramienta opcional. | Coherencia documental. |
| ADR-002 | Web y PDF antes de PPTX editable. | Confirmada | Demostrar flujo completo con menor alcance. | AC-001, AC-009. |
| ADR-003 | CSV y gráfico reproducible en MVP. | Confirmada | Demostrar trazabilidad científica temprana. | AC-003. |
| ADR-004 | Archivos/herramientas locales con IA configurada en el agente. | Confirmada | Evitar backend obligatorio; offline completo queda como extensión. | AC-009, AC-011. |
| ADR-005 | Slidev como motor principal inicial. | Pendiente de PoC | Mejor ajuste a Markdown, Vue y agentes que alternativas evaluadas. | PoC 1, 5, 6. |
| ADR-006 | Markdown y metadatos estructurados, sin IR universal en MVP. | Pendiente de PoC | Evitar duplicación y coste de un compilador generalista; IR derivada futura. | PoC 1, 6; edición incremental. |
| ADR-007 | Cuatro paquetes TS y CLI local; sin arquitectura distribuida. | Recomendada | Responsabilidades separadas con ejecución simple. | Revisión de dependencias y gates MVP. |
| ADR-008 | Agente principal y especialistas opcionales. | Recomendada | Módulo lógico no exige un agente; multiagente permanente añade coste. | Comparación de calidad/coste en fase 5. |
| ADR-009 | CLI como contrato común; MCP Slidev opcional. | Pendiente de PoC | Portabilidad y comprobación determinista; evitar servidor MCP propio temprano. | PoC 1 en ambos agentes. |
| ADR-010 | Declaración de temas/tokens y componentes Vue acotados. | Pendiente de PoC | Separar identidad visual y comportamiento. | PoC 6. |
| ADR-011 | Hashes y dependencias para reconstrucción parcial. | Recomendada | Reutilizar recursos; exportación final puede ser global. | AC-008, AC-010. |
| ADR-012 | Capturas completas y revisión determinista + humana. | Pendiente de PoC | Revisión visual del modelo no demuestra corrección. | PoC 5, AC-004–007. |
| ADR-013 | Manim opcional y vídeo segmentado con controlador Vue. | Pendiente de PoC | Control local sin imponer Qt/Manim Slides a cada proyecto. | PoC 2. |
| ADR-014 | Vue/Plotly para primera interacción; alternativa estática. | Pendiente de PoC | Un proveedor inicial y contrato desacoplado. | PoC 3. |
| ADR-015 | Medir Slidev editable y PptxGenJS; no prometer editabilidad total. | Pendiente de PoC | Fidelidad y objetos nativos son objetivos distintos. | PoC 4. |
| ADR-016 | Presenton permanece externo por defecto. | Pendiente de PoC | Integración solo ante ventaja suficiente. | PoC 7. |
| ADR-017 | GUI propia después del MVP, basada en problemas observados. | Recomendada | Agentes existentes cubren interfaz inicial. | ADR nuevo después de evaluación de uso. |
| ADR-018 | Skills versionadas y copias locales en `.agents/`. | Recomendada | `.agents/` está ignorado; evitar depender de symlinks Windows. | T-008 y detección de desactualización. |

## Actualización de una decisión

Registrar fecha, estado anterior/nuevo, responsable, evidencia enlazada, impacto en requisitos/contratos/tareas y migración si procede. Si una PoC falla, documentar el motivo y evaluar la alternativa definida en su protocolo antes de autorizar implementación dependiente. Los ADR extensos futuros se enlazarán desde este registro.
