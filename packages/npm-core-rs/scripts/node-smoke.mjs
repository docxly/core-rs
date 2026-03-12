import assert from "node:assert/strict";

import {
  analyzeMarkdown,
  generateDocx,
  generateDocxWithReport,
  generateHwpx,
  generateHwpxWithReport,
} from "../dist/node.js";
import { assertDocxArchive, SMOKE_MARKDOWN } from "./_smoke-common.mjs";

const bytes = await generateDocx(SMOKE_MARKDOWN);
assertDocxArchive(bytes);

const report = await analyzeMarkdown("<b>raw</b>", "docx");
assert.equal(report.fallbackCount, 1);

const docxWithReport = await generateDocxWithReport(SMOKE_MARKDOWN);
assertDocxArchive(docxWithReport.bytes);
assert.equal(docxWithReport.report.unsupportedCount, 0);

const hwpxWithReport = await generateHwpxWithReport("1. alpha\n2. beta", {
  strictMode: false,
});
assert.equal(hwpxWithReport.report.issues[0].feature, "HWPX ordered list in strict mode");

const styledHwpx = await generateHwpxWithReport("# Heading\n\n본문", {
  style: {
    bodyFont: "함초롬바탕",
    headingFont: "함초롬돋움",
    textColor: "#222222",
    headingColor: "#0F4C81",
    linkColor: "#0F4C81",
    paragraphAlign: "center",
  },
});
assert.ok(styledHwpx.bytes.length > 0);

await assert.rejects(
  () => generateHwpxWithReport("1. alpha\n2. beta"),
  (error) => {
    assert.ok(error instanceof Error);
    assert.equal(error.name, "GenerationFailure");
    assert.equal(error.error, error.message);
    assert.equal(error.report.unsupportedCount, 1);
    assert.equal(error.report.issues[0].feature, "HWPX ordered list in strict mode");
    assert.match(error.report.issues[0].message, /ordered list/u);
    return true;
  },
);

await assert.rejects(
  () => generateHwpx("1. alpha\n2. beta"),
  (error) => typeof error === "string" && error.includes("ordered list"),
);
