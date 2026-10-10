# Especificación 001 — MVP

**Propósito:** fijar el primer flujo completo. **Cuándo leer:** antes de T-013–T-025. Referencias: [requisitos](../../docs/PRODUCT_REQUIREMENTS.md), [plan](plan.md), [tareas](tasks.md).

## Objetivo

Un investigador crea y modifica una presentación científica mediante Codex o AGY usando artefactos locales y entrega web/PDF verificables con fuentes. El corpus de aceptación tendrá 15 slides con notas, referencias públicas/aportadas, figura y gráfico reproducible desde CSV sintético etiquetado. No se presentan resultados sintéticos como experimentos.

## Escenarios

1. Solicitud → requisitos/storyboard → aprobación → elección entre dos temas → aprobación → generación → revisión/corrección → aprobación final → entrega.
2. El usuario cambia una slide por ID: otras fuentes se conservan y derivados afectados se invalidan.
3. Datos cambian: todos sus consumidores se actualizan y unidades/significado se verifican.
4. Tema cambia: contenido/notas se conservan, capturas de diseño se renuevan.
5. Entrada inválida, falta de permiso o versión conflictiva: error explícito y última revisión válida intacta.
6. Solicitud exige PPTX/Manim/interacción: capacidad no soportada declarada, sin simulación ni instalación silenciosa.

## Criterios de aceptación propietarios

| ID | Criterio |
|---|---|
| AC-001 | Crear y entregar el corpus de 15 slides desde solicitud mediante cada agente, con artefactos intermedios e inputs/versiones registrados. |
| AC-002 | Conservar notas, fuentes y requisitos/storyboard/diseño aprobados; evidencia científica referenciada y sin citas/cifras inventadas. |
| AC-003 | CSV y ChartSpec producen valores/unidades esperados y gráfico con hashes/parámetros; entradas erróneas se rechazan. |
| AC-004 | Capturar 100 % de slides y estados de aparición definidos de la revisión actual, con IDs y evidencia. |
| AC-005 | Entrega sin recursos ausentes, errores de renderizado ni desbordamientos conocidos no exentos justificadamente. |
| AC-006 | Aplicar contraste 4,5:1 normal/3:1 grande; revisar manualmente elementos no evaluables, con teclado/alt donde apliquen. |
| AC-007 | Cuerpo >=24 px y referencias >=16 px en 1280×720 como política inicial; excepciones revisadas explícitamente. |
| AC-008 | Actualizar slide por ID sin reescribir otras fuentes; reutilizar recursos cuyos inputs/dependencias no cambian. |
| AC-009 | Reconstruir web/PDF desde fuentes y entorno fijados sin nuevas llamadas al modelo; comparar contenido/estructura/render normalizado. |
| AC-010 | Detectar cambio posterior e invalidar reporte y aprobación dependientes; impedir entrega con revisión obsoleta. |
| AC-011 | Mantener runtime/herramientas locales y core independiente del proveedor; rutas negativas/control de secretos/autorización de fuentes pasan. |
| AC-012 | Entregar reporte, fuentes/datos autorizados, licencias, versiones y guía Windows; declarar cada pérdida de funcionalidad. |

Los números tipográficos son políticas iniciales a calibrar, no resultados científicos. Detalles de checks/severidad en [calidad](../../docs/QUALITY_STRATEGY.md); mapeo TEST-* en [trazabilidad](../TRACEABILITY.md).

## Alcance y preparación

FR-001–010 y NFR-001–012. Dos temas y siete layouts; CSV de dialecto declarado, gráfico estático bar/line. Sin GUI, PPTX editable/importación, Manim/interacción, Excel/análisis avanzado ni modelo local obligatorio.

La implementación requiere EV-007: evidencia de PoC 1/5/6 y arquitectura/contratos aceptados. La solicitud de materializar documentos no es esa evidencia. Estas especificaciones no describen código ya disponible.
