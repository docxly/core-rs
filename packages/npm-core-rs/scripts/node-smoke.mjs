import { generateDocx } from "../dist/node.js";
import { assertDocxArchive, SMOKE_MARKDOWN } from "./_smoke-common.mjs";

const bytes = await generateDocx(SMOKE_MARKDOWN);
assertDocxArchive(bytes);
