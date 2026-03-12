# Roadmap

This roadmap is directional. It shows what the project is optimizing for, not a release-date promise.

## Current Focus

`docxly` is prioritizing adoption as an embeddable document-generation engine for applications.

That means:

- excellent DOCX experience in Node and browser flows
- clear contracts around HWPX maturity
- deterministic outputs and reliable regression gates
- small, defensible public APIs

## Near-Term Priorities

1. Expand product-facing examples.
   Add examples that match real integration scenarios such as service-side export, browser download, and Korean document workflows.
2. Clarify HWPX maturity.
   Continue moving proven fixtures from provisional to approved and document what each promotion means for users, including broader table coverage and future image support.
3. Improve contributor onboarding.
   Keep contribution and issue flows lightweight, but explicit enough that external contributors can ship focused changes without guessing project norms.
4. Refine packaging strategy.
   Decide whether Rust crate publication should become part of the formal release process or remain workspace-first until demand is clearer.
5. Learn from real adopters.
   Prioritize fixes and API changes based on actual integration friction, not hypothetical completeness.

## Not the Immediate Goal

- becoming a general-purpose format conversion suite
- mirroring all of Pandoc's output and template workflows
- exposing internal modules as a broad extension surface
- claiming full HWPX parity before fixtures and validation support it

## Decision Heuristics

Changes are more likely to land when they:

- improve adoption for application teams
- preserve deterministic output guarantees
- add tests or fixtures for new behavior
- keep the public contract narrow and clear

Changes are less likely to land when they:

- widen API surface without clear user demand
- add partially validated format behavior
- optimize for niche abstractions over shipping reliability
