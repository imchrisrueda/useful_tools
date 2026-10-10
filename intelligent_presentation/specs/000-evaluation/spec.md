# Especificación 000 — Evaluación y arquitectura

**Propósito:** preparar implementación con riesgos críticos comprobados. **Cuándo leer:** antes de T-001–T-012. Referencias: [tareas](tasks.md), [plan](plan.md), [checklist](checklist.md).

## Objetivo y usuarios

El coordinador y los implementadores necesitan una arquitectura proporcional, contratos claros, backlog pequeño y evidencia de que el motor, la edición, la validación y los temas funcionan en el entorno Windows con ambos agentes.

## Alcance

Preparar todos los documentos del brief, modelos/contratos preliminares, siete protocolos PoC y ejecutar únicamente PoC 1/5/6 antes de cerrar arquitectura MVP. La documentación existente es una propuesta a revisar. Esta fase no implementa el producto completo ni incorpora servicios obligatorios.

## Escenarios

- El investigador identifica una capacidad solo documentada y diseña reproducción fijada a una release.
- El implementador prepara runtime local sin modificar instalaciones globales.
- Codex y AGY editan un corpus equivalente; se distinguen resultados CLI y MCP.
- El verificador detecta un desbordamiento deliberado y comprueba una reparación que preserva significado.
- El especialista visual cambia el tema del mismo corpus sin editar texto/notas.
- Una PoC falla o queda bloqueada: se registra evidencia y el ADR dependiente permanece abierto.

## Criterios de fase

- EV-001: cobertura del brief mediante requisitos, decisiones y documentos propietarios, sin contradicciones críticas abiertas.
- EV-002: once contratos con entradas, salidas, schemas preliminares, dependencias, efectos, errores y checks.
- EV-003: siete protocolos reproducibles y corpus público/redistribuible identificado; ninguna ejecución inventada.
- EV-004: entorno fijado, runtime compatible, versiones de agentes/superficies y licencias registradas.
- EV-005: PoC 1 pasa la vía portable con ambos agentes; estado MCP por plataforma registrado.
- EV-006: PoC 5 detecta defectos y verifica reparación/cobertura; PoC 6 conserva contenido en dos temas.
- EV-007: decisiones críticas del MVP resueltas y arquitectura/contratos aceptados explícitamente antes de código del núcleo.

EV-* son criterios de preparación, no sustituyen AC-* del producto. Requisitos aplicables: todos para cobertura; FR-005/006/010 y NFR-001/005/006/009 para experimentos.

## Exclusiones

No se requiere ejecutar PoC 2/3/4/7 todavía. No instalar tooling global, cambiar configuraciones reales para pruebas o evaluar documentos privados. Las dependencias locales de PoC solo se preparan bajo tareas autorizadas. Resultado documental no implica que EV-004–007 estén satisfechos.
