import { readFile } from "node:fs/promises";

const packageJsonPath = new URL("../package.json", import.meta.url);
const cargoTomlPath = new URL("../../core-rs/Cargo.toml", import.meta.url);
const cargoLockPath = new URL("../../../Cargo.lock", import.meta.url);

const packageJson = JSON.parse(await readFile(packageJsonPath, "utf8"));
const cargoToml = await readFile(cargoTomlPath, "utf8");
const cargoLock = await readFile(cargoLockPath, "utf8");
const cargoVersion = cargoToml.match(/^version\s*=\s*"([^"]+)"/m)?.[1];
const cargoLockVersion = cargoLock.match(
  /(\[\[package\]\]\nname = "core-rs"\nversion = )"([^"]+)"/m,
)?.[2];

if (!cargoVersion) {
  throw new Error("Could not read version from packages/core-rs/Cargo.toml");
}

if (!cargoLockVersion) {
  throw new Error('Could not read version for package "core-rs" from Cargo.lock');
}

if (cargoVersion !== packageJson.version) {
  throw new Error(`Version mismatch: Cargo.toml=${cargoVersion}, package.json=${packageJson.version}`);
}

if (cargoLockVersion !== packageJson.version) {
  throw new Error(
    `Version mismatch: Cargo.lock=${cargoLockVersion}, package.json=${packageJson.version}`,
  );
}

const explicitTag = process.env.RELEASE_TAG;
const githubRefType = process.env.GITHUB_REF_TYPE;
const githubRefName = process.env.GITHUB_REF_NAME;
const tag = explicitTag || (githubRefType === "tag" ? githubRefName : "");

if (tag) {
  const normalized = tag.startsWith("v") ? tag.slice(1) : tag;
  if (normalized !== packageJson.version) {
    throw new Error(`Tag mismatch: tag=${tag}, package.json=${packageJson.version}`);
  }
}

console.log(`Version check passed: ${packageJson.version}`);
