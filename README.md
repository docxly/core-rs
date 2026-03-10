# docxly core-rs

`docxly/core-rs` is a Rust-first mono repo for a Markdown-based DOCX/HWPX generation library, plus an npm-facing WASM wrapper package.

The project is being developed with a TDD-first workflow. The current milestone implements the DOCX rich slice and an approved HWPX baseline backed by manually validated golden fixtures.

## Live Demo

Try the browser demo on GitHub Pages:

- https://docxly.github.io/core-rs/

The live page uses the published WASM wrapper and downloads a real `.docx` file directly in the browser.

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
- Implemented: strict/fallback handling for unsupported HTML, non-data images, and deep nested lists
- Implemented: approved HWPX baseline backed by manually validated fixtures
- Current HWPX CI gates use these approved fixtures: `core-paragraph`, `blockquote-basic`, `code-block-basic`, `core-heading`, `core-inline-style`, `core-link-text`, `core-mixed`, `list-basic`, `list-nested-depth-2`, `style-typography`, `style-centered-layout`, and `style-brand-color`
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

Current options:

- `title: Option<String>`
- `author: Option<String>`
- `strict_mode: bool`
- `style: HwpxStyleOptions` on `HwpxOptions` only

Notes:

- `generate_docx` returns a deterministic `.docx` archive as `Vec<u8>`
- `generate_hwpx` targets the approved HWPX baseline reproduced by the committed golden fixtures
- `HwpxStyleOptions` currently supports document-level body/heading font, body/heading size, text/link/heading color, and paragraph alignment overrides
- Custom HWPX fonts are best-effort only; the current HWPX path records font family names but does not embed font binaries
- internal modules such as parser/model/generator helpers are not part of the public contract

## HWPX Supported Today

The current HWPX path is narrower than the DOCX rich slice.

- approved baseline: `core-paragraph`, `blockquote-basic`, `code-block-basic`, `core-heading`, `core-inline-style`, `core-link-text`, `core-mixed`
- approved list baseline: `list-basic`, `list-nested-depth-2`
- approved style baseline: `style-typography`, `style-centered-layout`, `style-brand-color`
- supported content today:
  - paragraphs
  - headings
  - visible-text emphasis/strong/code/link rendering inside the approved compatibility contract
  - document-level HWPX style overrides
- not yet part of the approved HWPX baseline:
  - tables
  - images

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
- public npm API: `generateDocx(markdown, options) -> Promise<Uint8Array>`
- HWPX is intentionally not exposed in npm v0.x even though the Rust crate now implements the HWPX core subset

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
```

Live demo URL:

- https://docxly.github.io/core-rs/

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

These files are the repository-level reference corpus for HWPX work. They combine curated implementation notes, Markdown conversions of Hancom reference material, and the local approved/reference fixtures used to keep the package contract stable.

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
