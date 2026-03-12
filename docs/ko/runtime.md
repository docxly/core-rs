# 런타임 가이드

## 지원 런타임

- Node
- 브라우저 번들러 환경
- Rust 워크스페이스 또는 경로 의존성

## 어떤 경로를 선택해야 하나

| 상황 | 권장 경로 |
| --- | --- |
| 가장 빠르게 도입하고 싶다 | npm DOCX |
| 브라우저에서 직접 다운로드해야 한다 | npm DOCX + 번들러 |
| HWPX가 꼭 필요하다 | npm 또는 Rust HWPX, 단 베타 계약으로 접근 |
| 내부 시스템에 깊게 붙는 Rust 통합이 필요하다 | Rust 크레이트 |

## Node

`@docxly/core-rs`는 비동기 API로 문서를 생성합니다.

- DOCX: `generateDocx(markdown, options)`
- HWPX: `generateHwpx(markdown, options)`
- 반환값: 두 API 모두 `Promise<Uint8Array>`
- 공통 옵션: `title`, `author`, `strictMode`
- 기본 프로덕션 권장 경로는 DOCX이며, HWPX API는 현재 베타입니다.

## 브라우저

브라우저에서는 `.wasm` 자산을 함께 처리할 수 있는 번들러가 필요합니다.

- 순수 `<script type="module">`만으로 바로 가져다 쓰는 형태는 지원하지 않습니다.
- ESM을 해석할 수 있는 런타임이 필요합니다.
- `.wasm` 파일을 함께 내보낼 수 있어야 합니다.

현재 데모는 브라우저에서 직접 DOCX와 HWPX를 생성하고, `blob:` 기반 다운로드로 결과 파일을 저장합니다.
브라우저에서도 HWPX를 사용할 수 있지만, 공개 계약상 성숙도는 DOCX보다 낮습니다.

## HWPX

HWPX는 Rust 코어와 npm 패키지 모두에서 사용할 수 있습니다. 현재 저장소의 브라우저 데모 역시 같은 API를 사용합니다.

현재 검증이 끝난 HWPX 기본 지원 범위는 다음과 같습니다.

- 문단
- 제목
- 인라인 강조, 굵게, 코드, 링크
- 인용문
- 코드 블록
- 2단계까지의 글머리표 목록
- 승인된 표 fixture 범위의 표
- 문서 단위 스타일 설정은 Rust API에서 지원

주의할 점:

- HWPX는 DOCX보다 지원 범위가 좁습니다.
- 승인된 fixture 기준 밖의 동작은 베타로 봐야 합니다.
- npm HWPX 공개 옵션은 `title`, `author`, `strictMode`만 제공합니다.
- 기본 프로덕션 경로를 선택해야 한다면 DOCX가 우선입니다.

## Strict Mode

- 기본값은 `strictMode = true`입니다.
- 지원하지 않는 입력은 Strict Mode에서 오류로 처리됩니다.
- Strict Mode를 끄면 일부 미지원 입력은 일반 텍스트로 완화 처리됩니다.

## 추천 운영 원칙

- 첫 출시 경로는 DOCX부터 시작합니다.
- HWPX는 승인 fixture로 검증된 범위만 약속으로 간주합니다.
- 문서 지원 범위가 불분명할 때는 실제 산출물과 fixture를 함께 확인합니다.
