export interface DocxOptions {
  title?: string;
  author?: string;
  strictMode?: boolean;
}

export interface HwpxOptions {
  title?: string;
  author?: string;
  strictMode?: boolean;
}

export declare function generateDocx(
  markdown: string,
  options?: DocxOptions,
): Promise<Uint8Array>;

export declare function generateHwpx(
  markdown: string,
  options?: HwpxOptions,
): Promise<Uint8Array>;
