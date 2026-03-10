# @docxly/core-rs

Language: [English](/Users/limchaesung/Github/docxly/core-rs/packages/npm-core-rs/README.md) · [한국어 문서](/Users/limchaesung/Github/docxly/core-rs/docs/ko/README.md)

`@docxly/core-rs` is the npm package for the `docxly` Rust core, an embeddable document generation engine for app integration. Where Pandoc is a general-purpose converter, this package is designed to expose DOCX generation directly inside Node and browser-based runtimes, with the current offline Node benchmark showing an 80 ms cold start and a 2 ms steady median for docxly versus 284 ms cold and 210 ms steady for Pandoc on the summary corpus.

## Status

- DOCX generation: supported
- HWPX generation: not exposed in npm v0.x
- API style: async only

## Live Demo

- https://docxly.github.io/core-rs/

## Install

```bash
npm install @docxly/core-rs
```

## Usage

### Node

```js
import { writeFile } from "node:fs/promises";
import { generateDocx } from "@docxly/core-rs";

const bytes = await generateDocx("# Hello\n\nThis is **docxly**.");
await writeFile("output.docx", bytes);
```

### Browser

Use a bundler/runtime that can resolve package exports and emit the `.wasm` asset. The package is
not meant to be imported directly from a raw `<script type="module">` page without a build step.

Example with a bundler-managed ESM app:

```js
import { generateDocx } from "@docxly/core-rs";

const bytes = await generateDocx("# Hello\n\nThis is **docxly**.");
const blob = new Blob([bytes], {
  type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
});
```

Requirements for browser usage:

- ESM-aware bundler/runtime
- `.wasm` asset emission support
- modern browser environment

For static hosting demos, browser downloads may use a `blob:` URL under the hood. The saved
filename can still be correct even if browser download history shows a UUID-like source entry.

## Local Demo and Pages

The browser demo lives in this repository and is built as a static artifact for GitHub Pages.

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
