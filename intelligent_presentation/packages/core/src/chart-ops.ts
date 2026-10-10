import { ChartSpec, ChartSpecSchema, Result, okResult, errorResult } from './models.js';
import { CsvDataset } from './csv-ops.js';
import { computeSha256 } from './hash-utils.js';

export interface RenderSvgChartInput {
  chartSpec: ChartSpec;
  dataset: CsvDataset;
}

export interface RenderSvgChartOutput {
  svgContent: string;
  dataHash: string;
  specHash: string;
  generatorVersion: string;
}

export function renderSvgChart(input: RenderSvgChartInput): Result<RenderSvgChartOutput> {
  let spec: ChartSpec;
  try {
    spec = ChartSpecSchema.parse(input.chartSpec);
  } catch (err: unknown) {
    return errorResult({
      code: 'INVALID_INPUT',
      message: `Invalid chart specification: ${(err as Error).message}`,
      recoverable: false
    });
  }

  const { x: xCol, y: yCol } = spec.columns;
  if (!input.dataset.headers.includes(xCol)) {
    return errorResult({
      code: 'INVALID_INPUT',
      message: `X-column "${xCol}" not found in dataset headers`,
      recoverable: false
    });
  }
  if (!input.dataset.headers.includes(yCol)) {
    return errorResult({
      code: 'INVALID_INPUT',
      message: `Y-column "${yCol}" not found in dataset headers`,
      recoverable: false
    });
  }

  // Parse numerical y values
  const parsedData: { x: string; y: number }[] = [];
  for (let i = 0; i < input.dataset.rows.length; i++) {
    const row = input.dataset.rows[i];
    const xVal = row[xCol];
    const yNum = Number(row[yCol]);

    if (!Number.isFinite(yNum)) {
      return errorResult({
        code: 'INVALID_INPUT',
        message: `Non-finite numeric value in Y column "${yCol}" at row ${i + 2}: "${row[yCol]}"`,
        recoverable: false,
        details: { row: i + 2, column: yCol, value: row[yCol] }
      });
    }

    parsedData.push({ x: xVal, y: yNum });
  }

  const width = 800;
  const height = 450;
  const padding = 60;
  const chartWidth = width - padding * 2;
  const chartHeight = height - padding * 2;

  const maxY = Math.max(...parsedData.map((d) => d.y), 1);
  const unitLabel = spec.units[yCol] ? ` (${spec.units[yCol]})` : '';

  let chartElements = '';

  if (spec.type === 'bar') {
    const barWidth = Math.max(10, (chartWidth / parsedData.length) * 0.7);
    const step = chartWidth / parsedData.length;

    parsedData.forEach((d, idx) => {
      const barHeight = (d.y / maxY) * chartHeight;
      const x = padding + idx * step + (step - barWidth) / 2;
      const y = padding + chartHeight - barHeight;

      chartElements += `
        <rect x="${x}" y="${y}" width="${barWidth}" height="${barHeight}" fill="#3b82f6" rx="3">
          <title>${d.x}: ${d.y}${unitLabel}</title>
        </rect>
        <text x="${x + barWidth / 2}" y="${height - padding + 20}" font-family="sans-serif" font-size="12" text-anchor="middle" fill="#333">${d.x}</text>
        <text x="${x + barWidth / 2}" y="${y - 6}" font-family="sans-serif" font-size="11" text-anchor="middle" fill="#555">${d.y}</text>
      `;
    });
  } else {
    // line chart
    const points: string[] = [];
    const step = parsedData.length > 1 ? chartWidth / (parsedData.length - 1) : chartWidth / 2;

    parsedData.forEach((d, idx) => {
      const x = padding + idx * step;
      const y = padding + chartHeight - (d.y / maxY) * chartHeight;
      points.push(`${x},${y}`);

      chartElements += `
        <circle cx="${x}" cy="${y}" r="4" fill="#2563eb" />
        <text x="${x}" y="${height - padding + 20}" font-family="sans-serif" font-size="12" text-anchor="middle" fill="#333">${d.x}</text>
        <text x="${x}" y="${y - 8}" font-family="sans-serif" font-size="11" text-anchor="middle" fill="#555">${d.y}</text>
      `;
    });

    chartElements = `
      <polyline fill="none" stroke="#2563eb" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" points="${points.join(' ')}" />
      ${chartElements}
    `;
  }

  const specHash = computeSha256(JSON.stringify(spec));
  const generatorVersion = '0.1.0';

  const svgContent = `<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${width} ${height}" width="${width}" height="${height}">
  <!-- Metadata & Provenance: dataHash=${input.dataset.rawHash} specHash=${specHash} -->
  <title id="title-${spec.id}">${spec.caption}</title>
  <desc id="desc-${spec.id}">${spec.altText}</desc>
  <rect width="100%" height="100%" fill="#ffffff" />
  
  <!-- Axes -->
  <line x1="${padding}" y1="${height - padding}" x2="${width - padding}" y2="${height - padding}" stroke="#999" stroke-width="1.5" />
  <line x1="${padding}" y1="${padding}" x2="${padding}" y2="${height - padding}" stroke="#999" stroke-width="1.5" />
  
  <!-- Y axis title -->
  <text x="${padding}" y="${padding - 15}" font-family="sans-serif" font-size="13" font-weight="bold" fill="#111">${yCol}${unitLabel}</text>
  
  <!-- Caption header -->
  <text x="${width / 2}" y="30" font-family="sans-serif" font-size="16" font-weight="bold" text-anchor="middle" fill="#111">${spec.caption}</text>

  <!-- Data elements -->
  ${chartElements}
</svg>`;

  return okResult({
    svgContent,
    dataHash: input.dataset.rawHash,
    specHash,
    generatorVersion
  });
}
