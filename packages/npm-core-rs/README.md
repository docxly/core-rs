# @docxly/core-rs

Language: [English](./README.md) · [한국어 문서](../../docs/ko/README.md)

`@docxly/core-rs` is the npm package for the `docxly` Rust core, an embeddable document generation engine for app integration. Where Pandoc is a general-purpose converter, this package is designed to expose DOCX generation directly inside Node and browser-based runtimes, with current benchmark details maintained in the shared comparison dataset and README comparison block.

## Status

- DOCX generation: supported
- HWPX generation: experimental HWPX API
- report APIs: recommended for user-generated Markdown
- API style: async only

## Live Demo

- https://docxly.github.io/core-rs/

## Install

```bash
npm install @docxly/core-rs
```

## Release Workflow

- Contributors add a changeset from the repository root with `npm run changeset:add`
- After that changeset merges into `main`, the `Version Packages` workflow opens or updates a PR that bumps this package version
- The same PR also syncs `packages/core-rs/Cargo.toml` and refreshes this package's `package-lock.json`
- The existing `Release` workflow still publishes npm when a matching `v*.*.*` tag is pushed

## Usage

### Node

```js
import { writeFile } from "node:fs/promises";
import { generateDocx, generateHwpxWithReport } from "@docxly/core-rs";

const bytes = await generateDocx("# Hello\n\nThis is **docxly**.");
await writeFile("output.docx", bytes);

const hwpx = await generateHwpxWithReport("1. alpha\n2. beta", {
  strictMode: false,
});
console.log(hwpx.report.issues);
```

### Browser

Preferred path: use a bundler/runtime that can resolve package exports and emit the `.wasm` asset.
The package is not meant to be imported directly from a raw `<script type="module">` page from a package CDN without a build step.

Static hosting is also possible when you serve the emitted ESM wrapper and `.wasm` asset together, as this repository's Pages demo does.

Example with a bundler-managed ESM app:

```js
import { analyzeMarkdown, generateDocx } from "@docxly/core-rs";

const bytes = await generateDocx("# Hello\n\nThis is **docxly**.");
const blob = new Blob([bytes], {
  type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
});

const report = await analyzeMarkdown("<b>raw</b>", "docx");
console.log(report.fallbackCount);
```

Requirements for browser usage:

- ESM-aware bundler/runtime or static host
- `.wasm` asset emission or static serving support
- modern browser environment

For static hosting demos, browser downloads may use a `blob:` URL under the hood. The saved
filename can still be correct even if browser download history shows a UUID-like source entry.

## Local Demo and Pages

The browser demo lives in this repository, is built as a static artifact for GitHub Pages, and currently exercises both DOCX and experimental HWPX generation. It is a repository demo artifact, not the recommended package-consumer integration path.

Live Pages URL:

- https://docxly.github.io/core-rs/

From the repository root, use:

```bash
npm install
npm run demo
```

To build the Pages artifact only:

```bash
npm run build:pages
```

## API

```ts
export interface DocxOptions {
  title?: string;
  author?: string;
  strictMode?: boolean;
}

export function generateDocx(markdown: string, options?: DocxOptions): Promise<Uint8Array>;
```

```ts
export type ConversionTarget = "docx" | "hwpx";

export interface ConversionIssue {
  feature: string;
  message: string;
  severity: "Warning" | "Error";
  degraded: boolean;
}

export interface ConversionReport {
  issues: ConversionIssue[];
  degraded: boolean;
  unsupportedCount: number;
  fallbackCount: number;
}

export interface GenerationResult {
  bytes: Uint8Array;
  report: ConversionReport;
}

export function analyzeMarkdown(
  markdown: string,
  target: ConversionTarget,
): Promise<ConversionReport>;

export function generateDocxWithReport(
  markdown: string,
  options?: DocxOptions,
): Promise<GenerationResult>;

export function generateHwpx(markdown: string, options?: HwpxOptions): Promise<Uint8Array>;
export function generateHwpxWithReport(
  markdown: string,
  options?: HwpxOptions,
): Promise<GenerationResult>;
```

## Capability Matrix

| Capability | DOCX | HWPX strict | HWPX compat |
| --- | --- | --- | --- |
| Headings and paragraphs | Yes | Yes | Yes |
| Inline emphasis, strong, code, links | Yes | Yes | Yes |
| Ordered lists up to depth 2 | Yes | No | Yes, semantic contract |
| Unordered lists up to depth 2 | Yes | Yes | Yes |
| Tables | Yes | Yes | Yes |
| `data:` URI images | Yes | No | Degraded fallback to alt text |
| Unsupported HTML | Strict: error, compat: literal text fallback | Error | Degraded literal text fallback |
| Footnotes, task lists, math | Strict: error, compat: visible text fallback | Error | Degraded visible text fallback |
| Deep nested lists | Strict: error, compat: plain text fallback | Error | Degraded plain text fallback |

`generateDocx` and `generateHwpx` are the simple byte-oriented APIs. For user-generated Markdown,
prefer `analyzeMarkdown`, `generateDocxWithReport`, or `generateHwpxWithReport` so degraded
fallbacks are visible in application code.
