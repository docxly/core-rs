import { generateDocx, generateHwpx } from "./browser-client.js";

const locales = {
  en: {
    defaultTitle: "Docxly Adoption Report",
    defaultFormat: "docx",
    sampleMarkdown: `# Why Teams Should Install docxly

docxly is an **embeddable document generation engine** for teams that need DOCX or HWPX output inside a product, not as a separate conversion step.

This report explains why installation is justified when product teams need faster document generation, browser-local workflows, and one shared Rust core across runtimes.

## Executive Summary

> Install docxly when document generation must live inside your application, your browser workflow, or your delivery pipeline without depending on an external conversion service.

- **105x faster than Pandoc** on the current summary DOCX benchmark
- Browser-local generation with the same Rust core used in Node
- HWPX and DOCX supported from the same product surface
- Open-source package with [public repository](https://github.com/docxly/core-rs)

## Why Installation Pays Off

### 1. Product teams need an embeddable engine

If your team generates proposals, reports, exports, or customer-facing files inside an application, the document engine should be part of the product stack.

- No backend conversion dependency
- No separate CLI orchestration for the main flow
- One package to integrate into Node and browser contexts

### 2. Speed changes user experience

The current benchmark headline is simple: docxly measured **105x faster** than Pandoc on the summary DOCX benchmark.

- Faster steady-state generation
- Better fit for interactive product workflows
- Lower friction for install justification

### 3. One core supports multiple output paths

docxly keeps the same core architecture across document workflows.

- DOCX generation for broad office compatibility
- HWPX generation for Korean document workflows
- Shared Markdown-to-document model across runtimes

## Recommended Installation Decision

\`\`\`bash
npm install @docxly/core-rs
\`\`\`

Install docxly if your team wants document generation to be a feature of the product instead of a separate conversion stage.`,
    formatLabels: {
      hwpx: "HWPX",
      docx: "DOCX",
    },
    defaultFilenames: {
      hwpx: "docxly-browser-demo.hwpx",
      docx: "docxly-browser-demo.docx",
    },
    statusDescriptions: {
      hwpx: "Generating HWPX in the browser...",
      docx: "Generating DOCX in the browser...",
    },
    comparison: {
      unavailable: "Unavailable",
      unavailableHeadline: "Offline benchmark data unavailable.",
      availableHeadline: "Summary DOCX benchmark powered by the shared comparison dataset.",
      fallbackProof:
        "Install the embeddable DOCX engine backed by the same Rust core in Node, browser, and HWPX workflows.",
      installProof: (ratio) =>
        `Install the embeddable DOCX engine that measured ${ratio} faster than Pandoc on the summary benchmark.`,
      label: "Offline Node benchmark for library selection, not browser runtime timing.",
      metaUnavailable: "comparison data unavailable",
      meta: (data) =>
        `Measured on ${data.machine_label} at ${data.measured_at} with Node ${data.node_version} and Pandoc ${data.pandoc_version}.`,
      badge: "HWPX support is docxly-only",
    },
    copy: {
      idle: "Copy",
      success: "Copied",
      fail: "Copy manually",
    },
    formatSummary(activeFormat, defaultFormat) {
      if (activeFormat === defaultFormat && defaultFormat === "hwpx") {
        return "HWPX is the default path for Korean-language users in this demo. Switch tabs to generate DOCX instead.";
      }

      if (activeFormat === defaultFormat) {
        return "DOCX is the default path for non-Korean users in this demo. Switch tabs to generate HWPX instead.";
      }

      if (defaultFormat === "hwpx") {
        return "DOCX is one tab away from the Korean-language default. Switch back to HWPX to return to the default path.";
      }

      return "HWPX is one tab away from the language-based default. Switch back to DOCX to return to the default path.";
    },
    generateLabel: (formatLabel) => `Generate ${formatLabel}`,
    ready: (formatLabel) => `Ready to generate ${formatLabel}.`,
    success: (filename, elapsed, byteLength) =>
      `Generation started for ${filename} in ${elapsed} (${byteLength} bytes).`,
    failed: (elapsed, message) => `Generation failed in ${elapsed}: ${message}`,
  },
  ko: {
    defaultTitle: "Docxly 도입 제안서",
    defaultFormat: "hwpx",
    sampleMarkdown: `# 왜 docxly를 도입해야 할까요?

docxly는 제품 안에서 바로 DOCX와 HWPX를 만들 수 있게 해주는 **문서 생성 엔진**입니다.

이 문서는 서비스 안에 제안서, 보고서, 다운로드 문서를 넣어야 하는 팀이 왜 docxly를 선택하는지 간단히 정리한 보고서입니다.

## 핵심 요약

> 문서 생성을 외부 변환 서버에 맡기지 않고 제품 안에서 직접 처리하려면 docxly가 더 잘 맞습니다.

- 요약 DOCX 벤치마크에서 **Pandoc 대비 105배 빠른 반복 생성 성능**
- Node와 브라우저에서 같은 Rust 코어 사용
- 하나의 흐름으로 DOCX와 HWPX 모두 지원
- [공개 저장소](https://github.com/docxly/core-rs)와 npm 패키지를 함께 제공

## 왜 도입하기 쉬운가

### 1. 제품 안에 바로 넣을 수 있습니다

문서 생성이 사용자 경험의 일부라면, 문서 엔진도 제품 코드 안에 들어오는 편이 자연스럽습니다.

- 별도 변환 서버에 의존하지 않아도 됩니다
- 핵심 흐름을 CLI 호출로 우회하지 않아도 됩니다
- Node와 브라우저에서 같은 API 흐름을 유지할 수 있습니다

### 2. 응답 속도가 사용자 경험을 바꿉니다

요약 DOCX 벤치마크 기준으로 docxly는 Pandoc보다 **105배 빠르게** 측정됐습니다.

- 반복 실행 구간에서 더 빠른 응답 속도
- 미리보기와 즉시 생성이 필요한 화면에 유리
- 도입 타당성을 설명하기 쉬운 명확한 수치

### 3. DOCX와 HWPX를 함께 다룰 수 있습니다

같은 코어를 기반으로 문서 형식을 확장할 수 있다는 점도 장점입니다.

- DOCX 생성으로 일반 오피스 호환성 확보
- HWPX 생성으로 한국 문서 워크플로 대응
- Markdown 입력 모델을 여러 런타임에서 공통으로 사용

## 권장 결론

\`\`\`bash
npm install @docxly/core-rs
\`\`\`

문서 생성을 별도 변환 단계가 아니라 제품 기능으로 만들고 싶다면 docxly를 검토할 가치가 충분합니다.`,
    formatLabels: {
      hwpx: "HWPX",
      docx: "DOCX",
    },
    defaultFilenames: {
      hwpx: "docxly-브라우저-데모.hwpx",
      docx: "docxly-브라우저-데모.docx",
    },
    statusDescriptions: {
      hwpx: "브라우저에서 HWPX 파일을 만드는 중입니다...",
      docx: "브라우저에서 DOCX 파일을 만드는 중입니다...",
    },
    comparison: {
      unavailable: "준비 중",
      unavailableHeadline: "비교 지표를 아직 불러오지 못했습니다.",
      availableHeadline: "Pandoc과 비교한 요약 DOCX 벤치마크 결과입니다.",
      fallbackProof:
        "Node와 브라우저에서 같은 Rust 코어를 쓰는 문서 생성 엔진이 필요하다면 docxly를 살펴보세요.",
      installProof: (ratio) =>
        `요약 DOCX 벤치마크에서 Pandoc보다 ${ratio} 빠르게 측정됐습니다.`,
      label: "브라우저 실측이 아니라 라이브러리 선택을 위한 Node 벤치마크입니다.",
      metaUnavailable: "비교 데이터를 아직 불러오지 못했습니다.",
      meta: (data) =>
        `${data.machine_label}에서 ${data.measured_at}에 측정했으며 Node ${data.node_version}, Pandoc ${data.pandoc_version} 기준입니다.`,
      badge: "docxly는 HWPX도 지원합니다",
    },
    copy: {
      idle: "복사",
      success: "복사됨",
      fail: "수동 복사",
    },
    formatSummary(activeFormat, defaultFormat) {
      if (activeFormat === defaultFormat && defaultFormat === "hwpx") {
        return "한국어 페이지에서는 HWPX가 기본 형식입니다. DOCX가 필요하면 탭을 바꿔 생성하세요.";
      }

      if (activeFormat === defaultFormat) {
        return "이 페이지에서는 DOCX가 기본 형식입니다. HWPX가 필요하면 탭을 바꾸세요.";
      }

      if (defaultFormat === "hwpx") {
        return "지금은 DOCX를 선택한 상태입니다. 기본 형식으로 돌아가려면 HWPX를 선택하세요.";
      }

      return "지금은 HWPX를 선택한 상태입니다. 기본 형식으로 돌아가려면 DOCX를 선택하세요.";
    },
    generateLabel: (formatLabel) => `${formatLabel} 생성`,
    ready: (formatLabel) => `${formatLabel}를 생성할 준비가 되었습니다.`,
    success: (filename, elapsed, byteLength) =>
      `${filename} 파일 생성을 ${elapsed} 만에 시작했습니다. (${byteLength} bytes)`,
    failed: (elapsed, message) => `${elapsed} 만에 파일 생성에 실패했습니다: ${message}`,
  },
};

