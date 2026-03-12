import { readFile, writeFile } from "node:fs/promises";

const packageJsonPath = new URL("../package.json", import.meta.url);
const cargoTomlPath = new URL("../../core-rs/Cargo.toml", import.meta.url);
const cargoLockPath = new URL("../../../Cargo.lock", import.meta.url);

const packageJson = JSON.parse(await readFile(packageJsonPath, "utf8"));
const cargoToml = await readFile(cargoTomlPath, "utf8");
const cargoLock = await readFile(cargoLockPath, "utf8");

const updatedCargoToml = cargoToml.replace(
  /^version\s*=\s*"[^"]+"/m,
  `version = "${packageJson.version}"`,
);
const updatedCargoLock = cargoLock.replace(
  /(\[\[package\]\]\nname = "core-rs"\nversion = )"[^"]+"/m,
  `$1"${packageJson.version}"`,
);

if (updatedCargoToml === cargoToml) {
  console.log(`Cargo.toml already matches ${packageJson.version}`);
} else {
  await writeFile(cargoTomlPath, updatedCargoToml);
  console.log(`Updated Cargo.toml to ${packageJson.version}`);
}

if (updatedCargoLock === cargoLock) {
  console.log(`Cargo.lock already matches ${packageJson.version}`);
} else {
  await writeFile(cargoLockPath, updatedCargoLock);
  console.log(`Updated Cargo.lock to ${packageJson.version}`);
}
