# Actualización de Pesos y Convergencia

<div class="grid grid-cols-2 gap-6 mt-3">

<div>

### Regla de Descenso de Gradiente

$$w_{ij} \leftarrow w_{ij} - \eta \frac{\partial \mathcal{L}}{\partial w_{ij}}$$

Donde $\eta = 0.5$ es la tasa de aprendizaje (*learning rate*).

### Gradientes por Regla de la Cadena

- **Salida:** $\delta^{(2)} = (\hat{y} - y) \cdot \sigma'(z^{(2)})$
- **Oculta:** $\delta_j^{(1)} = (\delta^{(2)} W_j^{(2)}) \cdot \sigma'(z_j^{(1)})$
- **Ajuste:** $\frac{\partial \mathcal{L}}{\partial w_{jk}^{(1)}} = \delta_j^{(1)} x_k$

</div>

<div class="flex flex-col items-center">

<img src="../assets/fig-01-loss-curve.svg" class="w-full rounded border border-slate-700" alt="Curva de pérdida">

<p class="text-xs text-slate-400 mt-2 text-center">
Evolución empírica: el error cuadrático desciende de 0.211 a 0.001 tras 10 épocas.
</p>

</div>

</div>

<!-- notas: Concluir destacando cómo el ajuste progresivo de pesos garantiza la convergencia al mínimo de error. -->