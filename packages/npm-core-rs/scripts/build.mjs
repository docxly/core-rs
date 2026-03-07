import { cp, mkdir, rm } from "node:fs/promises";
import { spawnSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const packageRoot = path.dirname(fileURLToPath(new URL("../package.json", import.meta.url)));
const repoRoot = path.resolve(packageRoot, "../..");
const crateManifestPath = path.resolve(packageRoot, "../core-rs/Cargo.toml");
const distRoot = path.resolve(packageRoot, "dist");
const generatedRoot = path.resolve(distRoot, "generated");
const wasmArtifactPath = path.resolve(repoRoot, "target/wasm32-unknown-unknown/release/core_rs.wasm");

function resolveCommand(command, fallback) {
  if (process.env[command]) {
    return process.env[command];
  }
  if (fallback && fs.existsSync(fallback)) {
    return fallback;
  }
  return command.toLowerCase();
}

const wasmBindgenCommand = resolveCommand("WASM_BINDGEN", path.resolve(process.env.HOME ?? "", ".cargo/bin/wasm-bindgen"));

function run(command, args, options = {}) {
  return spawnSync(command, args, {
    stdio: "pipe",
    encoding: "utf8",
    ...options,
  });
}

function resolveRustToolchain() {
  const rustupCommand = resolveCommand("RUSTUP", "/opt/homebrew/opt/rustup/bin/rustup");
  const rustupWhich = run(rustupCommand, ["which", "--toolchain", "stable", "rustc"], {
    cwd: repoRoot,
  });

  if (rustupWhich.status === 0) {
    const rustcPath = rustupWhich.stdout.trim();
    if (rustcPath) {
      const cargoPath = path.resolve(path.dirname(rustcPath), "cargo");
      if (fs.existsSync(cargoPath)) {
        return {
          cargoPath,
          rustcPath,
        };
      }
    }
  }

  const cargoPath = resolveCommand("CARGO", null);
  const rustcPath = resolveCommand("RUSTC", null);
  return { cargoPath, rustcPath };
}

const rustToolchain = resolveRustToolchain();
const cargoEnv = {
  ...process.env,
  PATH: `${path.dirname(rustToolchain.cargoPath)}:${process.env.PATH ?? ""}`,
  RUSTC: rustToolchain.rustcPath,
  CARGO_BUILD_RUSTC: rustToolchain.rustcPath,
};

await rm(distRoot, { recursive: true, force: true });
await mkdir(generatedRoot, { recursive: true });

const cargoBuild = spawnSync(
  rustToolchain.cargoPath,
  ["build", "--manifest-path", crateManifestPath, "--target", "wasm32-unknown-unknown", "--release"],
  { stdio: "inherit", cwd: repoRoot, env: cargoEnv },
);
if (cargoBuild.status !== 0) {
  process.exit(cargoBuild.status ?? 1);
}

const wasmBindgen = spawnSync(
  wasmBindgenCommand,
  [wasmArtifactPath, "--out-dir", generatedRoot, "--target", "web", "--out-name", "core_rs"],
  { stdio: "inherit", cwd: repoRoot },
);
if (wasmBindgen.status !== 0) {
  process.exit(wasmBindgen.status ?? 1);
}

await cp(path.resolve(packageRoot, "src/node.js"), path.resolve(distRoot, "node.js"));
await cp(path.resolve(packageRoot, "src/browser.js"), path.resolve(distRoot, "browser.js"));
await cp(path.resolve(packageRoot, "src/index.d.ts"), path.resolve(distRoot, "index.d.ts"));

console.log("Built npm package artifacts into packages/npm-core-rs/dist");
