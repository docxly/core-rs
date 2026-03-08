# Approved HWPX Fixtures

Only fixtures placed here are treated as CI/release-gate golden files.

Current status:

- `core-paragraph` is manually validated and approved.
- `core-heading` is manually validated and approved.
- `core-inline-style` is manually validated and approved.
- `core-link-text` is manually validated and approved.
- `core-mixed` is manually validated and approved.
- `style-typography` is manually validated and approved.
- `style-centered-layout` is manually validated and approved.
- `style-brand-color` is manually validated and approved.

Approved style fixtures use Hancom-safe built-in fonts only. External fonts remain best-effort
because the current HWPX path records font family names but does not embed font binaries.

Approved style fixtures serve as both package-compatibility and manually checked visual baselines.
If that dual role becomes noisy, split them into separate compatibility and visual groups.
