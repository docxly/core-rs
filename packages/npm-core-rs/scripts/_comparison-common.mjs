import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

export const packageRoot = path.dirname(fileURLToPath(new URL("../package.json", import.meta.url)));
export const repoRoot = path.resolve(packageRoot, "..", "..");
export const comparisonDataPath = path.join(packageRoot, "demo", "comparison-data.json");
export const comparisonContractPath = path.join(packageRoot, "demo", "comparison-contract.json");
export const readmePath = path.join(repoRoot, "README.md");
export const demoIndexPath = path.join(packageRoot, "demo", "index.html");
export const demoMainPath = path.join(packageRoot, "demo", "main.js");
export const comparisonStartMarker = "<!-- comparison:start -->";
export const comparisonEndMarker = "<!-- comparison:end -->";
const decoder = new TextDecoder();

export function formatMs(value) {
  return Number.isFinite(value) ? `${Math.round(value)} ms` : "Unavailable";
}

export function formatRatio(value) {
  return Number.isFinite(value) ? `${value.toFixed(2)}x` : "Unavailable";
}

export function decodeUtf8(bytes) {
  return decoder.decode(bytes);
}

export function validateComparisonContract(contract) {
  assert.equal(typeof contract, "object", "comparison contract must be an object");
  assert.equal(typeof contract.headline, "string", "headline must be a string");
  assert.equal(Array.isArray(contract.feature_matrix), true, "feature_matrix must be an array");
  assert.equal(Array.isArray(contract.caveats), true, "caveats must be an array");

  for (const entry of contract.feature_matrix) {
    assert.equal(typeof entry.feature, "string", "feature name must be a string");
    assert.equal(typeof entry.docx, "string", "feature docx value must be a string");
    assert.equal(typeof entry.hwpx_strict, "string", "feature hwpx_strict value must be a string");
    assert.equal(typeof entry.hwpx_compat, "string", "feature hwpx_compat value must be a string");
  }

  for (const caveat of contract.caveats) {
    assert.equal(typeof caveat, "string", "caveat must be a string");
  }

  return contract;
}

export function validateComparisonData(data) {
  assert.equal(typeof data, "object", "comparison data must be an object");
  assert.equal(typeof data.measured_at, "string", "measured_at must be a string");
  assert.equal(typeof data.machine_label, "string", "machine_label must be a string");
  assert.equal(typeof data.node_version, "string", "node_version must be a string");
  assert.equal(typeof data.pandoc_version, "string", "pandoc_version must be a string");
  assert.equal(typeof data.docxly_version, "string", "docxly_version must be a string");
  assert.equal(typeof data.benchmarks, "object", "benchmarks must be present");
  validateComparisonContract(data);

  for (const key of ["small", "medium", "large", "summary"]) {
    validateBenchmarkEntry(data.benchmarks[key], key);
  }

  return data;
}

function validateBenchmarkEntry(entry, name) {
  assert.equal(typeof entry, "object", `${name} benchmark must be an object`);
  assert.equal(typeof entry.label, "string", `${name}.label must be a string`);
  validateDuelMetric(entry.cold_ms, `${name}.cold_ms`);
  validateDuelMetric(entry.steady_median_ms, `${name}.steady_median_ms`);
  validateDuelMetric(entry.output_bytes, `${name}.output_bytes`);
  assert.equal(typeof entry.speed_ratio, "number", `${name}.speed_ratio must be a number`);
}

function validateDuelMetric(metric, name) {
  assert.equal(typeof metric, "object", `${name} must be an object`);
  assert.equal(typeof metric.docxly, "number", `${name}.docxly must be a number`);
  assert.equal(typeof metric.pandoc, "number", `${name}.pandoc must be a number`);
}

export async function loadComparisonData() {
  return loadValidatedJson(comparisonDataPath, validateComparisonData);
}

export async function loadComparisonContract() {
  return loadValidatedJson(comparisonContractPath, validateComparisonContract);
}

