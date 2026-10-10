# Sistema de diseño

**Propósito:** separar contenido, identidad y comportamiento. **Cuándo leer:** al crear temas, layouts o propuestas. Referencias: [calidad](QUALITY_STRATEGY.md), [PoC 6](pocs/06-templates.md).

## Catálogo

| Familia | Audiencia/uso | Incorporación |
|---|---|---|
| Scientific Minimal | Investigación, método y figuras; ornamentación limitada. | MVP |
| Tech Keynote | Tecnología, concepto y demostración; jerarquía visual marcada. | MVP |
| Data Storytelling | Resultados y comparación; gráfico y mensaje principal. | Fase 2 |
| Interactive Workshop | Enseñanza y exploración; controles y navegación visibles. | Fase 2; interacción en fase 3 |
| Executive Professional | Decisiones, implicaciones y síntesis institucional. | Fase 2 |

Cada entrada usa DesignManifest, versión y licencia. Añadir tema no exige modificar core: se registra manifiesto, recursos y layouts compatibles; las extensiones de código se revisan como código confiable.

## Tokens y layouts

Tokens semánticos: fondo/superficie/texto/acento, tipografías, escala, espaciado, margen seguro, bordes y movimiento. Los layouts iniciales son portada, sección, texto, dos columnas, figura, gráfico y conclusión. Los componentes reutilizables incluyen título, cita, figura con caption, gráfico y bloque de mensaje.

Tipografías y pesos se empaquetan localmente con licencia y fallback explícitos. Datos y texto aportados no se modifican al cambiar estilo. Fotos conservan relación de aspecto; recorte intencional se declara. Leyendas/unidades forman parte del significado de la figura, no decoración prescindible.

Las ecuaciones matemáticas se formatean en KaTeX estándar (`$` inline o `$$` en bloque) evitando etiquetas HTML que anulen el parser. En composiciones modulares (`slides/*.md`), los recursos multimedia y figuras se vinculan relativamente (`../assets/`) asegurando portabilidad y resolución por el bundler.

## Selección y recomendaciones

En MVP mostrar dos temas disponibles y permitir selección humana. En fase 2, recomendar según audiencia, tema, duración, densidad, datos, nivel técnico y formatos. Las reglas de capacidad filtran opciones incompatibles antes de puntuar preferencias.

Comparar tres propuestas mediante el mismo contenido: portada, slide densa y gráfico. Cada propuesta conserva tema/tokens/layouts, justificación, limitaciones y capturas reales. Variantes de una familia pueden contar como propuestas si su diferencia visual es explicable; no inventar opciones sin renderizado.

## Consistencia y variedad

Usar roles de contenido para elegir layout y mantener una escala tipográfica común. Avisar si se repite composición sin propósito, si hay densidad excesiva o si un layout pierde legibilidad. La revisión humana decide si la repetición apoya la narrativa; no se fuerza variedad por un número arbitrario.

Personalización por overrides de tokens validados. Cambios de layout que afecten significado o recorten evidencia requieren aprobación. Nunca reducir texto a tamaños ilegibles para superar automáticamente un check de desbordamiento. Los umbrales propietarios están en QUALITY_STRATEGY y la aceptación MVP.

## Exportación

Interacción y movimiento incluyen presentación estática, descripción textual y estado elegido para PDF. Temas declaran formatos disponibles. La fidelidad PPTX y la sustitución de fuentes se verifican por PoC; ninguna familia garantiza equivalencia visual automática entre navegador y PowerPoint.
