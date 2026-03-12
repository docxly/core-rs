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

## Who This Project Is For

`docxly` is for product teams that need document generation inside an application, not as a separate CLI step.

- backend teams generating DOCX files from Node services
- browser products that need client-side document export
- teams serving Korean market workflows that may need HWPX from the same Rust core
- platform teams that need deterministic output and fixture-based regression checks

`docxly` is not trying to replace Pandoc as a universal document conversion tool. It is a library-first engine for app-embedded generation.

## Install

The fastest way to start using docxly today is the npm package:

```bash
npm install @docxly/core-rs
```

If you are evaluating `docxly` for the first time, start with the npm package and verify the DOCX path first.
Rust consumers can use the workspace crate described in [packages/core-rs/README.md](./packages/core-rs/README.md).

## Quick Start

### Node

```js
import { writeFile } from "node:fs/promises";
import { generateDocx } from "@docxly/core-rs";

const bytes = await generateDocx("# Hello\n\nThis is **docxly**.");
await writeFile("output.docx", bytes);
```

### Browser

Use a bundler/runtime that can emit the `.wasm` asset.

```js
import { generateDocx } from "@docxly/core-rs";

const bytes = await generateDocx("# Hello\n\nBrowser-local DOCX.");
const blob = new Blob([bytes], {
  type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
});
const url = URL.createObjectURL(blob);
const anchor = document.createElement("a");
anchor.href = url;
anchor.download = "docxly-browser.docx";
anchor.click();
URL.revokeObjectURL(url);
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

## Adoption Path

Choose the entrypoint that matches your deployment target:

- First evaluation or production DOCX path: start with `@docxly/core-rs`
- Browser-local generation: use `@docxly/core-rs` with a bundler that emits `.wasm`
- Rust integration inside this repository or a sibling workspace: use `packages/core-rs`
- HWPX exploration: use the npm or Rust HWPX beta path today and stay inside the approved baseline for that surface

If you need broad format conversion, document publishing pipelines, or `reference.docx` template workflows, use Pandoc instead.

## Can You Use It Today?

- npm DOCX: yes, this is the primary production recommendation
- npm HWPX: yes, but treat it as beta and stay inside the approved baseline
- Rust DOCX: yes, for repository or workspace consumers
- Rust HWPX: yes, but treat it as beta and use the documented approved contract

## Support Matrix

For a fuller version, see [docs/support-matrix.md](./docs/support-matrix.md) and [docs/ko/support-matrix.md](./docs/ko/support-matrix.md).

| Surface | Status | Contract |
| --- | --- | --- |
| npm DOCX API (`generateDocx`) | Stable in `0.x` | Public package API, smoke-tested in Node and browser packaging flows |
| npm HWPX API (`generateHwpx`) | Beta | Shipped in the npm package, but limited to the current approved HWPX content baseline and default HWPX options |
| Rust DOCX API (`generate_docx`) | Stable in repository scope | High-level API only; internal modules remain private |
| Rust HWPX API (`generate_hwpx`) | Beta | Supported for the approved baseline and style contract documented in this README |
| Internal parser / model / generator modules | Private | May change at any time without notice |

Operational expectations:

- DOCX is the primary production path today.
- HWPX is available in both Rust and npm, but should be treated as beta and kept inside the approved fixture contract for each surface.
- Provisional and quarantined HWPX fixtures are not release-gate guarantees.
- Browser support assumes a modern bundler that can emit `.wasm` assets.

## Live Demo

Try the browser demo on GitHub Pages:

- https://docxly.github.io/core-rs/

The live page uses the published WASM wrapper and downloads real DOCX output directly in the browser, with a beta HWPX path available for the current approved baseline.

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

| Capability | docxly | Pandoc |
| --- | --- | --- |
| Embeddable in app | Yes, library-first for Node and browser bundlers | CLI-first with process invocation |
| Browser-local generation | First-party browser package and WASM path | Possible through pandoc.wasm, not the primary npm workflow |
| npm distribution | Published package | Not a first-party npm package |
| HWPX generation | Beta support in npm and Rust, scoped to the approved baseline | Not supported |
| Broad format conversion | Focused on DOCX and HWPX generation | Wide multi-format conversion |
| DOCX reference-template workflow | Not a reference.docx workflow | Supported via reference.docx |

| Capability | DOCX | HWPX strict | HWPX compat |
| --- | --- | --- | --- |
| Headings and paragraphs | Yes | Yes | Yes |
| Inline emphasis, strong, code, links | Yes | Yes | Yes |
| Ordered lists up to depth 2 | Yes | No | Yes, semantic contract |
| Unordered lists up to depth 2 | Yes | Yes | Yes |
| Tables | Yes | Yes | Yes |
| `data:` URI images | Yes | No | Degraded fallback to alt text |

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
- Current HWPX CI gates use these approved fixtures: `core-paragraph`, `blockquote-basic`, `code-block-basic`, `core-heading`, `core-inline-style`, `core-link-text`, `core-mixed`, `list-basic`, `list-nested-depth-2`, `ordered-list-basic`, `ordered-list-nested-depth-2`, `table-basic`, `table-alignment`, `style-typography`, `style-centered-layout`, and `style-brand-color`
- Approved fixture status is separate from strict/compat support claims; ordered-list fixtures are approved compatibility fixtures, while tables are supported in strict mode
- Provisional and quarantined HWPX artifacts are excluded from the release gate
- HWPX style options currently apply to body paragraphs and heading paragraphs; future block types such as lists and tables may add more paragraph categories

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

## HWPX Guide

The current HWPX path is narrower than the DOCX rich slice and remains beta on both Rust and npm surfaces.

- The npm HWPX path is still exposed as an Experimental API while the approved baseline continues to expand.

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

- Rust HWPX includes the approved compatibility, list, table, and style baselines.
- Ordered-list fixtures are approved compatibility fixtures, but they remain compat-only rather than strict-mode guarantees.
- npm HWPX uses the same core path, but exposes only `title`, `author`, and `strictMode`.
- images and broader table coverage are still outside the approved contract.

Detailed HWPX status, examples, and reference material live in [docs/hwpx.md](./docs/hwpx.md).

## Experimental API

Repository contributors also have a repo-only HWPX inspection path for debugging approved fixtures and generated archives:

```bash
cargo run -p core-rs --bin decode_hwpx -- --parse path/to/sample.hwpx
```

- This is developer tooling, not a stable Rust or npm API contract.
- `--parse` prints the current semantic interpretation of `content.hpf`, `header.xml`, and `section0.xml`.
- Use `--out <directory>` when you need the raw archive files on disk alongside that semantic dump.

## Public Roadmap

For more detail, see [docs/roadmap.md](./docs/roadmap.md) and [docs/ko/roadmap.md](./docs/ko/roadmap.md).

Near-term priorities:

1. Make the DOCX npm path easier to adopt with product-oriented examples for Node, browser, and service workflows.
2. Expand HWPX from the current approved baseline into a clearer staged contract for images, broader table coverage, and richer layout coverage.
3. Tighten repository onboarding with contribution docs, issue templates, and release expectations for external contributors.
4. Clarify packaging strategy for Rust consumers, including whether and when `crates.io` publication should become part of the release flow.
5. Collect real adoption feedback from integrators before widening the public API surface.

## Development

Development, testing, and workspace-level package details now live in [docs/development.md](./docs/development.md).

Common root commands:

```bash
npm install
npm run test:all
npm run demo
```

## Community

- Contribution guide: [CONTRIBUTING.md](./CONTRIBUTING.md)
- Code of conduct: [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md)
- Security policy: [SECURITY.md](./SECURITY.md)
- Roadmap: [docs/roadmap.md](./docs/roadmap.md)
- Support matrix: [docs/support-matrix.md](./docs/support-matrix.md)
- HWPX guide: [docs/hwpx.md](./docs/hwpx.md)
- Development guide: [docs/development.md](./docs/development.md)
