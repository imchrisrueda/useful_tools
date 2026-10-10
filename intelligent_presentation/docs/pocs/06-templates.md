# PoC 6 — Dos temas y contenido estable

**Propósito:** probar separación entre contenido y diseño. **Cuándo leer:** T-011 o modificación del catálogo. **Estado:** no ejecutada. Referencias: [reglas comunes](../POC_PLAN.md), [diseño](../DESIGN_SYSTEM.md).

## Hipótesis y alcance

Scientific Minimal y Tech Keynote pueden renderizar las mismas cinco slides y notas sin reescribir su contenido principal. Se prueban layouts compatibles, tokens y tipografías locales.

## Dependencias

Slidev fijado, dos manifiestos/temas, fuentes con licencia y renderer. El material de ambas variantes es idéntico en texto, datos, notas y referencias.

## Procedimiento reproducible

1. Preparar corpus y hashes de contenido/notas/CSV; documentar mappings de layouts comunes.
2. Renderizar Scientific Minimal; capturar todas las slides/estados y registrar manifest/hash de tema.
3. Cambiar únicamente selección/tokens/mapping permitido; renderizar Tech Keynote.
4. Comparar hashes de contenido y notas; comprobar mismos valores, captions, unidades y citas.
5. Revisar bounds, contraste, tamaño, imágenes y gráfico en ambos temas; generar hoja comparativa portada/denso/gráfico.
6. Añadir un override de acento validado y comprobar invalidación de capturas dependientes, no del dataset.

## Métricas y resultado esperado

Slides/estados renderizados, fuentes de contenido modificadas (esperado cero), defectos por tema, tiempo y assets reutilizados. Resultado esperado: diseños distintos con significado y contenido invariantes.

## Éxito, fallo y consecuencias

Éxito: corpus entero pasa checks en ambos temas; no se edita texto para corregir adaptación. Fallo: placeholders sustituyen contenido, notas se pierden o layouts dependen de duplicar narrativa. Revisar ADR-006/010 y el contrato de tema antes de MVP; no generalizar desde solo la portada.
