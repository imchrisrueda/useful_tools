# PROJECT BRIEF — Intelligent Presentation Studio

## 1. Rol y objetivo

Actúa como **arquitecto principal de software, ingeniero de sistemas de IA y especialista en automatización de presentaciones**.

Tu misión inicial es investigar, evaluar y planificar la arquitectura de un sistema denominado provisionalmente **Intelligent Presentation Studio (IPS)**.

El sistema permitirá crear, modificar y reutilizar presentaciones profesionales, científicas, educativas y tecnológicas mediante instrucciones en lenguaje natural, utilizando agentes de IA y herramientas programáticas de presentación, diseño, animación y visualización.

**En esta primera fase NO debes implementar el sistema completo.** Debes producir una planificación informática detallada, técnicamente justificada y verificable.

Se espera una propuesta modular, escalable, mantenible y reproducible, que permita comenzar con una implementación mínima funcional e incorporar capacidades adicionales conforme sean necesarias.

No presupongas que la solución descrita a continuación es la arquitectura definitiva. Evalúa críticamente las alternativas y propone mejoras cuando estén justificadas.

## 2. Visión del producto

El usuario debe poder solicitar:

"Prepara una presentación de 25 minutos sobre inteligencia artificial aplicada a la agricultura de precisión, dirigida a investigadores. Incluye fundamentos, metodología, resultados experimentales y conclusiones. Utiliza un diseño científico moderno, incorpora animaciones que expliquen los algoritmos y añade gráficos interactivos. Propón tres diseños visuales antes de generar la presentación".

El sistema debe interpretar la solicitud y ejecutar un flujo de trabajo completo:

1. Comprender el objetivo y los requisitos.
2. Identificar el público y contexto.
3. Determinar el contenido obligatorio.
4. Analizar la información proporcionada y las fuentes disponibles.
5. Proponer estructuras narrativas.
6. Recomendar estilos, diseños y plantillas.
7. Planificar las diapositivas individualmente.
8. Seleccionar las tecnologías necesarias.
9. Generar la presentación.
10. Renderizar y revisar visualmente el resultado.
11. Detectar y corregir defectos.
12. Verificar contenidos e interacciones.
13. Exportar a los formatos solicitados.
14. Entregar fuentes, recursos y documentación para futuras modificaciones.

El sistema no debe limitarse a convertir texto en diapositivas: debe aportar capacidad de planificación, diseño, comunicación visual y control de calidad.

## 3. Principios arquitectónicos

La arquitectura debe seguir los siguientes principios:

- **Modularidad:** separar presentación, contenido, diseño, animación, validación y exportación.
- **Extensibilidad:** incorporar nuevas tecnologías sin rediseñar el sistema.
- **Reutilización:** compartir componentes, plantillas y recursos.
- **Local-first:** priorizar ejecución local y evitar servicios de pago obligatorios.
- **Independencia del proveedor de IA:** no acoplar el sistema a un único modelo o proveedor.
- **Reproducibilidad:** una presentación debe poder reconstruirse a partir de sus fuentes y configuración.
- **Control humano:** el usuario decide el contenido definitivo, el estilo y las modificaciones relevantes.
- **Trazabilidad:** registrar las decisiones de diseño, las fuentes y las transformaciones importantes.
- **Simplicidad progresiva:** evitar una arquitectura excesiva para tareas sencillas.
- **Calidad verificable:** no dar por correcta una presentación únicamente porque el código compila.
- **Portabilidad:** funcionar principalmente en Windows, contemplando Docker o WSL2 si existen dependencias que lo requieran.
- **Seguridad:** proteger archivos, credenciales, información sensible y fuentes proporcionadas por el usuario.

Prioriza soluciones de código abierto, con licencias compatibles y documentación mantenida.

## 4. Casos de uso que debe soportar

El sistema debe poder adaptarse al nivel de complejidad de cada solicitud.

### Caso A — Presentación convencional

