import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";

import {
  assertDemoWiring,
  loadComparisonData,
  readmePath,
  renderReadmeBlock,
  replaceReadmeBlock,
} from "./_comparison-common.mjs";

const data = await loadComparisonData();
const readme = await readFile(readmePath, "utf8");
const expected = replaceReadmeBlock(readme, renderReadmeBlock(data));

assert.equal(readme, expected, "README comparison block is out of sync with comparison-data.json");
await assertDemoWiring();

console.log("comparison data, README markers, and demo wiring are in sync");
