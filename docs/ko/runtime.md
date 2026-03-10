# 런타임 가이드

## 지원 런타임

- Node
- 브라우저 번들러 환경
- Rust 워크스페이스 또는 path dependency

## Node

`@docxly/core-rs`는 비동기 API로 문서를 생성합니다.

- DOCX: `generateDocx(markdown, options)`
- HWPX: `generateHwpx(markdown, options)`
- 반환값: 두 API 모두 `Promise<Uint8Array>`
- 공통 옵션: `title`, `author`, `strictMode`

## 브라우저

브라우저에서는 `.wasm` 자산을 함께 처리할 수 있는 번들러가 필요합니다.

- 순수 `<script type="module">`만으로 바로 가져다 쓰는 형태는 지원하지 않습니다.
- ESM을 해석할 수 있는 런타임이 필요합니다.
- `.wasm` 파일을 함께 내보낼 수 있어야 합니다.

현재 데모는 브라우저에서 직접 DOCX와 HWPX를 생성하고, `blob:` 기반 다운로드로 결과 파일을 저장합니다.

## HWPX

HWPX는 Rust 코어뿐 아니라 npm 패키지에서도 사용할 수 있습니다. 현재 저장소의 브라우저 데모 역시 같은 API로 HWPX를 생성합니다.

현재 검증이 끝난 HWPX 기본 범위는 다음과 같습니다.

- 문단
- 제목
- 인라인 강조, 굵게, 코드, 링크
- 인용문
- 코드 블록
- 2단계까지의 글머리표 목록
- 문서 단위 스타일 설정

## Strict Mode

- 기본값은 `strictMode = true`입니다.
- 지원하지 않는 입력은 Strict Mode에서 오류로 처리됩니다.
- Strict Mode를 끄면 일부 미지원 입력은 일반 텍스트로 완화 처리됩니다.