export function assertComparisonDataMatchesContract(data, contract) {
  assert.equal(data.headline, contract.headline, "comparison headline drifted from contract");
  assert.deepEqual(
    data.feature_matrix,
    contract.feature_matrix,
    "comparison feature matrix drifted from contract",
  );
  assert.deepEqual(data.caveats, contract.caveats, "comparison caveats drifted from contract");
}

export function renderReadmeBlock(data) {
  validateComparisonData(data);

  const benchmarkRows = ["small", "medium", "large", "summary"]
    .map((key) => {
      const entry = data.benchmarks[key];
      return [
        entry.label,
        formatMs(entry.cold_ms.docxly),
        formatMs(entry.cold_ms.pandoc),
        formatMs(entry.steady_median_ms.docxly),
        formatMs(entry.steady_median_ms.pandoc),
        formatRatio(entry.speed_ratio),
      ].join(" | ");
    })
    .map((row) => `| ${row} |`)
    .join("\n");

  const featureRows = data.feature_matrix
    .map(
      (entry) =>
        `| ${escapeCell(entry.feature)} | ${escapeCell(entry.docx)} | ${escapeCell(entry.hwpx_strict)} | ${escapeCell(entry.hwpx_compat)} |`,
    )
    .join("\n");

  const caveatRows = data.caveats.map((caveat) => `- ${caveat}`).join("\n");

  return [
    "## Why docxly instead of Pandoc?",
    "",
    "Pandoc is a general-purpose converter; docxly is an embeddable generation engine.",
    "",
    "Use docxly when document generation must live inside a Node service, browser workflow, or product surface. Use Pandoc when you need broad format conversion and a CLI-first publishing workflow.",
    "",
    "| Corpus | docxly cold | Pandoc cold | docxly steady | Pandoc steady | Speed ratio |",
    "| --- | --- | --- | --- | --- | --- |",
    benchmarkRows,
    "",
    "| Capability | DOCX | HWPX strict | HWPX compat |",
    "| --- | --- | --- | --- |",
    featureRows,
    "",
    `Measured on ${data.machine_label} at ${data.measured_at} with Node ${data.node_version} and Pandoc ${data.pandoc_version}.`,
    "",
    caveatRows,
    "",
  ].join("\n");
}

export function replaceReadmeBlock(readme, block) {
  const start = readme.indexOf(comparisonStartMarker);
  const end = readme.indexOf(comparisonEndMarker);

  assert.notEqual(start, -1, "README comparison start marker is missing");
  assert.notEqual(end, -1, "README comparison end marker is missing");
  assert.ok(end > start, "README comparison markers are out of order");

  const prefix = readme.slice(0, start + comparisonStartMarker.length);
  const suffix = readme.slice(end);
  return `${prefix}\n\n${block}\n${suffix}`;
}

export async function readPackageVersion() {
  const packageJson = JSON.parse(await readFile(path.join(packageRoot, "package.json"), "utf8"));
  return packageJson.version;
}

export async function assertDemoWiring() {
  const [indexHtml, mainJs] = await Promise.all([
    readFile(demoIndexPath, "utf8"),
    readFile(demoMainPath, "utf8"),
  ]);

  assert.ok(indexHtml.includes('id="comparison-section"'), "demo comparison section is missing");
  assert.ok(indexHtml.includes('id="comparison-ratio"'), "demo comparison KPI is missing");
  assert.ok(
    mainJs.includes("fetch(comparisonDataUrl") || mainJs.includes('fetch("./comparison-data.json"'),
    "demo must fetch comparison JSON",
  );
  assert.ok(mainJs.includes("comparison data unavailable"), "demo fallback copy is missing");
}

function escapeCell(value) {
  return value.replaceAll("|", "\\|");
}

async function loadValidatedJson(filePath, validate) {
  const raw = await readFile(filePath, "utf8");
  return validate(JSON.parse(raw));
}
