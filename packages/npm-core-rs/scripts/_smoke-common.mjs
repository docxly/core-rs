import assert from "node:assert/strict";

import { unzipSync } from "fflate";

export const SMOKE_MARKDOWN = "# Hello\n\nThis is **docxly**.";

export function assertDocxArchive(bytes) {
  assert.ok(bytes instanceof Uint8Array, "generateDocx must return Uint8Array");

  const archive = unzipSync(bytes);
  const entries = Object.keys(archive);

  assert.ok(entries.includes("[Content_Types].xml"), "archive must include [Content_Types].xml");
  assert.ok(entries.includes("_rels/.rels"), "archive must include _rels/.rels");
  assert.ok(entries.includes("word/document.xml"), "archive must include word/document.xml");

  return archive;
}
