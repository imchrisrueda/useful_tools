# add_animation

**Propósito:** incorporar una explicación animada controlable. **Cuándo leer:** al preparar fase 3. Fase: 3; no soportada por MVP. Referencias: [PoC 2](../pocs/02-manim.md), [modelos](../DATA_MODELS.md).

## Entrada y salida

```ts
type Input = { projectId: string; expectedRevision: string; slideId: string;
  animation: { provider: string; sceneRef: string;
    parameters: Record<string, unknown>; segmentIds: string[];
    staticStateId: string; altText: string } };
type Output = { resourceIds: string[]; controlManifestRef: string;
  staticFallbackRef: string; revision: string };
```

Parámetros se validan contra el proveedor registrado; no se ejecuta código embebido arbitrario recibido como parámetro. El schema específico se cierra después de PoC 2.

## Precondiciones y dependencias

Proveedor habilitado, escena confiable, aprobación del significado y permiso de ejecución. Manim/Python opcionales con versiones fijadas; SVG/CSS sencillo puede evitar ese proveedor.

## Efectos

Renderiza segmentos/poster, registra hashes/parámetros, modifica slide e invalida sus derivados. Registra coste/duración; reutiliza caché compatible. No regenera datos científicos ni cambia evidencia.

## Errores y validación

CAPABILITY_UNSUPPORTED antes de escritura en MVP; DEPENDENCY_MISSING; INVALID_INPUT; PATH_DENIED; APPROVAL_REQUIRED; REVISION_CONFLICT; RENDER_FAILED. Verificar avance, retroceso, pausa, reinicio, movimiento reducido y estado estático. Animación semántica nueva necesita aprobación; vídeo no se declara editable en PPTX.
