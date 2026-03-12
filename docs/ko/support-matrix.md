# 지원 매트릭스

이 문서는 현재 `docxly`가 공개 계약으로 보는 범위를 정리합니다.

## 런타임 및 API 상태

| 표면 | 상태 | 설명 |
| --- | --- | --- |
| `@docxly/core-rs` DOCX API | `0.x` 안정 | Node와 브라우저 사용자의 기본 진입점 |
| `@docxly/core-rs` HWPX API | 베타 | 패키지에는 포함되어 있지만 DOCX보다 범위가 좁고, 기본 스타일 옵션만 사용하는 승인된 HWPX 내용 기준 안에서만 신뢰해야 함 |
| `core-rs::generate_docx` | 저장소 범위 안정 | 고수준 Rust API만 공개 계약으로 간주 |
| `core-rs::generate_hwpx` | 베타 | 승인된 HWPX 기준과 Rust 스타일 계약 범위에서 지원 |
| 내부 parser / model / generator 모듈 | 비공개 | 안정 계약 아님 |

## 포맷 지원 상태

| 포맷 | 표면 | 상태 | 설명 |
| --- | --- | --- | --- |
| DOCX | npm + Rust | 안정 | 현재의 주력 프로덕션 경로 |
| HWPX | Rust | 베타 | 승인된 호환, 목록, 표, 스타일 기준 범위를 지원 |
| HWPX | npm | 베타 | `generateHwpx`로 사용할 수 있지만 옵션 범위가 더 좁고 공개 스타일 재정의 기능은 없음 |

## Markdown 지원 수준

DOCX는 현재 다음을 지원합니다.

- headings
- paragraphs
- emphasis
- strong emphasis
- inline code
- links
- blockquotes
- fenced code blocks
- thematic breaks
- ordered and unordered lists
- nested lists up to depth 2
- GFM pipe tables
- `data:` URI images

Rust HWPX는 현재 다음 승인 기준을 지원합니다.

- paragraphs
- headings
- 승인 계약 안의 visible-text inline emphasis, strong, code, link
- 승인된 목록 fixture
- 승인된 표 fixture
- 승인된 스타일 fixture

npm HWPX의 공개 옵션은 현재 다음만 지원합니다.

- `title`
- `author`
- `strictMode`

아직 승인 기준 밖이거나 npm 공개 옵션 범위 밖에 있는 HWPX 항목:

- images
- 승인 fixture 범위를 넘는 일반 표 지원
- provisional fixture에 있는 더 넓은 레이아웃 및 블록 범위
- Rust의 `HwpxStyleOptions`에 해당하는 공개 npm 스타일 재정의 기능

## 안정성 규칙

- 프로덕션 기본 권장 경로는 DOCX입니다.
- Rust와 npm의 HWPX는 모두 베타이며, 각 표면에서 승인 fixture로 증명된 동작만 신뢰해야 합니다.
- provisional 및 quarantined HWPX fixture는 개발 신호이지 릴리스 약속이 아닙니다.
- 크레이트 내부 모듈은 사전 폐기 절차 없이 바뀔 수 있습니다.

## 어떤 채널로 요청해야 하나

이슈로 올릴 것:

- 문서화된 동작과 실제 동작이 다를 때
- 안정 또는 베타 공개 계약이 회귀했을 때
- 실제 사용 사례가 fixture나 smoke test에 편입되어야 할 때

기능 요청이나 논의로 올릴 것:

- 새로운 출력 기능이 필요할 때
- 새로운 공개 API가 필요할 때
- 더 넓은 npm HWPX 지원이나 `crates.io` 배포가 필요할 때
