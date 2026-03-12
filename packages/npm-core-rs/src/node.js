import { readFile } from "node:fs/promises";

import init, {
  analyzeMarkdownJson,
  generateDocxBytes,
  generateDocxWithReportJson,
  generateHwpxBytes,
  generateHwpxWithReportJson,
} from "./generated/core_rs.js";
import {
  normalizeReport,
  normalizeTarget,
  parseGenerationResponse,
  serializeHwpxStyle,
} from "./shared.js";

let initPromise;

async function ensureInit() {
  if (!initPromise) {
    initPromise = (async () => {
      const wasmBytes = await readFile(new URL("./generated/core_rs_bg.wasm", import.meta.url));
      await init({ module_or_path: wasmBytes });
    })();
  }

  await initPromise;
}

function generateDocxBytesInternal(markdown, options = {}) {
  return generateDocxBytes(
    markdown,
    options.title ?? null,
    options.author ?? null,
    options.strictMode ?? true,
  );
}

function generateHwpxBytesInternal(markdown, options = {}) {
  return generateHwpxBytes(
    markdown,
    options.title ?? null,
    options.author ?? null,
    options.strictMode ?? true,
    serializeHwpxStyle(options.style),
  );
}

export async function analyzeMarkdown(markdown, target) {
  await ensureInit();
  return normalizeReport(JSON.parse(analyzeMarkdownJson(markdown, normalizeTarget(target))));
}

export async function generateDocx(markdown, options = {}) {
  await ensureInit();
  return generateDocxBytesInternal(markdown, options);
}

export async function generateHwpx(markdown, options = {}) {
  await ensureInit();
  return generateHwpxBytesInternal(markdown, options);
}

export async function generateDocxWithReport(markdown, options = {}) {
  await ensureInit();
  return parseGenerationResponse(
    generateDocxWithReportJson(
      markdown,
      options.title ?? null,
      options.author ?? null,
      options.strictMode ?? true,
    ),
  );
}

export async function generateHwpxWithReport(markdown, options = {}) {
  await ensureInit();
  return parseGenerationResponse(
    generateHwpxWithReportJson(
      markdown,
      options.title ?? null,
      options.author ?? null,
      options.strictMode ?? true,
      serializeHwpxStyle(options.style),
    ),
  );
}
