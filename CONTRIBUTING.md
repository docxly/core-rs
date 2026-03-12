# Contributing to docxly

Thanks for contributing to `docxly`.

This repository is optimized for deterministic document generation and tight regression control. The goal of the contribution process is to keep the public contract clear while making it easy to ship focused improvements.

## Before You Start

- Read the root [README.md](./README.md) for current positioning, support boundaries, and roadmap.
- Prefer small pull requests with one clear purpose.
- Open an issue before large features, public API changes, or roadmap shifts.
- Treat DOCX and HWPX output contracts as user-facing behavior, not incidental implementation details.

## Good First Contributions

- fix incorrect docs or examples
- add product-oriented usage examples
- improve tests around supported Markdown behavior
- tighten error messages or strict-mode behavior
- promote an HWPX fixture from provisional to approved with clear validation evidence

## Development Setup

From the repository root:

```bash
npm install
```

Common commands:

```bash
npm run lint:rust
npm run test:rust
npm run test:web
npm run test:all
npm run demo
```

## Core Test Tips

- Understand the golden flow first. The primary comparison is between the committed hash/golden fixture set and the generator output, not between two abstract interpretations of the spec.
- Keep the full test suite green at all times. A change that needs explanation but leaves unrelated failures behind is not ready.
- Use the `open` command aggressively when reviewing generated documents, fixture artifacts, and rendered outputs. Visual confirmation is part of the workflow here.
- Trust your eyes over raw XML when the rendered result and the XML story disagree. The user-visible document is the contract that matters first.
- Trust your eyes over the official format documents when the spec text conflicts with validated real-world output behavior. This project values observed compatibility, not spec literalism by itself.

## Change Guidelines

- Keep the public API small unless there is strong adoption evidence for widening it.
- Add tests with behavior changes. For document output changes, update or add fixture coverage rather than relying on ad hoc assertions.
- Do not treat provisional HWPX fixtures as release-gate guarantees.
- Keep README and docs aligned when changing support boundaries or runtime expectations.
- Prefer additive, low-coupling changes over broad refactors.

## Pull Request Expectations

Each pull request should describe:

- the user problem being solved
- the public behavior change
- the verification steps you ran
- any compatibility or contract risks

If the change affects generated output, include which fixtures or smoke tests prove the new behavior.

## Issue Triage

When filing issues, include:

- runtime target: Node, browser, or Rust
- sample Markdown input
- expected document behavior
- actual result
- version and environment details

## Review Standard

Reviews prioritize:

- behavioral regressions
- contract clarity
- deterministic output guarantees
- test coverage for supported paths
- documentation accuracy

## Release Notes

The npm package is published from tagged releases. If your change affects release behavior, packaging, or version boundaries, call that out explicitly in the pull request.
