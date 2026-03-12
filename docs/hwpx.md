# HWPX Guide

This page captures the current HWPX contract for `docxly`.

## Surface Status

| Surface | Status | Notes |
| --- | --- | --- |
| Rust `generate_hwpx` | Beta | Supports the approved HWPX baseline, including the Rust style contract |
| npm `generateHwpx` | Beta | Uses the same core generation path, but with a narrower public option surface |
| Internal HWPX parser | Private | Used for approved-fixture reverse parsing and repository debugging only |

## Approved Baseline

Approved compatibility baseline:

- `core-paragraph`
- `blockquote-basic`
- `code-block-basic`
- `core-heading`
- `core-inline-style`
- `core-link-text`
- `core-mixed`

Approved list baseline:

- `list-basic`
- `list-nested-depth-2`
- `ordered-list-basic`
- `ordered-list-nested-depth-2`

Approved table baseline:

- `table-basic`
- `table-alignment`

Approved style baseline:

- `style-typography`
- `style-centered-layout`
- `style-brand-color`

## Supported Today

Rust HWPX currently supports:

- paragraphs
- headings
- blockquotes
- code blocks
- visible-text emphasis, strong, code, and links inside the approved compatibility contract
- ordered and unordered lists inside the approved list fixtures
- tables inside the approved table fixtures
- document-level HWPX style overrides through `HwpxStyleOptions`

The current internal HWPX parser is intentionally narrower than a general-purpose HWPX importer. It is designed to reverse-parse the approved `docxly` contract only.

Current parser assumptions:

- single-section packages only
- quote depth up to 2
- list depth up to 2
- tables only inside the approved table fixtures
- table cells with a single paragraph only
- no images, equations, charts, notes, or broader control/layout coverage

npm HWPX currently exposes:

- `title`
- `author`
- `strictMode`

The npm package does not yet expose the Rust HWPX style override surface.

## Not Yet Part of the Approved Contract

- images
- broader table coverage beyond the approved table fixtures
- wider layout and block coverage from provisional fixtures
- public npm style overrides matching `HwpxStyleOptions`

## Repository Debugging Path

Repository contributors can inspect generated or approved HWPX files with the internal semantic dump:

```bash
cargo run -p core-rs --bin decode_hwpx -- --parse path/to/sample.hwpx
```

Useful variants:

- `cargo run -p core-rs --bin decode_hwpx -- sample.hwpx`
  Raw archive summary and text-entry dump to stdout
- `cargo run -p core-rs --bin decode_hwpx -- --parse sample.hwpx`
  Archive summary plus the internal semantic interpretation
- `cargo run -p core-rs --bin decode_hwpx -- sample.hwpx --out /tmp/sample`
  Extracts the raw package for side-by-side inspection

This path is repository tooling only. It is not a stable Rust or npm API contract.

## Validation

Current HWPX validation in the repository covers both generation and reverse parsing:

- approved fixture generation tests
- approved fixture reverse-parse tests
- malformed archive and XML rejection tests
- unsupported-feature rejection tests
- generated-HWPX round-trip smoke tests for the approved fixture set

## npm Example

```js
import { generateHwpx } from "@docxly/core-rs";

const bytes = await generateHwpx("# Title\n\n본문 **강조**");
```

## Rust Example

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

## Reference Material

- `packages/core-rs/src/generators/hwpx/docs/README.md`
- `packages/core-rs/src/generators/hwpx/docs/schema-md/index.md`
- `packages/core-rs/src/generators/hwpx/reference/paragraph-only`

These files form the repository-level HWPX reference corpus used for implementation and contract review.
