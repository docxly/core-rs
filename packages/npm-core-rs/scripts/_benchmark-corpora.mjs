export function buildCorpora() {
  const smallMarkdown = `# Small Benchmark Title

Small benchmark paragraph with **bold**, *italic*, \`inline code\`, and [an example link](https://example.com).

Small benchmark keeps a second paragraph to force additional text runs and paragraph nodes in the DOCX tree.

> Small benchmark quote for DOCX output checks.

1. First benchmark ordered item
2. Second benchmark ordered item

- First benchmark bullet
  - First benchmark nested bullet
- Second benchmark bullet

\`\`\`text
small-benchmark-code
\`\`\`

| Capability | Value | Notes |
| --- | --- | --- |
| Engine | docxly | Small corpus |
| Scope | Small | Baseline |
| Depth | 2 | Nested list enabled |`;

  const mediumSections = Array.from({ length: 12 }, (_, index) =>
    buildBenchmarkSection({
      number: index + 1,
      size: "Medium",
      repeatParagraphs: 3,
      tableRows: 6,
      orderedCount: 3,
      unorderedCount: 3,
    }),
  ).join("\n\n");

  const largeSections = Array.from({ length: 28 }, (_, index) =>
    buildBenchmarkSection({
      number: index + 1,
      size: "Large",
      repeatParagraphs: 5,
      tableRows: 10,
      orderedCount: 4,
      unorderedCount: 4,
    }),
  ).join("\n\n");

  return [
    {
      key: "small",
      label: "Small",
      markdown: smallMarkdown,
      tokens: [
        "Small Benchmark Title",
        "Small benchmark paragraph",
        "First benchmark ordered item",
        "small-benchmark-code",
      ],
    },
    {
      key: "medium",
      label: "Medium",
      markdown: `# Medium Benchmark Title\n\n${buildCorpusLead("Medium", 12)}\n\n${mediumSections}\n\n${buildCorpusTail("Medium", 12)}`,
      tokens: [
        "Medium Benchmark Title",
        "Medium Section 1",
        "Medium nested bullet 3-2",
        "Medium code block 12",
      ],
    },
    {
      key: "large",
      label: "Large",
      markdown: `# Large Benchmark Title\n\n${buildCorpusLead("Large", 28)}\n\n${largeSections}\n\n${buildCorpusTail("Large", 28)}`,
      tokens: [
        "Large Benchmark Title",
        "Large Section 1",
        "Large ordered item 5-1",
        "Large nested bullet 28-3",
      ],
    },
  ];
}

function buildBenchmarkSection({
  number,
  size,
  repeatParagraphs,
  tableRows: tableRowCount,
  orderedCount,
  unorderedCount,
}) {
  const paragraphs = Array.from({ length: repeatParagraphs }, (_, index) => {
    const paragraphNumber = index + 1;
    return `${size} paragraph ${number}-${paragraphNumber} combines **bold**, *italic*, \`code-${number}-${paragraphNumber}\`, and [links](https://example.com/${size.toLowerCase()}/${number}/${paragraphNumber}) for DOCX run generation.`;
  }).join("\n\n");

  const orderedList = Array.from({ length: orderedCount }, (_, index) => {
    const itemNumber = index + 1;
    return `${itemNumber}. ${size} ordered item ${number}-${itemNumber}`;
  }).join("\n");

  const unorderedList = Array.from({ length: unorderedCount }, (_, index) => {
    const itemNumber = index + 1;
    return `- ${size} unordered item ${number}-${itemNumber}\n  - ${size} nested bullet ${number}-${itemNumber}`;
  }).join("\n");

  const tableRows = Array.from({ length: tableRowCount }, (_, index) => {
    const rowNumber = index + 1;
    return `| ${number}.${rowNumber} | ${size} table value ${number}-${rowNumber} | ${size} table note ${number}-${rowNumber} |`;
  }).join("\n");

  return `## ${size} Section ${number}

${paragraphs}

> ${size} quote ${number} keeps blockquote output active for the benchmark corpus.

${orderedList}

${unorderedList}

\`\`\`json
{"section": ${number}, "kind": "${size.toLowerCase()}", "marker": "${size} code block ${number}"}
\`\`\`

| Metric | Value | Notes |
| --- | --- | --- |
${tableRows}`;
}

function buildCorpusLead(size, sectionCount) {
  return `${size} benchmark lead paragraph with **bold**, *italic*, \`lead-code\`, and [overview link](https://example.com/${size.toLowerCase()}/overview).

${size} benchmark includes ${sectionCount} sections to exercise repeated block generation, list rendering, table packaging, and code block serialization.`;
}

function buildCorpusTail(size, sectionCount) {
  return `## ${size} Closing Notes

This closing section confirms that the ${size.toLowerCase()} benchmark rendered ${sectionCount} content sections and preserves a final paragraph for end-of-document verification.`;
}