Ejemplo: una presentación académica de 15 diapositivas sobre una metodología de investigación.

Características:
- Texto estructurado.
- Fotografías y diagramas.
- Gráficos.
- Diseño profesional.
- Notas del presentador.
- Exportación PDF y PowerPoint.

### Caso B — Presentación animada

Ejemplo: explicar paso a paso el funcionamiento de una red neuronal.

Características:
- Animaciones matemáticas y científicas.
- Transiciones secuenciales.
- Sincronización de explicaciones visuales.
- Posibilidad de avanzar o retroceder por etapas.

### Caso C — Presentación interactiva

Ejemplo: explicar la detección de objetos mediante visión artificial.

Características:
- Gráficos interactivos.
- Controles para modificar parámetros.
- Visualización dinámica de resultados.
- Diagramas y simulaciones.
- Posibilidad de interactuar durante la exposición.

### Caso D — Presentación científica reproducible

Ejemplo: resultados de un experimento de detección de especies vegetales.

Características:
- Lectura de CSV, Excel, JSON u otros datos estructurados.
- Gráficos derivados de datos reales.
- Referencias bibliográficas.
- Figuras científicas.
- Trazabilidad de resultados.
- Separación entre datos, análisis y presentación.
- Actualización de gráficos cuando cambien los datos.

### Caso E — Presentación ejecutiva o divulgativa

Ejemplo: exponer resultados de un proyecto tecnológico ante responsables institucionales.

Características:
- Narrativa visual.
- Mensajes principales.
- Infografías.
- Gráficos resumidos.
- Recursos multimedia.
- Consistencia estética.

### Caso F — Modificación de presentaciones existentes

El usuario debe poder solicitar:

- Cambia el estilo visual.
- Reorganiza el contenido.
- Sustituye un gráfico.
- Añade una sección.
- Incorpora una animación.
- Simplifica la diapositiva 8.
- Genera una versión para otro público.
- Actualiza los resultados usando nuevos datos.

Evalúa la viabilidad de importar y transformar PPTX existentes, distinguiéndola de la edición de presentaciones creadas originalmente por el sistema.

## 5. Tecnologías candidatas

Investiga su estado actual mediante documentación oficial, versiones, licencias, API y limitaciones.

No adoptes una herramienta exclusivamente por estar incluida en esta lista.

### Motor principal de presentaciones

Candidatos:
- Slidev.
- reveal.js.
- Quarto.
- Marp.
- Otras alternativas relevantes.

Slidev es la hipótesis principal por su integración con Vue, sus capacidades web, su sistema de temas, la generación desde Markdown y su soporte MCP.

Evalúa específicamente su utilidad para agentes que crean, inspeccionan y modifican diapositivas.

### Animaciones

Candidatos:
- Manim.
- Manim Slides.
- Motion Canvas.
- Animaciones SVG/CSS.
- Otras bibliotecas justificadas.

Determina qué tecnología conviene utilizar según el tipo de animación.

### Visualización e interacción

Candidatos:
- Vue.
- Plotly.js.
- D3.js.
- ECharts.
- Three.js.
- Mermaid.
- Componentes SVG interactivos.

No instales todas las tecnologías por defecto. Diseña un mecanismo de selección por necesidad.

### Generación PowerPoint

Candidatos:
- Exportación de Slidev.
- Exportación editable de Slidev.
- PptxGenJS.
- Presenton.
- Otras soluciones justificadas.

Evalúa:
- Editabilidad.
- Fidelidad visual.
- Soporte de notas.
- Gráficos y tablas.
- Limitaciones de animaciones.
- Compatibilidad con PowerPoint.
- Esfuerzo de mantenimiento.

### Automatización y validación

Candidatos:
- Playwright.
- Herramientas de renderizado y comparación visual.
- Validadores estructurales.
- Herramientas de accesibilidad.
- Sistemas de pruebas unitarias y de integración.

Verifica las capacidades actuales; no presupongas compatibilidades no documentadas.

