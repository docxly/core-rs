import assert from "node:assert/strict";
import * as esbuild from "esbuild";
import { mkdir, readFile, readdir, rm } from "node:fs/promises";
import path from "node:path";
import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { fileURLToPath } from "node:url";

const execFileAsync = promisify(execFile);
const packageRoot = path.dirname(fileURLToPath(new URL("../package.json", import.meta.url)));
const distDir = path.join(packageRoot, "dist");
const outdir = path.join(packageRoot, "tmp", "browser-smoke");

await rm(outdir, { recursive: true, force: true });
await mkdir(outdir, { recursive: true });

try {
  await execFileAsync("npm", ["run", "build"], { cwd: packageRoot });

  await esbuild.build({
    absWorkingDir: packageRoot,
    bundle: true,
    format: "esm",
    outdir,
    platform: "browser",
    write: true,
    loader: {
      ".wasm": "file",
    },
    stdin: {
      contents: [
        'import { analyzeMarkdown, generateDocx, generateDocxWithReport, generateHwpxWithReport } from "./browser.js";',
        "console.log(typeof analyzeMarkdown, typeof generateDocx, typeof generateDocxWithReport, typeof generateHwpxWithReport);",
      ].join("\n"),
      resolveDir: distDir,
      sourcefile: "browser-smoke-entry.js",
    },
  });

  const outputPaths = (await readdir(outdir)).map((file) => path.join(outdir, file));
  const jsBundles = outputPaths.filter((file) => file.endsWith(".js"));

  assert.ok(jsBundles.length > 0, "browser smoke must emit a bundle entry");

  const bundleSource = await readFile(jsBundles[0], "utf8");
  assert.match(bundleSource, /core_rs_bg\.wasm/, "browser bundle must reference the wasm runtime asset");
} finally {
  await rm(outdir, { recursive: true, force: true });
}
