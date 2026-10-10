# Intelligent Presentation Studio (IPS)

IPS es un sistema para crear, revisar y modificar presentaciones con Codex o Google Antigravity (AGY), conservando contenido, datos científicos, decisiones y fuentes reproducibles.

El monorepo cuenta con `@ips/core`, `@ips/cli`, el catálogo completo de 5 familias de diseño, recomendador multicriterio, integración de animaciones científicas con Manim v0.22.0, componentes interactivos y Slidev para renderizado y exportación web/PDF.

## Estado de implementación

- **Fase 1 (MVP)**: Implementada al 100%. Pipeline determinista con control criptográfico de revisiones (`computeCanonicalRevision`), ingesta estricta de CSV, gráficos SVG accesibles y reproducibles, Slidev modular y exportación auditada.
- **Fase 2 (Sistema de Diseño Avanzado)**: 5 familias de diseño (`scientific-minimal`, `tech-keynote`, `data-storytelling`, `interactive-workshop`, `executive-professional`) y motor de recomendación multicriterio (`suggestDesigns`).
- **Fase 3 (Animación Matemática e Interactividad)**: Módulo de animación científica con Manim (`python/backprop_scene.py`), contratos de control con teclado, posters estáticos de respaldo (`addAnimation`) y componentes interactivos confiables (`addInteraction`).
- **Presentación Piloto Concreta**: Proyecto real [`backpropagation-neural-network/`](backpropagation-neural-network/) con 5 diapositivas, fórmulas LaTeX/KaTeX, curvas de pérdida y video Manim de la red neuronal 2-3-1 con actualización gradual de pesos.
- **Suite de Pruebas**: 21 pruebas automatizadas pasando al 100% (`npm test` y `pytest`).

## Entrada por objetivo

| Necesidad | Documento |
|---|---|
| Trabajar como agente | [Instrucciones del componente](AGENTS.md) |
| Localizar información sin leer todo | [Índice de lectura](docs/INDEX.md) |
| Consultar backlog y tareas | [Backlog](TASKS.md) |
| Entender el producto | [Requisitos](docs/PRODUCT_REQUIREMENTS.md) |
| Entender la solución | [Arquitectura](docs/ARCHITECTURE.md) |
| Evaluar herramientas | [Evaluación tecnológica](docs/TECHNOLOGY_EVALUATION.md) |
| Consultar el orden de desarrollo | [Roadmap](docs/ROADMAP.md) |

## Comandos rápidos

```powershell
# Ejecutar suite completa de pruebas
npm test
.venv\Scripts\pytest

# Desplegar la presentación piloto de Backpropagation en Slidev
cd backpropagation-neural-network
npx slidev slides.md --open
```

La solicitud original se conserva en [idea.md](idea.md). Los ejemplos de datos son sintéticos y transparentes para docencia e investigación.
