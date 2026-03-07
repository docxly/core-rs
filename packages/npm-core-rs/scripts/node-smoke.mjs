import assert from "node:assert/strict";

import { generateDocx } from "../dist/node.js";

const bytes = await generateDocx("# Hello\n\nThis is **docxly**.");

assert.ok(bytes instanceof Uint8Array, "generateDocx must return Uint8Array");
assert.ok(bytes.length > 0, "generateDocx must return non-empty bytes");
