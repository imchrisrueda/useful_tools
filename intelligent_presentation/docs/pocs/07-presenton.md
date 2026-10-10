# PoC 7 — Presenton opcional

**Propósito:** decidir si aporta una ventaja suficiente. **Cuándo leer:** cuando se considere incorporar ese motor. **Estado:** no ejecutada. Referencias: [reglas comunes](../POC_PLAN.md), [riesgos](../RISKS.md).

## Hipótesis y alcance

API/MCP local puede reducir trabajo de generación/exportación sin perder procedencia, control ni portabilidad. Por defecto permanece fuera de la arquitectura obligatoria.

## Dependencias

Release/imagen fijada con licencia/dependencias auditadas, entorno local aislado, recursos públicos y proveedor autorizado. Verificar qué distribución realmente expone MCP; no asumir equivalencia de app desktop y servidor. API/documentación se verifica para esa release.

## Procedimiento reproducible

1. Auditar configuración/requisitos/red de la distribución elegida; conservar tag/digest, licencia y endpoints.
2. Desplegar localmente solo al autorizar este experimento, sin configurar cuentas globales ni publicar puertos.
3. Enviar el mismo corpus/objetivo de cinco slides mediante API y MCP disponible; registrar schema real y errores.
4. Generar/exportar, inspeccionar editabilidad como en PoC 4 y revisar notas, datos, citas y recursos.
5. Comprobar si una edición individual conserva fuentes no afectadas y si es posible reconstruir con entradas/versiones conocidas.
6. Registrar peticiones externas, consumo, recursos del servicio, tiempos, esfuerzo de integración y degradaciones respecto al flujo IPS.

## Métricas y resultado esperado

Capacidades adicionales demostradas, objetos editables, defectos, pasos manuales, tiempo/coste configurado, RAM/disco y trazabilidad conservada. El beneficio debe concretarse: resolver una capacidad requerida que el flujo actual no cubra o reducir trabajo sin degradar criterios.

## Éxito, fallo y consecuencias

Incorporación opcional solo si existe ventaja verificada, ejecución/red autorizadas, licencia compatible y contrato estable que conserve fuentes/datos. Si no hay ventaja o hay dependencia externa obligatoria incompatible, documentar descarte y mantener ADR-016 externo. Fallo/blocker no implica adoptar servicio cloud como sustitución automática.
