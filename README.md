<div align="center">
  <img src="./packages/npm-core-rs/demo/assets/logo.png" alt="docxly logo" width="96">
  <h1>docxly core-rs</h1>
  <p><strong>Embeddable Rust/WASM document generation for DOCX and HWPX.</strong></p>
  <p>Where Pandoc is a general-purpose converter, docxly is designed to live inside Node services, browser workflows, and product surfaces as a library.</p>
  <p>
    <a href="https://github.com/docxly/core-rs/actions/workflows/ci.yml"><img alt="CI" src="https://img.shields.io/github/actions/workflow/status/docxly/core-rs/ci.yml?branch=main&label=ci"></a>
    <a href="https://www.npmjs.com/package/@docxly/core-rs"><img alt="npm" src="https://img.shields.io/npm/v/%40docxly%2Fcore-rs?label=npm"></a>
    <a href="https://docxly.github.io/core-rs/"><img alt="demo" src="https://img.shields.io/badge/demo-live-1f7a4c"></a>
    <a href="https://github.com/docxly/core-rs/blob/main/LICENSE"><img alt="license" src="https://img.shields.io/github/license/docxly/core-rs"></a>
  </p>
</div>

Language: [English](./README.md) · [한국어](./docs/ko/README.md) · [Docs](./docs/README.md)

`docxly/core-rs` is an embeddable Rust/WASM document generation engine for DOCX and HWPX. Current benchmark details live in the comparison block below so the published README stays aligned with the checked-in benchmark dataset.

The project is developed with a TDD-first workflow. The current milestone implements the DOCX rich slice and an approved HWPX baseline backed by manually validated golden fixtures.

## Install

The fastest way to start using docxly today is the npm package:

```bash
npm install @docxly/core-rs
```

The Rust crate is the source of truth in this repository and can be consumed from the workspace or as a path dependency:

```toml
[dependencies]
core-rs = { path = "packages/core-rs" }
```

## Quick Start

### Node

```js
import { writeFile } from "node:fs/promises";
import { generateDocx } from "@docxly/core-rs";

const bytes = await generateDocx("# Hello\n\nThis is **docxly**.");
await writeFile("output.docx", bytes);
```

### Rust

```rust
use std::fs;

use core_rs::{DocxOptions, generate_docx};

let docx = generate_docx("# Hello\n\nThis is **docxly**.", DocxOptions::default())?;
fs::write("output.docx", docx)?;
```

## Why docxly

- Build document generation directly into a product instead of shelling out to a converter.
- Keep benchmark numbers in the checked-in comparison block below instead of scattering stale point-in-time claims through the README.
- Use the same Rust core across Node, browser, and HWPX workflows.
- Start with DOCX today and expand into HWPX from the same repository.
- Ship deterministic outputs backed by fixture-driven tests and normalized archive checks.

## Live Demo

Try the browser demo on GitHub Pages:

- https://docxly.github.io/core-rs/

The live page uses the published WASM wrapper and downloads real `.docx` and `.hwpx` files directly in the browser.

<!-- comparison:start -->

## Why docxly instead of Pandoc?

Pandoc is a general-purpose converter; docxly is an embeddable generation engine.

Use docxly when document generation must live inside a Node service, browser workflow, or product surface. Use Pandoc when you need broad format conversion and a CLI-first publishing workflow.

| Corpus | docxly cold | Pandoc cold | docxly steady | Pandoc steady | Speed ratio |
| --- | --- | --- | --- | --- | --- |
| Small | 63 ms | 454 ms | 1 ms | 253 ms | 379.53x |
| Medium | 83 ms | 306 ms | 2 ms | 95 ms | 60.33x |
| Large | 83 ms | 192 ms | 4 ms | 193 ms | 53.92x |
| Summary | 83 ms | 306 ms | 2 ms | 193 ms | 96.50x |