## 6. Arquitectura funcional propuesta para evaluar

Considera los siguientes módulos lógicos:

### A. Interface / Request Manager

Responsable de:
- Recibir solicitudes en lenguaje natural.
- Detectar requisitos explícitos e implícitos.
- Definir contenido, público, duración, objetivos y formatos.
- Gestionar decisiones y preferencias.
- Permitir modificaciones posteriores.

Inicialmente, la interfaz puede ser Codex o un agente compatible. No es necesario crear una interfaz web propia en el MVP.

### B. Planning / Orchestration Engine

Responsable de:
- Convertir solicitudes en planes ejecutables.
- Dividir actividades en tareas.
- Identificar dependencias.
- Seleccionar herramientas.
- Coordinar la generación y validación.
- Gestionar errores y revisiones.

Analiza si es preferible comenzar con un único agente que use herramientas especializadas o con varios agentes independientes.

Evita una arquitectura multiagente innecesariamente compleja.

### C. Content & Research Engine

Responsable de:
- Organizar contenidos.
- Construir la narrativa.
- Analizar documentos y datasets.
- Estructurar apartados.
- Gestionar citas bibliográficas.
- Distinguir datos proporcionados, información verificada e hipótesis.
- Evitar cifras o referencias inventadas.

El contenido científico debe preservar su significado y trazabilidad.

### D. Design Intelligence Engine

Responsable de:
- Recomendar estilos.
- Proponer varias alternativas visuales.
- Elegir tipografía, colores, composición y layouts.
- Mantener identidad visual.
- Adaptar el diseño al contenido.
- Evitar saturación y elementos innecesarios.

Define cómo representar un sistema de diseño reutilizable, incluyendo tokens, temas, variantes y componentes.

### E. Presentation Generation Engine

Responsable de:
- Transformar el plan aprobado en diapositivas.
- Componer layouts.
- Insertar textos, imágenes, tablas y gráficos.
- Gestionar notas del presentador.
- Integrar componentes externos.
- Generar presentaciones reproducibles.

### F. Animation & Interaction Engine

Responsable de:
- Elegir el tipo de animación.
- Generar animaciones Manim cuando proceda.
- Incorporar secuencias animadas.
- Crear gráficos interactivos.
- Gestionar controles y eventos.
- Incorporar simulaciones.

Las animaciones complejas deben ser opcionales, no una dependencia obligatoria de todas las presentaciones.

### G. Asset & Template Manager

Responsable de:
- Almacenar plantillas.
- Gestionar recursos multimedia.
- Mantener componentes reutilizables.
- Controlar procedencia y licencias.
- Registrar versiones.
- Reutilizar estilos aprobados.
- Incorporar nuevas plantillas.

### H. Validation & Quality Engine

Responsable de:
- Renderizar presentaciones.
- Obtener capturas de todas las diapositivas.
- Detectar desbordamientos.
- Comprobar legibilidad y contraste.
- Validar recursos e interacciones.
- Verificar exportaciones.
- Identificar fallos estructurales.
- Facilitar revisiones automáticas y humanas.

### I. Export & Delivery Engine

Responsable de:
- Generar presentación web.
- Exportar PDF.
- Exportar PPTX.
- Generar imágenes o vídeos cuando sea adecuado.
- Conservar los archivos fuente.
- Informar de pérdidas de funcionalidad durante la exportación.

## 7. Modelo interno de representación

Analiza si necesitamos una representación intermedia común (*Presentation Intermediate Representation*, Presentation IR).

Podría almacenar:

- Identificador del proyecto.
- Metadatos.
- Audiencia y objetivos.
- Narrativa.
- Diapositivas.
- Contenido y fuentes.
- Layout asignado.
- Elementos visuales.
- Animaciones.
- Interacciones.
- Notas.
- Recursos.
- Configuración de exportación.
- Estado de validación.

Evalúa dos enfoques:

**Opción A:** Slidev Markdown como fuente principal de verdad.

**Opción B:** modelo estructurado JSON/YAML como fuente principal, con adaptadores para distintos motores.

Decide justificadamente cuál conviene para el MVP y qué camino de evolución evita migraciones costosas.

Considera validación de esquemas, versiones y compatibilidad.

No construyas una representación intermedia generalista si no aporta beneficios demostrables en la primera fase.

## 8. Biblioteca inteligente de diseños

Diseña un catálogo extensible de:

- Temas visuales.
- Layouts.
- Componentes.
- Tipografías.
- Paletas de colores.
- Gráficos.
- Transiciones.
- Patrones de animación.
- Plantillas completas.

Incluye como familias iniciales:

1. Scientific Minimal.
2. Tech Keynote.
3. Data Storytelling.
4. Interactive Workshop.
5. Executive Professional.

El agente debe poder recomendar diseños en función de:
- Audiencia.
- Temática.
- Duración.
- Densidad de información.
- Tipos de datos.
- Nivel técnico.
- Formato de salida.

Propón un mecanismo para generar miniaturas y comparar diseños antes de seleccionar uno.

Debe ser posible extender la biblioteca sin modificar el núcleo del sistema.

Investiga mecanismos para evitar diseños repetitivos, garantizar consistencia y permitir personalización.

## 9. Sistema de agentes y herramientas

Propón una arquitectura agentic compatible con Codex y extensible a otros agentes.

Evalúa:
- AGENTS.md.
- Skills reutilizables.
- MCP.
- Herramientas CLI.
- Subagentes especializados cuando aporten ventajas reales.
- Contratos de entrada y salida.
- Gestión de contexto.
- Estado del proyecto.
- Supervisión humana.
- Reintentos y recuperación de errores.

Establece un mecanismo para que el agente pueda conocer:
- Qué herramientas existen.
- Qué capacidades tienen.
- Cuándo utilizarlas.
- Qué dependencias requieren.
- Qué limitaciones presentan.
- Cómo verificar su ejecución.

Distingue claramente los módulos lógicos de los agentes independientes.

No asumas que cada módulo necesita su propio proceso o modelo de IA.

Define también un flujo para modificar presentaciones ya generadas sin reconstruir innecesariamente todo el proyecto.

## 10. Flujo de generación esperado

Diseña el siguiente pipeline conceptual:

REQUEST
→ REQUIREMENTS
→ CONTENT ANALYSIS
→ STORYBOARD
→ DESIGN PROPOSALS
→ DESIGN SELECTION
→ SLIDE PLANNING
→ GENERATION
→ RENDERING
→ VALIDATION
→ REVISION
→ EXPORT
→ DELIVERY

El sistema debe admitir iteraciones en cualquiera de las etapas.

Especifica qué etapas requieren aprobación humana y cuáles pueden realizarse automáticamente.

Incluye estados de trabajo, artefactos intermedios, errores recuperables y criterios de transición.

Considera el uso de caché y reconstrucción parcial cuando únicamente se modifica una diapositiva, un gráfico o una animación.

## 11. Requisitos no funcionales

### Rendimiento

Evitar regeneraciones innecesarias.

Evaluar el coste de renderizado de Manim y otros motores.

Establecer un mecanismo de caché de recursos.

Diseñar para proyectos de diferente tamaño.

### Portabilidad

Priorizar Windows como entorno principal.

Contemplar WSL2 o Docker cuando sean necesarios.

Separar dependencias Python de dependencias Node.js.

Documentar la instalación y su diagnóstico.

### Seguridad

No ejecutar código ni scripts procedentes de documentos no confiables sin controles apropiados.

Proteger credenciales.

Validar rutas y entradas.

Definir permisos de lectura, escritura, red y ejecución.

No enviar datos privados a proveedores externos sin autorización.

### Mantenibilidad

Separar responsabilidades.

Utilizar interfaces estables.

