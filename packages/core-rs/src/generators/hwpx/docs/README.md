# HWPX Reference Notes

This directory contains implementation notes and local reference material used for the HWPX generator.
The current status is an approved HWPX baseline plus a smaller provisional area for future extensions.

## How to use this directory

- `HWP_DocInfo_Structure.md`, `HWP_CharShape_Structure.md`, `HWP_Document_Data_Records.md`: curated notes that map binary HWP structures to XML/HWPX-oriented implementation decisions.
- `IMPROVEMENTS_SUMMARY.md`, `Bold_Text_Implementation.md`, `Line_Wrapping_Fix.md`: implementation history and experiments from earlier HWP/HWPML work.
- `../reference/`: unzip trees and provenance for local synthetic Hancom-generated samples used as package-level compatibility references.

## Recommended reading order for HWPX work

1. `packages/core-rs/tests/fixtures/hwpx/approved/*/expected`
2. [`../reference/paragraph-only`](../reference/paragraph-only)
3. [`schema-md/index.md`](./schema-md/index.md)
4. [`HWP_DocInfo_Structure.md`](./HWP_DocInfo_Structure.md)
5. [`HWP_CharShape_Structure.md`](./HWP_CharShape_Structure.md)
6. [`HWP_Document_Data_Records.md`](./HWP_Document_Data_Records.md)

## Scope note

This repository includes a checked-in `schema-md/` corpus for spec reference, but package-level HWPX files such as `content.hpf`, `header.xml`, and `section0.xml` still need to be validated against local approved fixtures and reference unzip trees. For those package-level details, use:

- the manually approved `.hwpx` fixtures under `packages/core-rs/tests/fixtures/hwpx/approved`
- the synthetic compatibility corpus under `../reference/`
- the local implementation notes in this directory

## Inspecting and comparing HWPX

Use the local Rust binaries when you need to inspect a generated `.hwpx` or compare it against an approved golden/reference sample.

### Decode a single HWPX

```bash
cargo run -p core-rs --bin decode_hwpx -- <path-to.hwpx>
```

- Without `--out`, the command prints text-like entries (`.xml`, `.hpf`, `.rdf`, `.txt`, `mimetype`) to the console.
- With `--out`, it extracts the archive and normalizes XML line endings:

```bash
cargo run -p core-rs --bin decode_hwpx -- <path-to.hwpx> --out /tmp/hwpx-decoded
```

### Compare generated HWPX against golden/reference XML

```bash
cargo run -p core-rs --bin compare_hwpx_xml -- <generated.hwpx> <golden.hwpx>
```

The comparer:
- reads both HWPX archives as ZIP packages
- normalizes XML/HWPX text entries using the same rule as the fixture tests
- compares entry order, entry set, per-entry normalized contents, and overall normalized hash

Use it when:
- a generated HWPX opens differently from an approved golden
- you want to verify that a manual Hancom sample and a generated fixture are structurally identical
- you need a quick package-level diff before touching the generator

### Refresh fixture metadata from approved golden HWPX

```bash
cargo run -p core-rs --bin refresh_hwpx_fixture_metadata
```

- Rebuilds each approved fixture's `expected/` tree and `hash.txt` from its checked-in `golden.hwpx`.
- Keeps each fixture's existing binary representation choice: raw binaries stay raw, hashed sidecars stay `*.sha256`.
- Pass one or more fixture names to refresh only a subset:

```bash
cargo run -p core-rs --bin refresh_hwpx_fixture_metadata -- core-paragraph table-basic
```

## Approved baseline note

- `approved/` fixtures are the only CI/release gate for HWPX. `core-paragraph`, `blockquote-basic`, `code-block-basic`, `core-heading`, `core-inline-style`, `core-link-text`, `core-mixed`, `list-basic`, `list-nested-depth-2`, `ordered-list-basic`, `ordered-list-nested-depth-2`, `table-basic`, `table-alignment`, `style-typography`, `style-centered-layout`, and `style-brand-color` are currently approved.
- `provisional/` fixtures are not part of the release gate and currently contain unapproved candidates such as images, thematic breaks, alignment-rich tables, and mixed rich documents.
- `quarantine/` stores stale or known-broken generated artifacts kept only for reverse-engineering.
- Approved style fixtures use Hancom-safe built-in fonts only. External fonts remain best-effort because the current HWPX path does not embed font binaries.
- `paragraph_align` currently applies to body and heading paragraph styles only.
- The current HWPX style contract is derived from the approved fixture set and the local reference unzip tree, not from an official checked-in schema corpus.
