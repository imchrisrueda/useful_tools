# generate_slides

**Propósito:** aplicar contenido aprobado como fuentes Slidev. **Cuándo leer:** al implementar generación/skill. Fase: MVP. Referencias: [modelos](../DATA_MODELS.md), [diseño](../DESIGN_SYSTEM.md), [ciclo](../PROJECT_LIFECYCLE.md).

## Entrada y salida

```ts
type Input = { projectId: string; expectedRevision: string;
  storyboardRevision: string; designRef: string;
  proposal: { slideId: string; markdown: string; resourceIds: string[] }[] };
type Output = { entryPath: string; slidePaths: string[];
  dependencyIndexRef: string; revision: string };
```

El agente redacta proposal; las herramientas la validan/aplican y generan gráficos deterministas. Markdown conserva contenido/notas/frontmatter; metadatos no duplican texto.

### Reglas críticas de fuentes y assets
1. **Rutas relativas a assets:** Las diapositivas se alojan en el subdirectorio `slides/`. Cualquier recurso en `assets/` (imágenes SVG, posters, videos MP4) debe referenciarse como `../assets/<archivo>`. Nunca usar `assets/<archivo>` ni `./assets/<archivo>` para evitar errores de resolución en Vite (`plugin:vite:import-analysis`).
2. **Ecuaciones KaTeX:** Las fórmulas matemáticas (`$` y `$$`) no deben encerrarse en etiquetas HTML en bloque (`<p>`, `<div>` continuos). Emplear bloques display `$$ ... $$` o listas Markdown nativas, separando cualquier contenedor HTML con líneas en blanco.

## Precondiciones y dependencias

Requisitos/storyboard y diseño aprobados y actuales; recursos autorizados, layouts registrados y engine-slidev. Código Vue solo de proyecto confiable/componentes registrados, no de documentos importados.

## Efectos

Escribe cada slide e índice de orden, assets derivados y dependencias mediante staging/lock. Invalida capturas, validación y aprobación final. No exporta ni declara calidad verificada.

## Errores y validación

APPROVAL_REQUIRED; REVISION_CONFLICT; INVALID_INPUT por IDs duplicados/layout inexistente; SOURCE_UNAUTHORIZED; PATH_DENIED; DEPENDENCY_MISSING. Comprobar correspondencia storyboard→slides, fuentes/notas, recursos y unidades. Renderer/quality posteriores detectan composición y ejecución; una aplicación sintácticamente válida no supera aceptación por sí sola.
