# Requisitos de producto

**Propósito:** definir qué debe lograr IPS. **Cuándo leer:** al especificar o aclarar alcance. Referencias: [brief](../idea.md), [aceptación MVP](../specs/001-mvp/spec.md), [roadmap](ROADMAP.md).

## Usuarios y alcance

Investigadores, docentes y profesionales que necesitan presentaciones mantenibles y verificables. La interfaz inicial será Codex o Google Antigravity; los archivos y herramientas operarán localmente. Se permite IA externa autorizada mediante el agente existente, sin backend propio obligatorio.

## Requisitos funcionales

| ID | Capacidad | Primera fase |
|---|---|---|
| FR-001 | Interpretar objetivo, audiencia, duración, contenido y formatos; explicitar preguntas pendientes. | MVP |
| FR-002 | Organizar fuentes y storyboard con mensajes, evidencia y tiempos. | MVP |
| FR-003 | Seleccionar un tema disponible y registrar aprobación del diseño. | MVP |
| FR-004 | Generar diapositivas, notas, referencias y recursos conservando fuentes. | MVP |
| FR-005 | Previsualizar, capturar y revisar todas las diapositivas y estados definidos. | MVP |
| FR-006 | Modificar por ID estable y preservar contenido no afectado. | MVP |
| FR-007 | Entregar web y PDF con informe y fuentes de reconstrucción. | MVP |
| FR-008 | Leer CSV declarado y generar barras/líneas SVG con procedencia y unidades. | MVP |
| FR-009 | Registrar estado, revisiones, aprobaciones e invalidaciones. | MVP |
| FR-010 | Ofrecer CLI común y procedimientos compatibles con Codex y AGY. | MVP |
| FR-011 | Recomendar tres diseños comparables y ampliar el catálogo a cinco familias. | 2 |
| FR-012 | Integrar animación científica con control y alternativa estática. | 3 |
| FR-013 | Integrar gráficos interactivos y pruebas de sus estados significativos. | 3 |
| FR-014 | Exportar PPTX con editabilidad y degradaciones verificadas por objeto. | 4 |
| FR-015 | Evaluar importación de PPTX ajenos separadamente de edición nativa IPS. | 4 |
| FR-016 | Ampliar datos a Excel/JSON y análisis reproducible cuando exista necesidad. | 3–5 |
| FR-017 | Ampliar automatización, reutilización y delegación según evidencia de uso. | 5 |

Los casos A y E del brief se demuestran inicialmente; B y C llegan en fase 3; D empieza con CSV y crece por FR-016; F empieza con proyectos IPS y evalúa PPTX externo por FR-015.

## Requisitos no funcionales

| ID | Regla verificable |
|---|---|
| NFR-001 | Windows es la plataforma principal; comandos desde el componente y dependencias diagnosticables. |
| NFR-002 | Runtime, datos, assets, fuentes y parámetros permiten reconstruir sin nuevas llamadas al modelo. |
| NFR-003 | Rutas y escritura están restringidas; documentos no confiables no ejecutan scripts; credenciales no se registran. |
| NFR-004 | La transmisión de información privada requiere autorización para contenido y destino. |
| NFR-005 | Cada resultado científico enlaza datos, unidades y transformación; una reparación visual preserva significado. |
| NFR-006 | Toda diapositiva y estado significativo recibe comprobación; defectos conocidos bloquean entrega según severidad. |
| NFR-007 | Teclado, contraste, texto alternativo y reducción de movimiento se evalúan donde apliquen. |
| NFR-008 | Los cambios invalidan derivados por dependencias; recursos inalterados se reutilizan. |
| NFR-009 | Contratos versionados, dependencias fijadas, módulos sin ciclos y cambios revisables. |
| NFR-010 | Ningún proveedor de IA ni servicio de pago forma parte obligatoria del núcleo. |
| NFR-011 | Assets y dependencias mantienen procedencia, licencias y atribución. |
| NFR-012 | Las exportaciones declaran pérdidas; no prometen trasladar interacción web a formatos estáticos. |

## Control humano y límites iniciales

El usuario aprueba requisitos/storyboard, diseño, cambios semánticos posteriores y entrega final. Los detalles se definen en [ciclo de proyecto](PROJECT_LIFECYCLE.md).

El MVP no incluye GUI propia, importación general PPTX, generación completamente offline con modelo local, Manim obligatorio, tres diseños automáticos ni PPTX editable. Una solicitud fuera de capacidades obtiene explicación y opción de ajustar alcance; no se simula que está soportada.

Los datos de ejemplo sintéticos se etiquetan como tales. Se conservan resultados aportados sin inventar experimentos, cifras o referencias. La duración se estima mediante storyboard y se confirma mediante revisión humana, no mediante una equivalencia fija entre minutos y diapositivas.

## Aceptación

Los criterios AC-001–AC-012 se definen únicamente en [la especificación MVP](../specs/001-mvp/spec.md). [QUALITY_STRATEGY](QUALITY_STRATEGY.md) explica su comprobación y umbrales iniciales. [TRACEABILITY](../specs/TRACEABILITY.md) relaciona requisitos, tareas y tests.
