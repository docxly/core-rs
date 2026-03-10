import { readFile, writeFile } from "node:fs/promises";

import { generateDocx } from "../dist/node.js";

const [, , inputPath, outputPath] = process.argv;

if (!inputPath || !outputPath) {
  console.error("usage: node ./scripts/_benchmark_docxly_once.mjs <input.md> <output.docx>");
  process.exit(1);
}

const markdown = await readFile(inputPath, "utf8");
const bytes = await generateDocx(markdown);
await writeFile(outputPath, bytes);
