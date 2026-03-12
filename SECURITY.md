# Security Policy

## Supported Surfaces

Security reports are most actionable when they involve currently documented public entrypoints:

- npm package `@docxly/core-rs`
- Rust high-level APIs in `packages/core-rs`
- release artifacts and package distribution flow

Internal modules and experimental HWPX areas may change quickly, but security issues affecting them should still be reported.

## Reporting a Vulnerability

Do not open a public issue for a suspected vulnerability.

Use GitHub's private vulnerability reporting flow from the repository Security tab when available. Include:

- a short summary
- affected surface and version
- reproduction steps or proof of concept
- severity assessment if known
- any suggested mitigation

If private reporting is unavailable in your environment, contact a maintainer directly through GitHub and avoid posting exploit details publicly.

## Scope Notes

- Document rendering mismatches without security impact should go to normal issues.
- Availability issues from intentionally unsupported input should be reported as bugs, unless they create a real denial-of-service vector.
- Supply-chain or release-process concerns are in scope when they affect published artifacts.
