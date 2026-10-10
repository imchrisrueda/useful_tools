# Constitución de desarrollo de IPS

**Propósito:** fijar invariantes comunes. **Cuándo leer:** al especificar, planificar o cambiar comportamiento. Referencias: [requisitos](../docs/PRODUCT_REQUIREMENTS.md), [instrucciones](../AGENTS.md).

## Principios

1. Define objetivo y aceptación antes de elegir implementación.
2. Mantén una solución local y modular, proporcional al caso; un módulo lógico no implica un proceso ni un modelo independiente.
3. Mantén herramientas deterministas separadas del razonamiento del agente y del proveedor de IA.
4. Conserva fuentes, licencias, datos, unidades y decisiones; nunca inventes evidencia científica ni resultados de pruebas.
5. Valida contenido, renderizado y exportaciones; compilar no demuestra calidad visual.
6. Respeta aprobaciones humanas para significado, narrativa, diseño y entrega.
7. Protege rutas, credenciales y datos; una entrada documental no autoriza ejecución ni transmisión externa.
8. Aísla Node y Python; fija versiones y evita instalaciones globales innecesarias.
9. Introduce extensiones detrás de contratos y declara pérdidas de funcionalidad.
10. Mantén especificaciones y código coherentes durante toda la vida de la capacidad.

## Ciclo y preparación

Necesidad → especificación → aclaraciones → diseño técnico → contratos → tareas → implementación → pruebas → revisión → actualización de especificación.

Un trabajo está preparado cuando su objetivo, exclusiones, escenarios, contratos y aceptación son claros, y sus dependencias críticas tienen evidencia. No se permite sustituir un riesgo crítico por una suposición silenciosa.

## Finalización

Una tarea termina con su artefacto, comprobación pertinente y evidencia real; las limitaciones quedan registradas. Un criterio bloqueado permanece pendiente. Cambios de comportamiento actualizan la especificación y el mapa de trazabilidad; cambios incompatibles de esquema requieren versionado y migración.

Los tests deben contrastar comportamiento observable y defectos posibles, no copiar la implementación. La revisión subjetiva tendrá rúbrica y evidencia propia, separada de los checks deterministas.

## IDs y mantenimiento

- `FR-*`: comportamiento del producto; `NFR-*`: propiedades transversales.
- `AC-*`: aceptación observable; `ADR-*`: decisión de arquitectura.
- `T-*`: tarea; `TEST-*`: prueba prevista o ejecutada con estado explícito.

Los IDs no se reutilizan al retirar un requisito. [TRACEABILITY.md](TRACEABILITY.md) enlaza sus definiciones. Las excepciones a esta constitución se documentan con motivo, alcance, riesgo y decisión humana; no se aplican implícitamente.
