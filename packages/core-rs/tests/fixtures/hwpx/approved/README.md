# Approved HWPX Fixtures

Only fixtures placed here are treated as CI/release-gate golden files.

Current status:

- `core-paragraph` is manually validated and approved.
- `blockquote-basic` is manually validated and approved.
- `code-block-basic` is manually validated and approved.
- `core-heading` is manually validated and approved.
- `core-inline-style` is manually validated and approved.
- `core-link-text` is manually validated and approved.
- `core-mixed` is manually validated and approved.
- `list-basic` is manually validated and approved.
- `list-nested-depth-2` is manually validated and approved.
- `ordered-list-basic` is manually validated and approved for compat mode.
- `ordered-list-nested-depth-2` is manually validated and approved for compat mode.
- `table-basic` is manually validated and approved.
- `table-alignment` is manually validated and approved.
- `style-typography` is manually validated and approved.
- `style-centered-layout` is manually validated and approved.
- `style-brand-color` is manually validated and approved.

Each approved fixture must record `manual_verified = true` in `fixture.toml`. Ordered-list fixtures are
approved compatibility fixtures, but they remain compat-only rather than strict-mode guarantees.

Approved style fixtures use Hancom-safe built-in fonts only. External fonts remain best-effort
because the current HWPX path records font family names but does not embed font binaries.

Approved style fixtures serve as both package-compatibility and manually checked visual baselines.
If that dual role becomes noisy, split them into separate compatibility and visual groups.
