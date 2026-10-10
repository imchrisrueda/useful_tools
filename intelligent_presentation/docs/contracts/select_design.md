# select_design

**Propósito:** aplicar selección humana de un diseño. **Cuándo leer:** al implementar aprobación y temas. Fase: MVP. Referencias: [modelos](../DATA_MODELS.md), [ciclo](../PROJECT_LIFECYCLE.md).

## Entrada y salida

```ts
type Input = { projectId: string; expectedRevision: string;
  proposalId: string; approval: ApprovalRecord };
type Output = { designRef: string; approvalRef: string; revision: string };
```

ApprovalRecord debe identificar actor humano, alcance y revisión de la propuesta. Es un registro de una decisión recibida; el agente no inventa consentimiento.

## Precondiciones y dependencias

Propuesta existente compatible con salidas y storyboard actual; catálogo y estado. No es preciso instalar tecnologías opcionales ajenas al tema seleccionado.

## Efectos

Registra selección y aprobación con snapshot resultante, actualiza manifiesto e invalida derivados dependientes del diseño. No reescribe narrativa, datos ni notas.

## Errores y validación

INVALID_INPUT por propuesta/approval inválidos; APPROVAL_REQUIRED si falta decisión humana; REVISION_CONFLICT si la propuesta o entradas están obsoletas; CAPABILITY_UNSUPPORTED por salida incompatible. Verificar tokens, versión, fuentes/licencias y conservación del contenido.