const markdownInput = document.querySelector("#markdown-input");
const titleInput = document.querySelector("#title-input");
const authorInput = document.querySelector("#author-input");
const strictModeInput = document.querySelector("#strict-mode-input");
const generateButton = document.querySelector("#generate-button");
const formatTabs = Array.from(document.querySelectorAll(".format-tab"));
const formatSummary = document.querySelector("#format-summary");
const generateButtonLabel = document.querySelector("#generate-button-label");
const status = document.querySelector("#status");
const copyInstallButton = document.querySelector("#copy-install-button");
const installCommand = document.querySelector("#install-command");
const installProof = document.querySelector("#install-proof");
const comparisonHeadline = document.querySelector("#comparison-headline");
const comparisonDocxlyMs = document.querySelector("#comparison-docxly-ms");
const comparisonPandocMs = document.querySelector("#comparison-pandoc-ms");
const comparisonRatio = document.querySelector("#comparison-ratio");
const comparisonLabel = document.querySelector("#comparison-label");
const comparisonMeta = document.querySelector("#comparison-meta");
const comparisonHwpxBadge = document.querySelector("#comparison-hwpx-badge");
const mimeTypes = {
  hwpx: "application/haansofthwp",
  docx: "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
};
const pageLocale = (document.body.dataset.pageLocale || document.documentElement.lang || "en")
  .toLowerCase()
  .trim();
