# 런타임 가이드

## 지원 런타임

- Node
- 브라우저 번들러 환경
- Rust workspace/path dependency

## Node

`@docxly/core-rs`는 비동기 API로 DOCX 생성을 제공합니다.

- 기본 엔트리포인트: `generateDocx(markdown, options)`
- 반환값: `Promise<Uint8Array>`
- 주요 옵션: `title`, `author`, `strictMode`

## 브라우저

브라우저에서는 `.wasm` 자산을 처리할 수 있는 번들러가 필요합니다.

- raw `<script type="module">` 직접 import 용도가 아님
- ESM 해석 가능 런타임 필요
- `.wasm` asset emission 지원 필요

현재 데모는 브라우저에서 직접 DOCX와 HWPX를 생성하고, `blob:` 기반 다운로드로 결과 파일을 저장합니다.

## HWPX

HWPX는 npm v0.x의 공식 public API에는 아직 노출되지 않았지만, Rust 코어에서는 구현돼 있습니다.

현재 승인된 HWPX baseline:

- paragraph
- heading
- inline emphasis/strong/code/link
- blockquote
- code block
- unordered list depth 2
- document-level style overrides

## strict mode

- 기본값은 `strict_mode = true`
- 지원하지 않는 입력은 strict mode에서 실패
- strict mode를 끄면 일부 unsupported input이 plain text fallback으로 처리됨
