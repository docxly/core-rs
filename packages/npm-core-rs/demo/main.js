import { generateDocx } from "./browser-client.js";

const sampleMarkdown = `# Browser Demo

This DOCX file is generated in the browser with **docxly** and the same Rust core used by the npm package.

> Markdown in. DOCX out. Browser only.

## Highlights

1. WebAssembly runtime
2. No backend conversion
3. Local download

| Capability | Status |
| --- | --- |
| Markdown parsing | Browser |
| DOCX packaging | Browser |
| Open source package | npm + GitHub |`;

const markdownInput = document.querySelector("#markdown-input");
const titleInput = document.querySelector("#title-input");
const authorInput = document.querySelector("#author-input");
const strictModeInput = document.querySelector("#strict-mode-input");
const generateButton = document.querySelector("#generate-button");
const downloadLink = document.querySelector("#download-link");
const status = document.querySelector("#status");
const copyInstallButton = document.querySelector("#copy-install-button");
const installCommand = document.querySelector("#install-command");
const defaultFilename = "docxly-browser-demo.docx";
let activeObjectUrl = null;
let copyResetTimer = null;

markdownInput.value = sampleMarkdown;

function setStatus(message, type = "idle") {
  status.textContent = message;
  status.dataset.state = type;
}

function buildFilename(rawTitle) {
  const normalized = (rawTitle || "")
    .trim()
    .replace(/[<>:"/\\|?*\u0000-\u001f]/g, "")
    .replace(/\s+/g, "-")
    .replace(/-+/g, "-")
    .replace(/^-|-$/g, "");

  const base = normalized || "docxly-browser-demo";
  return base.toLowerCase().endsWith(".docx") ? base : `${base}.docx`;
}

function updateDownloadLinkLabel(filename) {
  downloadLink.innerHTML = `
    <span class="action-symbol" aria-hidden="true">↺</span>
    <span>Download ${filename} again</span>
  `;
}

function resetCopyButton() {
  copyInstallButton.innerHTML = '<span aria-hidden="true">⧉</span><span>Copy</span>';
}

async function copyInstallCommand() {
  const command = installCommand.textContent?.trim() || "npm install @docxly/core-rs";

  try {
    await navigator.clipboard.writeText(command);
    copyInstallButton.innerHTML = '<span aria-hidden="true">✓</span><span>Copied</span>';
    window.clearTimeout(copyResetTimer);
    copyResetTimer = window.setTimeout(resetCopyButton, 1600);
  } catch {
    copyInstallButton.innerHTML = '<span aria-hidden="true">!</span><span>Copy manually</span>';
    window.clearTimeout(copyResetTimer);
    copyResetTimer = window.setTimeout(resetCopyButton, 2000);
  }
}

function triggerDownload(blob, filename) {
  if (activeObjectUrl) {
    URL.revokeObjectURL(activeObjectUrl);
  }

  activeObjectUrl = URL.createObjectURL(blob);
  downloadLink.href = activeObjectUrl;
  downloadLink.download = filename;
  updateDownloadLinkLabel(filename);
  downloadLink.classList.remove("hidden");

  const triggerLink = document.createElement("a");
  triggerLink.href = activeObjectUrl;
  triggerLink.download = filename;
  triggerLink.textContent = `Download ${filename}`;
  triggerLink.style.position = "fixed";
  triggerLink.style.left = "-9999px";
  triggerLink.style.top = "0";
  document.body.append(triggerLink);
  triggerLink.click();
  triggerLink.remove();
}

window.addEventListener("pagehide", () => {
  if (activeObjectUrl) {
    URL.revokeObjectURL(activeObjectUrl);
    activeObjectUrl = null;
  }
});

copyInstallButton.addEventListener("click", () => {
  void copyInstallCommand();
});

generateButton.addEventListener("click", async () => {
  generateButton.disabled = true;
  downloadLink.classList.add("hidden");
  downloadLink.removeAttribute("href");
  setStatus("Generating DOCX in the browser...", "pending");

  try {
    const bytes = await generateDocx(markdownInput.value, {
      title: titleInput.value || undefined,
      author: authorInput.value || undefined,
      strictMode: strictModeInput.checked,
    });

    const blob = new Blob([bytes], {
      type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    });
    const filename = buildFilename(titleInput.value || defaultFilename);
    triggerDownload(blob, filename);
    setStatus(`Download started for ${filename} (${bytes.length} bytes).`, "success");
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    setStatus(`Generation failed: ${message}`, "error");
  } finally {
    generateButton.disabled = false;
  }
});
