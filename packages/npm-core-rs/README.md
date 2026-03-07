# @docxly/core-rs

`@docxly/core-rs` is the npm package for the `docxly` Rust core. It exposes DOCX generation through a WASM wrapper for Node and browser-based runtimes.

## Status

- DOCX generation: supported
- HWPX generation: not exposed in npm v0.x
- API style: async only

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

```js
import { generateDocx } from "@docxly/core-rs";

const bytes = await generateDocx("# Hello\n\nThis is **docxly**.");
const blob = new Blob([bytes], {
  type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
});
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