| Capability | DOCX | HWPX strict | HWPX compat |
| --- | --- | --- | --- |
| Headings and paragraphs | Yes | Yes | Yes |
| Inline emphasis, strong, code, links | Yes | Yes | Yes |
| Ordered lists up to depth 2 | Yes | No | Yes, semantic contract |
| Unordered lists up to depth 2 | Yes | Yes | Yes |
| Tables | Yes | Yes | Yes |
| `data:` URI images | Yes | No | Degraded fallback to alt text |
| Unsupported HTML | Strict: error, compat: literal text fallback | Error | Degraded literal text fallback |
| Footnotes, task lists, math | Strict: error, compat: visible text fallback | Error | Degraded visible text fallback |
| Deep nested lists | Strict: error, compat: plain text fallback | Error | Degraded plain text fallback |
| HWPX API in npm | N/A | Experimental API | Experimental API |

Measured on darwin 25.2.0 / arm64 at 2026-03-11T08:19:00.589Z with Node v23.7.0 and Pandoc 3.9.

- This benchmark measures DOCX generation only and does not compare HWPX.
- The numbers above come from an offline Node environment and are not browser runtime timings.
- Cold timings include WASM initialization for docxly and process startup for Pandoc.
- Steady timings are medians from 15 runs after one warm-up per corpus.

<!-- comparison:end -->

## Workspace Layout

```text
.
├── Cargo.toml
├── package.json
└── packages/
    ├── core-rs/
    │   ├── src/
    │   ├── tests/
    │   └── README.md
    └── npm-core-rs/
        ├── demo/
        ├── dist/
        ├── site-dist/
        └── README.md
```

## Current Status

- Implemented: Markdown parser, internal shared intermediate model, deterministic DOCX packaging
- Implemented: fixture-driven integration tests with normalized hash comparison
- Implemented: strict/fallback handling plus report APIs for unsupported HTML, images, task lists, math, and deep nested lists
- Implemented: approved HWPX baseline backed by manually validated fixtures
- Approved HWPX fixtures are the release gate; provisional and quarantined artifacts are not
- Approved fixture status is separate from strict/compat support claims; ordered lists are compat-only, while tables are supported in strict mode
- Package-specific details such as the current approved fixture set and Rust API notes live in `packages/core-rs/README.md` and `packages/core-rs/src/generators/hwpx/docs/README.md`

## Supported Markdown Today

The current implementation supports the DOCX rich slice:

- headings
- paragraphs
- emphasis
- strong emphasis
- inline code
- links
- blockquotes
- fenced code blocks
- thematic breaks
- ordered and unordered lists
- nested lists up to depth 2
- GFM pipe tables
- `data:` URI images

Current behavior:

- `strict_mode = true` by default
- unsupported input fails in strict mode
- unsupported input falls back to plain text when strict mode is disabled

## Public API

The Rust crate intentionally exposes only the high-level API. Parser, model, generator, and utility modules are internal implementation details.
Repository-level guidance stays high level here; concrete Rust API details live in `packages/core-rs/README.md`, and npm/browser usage lives in `packages/npm-core-rs/README.md`.

Recommended first flow:

1. generate a DOCX archive from Markdown
2. write the returned bytes to `output.docx`
3. use HWPX generation for the current core subset only

```rust
use std::fs;

use core_rs::{DocxOptions, generate_docx};

let docx = generate_docx(
    "# Hello\n\nThis is **docxly**.",
    DocxOptions::default(),
)?;
fs::write("output.docx", docx)?;
```

Notes:

- `generate_docx` returns a deterministic `.docx` archive as `Vec<u8>`
- `analyze_markdown`, `generate_docx_with_report`, and `generate_hwpx_with_report` surface strict/compat compatibility issues without widening the core generation API
- `generate_hwpx` targets the approved HWPX baseline reproduced by the committed golden fixtures
- `HwpxStyleOptions` currently supports document-level body/heading font, body/heading size, text/link/heading color, and paragraph alignment overrides
- Custom HWPX fonts are best-effort only; the current HWPX path records font family names but does not embed font binaries
- Internal modules such as parser/model/generator helpers are not part of the public contract

## HWPX Supported Today

The current HWPX path is narrower than the DOCX rich slice.

- approved fixture details: see `packages/core-rs/README.md` and `packages/core-rs/src/generators/hwpx/docs/README.md`
- strict-supported today:
  - paragraphs and headings
  - blockquotes
  - visible-text emphasis/strong/code/link rendering
  - unordered lists up to depth 2
  - tables
  - document-level HWPX style overrides
- compat-only semantic support:
  - ordered lists up to depth 2
