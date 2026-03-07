import { cp, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { spawnSync } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";

const packageRoot = path.dirname(fileURLToPath(new URL("../package.json", import.meta.url)));
const siteRoot = path.join(packageRoot, "site-dist");
const distRoot = path.join(packageRoot, "dist");
const demoRoot = path.join(packageRoot, "demo");
const browserClientSource = path.join(demoRoot, "browser-client.js");

function run(command, args, options = {}) {
  const result = spawnSync(command, args, {
    stdio: "inherit",
    ...options,
  });

  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

await rm(siteRoot, { recursive: true, force: true });
run("npm", ["run", "build"], { cwd: packageRoot });

await mkdir(siteRoot, { recursive: true });
await cp(distRoot, path.join(siteRoot, "dist"), { recursive: true });
await cp(path.join(demoRoot, "index.html"), path.join(siteRoot, "index.html"));
await cp(path.join(demoRoot, "main.js"), path.join(siteRoot, "main.js"));
await cp(path.join(demoRoot, "styles.css"), path.join(siteRoot, "styles.css"));

const browserClient = await readFile(browserClientSource, "utf8");
const siteBrowserClient = browserClient.replaceAll("../dist/generated/", "./dist/generated/");
await writeFile(path.join(siteRoot, "browser-client.js"), siteBrowserClient);

console.log("Built GitHub Pages demo artifact into packages/npm-core-rs/site-dist");
