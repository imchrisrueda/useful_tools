# PoC 3 — Presentación interactiva

**Propósito:** verificar parámetros y resultados durante la exposición. **Cuándo leer:** antes de fase 3. **Estado:** no ejecutada. Referencias: [reglas comunes](../POC_PLAN.md), [contrato](../contracts/add_interaction.md).

## Hipótesis y alcance

Un componente Vue/Plotly con slider modifica un gráfico de forma correcta, accesible y local. Usar una función conocida, por ejemplo y=a·x, con a en [0,2], y datos públicos; no simular resultados experimentales reales.

## Dependencias

Slidev, Vue, Plotly.js y Playwright fijados; ninguna API remota. Dataset/función, unidades si aplican y estados esperados declarados.

## Procedimiento reproducible

1. Crear componente registrado con a inicial 1, estados 0/1/2 y botón reset; conservar versión y fuente.
2. Montarlo en una slide del corpus y generar alt/caption explicando el ejemplo sintético.
3. Playwright manipula slider mediante teclado y pointer, verifica valores para puntos conocidos y reset a=1.
4. Entrar/salir de slide y comprobar política de estado declarada; para este experimento reset al reiniciar explícitamente, conservar al navegar.
5. Probar límites inválidos contra schema, foco visible, movimiento reducido y ausencia de peticiones externas.
6. Capturar todos los estados y generar snapshot de a=1 para PDF con informe de pérdida de controles.

## Métricas y resultado esperado

Estados verificados/definidos, igualdad numérica de resultados conocidos con tolerancia flotante declarada, errores de consola, tiempo de actualización y bytes de recursos. Resultado esperado: todos los controles producen la curva correcta sin red y con uso por teclado.

## Éxito, fallo y consecuencias

Éxito: 100 % de estados definidos y reset pasan; gráfico, texto y parámetros concuerdan; PDF declara estado y pérdida. Fallo: cambio solo cosmético, números incorrectos, navegación bloqueada o recursos remotos. Revisar componente/registro antes de adoptar ADR-014; no instalar otros motores de gráficos hasta identificar una limitación concreta.