const ui = pageLocale.startsWith("ko") ? locales.ko : locales.en;
const rootPath = document.body.dataset.rootPath || ".";
const comparisonDataUrl = new URL("comparison-data.json", new URL(`${rootPath}/`, window.location.href));
const defaultFormat = ui.defaultFormat;
let activeFormat = defaultFormat;
let copyResetTimer = null;

markdownInput.value = ui.sampleMarkdown;
titleInput.value = ui.defaultTitle;

function setStatus(message, type = "idle") {
  status.textContent = message;
  status.dataset.state = type;
}

function formatElapsedMs(startedAt) {
  return `${Math.round(performance.now() - startedAt)} ms`;
}

function formatMetricMs(value) {
  return Number.isFinite(value) ? `${Math.round(value)} ms` : ui.comparison.unavailable;
}

function formatMetricRatio(value) {
  if (!Number.isFinite(value)) {
    return ui.comparison.unavailable;
  }

  if (Math.abs(value - Math.round(value)) < 0.01) {
    return `${Math.round(value)}x`;
  }

  return value >= 10 ? `${value.toFixed(1)}x` : `${value.toFixed(2)}x`;
}

function buildFilename(rawTitle, format) {
  const normalized = (rawTitle || "")
    .trim()
    .replace(/[<>:"/\\|?*\u0000-\u001f]/g, "")
    .replace(/\s+/g, "-")
    .replace(/-+/g, "-")
    .replace(/^-|-$/g, "");

  const base = normalized || ui.defaultFilenames[format].replace(/\.[^.]+$/, "");
  const extension = `.${format}`;
  return base.toLowerCase().endsWith(extension) ? base : `${base}${extension}`;
}

function setCopyButton(state) {
  const labels = ui.copy;
  const variants = {
    idle: { icon: "⧉", label: labels.idle },
    success: { icon: "✓", label: labels.success },
    fail: { icon: "!", label: labels.fail },
  };
  const variant = variants[state];
  copyInstallButton.innerHTML = `<span aria-hidden="true">${variant.icon}</span><span>${variant.label}</span>`;
}

function updateFormatUi() {
  for (const tab of formatTabs) {
    const isActive = tab.dataset.format === activeFormat;
    tab.classList.toggle("is-active", isActive);
    tab.setAttribute("aria-selected", String(isActive));
  }

  generateButtonLabel.textContent = ui.generateLabel(ui.formatLabels[activeFormat]);
  formatSummary.textContent = ui.formatSummary(activeFormat, defaultFormat);
}

function resetCopyButton() {
  setCopyButton("idle");
}

async function copyInstallCommand() {
  const command = installCommand.textContent?.trim() || "npm install @docxly/core-rs";

  try {
    await navigator.clipboard.writeText(command);
    setCopyButton("success");
    window.clearTimeout(copyResetTimer);
    copyResetTimer = window.setTimeout(resetCopyButton, 1600);
  } catch {
    setCopyButton("fail");
    window.clearTimeout(copyResetTimer);
    copyResetTimer = window.setTimeout(resetCopyButton, 2000);
  }
}

function triggerDownload(blob, filename) {
  const objectUrl = URL.createObjectURL(blob);
  const triggerLink = document.createElement("a");
  triggerLink.href = objectUrl;
  triggerLink.download = filename;
  triggerLink.textContent = `Generate ${filename}`;
  triggerLink.style.position = "fixed";
  triggerLink.style.left = "-9999px";
  triggerLink.style.top = "0";
  document.body.append(triggerLink);
  triggerLink.click();
  triggerLink.remove();
  window.setTimeout(() => URL.revokeObjectURL(objectUrl), 0);
}

function renderComparisonUnavailable() {
  comparisonHeadline.textContent = ui.comparison.unavailableHeadline;
  comparisonDocxlyMs.textContent = ui.comparison.unavailable;
  comparisonPandocMs.textContent = ui.comparison.unavailable;
  comparisonRatio.textContent = ui.comparison.unavailable;
  installProof.textContent = ui.comparison.fallbackProof;
  comparisonLabel.textContent = ui.comparison.label;
  comparisonMeta.textContent = ui.comparison.metaUnavailable;
  comparisonHwpxBadge.textContent = ui.comparison.badge;
}

function renderComparison(data) {
  const summary = data?.benchmarks?.summary;

  if (!summary) {
    renderComparisonUnavailable();
    return;
  }

  comparisonHeadline.textContent = ui.comparison.availableHeadline;
  comparisonDocxlyMs.textContent = formatMetricMs(summary.steady_median_ms?.docxly);
  comparisonPandocMs.textContent = formatMetricMs(summary.steady_median_ms?.pandoc);
  comparisonRatio.textContent = formatMetricRatio(summary.speed_ratio);
  installProof.textContent = ui.comparison.installProof(formatMetricRatio(summary.speed_ratio));
  comparisonLabel.textContent = ui.comparison.label;
  comparisonMeta.textContent = ui.comparison.meta(data);
  comparisonHwpxBadge.textContent = ui.comparison.badge;
}

async function loadComparison() {
  try {
    const response = await fetch(comparisonDataUrl, { cache: "no-store" });
    if (!response.ok) {
      throw new Error(`failed to load comparison data: ${response.status}`);
    }

    const data = await response.json();
    renderComparison(data);
  } catch {
    renderComparisonUnavailable();
  }
}

copyInstallButton.addEventListener("click", () => {
  void copyInstallCommand();
});

for (const tab of formatTabs) {
  tab.addEventListener("click", () => {
    activeFormat = tab.dataset.format;
    updateFormatUi();
    setStatus(ui.ready(ui.formatLabels[activeFormat]));
  });
}

generateButton.addEventListener("click", async () => {
  generateButton.disabled = true;
  setStatus(ui.statusDescriptions[activeFormat], "pending");
  const startedAt = performance.now();

  try {
    const generator = activeFormat === "hwpx" ? generateHwpx : generateDocx;
    const bytes = await generator(markdownInput.value, {
      title: titleInput.value || undefined,
      author: authorInput.value || undefined,
      strictMode: strictModeInput.checked,
    });

    const blob = new Blob([bytes], { type: mimeTypes[activeFormat] });
    const filename = buildFilename(titleInput.value || ui.defaultFilenames[activeFormat], activeFormat);
    triggerDownload(blob, filename);
    setStatus(ui.success(filename, formatElapsedMs(startedAt), bytes.length), "success");
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    setStatus(ui.failed(formatElapsedMs(startedAt), message), "error");
  } finally {
    generateButton.disabled = false;
  }
});

updateFormatUi();
resetCopyButton();
setStatus(ui.ready(ui.formatLabels[activeFormat]));
void loadComparison();
