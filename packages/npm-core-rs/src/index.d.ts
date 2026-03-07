export interface DocxOptions {
  title?: string;
  author?: string;
  strictMode?: boolean;
}

export declare function generateDocx(
  markdown: string,
  options?: DocxOptions,
): Promise<Uint8Array>;