Establecer pruebas automatizadas.

Documentar decisiones.

Definir políticas de versionado.

### Calidad visual

Prevenir:
- Textos fuera de los límites.
- Elementos superpuestos.
- Imágenes deformadas.
- Gráficos ilegibles.
- Contraste insuficiente.
- Exceso de contenido.
- Animaciones innecesarias.
- Variaciones arbitrarias de estilo.

### Reproducibilidad y rigor

Conservar las fuentes y parámetros utilizados.

No inventar resultados científicos.

Mantener trazabilidad entre datos y figuras.

Registrar versiones de herramientas.

Evitar que una modificación visual altere silenciosamente el significado de los resultados.

### Accesibilidad

Considerar tamaños de letra apropiados, contraste, navegación por teclado, alternativas textuales y reducción de movimiento cuando resulte aplicable.

## 12. Estrategia de calidad y pruebas

Diseña una pirámide de pruebas que incluya:

- Pruebas unitarias de componentes y transformación de datos.
- Pruebas de integración de motores.
- Pruebas de generación.
- Pruebas de exportación.
- Pruebas de interacción.
- Pruebas visuales.
- Pruebas de accesibilidad.
- Evaluación de calidad del contenido.
- Pruebas end-to-end desde una solicitud hasta el entregable.

La evaluación visual debe incluir todas las diapositivas y los estados significativos de las animaciones o interacciones, no únicamente una captura de portada.

Distingue entre comprobaciones automáticas deterministas y revisiones de naturaleza subjetiva.

Propón métricas medibles, umbrales iniciales y criterios de aceptación, justificándolos y evitando aparentar que los umbrales arbitrarios están validados científicamente.

## 13. Prototipos de validación tecnológica

Antes de comprometer la arquitectura definitiva, diseña pruebas de concepto pequeñas.

Como mínimo:

**PoC 1 — Slidev + agente**
Generar y editar una presentación con varias diapositivas mediante Codex y las integraciones compatibles.

**PoC 2 — Slidev + Manim**
Incorporar una animación científica con control adecuado durante la presentación.

**PoC 3 — Presentación interactiva**
Crear una diapositiva con un gráfico cuyos parámetros se puedan modificar durante la exposición.

**PoC 4 — Exportación editable**
Comparar exportación Slidev a PowerPoint editable frente a generación nativa con PptxGenJS.

**PoC 5 — Validación automatizada**
Generar capturas, detectar un desbordamiento deliberado y verificar el mecanismo de corrección.

**PoC 6 — Plantillas**
Generar una misma presentación con dos temas distintos sin modificar su contenido principal.

**PoC 7 — Presenton**
Evaluar si su API o MCP pueden proporcionar ventajas suficientes para incorporarlo como motor opcional, frente a mantenerlo fuera de la arquitectura.

Para cada PoC, describe:
- Hipótesis.
- Alcance.
- Dependencias.
- Procedimiento reproducible.
- Resultado esperado.
- Métricas.
- Criterio de éxito o fracaso.
- Consecuencias arquitectónicas.

En esta fase, diseña las PoC; no es obligatorio ejecutarlas todavía.

## 14. Plan de desarrollo por etapas

Propón un roadmap incremental.

### Fase 0 — Evaluación y arquitectura

Investigación tecnológica, decisiones arquitectónicas, riesgos y especificaciones.

### Fase 1 — MVP

Generar presentaciones mediante Codex y un motor principal.

Capacidades mínimas:
- Interpretación de solicitud.
- Planificación del contenido.
- Selección de plantilla.
- Generación.
- Previsualización.
- Validación básica.
- Exportación PDF.
- Conservación de fuentes.

### Fase 2 — Diseño inteligente

Añadir recomendaciones de diseño, variantes visuales, biblioteca de componentes y generación de previsualizaciones comparables.

### Fase 3 — Animación e interacción

Integrar Manim y componentes interactivos mediante adaptadores desacoplados.

