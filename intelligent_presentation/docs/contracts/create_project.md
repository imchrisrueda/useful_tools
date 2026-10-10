# create_project

**Propósito:** inicializar un proyecto sin sobrescribir archivos. **Cuándo leer:** al implementar creación. Fase: MVP. Referencias: [modelos/resultados](../DATA_MODELS.md), [estado](../PROJECT_LIFECYCLE.md).

## Entrada y salida

```ts
type Input = { name: string; root: string; config: {
  language: string; aspectRatio: '16:9'; engineId: 'slidev';
} };
type Output = { projectId: string; manifestPath: string; revision: string };
```

root debe encontrarse en una ubicación autorizada; nombre no vacío y sin separadores de ruta. Devuelve Result<Output>. El manifiesto satisface ProjectManifest y declara schemaVersion.

## Precondiciones y dependencias

Permiso de escritura, destino libre, core/esquemas y utilidades de rutas. No requiere modelo IA, servidor ni instalación del motor para crear metadatos; doctor puede advertir dependencias de generación ausentes.

## Efectos

Crea manifiesto, estructura del proyecto e historial inicial mediante staging. No copia datos privados ni abre servidor. No modifica contenido de una ubicación existente. Fallo de inicialización limpia únicamente staging propio y no deja manifiesto válido parcial.

## Errores y validación

INVALID_INPUT: nombre/config inválidos o destino ya ocupado; PATH_DENIED: ruta fuera de permisos o escape; DEPENDENCY_MISSING: runtime obligatorio ausente. Validar manifest, resolución de rutas incluidas, ausencia de sobrescritura y lectura posterior. Prueba negativa: ruta con `..` que escapa, enlace externo y destino ocupado.
