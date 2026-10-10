# PoC 4 — Exportación editable

**Propósito:** medir editabilidad por objeto. **Cuándo leer:** antes de fase 4. **Estado:** no ejecutada. Referencias: [reglas comunes](../POC_PLAN.md), [evaluación](../TECHNOLOGY_EVALUATION.md).

## Hipótesis y alcance

Slidev editable puede cubrir objetos simples; PptxGenJS permite una composición nativa más controlada con esfuerzo adicional. Ninguna etiqueta editable garantiza fidelidad o equivalencia de componentes web.

## Dependencias

Releases fijadas de Slidev/PptxGenJS y Chromium, fuentes idénticas y acceso a PowerPoint para la dimensión de compatibilidad destino. OOXML/LibreOffice son evidencias complementarias, no sustitutos de apertura PowerPoint.

## Procedimiento reproducible

1. Preparar corpus con texto corto/largo, tabla, gráfico desde CSV, foto, SVG/Mermaid, fórmula, vídeo, gradiente y notas.
2. Exportar referencia raster (`--format pptx`) y editable (`--format pptx-editable`) mediante CLI Slidev instalada; guardar opciones de clicks y advertencias.
3. Generar los objetos equivalentes mediante PptxGenJS desde el mismo contenido/parámetros, sin usar una imagen de slide como sustituto de objeto nativo.
4. Inspeccionar XML/relaciones del PPTX: tipos de objeto, imágenes, texto, tablas, gráfico y notas; documentar rasterización por elemento/slide.
5. Abrir en PowerPoint, editar texto, mover/recolorear forma, editar celdas y datos de gráfico cuando existan; guardar y reabrir.
6. Comparar capturas por slide, wrapping, tipografías, legibilidad y notas. Marcar interacciones/animaciones no transferidas.

## Métricas y resultado esperado

Matriz por objeto/exportador: nativo editable, rasterizado, ausente, degradado; notas presentes, diferencias visuales, tamaño, tiempo y esfuerzo de mantenimiento. Resultado esperado: límites demostrados y capacidades publicables por tipo de objeto.

## Éxito, fallo y consecuencias

Éxito de evaluación: matriz completa con evidencia, incluso si algún objeto no es editable. Adopción de exportador para un perfil requiere que todos los objetos declarados nativos puedan editarse y que pérdidas restantes sean explícitas. Ausencia de PowerPoint deja esa dimensión blocked. ADR-015 decide por perfiles de salida y subconjunto portable; no impone IR universal ni anuncia PPTX totalmente editable.
