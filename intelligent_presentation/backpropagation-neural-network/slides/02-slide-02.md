# Arquitectura de la Red: 1 Capa Oculta

<div class="grid grid-cols-2 gap-6 mt-4">

<div>

### Topología del Modelo (2-3-1)

- **Capa de Entrada:** 2 neuronas ($x_1, x_2$)
- **Capa Oculta:** 3 neuronas ($h_1, h_2, h_3$)
- **Capa de Salida:** 1 neurona ($\hat{y}$)

### Matrices de Parámetros

- $W^{(1)} \in \mathbb{R}^{3 \times 2}$ (Entrada $\to$ Oculta)
- $W^{(2)} \in \mathbb{R}^{1 \times 3}$ (Oculta $\to$ Salida)

</div>

<div>

### Función de Activación

Activación sigmoide logística:

$$\sigma(z) = \frac{1}{1 + e^{-z}}$$

Derivada analítica directa:

$$\sigma'(z) = \sigma(z)(1 - \sigma(z))$$

*Esta propiedad reduce drásticamente el coste computacional en la fase de gradientes.*

</div>

</div>

<!-- notas: Detallar la estructura y enfatizar que con 1 capa oculta ya es posible aproximar funciones no lineales continuas. -->