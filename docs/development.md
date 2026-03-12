# Development Guide

This page captures the workspace-level development and test workflow for `docxly`.

## Package Surfaces

npm package:

- package name: `@docxly/core-rs`
- runtime target: Node + Browser
- public npm APIs:
  - `generateDocx(markdown, options) -> Promise<Uint8Array>`
  - `generateHwpx(markdown, options) -> Promise<Uint8Array>` as a beta surface for the current approved HWPX content baseline
  - npm HWPX currently exposes `title`, `author`, and `strictMode`, but not the Rust HWPX style override surface

Rust crate:

- source of truth: `packages/core-rs`
- publication status: not currently published to `crates.io`
- use cases today:
  - use `@docxly/core-rs` from npm for Node and browser runtimes
  - use the Rust crate from this repository workspace or as a path dependency

## Workspace Commands

Install workspace dependencies from the repository root:

```bash
npm install
```

Common root commands:

```bash
npm run build:web
npm run test:web
npm run demo
npm run build:pages
npm run test:all
```

What each command does:

- `build:web`: builds the npm WASM wrapper package
- `test:web`: runs Node, browser bundle, and packed package smoke tests
- `demo`: builds the Pages artifact and serves the browser demo locally
- `build:pages`: creates the static GitHub Pages artifact at `packages/npm-core-rs/site-dist`
- `test:all`: runs Rust lint, Rust tests, and web smoke tests from one root entrypoint

## Running Tests

Run the Rust workspace tests from the repository root:

```bash
cargo test
```

Run tests for the crate only:

```bash
cargo test -p core-rs
```

Run only the internal HWPX parser tests:

```bash
cargo test -p core-rs parser::hwpx
```
Run a single integration test target:

```bash
cargo test -p core-rs --test docx_test
```

Run strict lint checks for the crate:

```bash
cargo clippy -p core-rs --all-targets -- -D warnings
```

Inspect an HWPX archive from the repository tooling path:

```bash
cargo run -p core-rs --bin decode_hwpx -- --parse path/to/sample.hwpx
```

Useful flags:

- `--help`: print CLI usage
- `--parse`: print the internal semantic dump instead of raw XML console output
- `--out <directory>`: extract the package to disk for inspection
## Test Strategy

The project uses golden DOCX fixtures plus normalized hashing instead of comparing raw archive bytes.

Test checks include:

- the generated archive can be unzipped
- required DOCX entries exist
- XML files are well-formed
- `golden.docx` normalizes to the committed `expected/` tree
- normalized SHA-256 hash matches the committed `hash.txt`
- generated output normalizes to the same contents as `golden.docx`

Fixture files currently live under:

```text
packages/core-rs/tests/fixtures/
```

Each DOCX fixture directory contains:

```text
input.md
fixture.toml
golden.docx
expected/
hash.txt
```

`golden.docx` is a read-only baseline. There is no general-purpose command in the normal workflow that rewrites approved golden fixtures.

HWPX adds a second repository-level strategy on top of the golden generation checks:

- approved HWPX fixtures are release-gate goldens
- the internal parser reverse-parses approved `golden.hwpx` files back into the internal `Document` model
- parser tests use fixture-specific expected documents instead of relying on the Markdown parser as a runtime oracle
- malformed archives and unsupported HWPX structures are expected to fail fast
## Development Notes

- Keep README content and Git commit messages in English.
- Prefer adding tests before implementation changes.
- Keep archive output deterministic so fixture hashes remain stable.
- Keep new public surface area small. High-level generation functions and option types are the supported API.
- Keep repo-only debugging paths such as `decode_hwpx --parse` out of the stable public contract.
- npm release is gated by a successful WASM build in CI.
- GitHub Pages deploys the static demo from the mono repo using a dedicated Pages workflow.
