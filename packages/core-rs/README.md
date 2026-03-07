# core-rs

`core-rs` is the Rust engine for `docxly`. It turns Markdown into deterministic DOCX and, later, HWPX archives. This crate is also compiled to WASM for the npm package in `../npm-core-rs`.

The crate is developed with a TDD-first workflow. The current milestone covers the DOCX rich slice.

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

Add the crate to `Cargo.toml`:

```toml
[dependencies]
core-rs = "0.1.0"
```

Use it in Rust code with the crate name `core_rs`.

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

Current high-level API behavior:

- `generate_docx(...) -> Result<Vec<u8>, CoreRsError>`
- `generate_hwpx(...) -> Result<Vec<u8>, CoreRsError>`
- `generate_hwpx` currently returns `UnsupportedFeature` until HWPX generation is implemented

## WASM / npm Packaging

This crate is configured with `cdylib` output for the npm wrapper build.

- target package: `@docxly/core-rs`
- wrapper location: `../npm-core-rs`
- CI verifies native Rust checks and a `wasm32-unknown-unknown` build before npm publish

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
- HWPX generation: planned, not implemented yet
- Public API: high-level generation functions and option/error types only

For repository-level usage and roadmap details, see the root [`README.md`](../../README.md).
