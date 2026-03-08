# HWPX Reference Notes

This directory contains implementation notes and external reference conversions used for the HWPX generator.
The current status is compatibility bring-up, not finished feature parity.

## How to use this directory

- `schema-md/`: Markdown conversions of official Hancom PDF references.
- `HWP_DocInfo_Structure.md`, `HWP_CharShape_Structure.md`, `HWP_Document_Data_Records.md`: curated notes that map binary HWP structures to XML/HWPX-oriented implementation decisions.
- `IMPROVEMENTS_SUMMARY.md`, `Bold_Text_Implementation.md`, `Line_Wrapping_Fix.md`: implementation history and experiments from earlier HWP/HWPML work.

## Recommended reading order for HWPX work

1. [`schema-md/index.md`](./schema-md/index.md)
2. [`schema-md/02-hwpml-3.0-xml-structure.md`](./schema-md/02-hwpml-3.0-xml-structure.md)
3. [`HWP_DocInfo_Structure.md`](./HWP_DocInfo_Structure.md)
4. [`HWP_CharShape_Structure.md`](./HWP_CharShape_Structure.md)
5. [`HWP_Document_Data_Records.md`](./HWP_Document_Data_Records.md)

## Scope note

The free Hancom PDFs converted under `schema-md/` are useful for HWP/HWPML semantics, but they do not fully describe modern HWPX package files such as `content.hpf`, `header.xml`, and `section0.xml`. For those package-level details, use:

- the KS X 6101 OWPML/HWPX standard source links recorded in [`schema-md/index.md`](./schema-md/index.md)
- the local implementation notes in this directory
- the synthetic compatibility corpus under `../reference/`
- manually approved `.hwpx` fixtures under `packages/core-rs/tests/fixtures/hwpx/approved`

## Compatibility bring-up note

- `approved/` fixtures are the only CI/release gate for HWPX. `core-paragraph`, `core-heading`, `core-inline-style`, `core-link-text`, `core-mixed`, `style-typography`, `style-centered-layout`, and `style-brand-color` are currently approved.
- `provisional/` fixtures are compatibility snapshots and are not treated as approved golden files.
- `quarantine/` stores stale or known-broken generated artifacts kept only for reverse-engineering.
- During `Phase A`, heading and inline styling are intentionally flattened to visible text until Hancom-open compatibility is proven.
- Approved style fixtures use Hancom-safe built-in fonts only. External fonts remain best-effort because the current HWPX path does not embed font binaries.
- `paragraph_align` currently applies to body and heading paragraph styles only.
