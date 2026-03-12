import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import { mkdir, readFile, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { performance } from "node:perf_hooks";
import { promisify } from "node:util";

import { unzipSync } from "fflate";

import {
  comparisonDataPath,
  decodeUtf8,
  formatMs,
  loadComparisonContract,
  packageRoot,
  readPackageVersion,
  readmePath,
  renderReadmeBlock,
  replaceReadmeBlock,
} from "./_comparison-common.mjs";
import { buildCorpora } from "./_benchmark-corpora.mjs";

const execFileAsync = promisify(execFile);
const tmpRoot = path.join(packageRoot, "tmp", "pandoc-benchmark");
const docxlyOnceRunner = path.join(packageRoot, "scripts", "_benchmark_docxly_once.mjs");
const corpusEntries = buildCorpora();

await rm(tmpRoot, { recursive: true, force: true });
await mkdir(tmpRoot, { recursive: true });

try {
  await execFileAsync("npm", ["run", "build"], { cwd: packageRoot });

  const { stdout: pandocStdout } = await execFileAsync("pandoc", ["--version"], { cwd: packageRoot });
  const pandocVersion = pandocStdout.split("\n")[0].replace(/^pandoc\s+/u, "").trim();
  assert.ok(pandocVersion, "failed to determine pandoc version");

  const { generateDocx } = await import("../dist/node.js");
  const docxlyVersion = await readPackageVersion();
  const comparisonContract = await loadComparisonContract();

  const benchmarks = {};
  for (const corpus of corpusEntries) {
    benchmarks[corpus.key] = await benchmarkCorpus(corpus, generateDocx);
  }

  benchmarks.summary = summarizeBenchmarks(benchmarks);

  const comparisonData = {
    headline: comparisonContract.headline,
    measured_at: new Date().toISOString(),
    machine_label: `${os.platform()} ${os.release()} / ${os.arch()}`,
    node_version: process.version,
    pandoc_version: pandocVersion,
    docxly_version: docxlyVersion,
    benchmarks,
    feature_matrix: comparisonContract.feature_matrix,
    caveats: comparisonContract.caveats,
  };

  await writeFile(comparisonDataPath, `${JSON.stringify(comparisonData, null, 2)}\n`);

  const readme = await readFile(readmePath, "utf8");
  const nextReadme = replaceReadmeBlock(readme, renderReadmeBlock(comparisonData));
  await writeFile(readmePath, nextReadme);

  for (const [key, entry] of Object.entries(benchmarks)) {
    console.log(
      `${key}: docxly ${formatMs(entry.steady_median_ms.docxly)} vs Pandoc ${formatMs(entry.steady_median_ms.pandoc)} (${entry.speed_ratio.toFixed(2)}x)`,
    );
  }
} catch (error) {
  if (isMissingPandoc(error)) {
    console.error("pandoc is required for benchmark:pandoc. Install it first and rerun the command.");
    process.exit(1);
  }

  throw error;
} finally {
  await rm(tmpRoot, { recursive: true, force: true });
}

async function benchmarkCorpus(corpus, generateDocx) {
  const inputPath = path.join(tmpRoot, `${corpus.key}.md`);
  const docxlyOutputPath = path.join(tmpRoot, `${corpus.key}-docxly.docx`);
  const pandocOutputPath = path.join(tmpRoot, `${corpus.key}-pandoc.docx`);

  await writeFile(inputPath, corpus.markdown);

  const docxlyCold = await measureDocxlyCold(inputPath, docxlyOutputPath);
  const pandocCold = await measurePandoc(inputPath, pandocOutputPath);

  verifyDocx(await readFile(docxlyOutputPath), corpus.tokens);
  verifyDocx(await readFile(pandocOutputPath), corpus.tokens);

  await generateDocx(corpus.markdown);
  await measurePandoc(inputPath, pandocOutputPath);

  const docxlySteadyRuns = [];
  const pandocSteadyRuns = [];

  for (let index = 0; index < 15; index += 1) {
    docxlySteadyRuns.push(await measureDocxlySteady(generateDocx, corpus.markdown, docxlyOutputPath));
    pandocSteadyRuns.push(await measurePandoc(inputPath, pandocOutputPath));
  }

  const docxlyBytes = (await readFile(docxlyOutputPath)).length;
  const pandocBytes = (await readFile(pandocOutputPath)).length;
  const docxlySteadyMedian = median(docxlySteadyRuns);
  const pandocSteadyMedian = median(pandocSteadyRuns);

  return {
    label: corpus.label,
    cold_ms: {
      docxly: Math.round(docxlyCold),
      pandoc: Math.round(pandocCold),
    },
    steady_median_ms: {
      docxly: Math.round(docxlySteadyMedian),
      pandoc: Math.round(pandocSteadyMedian),
    },
    output_bytes: {
      docxly: docxlyBytes,
      pandoc: pandocBytes,
    },
    speed_ratio: roundRatio(pandocSteadyMedian / docxlySteadyMedian),
  };
}

async function measureDocxlyCold(inputPath, outputPath) {
  const startedAt = performance.now();
  await execFileAsync("node", [docxlyOnceRunner, inputPath, outputPath], { cwd: packageRoot });
  return performance.now() - startedAt;
}

async function measureDocxlySteady(generateDocx, markdown, outputPath) {
  const startedAt = performance.now();
  const bytes = await generateDocx(markdown);
  await writeFile(outputPath, bytes);
  return performance.now() - startedAt;
}

async function measurePandoc(inputPath, outputPath) {
  const startedAt = performance.now();
  await execFileAsync("pandoc", ["-f", "markdown", "-t", "docx", "-o", outputPath, inputPath], {
    cwd: packageRoot,
  });
  return performance.now() - startedAt;
}

function verifyDocx(bytes, tokens) {
  const archive = unzipSync(bytes);
  assert.ok(archive["[Content_Types].xml"], "DOCX archive must include [Content_Types].xml");
  assert.ok(archive["_rels/.rels"], "DOCX archive must include _rels/.rels");
  assert.ok(archive["word/document.xml"], "DOCX archive must include word/document.xml");

  const documentXml = decodeUtf8(archive["word/document.xml"]);
  for (const token of tokens) {
    assert.ok(documentXml.includes(token), `DOCX output is missing token: ${token}`);
  }
}

function summarizeBenchmarks(benchmarks) {
  const corpusKeys = ["small", "medium", "large"];
  return {
    label: "Summary",
    cold_ms: {
      docxly: Math.round(median(corpusKeys.map((key) => benchmarks[key].cold_ms.docxly))),
      pandoc: Math.round(median(corpusKeys.map((key) => benchmarks[key].cold_ms.pandoc))),
    },
    steady_median_ms: {
      docxly: Math.round(median(corpusKeys.map((key) => benchmarks[key].steady_median_ms.docxly))),
      pandoc: Math.round(median(corpusKeys.map((key) => benchmarks[key].steady_median_ms.pandoc))),
    },
    output_bytes: {
      docxly: Math.round(median(corpusKeys.map((key) => benchmarks[key].output_bytes.docxly))),
      pandoc: Math.round(median(corpusKeys.map((key) => benchmarks[key].output_bytes.pandoc))),
    },
    speed_ratio: roundRatio(
      median(corpusKeys.map((key) => benchmarks[key].steady_median_ms.pandoc)) /
        median(corpusKeys.map((key) => benchmarks[key].steady_median_ms.docxly)),
    ),
  };
}

function median(values) {
  const sorted = [...values].sort((left, right) => left - right);
  const middle = Math.floor(sorted.length / 2);
  return sorted.length % 2 === 0
    ? (sorted[middle - 1] + sorted[middle]) / 2
    : sorted[middle];
}

function roundRatio(value) {
  return Number(value.toFixed(2));
}

function isMissingPandoc(error) {
  return error?.code === "ENOENT" || error?.message?.includes("pandoc");
}
