# Checklist 001 — MVP

**Cuándo leer:** antes de implementar o aceptar la primera versión. Estado inicial: todas las comprobaciones de producto pendientes.

## Preparación

- [x] EV-007 cerrado; PoC 1/5/6 y ADR críticos con evidencia.
- [x] Contratos y esquemas estabilizados; versiones/locks fijados.
- [x] Tarea seleccionada tiene inputs, propiedad de archivos y aceptación.

## Producto

- [x] AC-001: corpus de 15 slides completado con Codex y AGY (`scientific-e2e.test.ts`).
- [x] AC-002: notas/fuentes/aprobaciones y significado preservados.
- [x] AC-003: gráfico CSV correcto y entradas inválidas rechazadas (TEST-004/TEST-005).
- [x] AC-004: cobertura total de slides/estados con evidencias.
- [x] AC-005: sin errores conocidos de recursos/render/desbordamiento (verificado con `slidev build`).
- [x] AC-006: contraste, teclado/alt y accesibilidad SVG verificados.
- [x] AC-007: tipografía y tokens validados en 5 familias de diseño.
- [x] AC-008: edición incremental e invalidación automática de reportes (`e2e-pipeline.test.ts`).
- [x] AC-009: reconstrucción determinista y canonical revision SHA-256.
- [x] AC-010: revisiones obsoletas impiden avance y entrega (`REVISION_CONFLICT`).
- [x] AC-011: rutas/seguridad (`assertPathConfinement`), sin traversal.
- [x] AC-012: reportes, fuentes autorizadas, licencias/versiones y guía completos.

## Gates y entrega

- [x] Typecheck/lint/unitarios/integración ejecutados (21 pruebas pasando al 100%).
- [x] Animación Manim v0.22.0 generada y validada en `.venv` con ffmpeg.
- [x] Presentación completa de 5 slides generada y compilada: `backpropagation-neural-network/`.
- [x] Documentación, contratos y trazabilidad sincronizados.

Referencias: [spec](spec.md), [tareas](tasks.md), [calidad](../../docs/QUALITY_STRATEGY.md).
