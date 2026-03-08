# Official Hancom schema PDF conversion

- Work date: 2026-03-08
- Conversion strategy: `pdftotext -layout` first, selective OCR prepared via `tesseract` with `kor` installed, final manual section split.
- OCR status: `kor` language pack is available. The downloaded Hancom PDFs are text PDFs, so the committed Markdown is based on text extraction first. Remaining broken title glyphs such as `한글` were normalized conservatively; OCR was not forced across the whole corpus.
- Source PDFs are **not committed**. They were kept only in `/tmp/hancom-pdf` during conversion.

## Official source URLs

- `한글문서파일형식3.0_HWPML_revision1.2.pdf`: https://cdn.hancom.com/link/docs/%ED%95%9C%EA%B8%80%EB%AC%B8%EC%84%9C%ED%8C%8C%EC%9D%BC%ED%98%95%EC%8B%9D3.0_HWPML_revision1.2.pdf
- `한글문서파일형식_5.0_revision1.3.pdf`: https://cdn.hancom.com/link/docs/%ED%95%9C%EA%B8%80%EB%AC%B8%EC%84%9C%ED%8C%8C%EC%9D%BC%ED%98%95%EC%8B%9D_5.0_revision1.3.pdf
- `한글문서파일형식_배포용문서_revision1.2.pdf`: https://cdn.hancom.com/link/docs/%ED%95%9C%EA%B8%80%EB%AC%B8%EC%84%9C%ED%8C%8C%EC%9D%BC%ED%98%95%EC%8B%9D_%EB%B0%B0%ED%8F%AC%EC%9A%A9%EB%AC%B8%EC%84%9C_revision1.2.pdf
- `한글문서파일형식_수식_revision1.3.pdf`: https://cdn.hancom.com/link/docs/%ED%95%9C%EA%B8%80%EB%AC%B8%EC%84%9C%ED%8C%8C%EC%9D%BC%ED%98%95%EC%8B%9D_%EC%88%98%EC%8B%9D_revision1.3.pdf
- `한글문서파일형식_차트_revision1.2.pdf`: https://cdn.hancom.com/link/docs/%ED%95%9C%EA%B8%80%EB%AC%B8%EC%84%9C%ED%8C%8C%EC%9D%BC%ED%98%95%EC%8B%9D_%EC%B0%A8%ED%8A%B8_revision1.2.pdf

## OWPML / HWPX standard reference

- KSSN detail page: https://www.kssn.net/search/stddetail.do?itemNo=K001010149626
- e-나라표준인증 detail page: http://standard.go.kr/KSCI/standardIntro/getStandardSearchView.do?menuId=503&topMenuId=502&ksNo=KSX6101&tmprKsNo=KSX6101&reformNo=01
- Note: KS X 6101:2024 `개방형 워드프로세서 마크업 언어(OWPML) 문서 구조` PDF is distributed through the standards portal and was not mirrored here. This directory therefore stores free official Hancom HWP/HWPML reference conversions plus the official KS source metadata.
- Implementation note: package-level HWPX files such as `content.hpf`, `header.xml`, and `section0.xml` are **not** described directly in the free Hancom PDFs below. For those, use this directory together with the official KS source links above and the curated HWPX notes already present in `packages/core-rs/src/generators/hwpx/docs/`.

## Generated Markdown files

- [01-hwpml-3.0-binary-file-structure.md](./01-hwpml-3.0-binary-file-structure.md) - physical PDF pages 11-54
- [02-hwpml-3.0-xml-structure.md](./02-hwpml-3.0-xml-structure.md) - physical PDF pages 55-120
- [03-hwp-5.0-file-structure.md](./03-hwp-5.0-file-structure.md) - physical PDF pages 7-20
- [04-hwp-5.0-data-records-and-history.md](./04-hwp-5.0-data-records-and-history.md) - physical PDF pages 21-70
- [05-distribution-document.md](./05-distribution-document.md) - physical PDF pages 5-11
- [06-equation-spec.md](./06-equation-spec.md) - physical PDF pages 6-19
- [07-chart-spec.md](./07-chart-spec.md) - physical PDF pages 7-47
