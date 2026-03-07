import * as esbuild from "esbuild";
import path from "node:path";
import { fileURLToPath } from "node:url";

const packageRoot = path.dirname(fileURLToPath(new URL("../package.json", import.meta.url)));

await esbuild.build({
  absWorkingDir: packageRoot,
  bundle: true,
  format: "esm",
  outdir: "tmp/browser-smoke",
  platform: "browser",
  write: false,
  loader: {
    ".wasm": "file",
  },
  stdin: {
    contents: `import { generateDocx } from "./dist/browser.js"; console.log(typeof generateDocx);`,
    resolveDir: packageRoot,
    sourcefile: "browser-smoke-entry.js",
  },
});
