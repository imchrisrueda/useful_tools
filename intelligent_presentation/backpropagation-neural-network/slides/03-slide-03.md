# Forward Pass y Función de Pérdida

<div class="space-y-4 mt-4">

<div>

### 1. Flujo Hacia Adelante (Forward Pass)

- **Capa Oculta:** $z^{(1)} = W^{(1)}x + b^{(1)} \implies a^{(1)} = \sigma(z^{(1)})$
- **Capa de Salida:** $z^{(2)} = W^{(2)}a^{(1)} + b^{(2)} \implies \hat{y} = \sigma(z^{(2)})$

</div>

<div>

### 2. Función de Pérdida Cuadrática (MSE)

Para un ejemplo con valor objetivo $y$ y predicción $\hat{y}$:

$$\mathcal{L}(y, \hat{y}) = \frac{1}{2} (y - \hat{y})^2$$

*Discrepancia inicial en la demo:* $y = 0.20$ vs $\hat{y} = 0.85 \implies \mathcal{L} = 0.211$.

</div>

</div>

<!-- notas: Explicar cómo el error en la neurona de salida marca el punto de partida para el cálculo de gradientes. -->