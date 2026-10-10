# PoC 2 — Slidev y Manim

**Propósito:** probar animación científica controlable. **Cuándo leer:** antes de fase 3. **Estado:** no ejecutada. Referencias: [reglas comunes](../POC_PLAN.md), [contrato](../contracts/add_animation.md).

## Hipótesis y alcance

Tres segmentos de Manim y un controlador Vue permiten avance, retroceso, pausa y reinicio sin imponer player Qt a cada presentación. Comparar secuencia de imágenes/Manim Slides solo si la primera vía no cumple controles.

## Dependencias

Slidev/Vue fijados; uv y entorno Python separado; Manim release fijada y sus dependencias comprobadas. Escena geométrica sencilla evita exigir LaTeX para este experimento. Preparar WSL2/Docker solo si Windows nativo no funciona, registrando esa limitación.

## Procedimiento reproducible

1. Preparar escena pública de tres pasos con parámetros fijos, semilla si aplica, captions y una figura final.
2. Renderizar mediante binario del entorno (`uv run manim -ql scene.py DemoScene`) y producir segmentos con nombres/IDs estables; guardar comandos reales y versiones.
3. Registrar duración, tamaño y hash de cada segmento/poster; integrar con componente Vue local.
4. Avanzar 1→2→3, retroceder 3→2→1, pausar, reiniciar y repetir entradas/salidas de la slide.
5. Probar foco/teclado sin secuestrar navegación general. Probar movimiento reducido y poster de exportación.
6. Capturar estados límite y exportar PDF del estado estático; repetir render sin cambios para comprobar reutilización.

## Métricas y resultado esperado

Tiempo frío/caliente, tamaño, pasos/controles correctos, latencia observada en hardware registrado, errores de consola y capturas. No fijar un SLA antes de medir. Resultado esperado: control secuencial estable y explicación estática que preserva significado.

## Éxito, fallo y consecuencias

Éxito: todos los controles y estados correctos, sin reproducción inesperada al volver a la slide y sin cambiar la explicación científica. Fallo: saltos, reverse no definido, pérdida de foco o dependencia no reproducible. Si vídeo segmentado falla, evaluar imágenes por estado o adaptador Manim Slides y registrar coste/limitaciones antes de cerrar ADR-013.
