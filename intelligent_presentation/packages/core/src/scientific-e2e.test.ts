import test from 'node:test';
import assert from 'node:assert/strict';
import path from 'node:path';
import fs from 'node:fs/promises';
import {
  createProject,
  applyRequirements,
  applyStoryboard,
  suggestDesigns,
  generateSlides,
  parseScientificCsv,
  renderSvgChart,
  validatePresentation,
  exportPresentation,
  recordApproval
} from './index.js';

test('T-024 Full Scientific 15-Slide Presentation E2E Example', async () => {
  const exampleRoot = path.resolve(process.cwd().includes('packages') ? path.join(process.cwd(), '..', '..', 'examples') : path.join(process.cwd(), 'examples'));
  const projName = 'scientific-study-15';
  const projectDir = path.join(exampleRoot, projName);

  // Clean if previous run exists
  await fs.rm(projectDir, { recursive: true, force: true });

  try {
    // 1. Create project
    const projRes = await createProject({
      name: projName,
      root: exampleRoot,
      config: {
        language: 'es',
        aspectRatio: '16/9',
        engineId: 'slidev',
        title: 'Estudio de Eficacia Terapéutica en Expresión Génica'
      }
    });

    assert.equal(projRes.ok, true);
    if (!projRes.ok) return;
    const { revision: rev0 } = projRes.value;

    // 2. Parse scientific CSV
    const csvPath = path.join(exampleRoot, 'scientific-presentation', 'data', 'expression_data.csv');
    const csvContent = await fs.readFile(csvPath, 'utf-8');
    const csvRes = parseScientificCsv(csvContent);
    assert.equal(csvRes.ok, true);
    if (!csvRes.ok) return;

    // Copy data into project data/ folder
    await fs.writeFile(path.join(projectDir, 'data', 'expression_data.csv'), csvContent, 'utf-8');

    // 3. Render chart
    const chartRes = renderSvgChart({
      chartSpec: {
        id: 'fig-expression',
        datasetId: 'ds-expression',
        columns: { x: 'group', y: 'expression_level' },
        units: { expression_level: 'TPM' },
        type: 'bar',
        transform: { kind: 'identity' },
        missingPolicy: 'reject',
        caption: 'Figura 1: Niveles de expresión génica por tratamiento',
        altText: 'Gráfico de barras de niveles de expresión génica en TPM'
      },
      dataset: csvRes.value
    });
    assert.equal(chartRes.ok, true);
    if (!chartRes.ok) return;

    await fs.writeFile(path.join(projectDir, 'assets', 'fig-expression.svg'), chartRes.value.svgContent, 'utf-8');

    // 4. Requirements
    const reqRes = await applyRequirements({
      projectDir,
      expectedRevision: rev0,
      request: 'Presentar resultados del ensayo de expresión génica ante el comité',
      authorizedSourceIds: ['paper-ref-2026', 'dataset-expression'],
      proposal: {
        schemaVersion: '1.0',
        projectId: projRes.value.projectId,
        objective: 'Demostrar eficacia y significancia estadística de la combinación terapéutica',
        audience: 'Comité evaluador científico',
        durationMinutes: 15,
        requiredTopics: [
          'Contexto clínico',
          'Metodología',
          'Resultados cuantitativos',
          'Discusión y conclusiones'
        ],
        outputs: ['web', 'pdf'],
        constraints: ['15 diapositivas estructuradas', 'Datos sintéticos auditados'],
        unresolvedQuestions: []
      }
    });
    assert.equal(reqRes.ok, true);
    if (!reqRes.ok) return;
    const rev1 = reqRes.value.revision;

    // 5. Storyboard (15 slides)
    const slidesPlan = [
      { id: 's01', purpose: 'Portada y título del estudio', message: 'Eficacia terapéutica y modulación génica', time: 60 },
      { id: 's02', purpose: 'Resumen ejecutivo', message: 'Incremento significativo de expresión con sinergia comprobada', time: 60 },
      { id: 's03', purpose: 'Marco teórico y patología', message: 'Mecanismo molecular de dianas relevantes', time: 60 },
      { id: 's04', purpose: 'Hipótesis de investigación', message: 'La combinación A+B supera tratamientos mono-fármaco', time: 60 },
      { id: 's05', purpose: 'Diseño experimental', message: 'Cohortes celulares estandarizadas bajo normas GLP', time: 60 },
      { id: 's06', purpose: 'Protocolo de dosificación', message: 'Dosis equimolares mantenidas durante 48 horas', time: 60 },
      { id: 's07', purpose: 'Metodología analítica', message: 'Secuenciación de alta cobertura y calibración estricta', time: 60 },
      { id: 's08', purpose: 'Control de calidad de muestras', message: 'Integridad de ARN RIN > 9.0 en todas las réplicas', time: 60 },
      { id: 's09', purpose: 'Resultados: Grupos control y A', message: 'Respuesta parcial observada con compuesto A', time: 60 },
      { id: 's10', purpose: 'Resultados: Compuestos B y C', message: 'Compuesto B muestra mayor afinidad que C', time: 60 },
      { id: 's11', purpose: 'Gráfico cuantitativo de expresión', message: 'Sinergia estadísticamente superior (p < 0.0001)', time: 60 },
      { id: 's12', purpose: 'Análisis de significancia estadística', message: 'Intervalos de confianza al 99% confirman solidez', time: 60 },
      { id: 's13', purpose: 'Perfil de seguridad y tolerancia', message: 'Ausencia de toxicidad celular no deseada', time: 60 },
      { id: 's14', purpose: 'Conclusiones y próximos pasos', message: 'Validación preclínica exitosa para fase in vivo', time: 60 },
      { id: 's15', purpose: 'Agradecimientos y referencias', message: 'Agradecimiento a colaboradores y fuentes aportadas', time: 60 }
    ];

    const sbRes = await applyStoryboard({
      projectDir,
      expectedRevision: rev1,
      requirementsRevision: rev1,
      sourceIds: ['paper-ref-2026', 'dataset-expression'],
      proposal: {
        schemaVersion: '1.0',
        projectId: projRes.value.projectId,
        requirementsRevision: rev1,
        slides: slidesPlan.map((s) => ({
          slideId: s.id,
          purpose: s.purpose,
          message: s.message,
          sourceIds: ['paper-ref-2026'],
          assetIds: [],
          estimatedSeconds: s.time
        })),
        unresolvedItems: []
      }
    });
    assert.equal(sbRes.ok, true);
    if (!sbRes.ok) return;
    const rev2 = sbRes.value.revision;

    // 6. Suggest Designs (T-019)
    const designRes = suggestDesigns({
      projectId: projRes.value.projectId,
      revision: rev2,
      storyboardRevision: rev2,
      formats: ['web', 'pdf'],
      requestedCount: 2
    });
    assert.equal(designRes.ok, true);
    if (!designRes.ok) return;
    assert.equal(designRes.value.proposals.length, 2);

    // 7. Human approval
    const appRes = await recordApproval({
      projectDir,
      expectedRevision: rev2,
      approval: {
        id: 'app-sb-01',
        projectId: projRes.value.projectId,
        stage: 'storyboard',
        revision: rev2,
        decision: 'approved',
        actor: 'Comité Científico',
        timestamp: new Date().toISOString(),
        scope: '15 slides storyboard and scientific minimal theme'
      }
    });
    assert.equal(appRes.ok, true);

    // 8. Generate 15 slides
    const slideProposals = slidesPlan.map((s, idx) => {
      const isChartSlide = s.id === 's11';
      const body = isChartSlide
        ? `![Gráfico](assets/fig-expression.svg)\n\n*Nota: Datos sintéticos generados para demostración formal.*`
        : `### ${s.message}\n\nDetalles del ensayo según protocolo estandarizado.`;

      return {
        slideId: s.id,
        markdown: `# ${s.purpose}\n\n${body}\n\n<!-- notas: Presentación de ${s.purpose} ante el comité -->`,
        resourceIds: isChartSlide ? ['fig-expression'] : []
      };
    });

    const genRes = await generateSlides({
      projectDir,
      expectedRevision: rev2,
      storyboardRevision: rev2,
      designRef: 'scientific-minimal',
      proposal: slideProposals
    });
    assert.equal(genRes.ok, true);
    if (!genRes.ok) return;
    const rev3 = genRes.value.revision;
    assert.equal(genRes.value.slidePaths.length, 15);

    // 9. Validate presentation (T-021)
    const valRes = await validatePresentation({
      projectDir,
      revision: rev3,
      profile: 'mvp'
    });
    assert.equal(valRes.ok, true);
    if (!valRes.ok) return;
    assert.equal(valRes.value.result, 'pass');
    assert.equal(valRes.value.inspected.length, 15);

    // 10. Delivery approval & Export
    const deliveryAppRes = await recordApproval({
      projectDir,
      expectedRevision: rev3,
      approval: {
        id: 'app-deliv-01',
        projectId: projRes.value.projectId,
        stage: 'export',
        revision: rev3,
        decision: 'approved',
        actor: 'Investigador Principal',
        timestamp: new Date().toISOString(),
        scope: 'Final validated presentation export'
      }
    });
    assert.equal(deliveryAppRes.ok, true);

    const expRes = await exportPresentation({
      projectDir,
      revision: rev3,
      format: 'web',
      validationReportId: valRes.value.id,
      deliveryApprovalId: 'app-deliv-01'
    });
    assert.equal(expRes.ok, true);
    if (!expRes.ok) return;
    assert.equal(expRes.value.files.length, 1);
  } finally {
    await fs.rm(projectDir, { recursive: true, force: true });
  }
});