- compat degraded fallback:
  - images
  - unsupported rich blocks such as code blocks and thematic breaks
  - deep nested lists

Example HWPX generation:

```rust
use std::fs;

use core_rs::{
    HwpxOptions, HwpxParagraphAlign, HwpxStyleOptions, generate_hwpx,
};

let hwpx = generate_hwpx(
    "# Title\n\n본문 **강조** [링크](https://example.com)",
    HwpxOptions {
        style: HwpxStyleOptions {
            body_font: Some("함초롬바탕".to_string()),
            heading_font: Some("함초롬돋움".to_string()),
            body_font_size: Some(1050),
            heading_font_size: Some(1500),
            text_color: Some("#222222".to_string()),
            heading_color: Some("#AA2200".to_string()),
            link_color: Some("#0055AA".to_string()),
            paragraph_align: Some(HwpxParagraphAlign::Justify),
        },
        ..HwpxOptions::default()
    },
)?;

fs::write("output.hwpx", hwpx)?;
```

## npm Package

The repository also contains an npm package at `packages/npm-core-rs/`.

- package name: `@docxly/core-rs`
- runtime target: Node + Browser
- public npm APIs: `generateDocx`, `generateHwpx`, `analyzeMarkdown`, `generateDocxWithReport`, `generateHwpxWithReport`
- HWPX is exposed as an experimental API in npm, with support differing between strict and compat mode

## Rust Crate Status

`packages/core-rs` is the Rust source of truth, but it is not published to `crates.io` in the
current release flow.

Use cases today:

- use `@docxly/core-rs` from npm for Node and browser runtimes
- use the Rust crate from this repository workspace or as a path dependency

Current HWPX status in the Rust crate:

```rust
use core_rs::{HwpxOptions, generate_hwpx};

let hwpx = generate_hwpx("# Hello\n\nThis is *core* HWPX.", HwpxOptions::default())?;
assert!(!hwpx.is_empty());
```

## Mono Repo Commands

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
npm run changeset:add
```

Live demo URL:

- https://docxly.github.io/core-rs/

What each command does:

- `build:web`: builds the npm WASM wrapper package
- `test:web`: runs Node, browser bundle, and packed package smoke tests
- `demo`: builds the Pages artifact and serves the browser demo locally
- `build:pages`: creates the static GitHub Pages artifact at `packages/npm-core-rs/site-dist`
- `test:all`: runs Rust lint, Rust tests, and web smoke tests from one root entrypoint
- `changeset:add`: creates a release note entry that feeds the automated npm version PR workflow

## Release Flow

- Add a changeset for npm-facing changes with `npm run changeset:add`
- When that changeset lands on `main`, the `Version Packages` workflow opens or updates a version PR
- That PR updates `packages/npm-core-rs/package.json`, syncs `packages/core-rs/Cargo.toml`, and refreshes `packages/npm-core-rs/package-lock.json`
- Publishing still happens from the existing tag-based `Release` workflow when a matching `v*.*.*` tag is pushed

## Running Tests

Run the Rust workspace tests from the repository root:

```bash
cargo test
```

Run tests for the crate only:

```bash
cargo test -p core-rs
```

Run a single integration test target:

```bash
cargo test -p core-rs --test docx_test
```

Run strict lint checks for the crate:

```bash
cargo clippy -p core-rs --all-targets -- -D warnings
```

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

## HWPX Reference Material

- `packages/core-rs/src/generators/hwpx/docs/README.md`
- `packages/core-rs/src/generators/hwpx/docs/schema-md/index.md`
- `packages/core-rs/src/generators/hwpx/reference/paragraph-only`

These files are contributor-facing HWPX references. They combine curated implementation notes, Markdown conversions of Hancom reference material, and the local approved/reference fixtures used to keep the package contract stable.

## Development Notes

- Keep README content and Git commit messages in English.
- Prefer adding tests before implementation changes.
- Keep archive output deterministic so fixture hashes remain stable.
- Keep new public surface area small. High-level generation functions and option types are the supported API.
- npm release is gated by a successful WASM build in CI.
- GitHub Pages deploys the static demo from the mono repo using a dedicated Pages workflow.

## Roadmap

- extend HWPX support from the current core subset to rich blocks
- extend conformance coverage with spec-based HWPX fixtures
