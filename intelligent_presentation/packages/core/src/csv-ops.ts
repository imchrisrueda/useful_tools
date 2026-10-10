import { Result, okResult, errorResult } from './models.js';
import { computeSha256 } from './hash-utils.js';

export interface CsvDataset {
  headers: string[];
  rows: Record<string, string>[];
  rawHash: string;
}

export function parseScientificCsv(csvContent: string): Result<CsvDataset> {
  const trimmed = csvContent.trim();
  if (!trimmed) {
    return errorResult({
      code: 'INVALID_INPUT',
      message: 'CSV content is empty',
      recoverable: false
    });
  }

  const lines = trimmed.split(/\r?\n/).filter((line) => line.trim().length > 0);
  if (lines.length < 2) {
    return errorResult({
      code: 'INVALID_INPUT',
      message: 'CSV must contain at least a header row and one data row',
      recoverable: false
    });
  }

  const rawHeaders = lines[0].split(',').map((h) => h.trim());
  const headerSet = new Set<string>();

  for (let colIdx = 0; colIdx < rawHeaders.length; colIdx++) {
    const h = rawHeaders[colIdx];
    if (!h) {
      return errorResult({
        code: 'INVALID_INPUT',
        message: `Empty header column name at column index ${colIdx}`,
        recoverable: false
      });
    }
    if (headerSet.has(h)) {
      return errorResult({
        code: 'INVALID_INPUT',
        message: `Duplicate header name "${h}" at column index ${colIdx}`,
        recoverable: false
      });
    }
    headerSet.add(h);
  }

  const rows: Record<string, string>[] = [];
  const expectedColCount = rawHeaders.length;

  for (let rowIdx = 1; rowIdx < lines.length; rowIdx++) {
    const cells = lines[rowIdx].split(',').map((c) => c.trim());
    if (cells.length !== expectedColCount) {
      return errorResult({
        code: 'INVALID_INPUT',
        message: `Row ${rowIdx + 1} has ${cells.length} columns, expected ${expectedColCount}`,
        recoverable: false,
        details: { row: rowIdx + 1, expectedCols: expectedColCount, actualCols: cells.length }
      });
    }

    const rowObj: Record<string, string> = {};
    for (let c = 0; c < expectedColCount; c++) {
      const cellVal = cells[c];
      if (cellVal === '' || cellVal === 'NaN') {
        return errorResult({
          code: 'INVALID_INPUT',
          message: `Empty or NaN value at row ${rowIdx + 1}, column "${rawHeaders[c]}"`,
          recoverable: false,
          details: { row: rowIdx + 1, column: rawHeaders[c] }
        });
      }
      rowObj[rawHeaders[c]] = cellVal;
    }
    rows.push(rowObj);
  }

  return okResult({
    headers: rawHeaders,
    rows,
    rawHash: computeSha256(csvContent)
  });
}
