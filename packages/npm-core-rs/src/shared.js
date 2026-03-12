export function normalizeTarget(target) {
  if (target !== "docx" && target !== "hwpx") {
    throw new TypeError("target must be `docx` or `hwpx`");
  }
  return target;
}

export function normalizeReport(report) {
  return {
    issues: report.issues ?? [],
    degraded: Boolean(report.degraded),
    unsupportedCount: Number(report.unsupported_count ?? 0),
    fallbackCount: Number(report.fallback_count ?? 0),
  };
}

export function parseGenerationResponse(rawJson) {
  const payload = JSON.parse(rawJson);
  const report = normalizeReport(payload.report ?? {});

  if (payload.ok) {
    return {
      bytes: decodeBase64(payload.bytes_base64 ?? ""),
      report,
    };
  }

  throw generationFailure(payload.error ?? "generation failed", report);
}

export function generationFailure(message, report, cause) {
  const failure = cause instanceof Error
    ? new Error(message, { cause })
    : new Error(message);
  failure.name = "GenerationFailure";
  failure.report = report;
  failure.error = failure.message;
  return failure;
}

export function serializeHwpxStyle(style) {
  if (style == null) {
    return null;
  }

  return JSON.stringify({
    bodyFont: style.bodyFont ?? null,
    headingFont: style.headingFont ?? null,
    bodyFontSize: style.bodyFontSize ?? null,
    headingFontSize: style.headingFontSize ?? null,
    textColor: style.textColor ?? null,
    headingColor: style.headingColor ?? null,
    linkColor: style.linkColor ?? null,
    paragraphAlign: style.paragraphAlign ?? null,
  });
}

function decodeBase64(value) {
  if (typeof globalThis.atob === "function") {
    const binary = globalThis.atob(value);
    const bytes = new Uint8Array(binary.length);
    for (let index = 0; index < binary.length; index += 1) {
      bytes[index] = binary.charCodeAt(index);
    }
    return bytes;
  }

  if (typeof Buffer !== "undefined") {
    return new Uint8Array(Buffer.from(value, "base64"));
  }

  throw new Error("base64 decoder is unavailable in this runtime");
}
