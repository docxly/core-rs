# Support Matrix

This page explains what `docxly` treats as part of its current public contract.

## Runtime and API Status

| Surface | Status | Notes |
| --- | --- | --- |
| `@docxly/core-rs` DOCX API | Stable in `0.x` | Primary adoption path for Node and browser users |
| `@docxly/core-rs` HWPX API | Beta | Published in the package, but narrower than DOCX and limited to the approved HWPX content baseline with default style options |
| `core-rs::generate_docx` | Stable in repository scope | High-level Rust API only |
| `core-rs::generate_hwpx` | Beta | Supported for the approved HWPX baseline, including the Rust style contract |
| Internal parser / model / generator modules | Private | Not a stable contract |

## Format Support

| Format | Surface | Status | Notes |
| --- | --- | --- | --- |
| DOCX | npm + Rust | Stable | Main production path |
| HWPX | Rust | Beta | Approved compatibility, list, table, and style baselines |
| HWPX | npm | Beta | Available through `generateHwpx`, but with a narrower option surface and no public style overrides |

## Markdown Support Level

DOCX currently supports:

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

Rust HWPX currently supports the approved baseline across:

- paragraphs
- headings
- visible-text inline emphasis, strong, code, and links inside the approved contract
- approved list fixtures
- approved table fixtures
- approved style fixtures

npm HWPX currently exposes the same core generation path, but its public options are limited to:

- `title`
- `author`
- `strictMode`

HWPX items that are still outside the approved baseline or public npm option surface:

- images
- broader table support beyond approved fixtures
- wider layout and block coverage from provisional fixtures
- public npm style overrides matching the Rust `HwpxStyleOptions` surface

## Stability Rules

- DOCX is the default production recommendation.
- HWPX in both Rust and npm should be treated as beta and trusted only for behavior proven by approved fixtures for that specific surface.
- Provisional and quarantined HWPX fixtures are useful development signals, not release promises.
- Internal crate modules may change without a deprecation window.

## Support Expectations

Use an issue when:

- documented behavior does not match actual behavior
- a stable or beta public contract regresses
- smoke tests or fixture gates miss a real-world case that should become part of support

Use a discussion or feature request when:

- you need a new output capability
- you want a new public API
- you need broader npm HWPX coverage or `crates.io` publication
