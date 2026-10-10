# Estrategia de calidad y pruebas

**Propósito:** definir cómo comprobar el producto. **Cuándo leer:** al diseñar pruebas, revisar o entregar. Referencias: [aceptación](../specs/001-mvp/spec.md), [PoC 5](pocs/05-validation.md), [trazabilidad](../specs/TRACEABILITY.md).

## Pirámide

| Nivel | Comprobaciones | Tipo de evidencia |
|---|---|---|
| Unitario | Esquemas, rutas, revisión/estado, hashes, CSV, valores/unidades y transformaciones. | Aserciones sobre casos válidos, inválidos y límites. |
| Integración | Assets, temas, Slidev, captura, exportación y reconstrucción. | Ejecución real con versiones y fixtures. |
| Visual | Todas las slides/estados, bounds, recursos, legibilidad y comparación. | Capturas, geometría, diferencias y diagnóstico. |
| Accesibilidad | Teclado, contraste, alt y movimiento reducido. | axe-core, recorrido de teclado y revisión manual. |
| E2E determinista | Artefactos aprobados → aplicación → render → validación → entrega. | Informe, salidas y manifiesto. |
| Evaluación de agentes | Solicitud natural → artefactos → presentación con Codex/AGY. | Rúbrica, conversación autorizada, revisiones y entrega. |

Muchos tests unitarios y menos integraciones/E2E; no se fija un porcentaje de cobertura que sustituya escenarios. Las pruebas con modelos son separadas y requieren autorización; la CI determinista no requiere credenciales IA.

## Cobertura visual y umbrales iniciales

- Capturar el 100 % de slides y estados definidos. Para MVP incluye cada estado de aparición; para interacción incluye mínimo/intermedio/máximo/reinicio y estados de error cuando existan.
- Antes de capturar esperar fuentes y recursos; fijar viewport 1280 × 720, navegador, fuentes y sistema. Congelar movimiento durante snapshots, pero probar controles/movimiento en escenarios específicos.
- Desbordamiento: un contenido fuera del lienzo o contenedor esperado es un defecto; adornos intencionales pueden declararse exentos. Tolerancia geométrica inicial 1 px para redondeo, pendiente de calibrar en PoC 5.
- Contraste de texto: 4,5:1 normal y 3:1 grande como política inicial. Gradientes, canvas y elementos no evaluables requieren revisión manual.
- Texto de cuerpo mínimo inicial 24 px; referencias 16 px en ese lienzo. Excepciones requieren revisión explícita y registro.
- Mantener relación de aspecto de imágenes, unidades y labels de gráficos; no admitir recursos ausentes ni errores de renderizado.
- Calibrar tolerancia de comparación visual con corpus positivo/negativo y ambiente fijado; no usar diferencia de píxeles como único juicio.

Los tamaños y tolerancias son políticas de producto iniciales, no umbrales científicamente validados. Las comprobaciones automáticas de accesibilidad no certifican accesibilidad completa. Fuentes oficiales en [evaluación tecnológica](TECHNOLOGY_EVALUATION.md).

## Revisión subjetiva

Rúbrica separada: adecuación a audiencia, mensaje por slide, conexión narrativa, densidad, legibilidad, significado científico y utilidad de movimiento/interacción. Registrar aceptado/cambio solicitado con slide/estado y motivo; sin promedio numérico que oculte un defecto grave.

La revisión por visión del modelo genera observaciones, no una prueba de verdad científica. Las cifras se contrastan con datos/transformaciones; referencias con fuentes aportadas/verificadas. Una reparación visual compara invariantes de contenido y números antes/después.

## Severidad y entrega

`blocker`: significado alterado, dato inventado, riesgo de ejecución/transmisión o artefacto ilegible. `error`: desbordamiento, recurso perdido, interacción/exports rotos. `warning`: observación que exige evaluación humana. Blocker/error impiden entrega; una warning exige resolución o aceptación explícita con evidencia. Un finding exento debe conservar razón y alcance.

Una validación solo sirve para su revisión/hash de dependencias. Ejecutar el E2E completo del ejemplo y comprobar todas las salidas antes de entregar. Registros de tests incluyen estado passed/failed/blocked/not_run; ausencia de ejecución no es passed.

## Rendimiento y reproducibilidad

Medir tiempo frío/caliente de generación de recursos, render y export, tamaño de outputs y cache hits para 15/50/100 slides. Registrar hardware, versión, corpus y opciones. Fijar objetivos tras baseline; el MVP exige reutilización demostrada, no un SLA inventado.

Reproducir desde fuentes fijadas sin IA. Comparar estructura, valores y render normalizado; metadatos de fecha de PDF/ZIP pueden variar y no se promete identidad binaria. Registrar hashes de fuentes y resultados; excluir secretos de bundles/logs.

## CI prevista

Workflow compartido filtrado por el componente; working-directory explícito. Gate principal Windows y comprobación secundaria Linux. Ejecutar schema/typecheck, lint sin reescritura, unitarios, integración y E2E aplicables. Comparaciones visuales mantienen baseline por entorno. Cambios de baseline requieren revisión, no actualización ciega.

Para la entrega documental actual: comprobar enlaces, IDs, cobertura y dependencias. No existen aún gates de aplicación ni resultados de PoC.
