# core-rs

`core-rs` is the Rust engine for `docxly`. It turns Markdown into deterministic DOCX and HWPX archives. This crate is also compiled to WASM for the npm package in `../npm-core-rs`.

The crate is developed with a TDD-first workflow. The current milestone covers the DOCX rich slice and an approved HWPX baseline backed by manually validated fixtures.

## Features

- Markdown to shared intermediate representation
- DOCX generation with deterministic ZIP packaging
- fixture-based regression testing with normalized hashes
- strict mode by default for unsupported syntax
- minimal public API surface

## Current Support

Supported in the current DOCX implementation:

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

## Installation

This crate is not currently published to `crates.io`.

Use it from this repository workspace or as a path dependency:

```toml
[dependencies]
core-rs = { path = "/path/to/docxly/core-rs/packages/core-rs" }
```

In Rust code, import it with the crate name `core_rs`.

Current public API:

```rust
use std::fs;

use core_rs::{DocxOptions, generate_docx};

let bytes = generate_docx(
    "# Hello\n\nThis is **docxly**.",
    DocxOptions::default(),
)?;

fs::write("output.docx", bytes)?;
```

The crate does not expose parser, IR, generator, or utility modules as stable public API.

Available options:

- `title: Option<String>`
- `author: Option<String>`
- `strict_mode: bool`
- `style: HwpxStyleOptions` on `HwpxOptions` only

Current high-level API behavior:

- `generate_docx(...) -> Result<Vec<u8>, CoreRsError>`
- `generate_hwpx(...) -> Result<Vec<u8>, CoreRsError>`
- `analyze_markdown(...) -> ConversionReport`
- `generate_docx_with_report(...) -> Result<GenerationResult, GenerationFailure>`
- `generate_hwpx_with_report(...) -> Result<GenerationResult, GenerationFailure>`
- `generate_hwpx` targets the approved HWPX golden baseline committed under `tests/fixtures/hwpx/approved`
- approved HWPX fixtures are the release gate; provisional/quarantined artifacts are not
- `HwpxStyleOptions` currently supports document-level body/heading font, body/heading size, text/link/heading color, and paragraph alignment overrides
- Custom HWPX fonts are best-effort only; the current HWPX path records font family names but does not embed font binaries

## HWPX Supported Today

The current HWPX implementation is narrower than the DOCX rich slice.

- approved compatibility fixtures:
  - `core-paragraph`
  - `blockquote-basic`
  - `code-block-basic`
  - `core-heading`
  - `core-inline-style`
  - `core-link-text`
  - `core-mixed`
  - `list-basic`
  - `list-nested-depth-2`
  - `ordered-list-basic`
  - `ordered-list-nested-depth-2`
- approved table fixtures:
  - `table-basic`
  - `table-alignment`
- approved style fixtures:
  - `style-typography`
  - `style-centered-layout`
  - `style-brand-color`
- supported content today:
  - paragraphs
  - headings
  - blockquotes
  - code blocks
  - visible-text emphasis/strong/code/link rendering inside the approved compatibility contract
  - ordered and unordered lists inside the approved list fixtures
  - tables inside the approved table fixtures
  - document-level style overrides through `HwpxStyleOptions`
- ordered-list fixtures are approved compatibility fixtures rather than strict-mode guarantees
- not yet approved for HWPX:
  - images
  - broader table coverage beyond the approved table fixtures

Example:

```rust
use std::fs;

use core_rs::{
    HwpxOptions, HwpxParagraphAlign, HwpxStyleOptions, generate_hwpx,
};

let bytes = generate_hwpx(
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

fs::write("output.hwpx", bytes)?;
```

## WASM / npm Packaging

This crate is configured with `cdylib` output for the npm wrapper build.

- target package: `@docxly/core-rs`
- wrapper location: `../npm-core-rs`
- CI verifies native Rust checks and a `wasm32-unknown-unknown` build before npm publish
- npm version PRs sync this crate's `Cargo.toml` version from the npm package version on `main`
- browser consumers should use the npm wrapper through either:
  - a bundler/runtime that can emit the `.wasm` asset
  - a static host that serves the emitted `.wasm` asset alongside the ESM wrapper

## Testing

From the workspace root:

```bash
cargo test
```

For this crate only:

```bash
cargo test -p core-rs
```

Run lint checks used by the repository:

```bash
cargo clippy -p core-rs --all-targets -- -D warnings
```

DOCX integration tests verify:

- archive unzip success
- required entries exist
- XML files are well-formed
- `golden.docx` normalizes to the committed `expected/` tree
- normalized SHA-256 hash matches the committed `hash.txt`
- generated output normalizes to the same contents as `golden.docx`

Each fixture directory is self-contained:

- `input.md`
- `fixture.toml`
- `golden.docx`
- `expected/`
- `hash.txt`

`golden.docx` is treated as a read-only baseline in the normal development workflow.

## Status

- DOCX rich: implemented
- HWPX approved baseline: implemented
- Only manually approved HWPX fixtures are treated as release-gate goldens; `core-paragraph`, `blockquote-basic`, `code-block-basic`, `core-heading`, `core-inline-style`, `core-link-text`, `core-mixed`, `list-basic`, `list-nested-depth-2`, `ordered-list-basic`, `ordered-list-nested-depth-2`, `table-basic`, `table-alignment`, `style-typography`, `style-centered-layout`, and `style-brand-color` are the current approved baselines
- HWPX `paragraph_align` currently targets body and heading paragraph styles; future paragraph categories may extend that scope
- Public API: high-level generation functions, report types, and option/error types only

For repository-level usage and roadmap details, see the root [`README.md`](../../README.md).
