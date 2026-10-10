# PoC 1 — Slidev con Codex y AGY

**Propósito:** probar edición portable. **Cuándo leer:** T-009 o integración de agentes. **Estado:** no ejecutada. Referencias: [reglas comunes](../POC_PLAN.md), [agentes](../AGENT_SYSTEM.md).

## Hipótesis y alcance

Ambos agentes pueden crear/editar cinco slides con notas y orden coherentes, mediante CLI y MCP disponible. No se prueba aún un orquestador autónomo de producto.

## Dependencias

Release Slidev fijada con MCP, Node 24 local, npm/Chromium/fuentes fijados y cuentas autorizadas de cada agente. La disponibilidad de CLI Codex debe diagnosticarse separadamente de la app; registrar superficie empleada.

## Procedimiento reproducible

1. Crear dos copias idénticas del corpus público, una por agente; registrar hashes/lockfile.
2. Iniciar preview local con CLI instalada (`npm exec -- slidev slides.md`); usar puerto loopback conocido.
3. Pedir a cada agente crear cinco slides del corpus con IDs estables y las mismas restricciones; guardar instrucciones exactas.
4. Pedir cambiar texto/notas de una slide, insertar una, reordenar y retirar la añadida.
5. Ejecutar primero vía archivos/CLI y repetir sobre copia limpia usando MCP. MCP puede usar `slidev mcp slides.md` vía binario local o endpoint dev documentado; no usar descarga implícita de latest.
6. Resolver ID→posición actual, leer fuentes y comprobar orden, notas, hashes de otras slides y ausencia de duplicación de texto.
7. Capturar/renderizar todas las slides y reiniciar desde fuentes para confirmar persistencia.

## Métricas y resultado esperado

Operaciones solicitadas/completadas por agente/vía, errores, iteraciones, duración, slides tocadas y discrepancias de contenido/notas. Resultado esperado: operaciones persistentes con cobertura visual total y sin cambios ajenos.

## Éxito, fallo y consecuencias

Éxito de portabilidad: ambos agentes completan las operaciones y renderizan el corpus mediante la vía CLI/archivos. Éxito MCP se registra por separado para cada plataforma. Si MCP falla pero CLI pasa, mantener CLI y declarar MCP no operativo; si edición/inclusiones no preservan contenido, ADR-005/006 necesitan revisión antes de MVP. No atribuir límites del agente al motor sin aislar una reproducción determinista.