### Fase 4 — Exportación avanzada

Incorporar capacidades ampliadas de PowerPoint editable, gestores de compatibilidad y formatos adicionales.

### Fase 5 — Automatización avanzada

Ampliar capacidades agentic, reutilización de recursos, evaluación integral, modificación incremental y funciones inteligentes adicionales.

Para cada fase especifica:
- Objetivo.
- Requisitos previos.
- Módulos afectados.
- Tareas.
- Entregables.
- Pruebas.
- Riesgos.
- Dependencias.
- Criterios de finalización.

Evita tareas grandes o ambiguas. Propón unidades de trabajo verificables y priorizadas.

## 15. Estructura informática del repositorio

Diseña una estructura de carpetas concreta.

Debe contemplar:
- Configuración.
- Instrucciones para agentes.
- Skills.
- Documentación.
- Arquitectura.
- Motores de presentación.
- Adaptadores.
- Componentes visuales.
- Animaciones.
- Temas.
- Plantillas.
- Proyectos.
- Assets.
- Datos.
- Validadores.
- Pruebas.
- Exportadores.
- Scripts.
- Salidas generadas.

No crees directorios sin una responsabilidad clara.

Identifica qué módulos pertenecerían a TypeScript y cuáles a Python.

Evalúa si conviene un monorepositorio ligero y cómo organizar sus dependencias.

Recomienda herramientas de empaquetado, entornos virtuales, formateo, tipado, pruebas y CI.

Describe el flujo de ejecución desde Windows.

## 16. Contratos e interfaces

Define contratos preliminares para las operaciones principales:

- create_project
- analyze_request
- create_storyboard
- suggest_designs
- select_design
- generate_slides
- add_animation
- add_interaction
- update_slide
- validate_presentation
- export_presentation

Cada contrato debe especificar:
- Finalidad.
- Entradas.
- Salidas.
- Esquema de datos.
- Errores previsibles.
- Dependencias.
- Efectos secundarios.
- Requisitos de validación.

Utiliza interfaces tipadas y esquemas formales cuando sea apropiado.

No es necesario implementar estas operaciones todavía.

## 17. Decisiones que debes estudiar especialmente

Analiza en profundidad:

1. Slidev frente a reveal.js como núcleo.
2. Fuente única de verdad: Markdown frente a IR estructurada.
3. Integración de Manim: vídeo, secuencias de imágenes o presentaciones controladas.
4. Interactividad nativa web frente a exportación estática.
5. Slidev editable frente a PptxGenJS para PowerPoint.
6. Conveniencia de integrar Presenton.
7. Agente único con herramientas frente a sistema multiagente.
8. Skills y MCP frente a herramientas CLI propias.
9. Plantillas declarativas frente a componentes programáticos.
10. Construcción incremental frente a regeneración completa.
11. Revisión visual automática y sus limitaciones.
12. Introducción futura de una interfaz gráfica propia.

Presenta una matriz de decisión tecnológica con criterios ponderados, puntuaciones justificadas y nivel de incertidumbre.

Diferencia las capacidades verificadas de las hipótesis pendientes de prueba.

## 18. Entregables obligatorios

Genera, como mínimo, la siguiente documentación:

- README.md — visión del proyecto y propuesta recomendada.
- docs/PRODUCT_REQUIREMENTS.md — requisitos funcionales y no funcionales.
- docs/TECHNOLOGY_EVALUATION.md — alternativas y comparación.
- docs/ARCHITECTURE.md — arquitectura técnica detallada.
- docs/COMPONENTS.md — responsabilidades y dependencias.
- docs/AGENT_SYSTEM.md — arquitectura agentic, skills y MCP.
- docs/DATA_MODELS.md — modelos de datos y contratos.
- docs/DESIGN_SYSTEM.md — planteamiento de temas, layouts y componentes.
- docs/QUALITY_STRATEGY.md — estrategia de pruebas y validación.
- docs/POC_PLAN.md — pruebas de concepto.
- docs/ROADMAP.md — fases de implementación.
- docs/RISKS.md — riesgos, incertidumbres y mitigaciones.
- docs/DECISIONS.md — decisiones arquitectónicas y alternativas descartadas.
- TASKS.md — backlog priorizado de tareas ejecutables.

