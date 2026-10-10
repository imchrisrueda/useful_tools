# PoC 5 — Validación automatizada

**Propósito:** demostrar detección y corrección de defectos. **Cuándo leer:** T-010 y desarrollo de quality. **Estado:** no ejecutada. Referencias: [reglas comunes](../POC_PLAN.md), [calidad](../QUALITY_STRATEGY.md).

## Hipótesis y alcance

Checks deterministas de geometría/recursos/contraste detectan defectos sembrados y una reparación acotada los elimina sin cambiar contenido ni resultados.

## Dependencias

Slidev/Playwright/axe-core/fuentes fijados; cinco slides y versiones sana/defectuosa del mismo corpus. No requiere servicio de visión IA para demostrar los checks básicos.

## Procedimiento reproducible

1. Renderizar corpus sano, esperar fuentes/recursos y capturar todas las slides/estados.
2. Crear tres variantes separadas: contenido fuera de contenedor/lienzo, imagen local ausente y texto de contraste insuficiente.
3. Medir bounds relativos al lienzo y contenedores declarados; registrar tolerancia inicial de 1 px y excepciones de decoración.
4. Registrar requests/errores de recursos, contraste calculable y estados no evaluables manualmente.
5. Ejecutar detector sobre corpus sano y cada variante; localizar finding en slide/estado y conservar evidencia.
6. Corregir composición/ruta/tokens sin cambiar cifras/citas/unidades, repetir checks y comparar fuentes/capturas.
7. Invalidar reporte después de un cambio posterior y comprobar que no permite entregar revisión nueva.

## Métricas y resultado esperado

Cobertura 100 %, tres clases de defecto detectadas, falsos positivos sobre corpus sano, contenido invariante y defecto eliminado tras corrección. Reportar cantidades reales; ampliar corpus si no permite calibración suficiente.

## Éxito, fallo y consecuencias

Éxito: cada defecto deliberado genera finding localizado, el sano no produce errores espurios y la reparación elimina el finding sin alterar significado. Fallo: solo portada inspeccionada, defecto oculto por clipping no reconocido, shrink ilegible o reporte obsoleto aceptado. Revisar reglas antes de cerrar ADR-012; visión IA queda auxiliar, sin sustituir evidencia.
