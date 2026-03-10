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
    sampleMarkdown: `# 왜 지금 docxly를 설치해야 하는가

docxly는 별도 변환 단계가 아니라 제품 안에서 직접 DOCX와 HWPX를 생성해야 하는 팀을 위한 **내장형 문서 생성 엔진**입니다.

이 보고서는 제품 팀이 더 빠른 문서 생성, 브라우저 로컬 워크플로, 그리고 런타임 전반에서 공유되는 하나의 Rust 코어를 원할 때 왜 docxly 설치가 합리적인지 설명합니다.

## 핵심 요약

> 문서 생성이 외부 변환 서비스가 아니라 애플리케이션과 브라우저 워크플로 안에 들어가야 한다면 docxly를 설치해야 합니다.

- 현재 요약 DOCX 벤치마크에서 **Pandoc 대비 105배 빠른 속도**
- Node와 동일한 Rust 코어를 사용하는 브라우저 로컬 생성
- 하나의 제품 표면에서 HWPX와 DOCX를 모두 지원
- [공개 저장소](https://github.com/docxly/core-rs)를 가진 오픈소스 패키지

## 설치가 곧 제품 경쟁력이 되는 이유

### 1. 제품 안에 들어가는 엔진이 필요합니다

제안서, 보고서, 내보내기 문서, 고객용 결과물을 애플리케이션 안에서 생성한다면 문서 엔진도 제품 스택의 일부여야 합니다.

- 메인 흐름에서 별도 백엔드 변환 의존성이 없습니다
- 핵심 사용자 경험을 위해 별도의 CLI 오케스트레이션이 필요하지 않습니다
- Node와 브라우저 컨텍스트를 하나의 패키지로 통합할 수 있습니다

### 2. 속도는 사용자 경험을 바꿉니다

현재 벤치마크의 핵심 문장은 명확합니다. docxly는 요약 DOCX 벤치마크에서 Pandoc보다 **105배 빠르게** 측정되었습니다.

- 반복 생성 구간에서 더 빠른 steady-state 성능
- 인터랙티브한 제품 워크플로에 더 적합한 응답성
- 설치 의사결정을 쉽게 만드는 명확한 수치

### 3. 하나의 코어가 여러 문서 경로를 지원합니다

docxly는 문서 워크플로 전체에서 같은 코어 아키텍처를 유지합니다.

- 광범위한 오피스 호환성을 위한 DOCX 생성
- 한국 문서 워크플로를 위한 HWPX 생성
- 런타임 전반에서 공유되는 Markdown 문서 모델

## 권장 설치 결정

\`\`\`bash
npm install @docxly/core-rs
\`\`\`

문서 생성을 별도 변환 단계가 아니라 제품 기능으로 만들고 싶다면 docxly를 설치해야 합니다.`,
    formatLabels: {
      hwpx: "HWPX",
      docx: "DOCX",
    },
    defaultFilenames: {
      hwpx: "docxly-브라우저-데모.hwpx",
      docx: "docxly-브라우저-데모.docx",
    },
    statusDescriptions: {
      hwpx: "브라우저에서 HWPX를 생성하고 있습니다...",
      docx: "브라우저에서 DOCX를 생성하고 있습니다...",
    },
    comparison: {
      unavailable: "준비 중",
      unavailableHeadline: "오프라인 벤치마크 데이터를 불러오지 못했습니다.",
      availableHeadline: "공유 비교 데이터셋으로 측정한 요약 DOCX 벤치마크입니다.",
      fallbackProof:
        "Node, 브라우저, HWPX 워크플로 전체에서 같은 Rust 코어를 사용하는 내장형 DOCX 엔진을 설치하세요.",
      installProof: (ratio) =>
        `요약 벤치마크에서 Pandoc보다 ${ratio} 빠르게 측정된 내장형 DOCX 엔진을 설치하세요.`,
      label: "브라우저 실행 시간이 아닌, 라이브러리 선택을 위한 Node 벤치마크입니다.",
      metaUnavailable: "비교 데이터를 불러오지 못했습니다.",
      meta: (data) =>
        `${data.machine_label}에서 ${data.measured_at}에 측정했으며, Node ${data.node_version} 및 Pandoc ${data.pandoc_version} 기준입니다.`,
      badge: "HWPX 지원은 docxly 전용입니다",
    },
    copy: {
      idle: "복사",
      success: "복사됨",
      fail: "수동 복사",
    },
    formatSummary(activeFormat, defaultFormat) {
      if (activeFormat === defaultFormat && defaultFormat === "hwpx") {
        return "한국어 사용자에게는 HWPX가 기본 경로입니다. 다른 형식이 필요하면 DOCX 탭으로 전환하세요.";
      }

      if (activeFormat === defaultFormat) {
        return "비한국어 사용자에게는 DOCX가 기본 경로입니다. HWPX가 필요하면 탭을 전환하세요.";
      }

      if (defaultFormat === "hwpx") {
        return "DOCX는 한국어 기본 경로에서 한 탭 떨어져 있습니다. 기본값으로 돌아가려면 HWPX를 선택하세요.";
      }

      return "HWPX는 언어 기반 기본 경로에서 한 탭 떨어져 있습니다. 기본값으로 돌아가려면 DOCX를 선택하세요.";
    },
    generateLabel: (formatLabel) => `${formatLabel} 생성`,
    ready: (formatLabel) => `${formatLabel}를 생성할 준비가 되었습니다.`,
    success: (filename, elapsed, byteLength) =>
      `${filename} 생성이 ${elapsed} 만에 시작되었습니다. (${byteLength} bytes)`,
    failed: (elapsed, message) => `${elapsed} 후 생성에 실패했습니다: ${message}`,
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