Incluye diagramas Mermaid donde ayuden a explicar:
- Arquitectura general.
- Dependencias.
- Flujo de generación.
- Flujo de revisión.
- Integración de motores.
- Estados del proyecto.

Los documentos deben ser consistentes entre sí y evitar duplicaciones innecesarias.

## 19. Metodología de trabajo

Sigue este procedimiento:

**Paso 1.** Inspecciona el repositorio y determina si contiene código o documentación previa.

**Paso 2.** Investiga las alternativas, priorizando fuentes oficiales y documentación actualizada.

**Paso 3.** Identifica requisitos, incertidumbres y dependencias.

**Paso 4.** Compara arquitecturas alternativas con sus ventajas, riesgos y costes de mantenimiento.

**Paso 5.** Selecciona una arquitectura inicial justificada.

**Paso 6.** Define módulos, interfaces, dependencias y estructuras de datos.

**Paso 7.** Diseña las pruebas de concepto necesarias para validar las decisiones arriesgadas.

**Paso 8.** Establece un roadmap incremental con criterios de aceptación.

**Paso 9.** Produce la documentación y el backlog.

**Paso 10.** Revisa la coherencia global, las dependencias circulares y la viabilidad del plan.

Conserva trazabilidad de las decisiones relevantes.

Si dispones de subagentes, utilízalos para investigaciones independientes cuando supongan una ventaja clara. No generes subagentes artificialmente.

No ocultes conflictos entre requisitos.

Cuando existan alternativas razonables, explica sus consecuencias antes de recomendar una.

## 20. Restricciones de esta primera ejecución

- No implementes todavía el producto.
- No instales dependencias innecesarias.
- No generes grandes cantidades de código de ejemplo.
- No modifiques archivos ajenos al proyecto.
- No introduzcas servicios externos obligatorios.
- No diseñes una arquitectura distribuida si una solución local es suficiente.
- No elijas tecnologías únicamente por popularidad.
- No consideres una exportación PPTX completamente editable sin demostrar sus límites.
- No supongas que las interacciones web pueden trasladarse intactas a PDF o PowerPoint.
- No inventes resultados de pruebas que no hayas ejecutado.

Puedes crear archivos de documentación y planificación dentro del repositorio.

Distingue claramente entre decisiones aprobadas, recomendaciones provisionales e hipótesis pendientes de validación.

## 21. Resultado esperado

Al finalizar, presenta un resumen ejecutivo que responda:

1. ¿Cuál es la arquitectura recomendada?
2. ¿Por qué se ha seleccionado?
3. ¿Qué tecnologías se utilizarán inicialmente?
4. ¿Qué tecnologías quedan como extensiones?
5. ¿Cómo se integra Codex?
6. ¿Qué agentes, skills y herramientas se necesitan realmente?
7. ¿Cómo se organizan los proyectos y las plantillas?
8. ¿Cómo se generan, verifican y exportan las presentaciones?
9. ¿Qué riesgos requieren pruebas de concepto?
10. ¿Cuál es el MVP más pequeño que demuestra el funcionamiento completo?
11. ¿Cuál es el orden recomendado de implementación?
12. ¿Qué decisiones deben confirmarse antes de empezar a programar?

**Criterio de éxito:** la planificación debe permitir comenzar la implementación mediante tareas pequeñas, con dependencias explícitas y pruebas reproducibles, sin tener que rediseñar el núcleo cada vez que se incorpora una nueva capacidad.

Primero diseña una arquitectura sólida y proporcional a los requisitos. Después prepara el plan de implementación. No comiences a desarrollar el producto hasta que se apruebe la planificación.