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
  packageRoot,
  readPackageVersion,
  readmePath,
  renderReadmeBlock,
  replaceReadmeBlock,
} from "./_comparison-common.mjs";

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

  const benchmarks = {};
  for (const corpus of corpusEntries) {
    benchmarks[corpus.key] = await benchmarkCorpus(corpus, generateDocx);
  }

  benchmarks.summary = summarizeBenchmarks(benchmarks);

  const comparisonData = {
    headline: "Offline Node benchmark for library selection.",
    measured_at: new Date().toISOString(),
    machine_label: `${os.platform()} ${os.release()} / ${os.arch()}`,
    node_version: process.version,
    pandoc_version: pandocVersion,
    docxly_version: docxlyVersion,
    benchmarks,
    feature_matrix: [
      {
        feature: "Embeddable in app",
        docxly: "Yes, library-first for Node and browser bundlers",
        pandoc: "CLI-first with process invocation",
      },
      {
        feature: "Browser-local generation",
        docxly: "First-party browser package and WASM path",
        pandoc: "Possible through pandoc.wasm, not the primary npm workflow",
      },
      {
        feature: "npm distribution",
        docxly: "Published package",
        pandoc: "Not a first-party npm package",
      },
      {
        feature: "HWPX generation",
        docxly: "Supported in the Rust core",
        pandoc: "Not supported",
      },
      {
        feature: "Broad format conversion",
        docxly: "Focused on DOCX and HWPX generation",
        pandoc: "Wide multi-format conversion",
      },
      {
        feature: "DOCX reference-template workflow",
        docxly: "Not a reference.docx workflow",
        pandoc: "Supported via reference.docx",
      },
    ],
    caveats: [
      "This benchmark measures DOCX generation only and does not compare HWPX.",
      "The numbers above come from an offline Node environment and are not browser runtime timings.",
      "Cold timings include WASM initialization for docxly and process startup for Pandoc.",
      "Steady timings are medians from 15 runs after one warm-up per corpus.",
    ],
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

function buildCorpora() {
  const smallMarkdown = `# Small Benchmark Title

Small benchmark paragraph with **bold**, *italic*, \`inline code\`, and [an example link](https://example.com).

Small benchmark keeps a second paragraph to force additional text runs and paragraph nodes in the DOCX tree.

> Small benchmark quote for DOCX output checks.

1. First benchmark ordered item
2. Second benchmark ordered item

- First benchmark bullet
  - First benchmark nested bullet
- Second benchmark bullet

\`\`\`text
small-benchmark-code
\`\`\`

| Capability | Value | Notes |
| --- | --- | --- |
| Engine | docxly | Small corpus |
| Scope | Small | Baseline |
| Depth | 2 | Nested list enabled |`;

  const mediumSections = Array.from({ length: 12 }, (_, index) =>
    buildBenchmarkSection({
      number: index + 1,
      size: "Medium",
      repeatParagraphs: 3,
      tableRows: 6,
      orderedCount: 3,
      unorderedCount: 3,
    }),
  ).join("\n\n");

  const largeSections = Array.from({ length: 28 }, (_, index) =>
    buildBenchmarkSection({
      number: index + 1,
      size: "Large",
      repeatParagraphs: 5,
      tableRows: 10,
      orderedCount: 4,
      unorderedCount: 4,
    }),
  ).join("\n\n");

  return [
    {
      key: "small",
      label: "Small",
      markdown: smallMarkdown,
      tokens: [
        "Small Benchmark Title",
        "Small benchmark paragraph",
        "First benchmark ordered item",
        "small-benchmark-code",
      ],
    },
    {
      key: "medium",
      label: "Medium",
      markdown: `# Medium Benchmark Title\n\n${buildCorpusLead("Medium", 12)}\n\n${mediumSections}\n\n${buildCorpusTail("Medium", 12)}`,
      tokens: [
        "Medium Benchmark Title",
        "Medium Section 1",
        "Medium nested bullet 3-2",
        "Medium code block 12",
      ],
    },
    {
      key: "large",
      label: "Large",
      markdown: `# Large Benchmark Title\n\n${buildCorpusLead("Large", 28)}\n\n${largeSections}\n\n${buildCorpusTail("Large", 28)}`,
      tokens: [
        "Large Benchmark Title",
        "Large Section 1",
        "Large ordered item 5-1",
        "Large nested bullet 28-3",
      ],
    },
  ];
}

function buildBenchmarkSection({
  number,
  size,
  repeatParagraphs,
  tableRows: tableRowCount,
  orderedCount,
  unorderedCount,
}) {
  const paragraphs = Array.from({ length: repeatParagraphs }, (_, index) => {
    const paragraphNumber = index + 1;
    return `${size} paragraph ${number}-${paragraphNumber} combines **bold**, *italic*, \`code-${number}-${paragraphNumber}\`, and [links](https://example.com/${size.toLowerCase()}/${number}/${paragraphNumber}) for DOCX run generation.`;
  }).join("\n\n");

  const orderedList = Array.from({ length: orderedCount }, (_, index) => {
    const itemNumber = index + 1;
    return `${itemNumber}. ${size} ordered item ${number}-${itemNumber}`;
  }).join("\n");

  const unorderedList = Array.from({ length: unorderedCount }, (_, index) => {
    const itemNumber = index + 1;
    return `- ${size} unordered item ${number}-${itemNumber}\n  - ${size} nested bullet ${number}-${itemNumber}`;
  }).join("\n");

  const tableRows = Array.from({ length: tableRowCount }, (_, index) => {
    const rowNumber = index + 1;
    return `| ${number}.${rowNumber} | ${size} table value ${number}-${rowNumber} | ${size} table note ${number}-${rowNumber} |`;
  }).join("\n");

  return `## ${size} Section ${number}

${paragraphs}

> ${size} quote ${number} keeps blockquote output active for the benchmark corpus.

${orderedList}

${unorderedList}

\`\`\`json
{"section": ${number}, "kind": "${size.toLowerCase()}", "marker": "${size} code block ${number}"}
\`\`\`

| Metric | Value | Notes |
| --- | --- | --- |
${tableRows}`;
}

function buildCorpusLead(size, sectionCount) {
  return `${size} benchmark lead paragraph with **bold**, *italic*, \`lead-code\`, and [overview link](https://example.com/${size.toLowerCase()}/overview).

${size} benchmark includes ${sectionCount} sections to exercise repeated block generation, list rendering, table packaging, and code block serialization.`;
}

function buildCorpusTail(size, sectionCount) {
  return `## ${size} Closing Notes

This closing section confirms that the ${size.toLowerCase()} benchmark rendered ${sectionCount} content sections and preserves a final paragraph for end-of-document verification.`;
}
