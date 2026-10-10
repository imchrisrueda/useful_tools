---
name: ips-review-export
description: Valida rigurosamente la calidad de una presentación IPS y emite el bundle final de entrega en Web o PDF.
---

# ips-review-export

## Cuándo activar
Activa esta skill cuando el usuario solicite validar la presentación final, inspeccionar defectos o exportar a Web / PDF.

## Entradas requeridas
- Ruta del proyecto (`projectDir`).
- Formato de exportación deseado (`web` o `pdf`).
- Identificador de la aprobación final de entrega por el usuario.

## Procedimiento determinista
1. **Validación de calidad:** Ejecuta `validatePresentation` con el perfil `mvp`.
2. **Comprobar findings:**
   - Si existen errores (`fail`), detén la exportación y reporta los defectos encontrados para su reparación.
   - Si existen advertencias (`needs_review`), presenta las observaciones al usuario para su aceptación explícita.
3. **Exportar:** Con validación aprobada y aprobación de entrega registrada, ejecuta `exportPresentation`.
4. **Verificación de entrega:** Comprueba que el archivo en `outputs/` y su hash SHA-256 en `export-report.json` coincidan exactamente.
