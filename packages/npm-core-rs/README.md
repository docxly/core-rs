# @docxly/core-rs

`@docxly/core-rs` is the npm package for the `docxly` Rust core. It exposes DOCX generation through a WASM wrapper for Node and browser-based runtimes.

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
