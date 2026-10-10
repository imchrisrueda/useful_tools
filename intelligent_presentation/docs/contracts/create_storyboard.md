# create_storyboard

**Propósito:** estructurar narrativa, evidencia y tiempos. **Cuándo leer:** al implementar planificación de contenido. Fase: MVP. Referencias: [modelos](../DATA_MODELS.md), [ciclo](../PROJECT_LIFECYCLE.md).

## Entrada y salida

```ts
type Input = { projectId: string; expectedRevision: string;
  requirementsRevision: string; sourceIds: string[]; proposal: Storyboard };
type Output = { storyboardRef: string; revision: string;
  unresolvedItems: string[] };
```

El agente produce proposal. La herramienta valida estructura y referencias; no sustituye una revisión científica o de narrativa.

## Precondiciones y dependencias

Requirements válidos, fuentes clasificadas y snapshot actual. Puede prepararse borrador antes de aprobación, pero generación exige aprobación conjunta de requisitos/storyboard.

## Efectos

Persiste storyboard y IDs estables; invalida planificación/generación/validación afectadas. No duplica texto definitivo ni calcula resultados experimentales por inferencia del modelo.

## Errores y validación

INVALID_INPUT por IDs duplicados, tiempos negativos, fuentes inexistentes; REVISION_CONFLICT por requisitos/revisión obsoletos; SOURCE_UNAUTHORIZED. Comprobar cobertura de temas, total temporal y fuentes por mensaje científico. Faltantes quedan en unresolvedItems y bloquean aprobación si son esenciales. Probar reordenación con conservación de IDs.
