---
name: ips-create-presentation
description: Crea una presentación científica estructurada con IPS, definiendo requisitos, storyboard y aplicando fuentes validadas.
---

# ips-create-presentation

## Cuándo activar
Activa esta skill cuando el usuario solicite crear una nueva presentación o diapositivas con Intelligent Presentation Studio (IPS).

## Entradas requeridas
- Nombre del proyecto y directorio raíz.
- Objetivo de la presentación y audiencia prevista.
- Duración estimada en minutos.
- Fuentes bibliográficas o datos (CSV) aportados.

## Procedimiento determinista
1. **Crear proyecto:** Ejecuta `ips create <nombre>` o la API `createProject` en `@ips/core`.
2. **Especificar requisitos:** Formaliza el objetivo y restricciones en `Requirements` y aplícalos mediante `applyRequirements`.
3. **Elaborar storyboard:** Asigna a cada slide un propósito claro, tiempo estimado y fuentes autorizadas (`Storyboard`), luego ejecuta `applyStoryboard`.
4. **Selección de diseño:** Consulta los temas disponibles (`scientific-minimal` o `tech-keynote`) con `suggestDesigns`. Solicita la aprobación del usuario antes de proceder a la generación.
5. **Generar diapositivas:** Con storyboard y diseño aprobados, genera las slides individuales en Markdown con `generateSlides`, respetando estrictamente:
   - **Rutas de assets:** Toda imagen, gráfico SVG o video en `slides/*.md` debe referenciar a `../assets/<archivo>` (nunca `assets/<archivo>`), para que Vite resuelva correctamente las importaciones.
   - **Ecuaciones KaTeX:** No envolver fórmulas matemáticas dentro de etiquetas HTML crudas (como `<p>`). Usar bloques display `$$ ... $$` o listas Markdown con líneas en blanco de separación antes y después de contenedores `<div>`.
6. **Validar y compilar:** Ejecuta `validatePresentation` para comprobar que no existan diapositivas vacías y que cuenten con notas de orador. Opcionalmente verifica con `slidev build slides.md` para asegurar compilación limpia sin advertencias de Vite.
