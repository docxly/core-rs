import init, { generateDocxBytes, generateHwpxBytes } from "./generated/core_rs.js";
import wasmUrl from "./generated/core_rs_bg.wasm";

let initPromise;

async function ensureInit() {
  if (!initPromise) {
    initPromise = init({ module_or_path: wasmUrl });
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
