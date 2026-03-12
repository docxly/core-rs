import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { spawnSync } from "node:child_process";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { assertDocxArchive, SMOKE_MARKDOWN } from "./_smoke-common.mjs";

const packageRoot = path.dirname(fileURLToPath(new URL("../package.json", import.meta.url)));
const cacheDir = path.join(os.tmpdir(), "docxly-npm-cache");

function run(command, args, options = {}) {
  const result = spawnSync(command, args, {
    stdio: "pipe",
    encoding: "utf8",
    ...options,
  });

  if (result.status !== 0) {
    throw new Error(
      [
        `Command failed: ${command} ${args.join(" ")}`,
        result.stdout?.trim(),
        result.stderr?.trim(),
      ]
        .filter(Boolean)
        .join("\n"),
    );
  }

  return result;
}

run("npm", ["run", "build"], {
  cwd: packageRoot,
});

const packResult = run("npm", ["pack", "--json", "--ignore-scripts", "--cache", cacheDir], {
  cwd: packageRoot,
});
const packJson = packResult.stdout.trim();
const [{ filename }] = JSON.parse(packJson);
const tarballPath = path.join(packageRoot, filename);
const tempRoot = await mkdtemp(path.join(os.tmpdir(), "docxly-pack-smoke-"));

try {
  await writeFile(
    path.join(tempRoot, "package.json"),
    JSON.stringify(
      {
        name: "docxly-pack-smoke",
        version: "0.0.0",
        private: true,
        type: "module",
      },
      null,
      2,
    ),
  );

  run("npm", ["install", "--cache", cacheDir, tarballPath], {
    cwd: tempRoot,
  });

  await writeFile(
    path.join(tempRoot, "run-smoke.mjs"),
    [
      'import { writeFile } from "node:fs/promises";',
      'import { analyzeMarkdown, generateDocxWithReport } from "@docxly/core-rs";',
      `const report = await analyzeMarkdown("<b>raw</b>", "docx");`,
      'if (report.fallbackCount !== 1) throw new Error("analyzeMarkdown contract changed");',
      `const result = await generateDocxWithReport(${JSON.stringify(SMOKE_MARKDOWN)});`,
      'await writeFile("output.docx", result.bytes);',
    ].join("\n"),
  );

  run("node", ["run-smoke.mjs"], {
    cwd: tempRoot,
  });

  const bytes = await readFile(path.join(tempRoot, "output.docx"));
  assertDocxArchive(bytes);
} finally {
  await rm(tempRoot, { recursive: true, force: true });
  await rm(tarballPath, { force: true });
}
