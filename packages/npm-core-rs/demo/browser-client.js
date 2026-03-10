import init, { generateDocxBytes, generateHwpxBytes } from "../dist/generated/core_rs.js";

let initPromise;

async function ensureInit() {
  if (!initPromise) {
    initPromise = init({
      module_or_path: new URL("../dist/generated/core_rs_bg.wasm", import.meta.url),
    });
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
