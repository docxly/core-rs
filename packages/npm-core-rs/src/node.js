import { readFile } from "node:fs/promises";

import init, { generateDocxBytes, generateHwpxBytes } from "./generated/core_rs.js";

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

export async function generateDocx(markdown, options = {}) {
  await ensureInit();
  return generateDocxBytes(
    markdown,
    options.title ?? null,
    options.author ?? null,
    options.strictMode ?? true,
  );
}

export async function generateHwpx(markdown, options = {}) {
  await ensureInit();
  return generateHwpxBytes(
    markdown,
    options.title ?? null,
    options.author ?? null,
    options.strictMode ?? true,
  );
}
