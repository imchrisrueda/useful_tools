---
name: ips-edit-presentation
description: Modifica diapositivas individuales de una presentación IPS existente de forma incremental sin alterar fuentes ajenas.
---

# ips-edit-presentation

## Cuándo activar
Activa esta skill cuando el usuario solicite corregir, actualizar o modificar diapositivas en un proyecto IPS ya generado.

## Entradas requeridas
- Ruta del proyecto (`projectDir`).
- Identificador de la diapositiva a modificar (`slideId`).
- Contenido nuevo de Markdown o recursos asociados.
- Indicador de cambio semántico (`semanticChange`: true si cambia significado, narrativa o conclusiones científicas).

## Procedimiento determinista
1. **Comprobar revisión actual:** Obtén la revisión canónica vigente del proyecto desde `project.json`.
2. **Revisión de impacto:** Si `semanticChange` es true, solicita la aprobación explícita del usuario antes de aplicar el cambio.
3. **Aplicar edición incremental:** Ejecuta `updateSlide` con el contenido Markdown completo de la diapositiva modificada, asegurando:
   - Rutas relativas a assets desde `slides/`: usar `../assets/<archivo>`.
   - Formato matemático KaTeX: evitar envolver fórmulas en etiquetas HTML `<p>` y mantener líneas en blanco alrededor de bloques matemáticos `$$ ... $$` o contenedores `<div>`.
4. **Invalidación de artefactos:** Verifica que el reporte de validación y los reportes de exportación previos hayan sido invalidados automáticamente.
5. **Revalidar:** Ejecuta `validatePresentation` sobre la nueva revisión generada.
