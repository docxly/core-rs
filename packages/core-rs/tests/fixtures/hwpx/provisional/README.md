# Provisional HWPX Fixtures

This directory stores compatibility snapshots that are useful for reverse-engineering
and local comparison, but are not approved golden files.

Rules:

- Do not use fixtures here as CI or release gates.
- Promote a fixture out of `provisional/` only after Hancom opens it without a
  repair warning.
- Keep fixture names stable so manual comparison remains traceable across bring-up
  iterations.

Current status:

- `image-data-uri-basic`
- `mixed-rich`
- `table-alignment`
- `thematic-break-basic`
