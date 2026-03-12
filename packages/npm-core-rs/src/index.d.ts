export interface DocxOptions {
  title?: string;
  author?: string;
  strictMode?: boolean;
}

export interface HwpxOptions {
  title?: string;
  author?: string;
  strictMode?: boolean;
  style?: HwpxStyleOptions;
}

export type HwpxParagraphAlign = "left" | "center" | "right" | "justify";

export interface HwpxStyleOptions {
  bodyFont?: string;
  headingFont?: string;
  bodyFontSize?: number;
  headingFontSize?: number;
  textColor?: string;
  headingColor?: string;
  linkColor?: string;
  paragraphAlign?: HwpxParagraphAlign;
}

export type ConversionTarget = "docx" | "hwpx";
export type IssueSeverity = "Warning" | "Error";

export interface ConversionIssue {
  feature: string;
  message: string;
  severity: IssueSeverity;
  degraded: boolean;
}

export interface ConversionReport {
  issues: ConversionIssue[];
  degraded: boolean;
  unsupportedCount: number;
  fallbackCount: number;
}

export interface GenerationResult {
  bytes: Uint8Array;
  report: ConversionReport;
}

export interface GenerationFailure extends Error {
  error: string;
  report: ConversionReport;
  cause?: unknown;
}

export declare function analyzeMarkdown(
  markdown: string,
  target: ConversionTarget,
): Promise<ConversionReport>;

export declare function generateDocx(
  markdown: string,
  options?: DocxOptions,
): Promise<Uint8Array>;

export declare function generateHwpx(
  markdown: string,
  options?: HwpxOptions,
): Promise<Uint8Array>;

export declare function generateDocxWithReport(
  markdown: string,
  options?: DocxOptions,
): Promise<GenerationResult>;

export declare function generateHwpxWithReport(
  markdown: string,
  options?: HwpxOptions,
): Promise<GenerationResult>;
