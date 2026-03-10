# docxly 데모 디자인 시스템

`docxly` 데모 UI를 blue-first 제품 스타일로 재구성하기 위한 구현 기준 문서다. 이 문서는 데모 페이지를 바로 다시 설계할 수 있도록 토큰, 위계, 컴포넌트 규칙, 로고 슬롯 규칙을 결정 완료 상태로 정의한다.

## Overview

### 제품 성격

- embeddable document engine
- Node, browser, Rust 코어를 공유하는 문서 생성 제품
- CLI 도구가 아니라 앱 내부에 들어가는 라이브러리 경험이 핵심

### 대상 사용자

- 앱과 서비스에 문서 생성을 직접 내장하려는 개발자
- 브라우저와 Node에서 같은 코어를 쓰고 싶은 팀
- DOCX뿐 아니라 HWPX까지 같은 제품 흐름으로 확장하려는 팀

### 디자인 목표

- product-like clarity
- generator-first usability
- blue-first technical brand identity

## Brand Foundation

### 핵심 브랜드 문장

- `docxly is an embeddable document generation engine.`
- `docxly brings DOCX and HWPX generation into the product surface, not a separate conversion step.`

### 로고 슬롯 규칙

- 위치: hero 내부 맨 위 좌측
- 형태: `logo mark + wordmark` 또는 `docxly` 텍스트 lockup
- 최소 높이: desktop 28px, mobile 24px
- clear space: 로고 높이의 0.5배
- 기본 배경: 밝은 surface 위 단색 사용
- dark surface 위 사용 시 단색 역상 버전만 허용

### 로고 없는 상태의 fallback

- 실제 로고 자산이 없으면 `docxly` 워드마크 텍스트를 사용한다.
- fallback 서체는 sans-serif display 계열로 고정한다.
- fallback은 headline과 분리된 독립 요소여야 하며, hero heading 안에 합치지 않는다.

## Color System

### Core Tokens

| Token | Value | Role |
| --- | --- | --- |
| `--color-primary-050` | `#eff6ff` | page tint, subtle highlight |
| `--color-primary-100` | `#dbeafe` | soft border, soft chip |
| `--color-primary-500` | `#2563eb` | selected state, active fill |
| `--color-primary-600` | `#1d4ed8` | primary CTA |
| `--color-primary-700` | `#1e40af` | hover / pressed CTA |
| `--color-neutral-950` | `#0f172a` | headline, strong surface |
| `--color-neutral-700` | `#334155` | body text |
| `--color-neutral-500` | `#64748b` | muted text |
| `--color-neutral-200` | `#e2e8f0` | border, divider |
| `--color-surface` | `#ffffff` | base panel surface |
| `--color-surface-muted` | `#f8fafc` | muted background |
| `--color-success` | `#15803d` | success status |
| `--color-error` | `#b91c1c` | error status |

### Usage Rules

- primary CTA는 `primary-600`
- primary CTA hover는 `primary-700`
- selected tab, selected chip, active proof highlight는 `primary-500`
- soft tint 배경은 `primary-050`
- border 기본값은 `neutral-200`
- body text는 `neutral-700`
- muted helper text는 `neutral-500`
- dark comparison/proof strip 표면은 `neutral-950` 기반으로 사용

### Explicit Constraints

- 기존 warm/orange accent는 primary palette에서 제거한다.
- warning/emphasis 보조색도 이번 문서 기준에서는 정의하지 않는다.
- text on primary는 white only다.
- 본문 텍스트 대비는 WCAG AA 이상을 유지한다.

## Typography

### Type Roles

- display: hero headline 전용
- heading: section title, card title
- body: paragraph, helper, note
- mono: install command, textarea, generated status
- label: tab, eyebrow, compact metadata

### Type Scale

| Token | Desktop | Mobile | Usage |
| --- | --- | --- | --- |
| `display-1` | `56/1.0` | `40/1.02` | hero headline |
| `heading-2` | `32/1.05` | `26/1.08` | section heading |
| `body-1` | `16/1.6` | `16/1.6` | default paragraph |
| `body-2` | `14/1.55` | `14/1.55` | helper, note |
| `label` | `12/1.2` | `12/1.2` | uppercase label |

### Type Rules

- hero body는 최대 2문장
- comparison meta는 1줄만 허용
- install helper는 1문장만 허용
- note/debug 문구는 body-2로만 표현

## Layout

### Page Frame

- page width: max 1120px
- panel radius: 20px
- hero top padding: 40px desktop, 24px mobile
- section gap: 24px

### Spacing Scale

- `8`
- `12`
- `16`
- `24`
- `32`
- `48`

### Hero Layout

- desktop: `content column + install card` 2열
- content column 내부 순서:
  - logo
  - one-line value proposition
  - short supporting sentence
  - external links max 2개
  - compact proof strip
- comparison strip은 content column 내부에만 배치
- install card는 독립 보조 카드 1개만 허용

### Mobile Layout

- 모바일 1열 순서:
  - logo/value proposition
  - install card
  - compact proof strip
  - generator panel
- `390x844` 기준 first viewport 안에 `logo + headline + install action`이 보여야 한다.
- first viewport 안에 proof strip 전체가 보일 필요는 없지만, strip 시작부는 보여야 한다.

## Component Rules

### Hero

- 최대 2개 text paragraph
- 외부 링크 최대 2개
- hero 안에 독립 강조 카드 2개 초과 금지
- value proposition은 1문장으로 끝낸다

### Install Card

- command
- copy button
- 1-line helper
- 1-line proof copy using the benchmark summary headline number

canonical install proof:

- `Install the embeddable DOCX engine that measured 105x faster than Pandoc on the summary benchmark.`

금지:

- 여러 installation option 동시 노출
- verbose explanation
- secondary CTA 추가

### Proof Strip

- KPI 3개 고정
  - docxly steady
  - pandoc steady
  - speed ratio
- 1줄 해석 허용
- badge 1개 허용
- long metadata는 1줄만 허용

금지:

- comparison table
- dual comparison cards
- 긴 explanatory paragraph
- feature matrix

### Generator Panel

- 데모의 가장 높은 interaction priority 유지
- 유지 대상:
  - format tabs
  - markdown textarea
  - title input
  - author input
  - strict mode toggle
  - generate button
  - status
  - note

### Tabs

- active = filled blue
- inactive = neutral ghost
- uppercase label
- tab label은 한 단어 또는 짧은 약어만 허용

### Buttons

- variants:
  - primary
  - secondary
  - ghost
- primary는 blue fill
- secondary는 neutral tint
- ghost는 borderless text action

## Content Hierarchy

### First Screen Order

1. logo
2. one-line value proposition
3. short supporting sentence
4. primary install action
5. compact proof strip

install proof line은 comparison strip보다 먼저 읽히는 핵심 설치 유도 문장으로 배치한다.

### Comparison Handling

- comparison은 landing에서 제거하지 않는다.
- 하지만 역할은 `compact support proof`로만 제한한다.
- `Choose docxly / Choose Pandoc` 장문 카피는 기본 landing에서 제거 대상이다.
- feature matrix와 장문 포지셔닝 설명은 docs 또는 하단 secondary content로 이동한다.

## Do / Don't

### Do

- hero에서 한 가지 핵심 행동만 강조한다.
- proof는 숫자 중심으로 압축한다.
- blue tokens만으로 CTA와 active state를 통일한다.
- generator panel을 가장 중요한 작업 영역으로 유지한다.
- 로고를 headline과 별도 계층으로 분리한다.

### Don't

- 상단에 독립 강조 카드 3개 이상 배치하지 않는다.
- comparison table을 landing first screen에 노출하지 않는다.
- warm/orange palette를 primary accent로 사용하지 않는다.
- 긴 비교 카피를 generator보다 먼저 배치하지 않는다.
- install card와 comparison strip을 같은 강도의 경쟁 블록으로 만들지 않는다.

## Acceptance Criteria

이 문서만 읽고 구현자는 추가 질문 없이 다음을 수행할 수 있어야 한다.

- CSS custom property 정의
- hero 구조 재배치
- install card / compact proof strip / generator panel 위계 적용
- 로고 자산 도입 전 fallback lockup 구현

추가 완료 조건:

- 모바일 `390x844` 기준 first screen에 `logo + headline + install action`이 모두 보여야 한다는 기준이 명시돼 있어야 한다.
- comparison은 `support proof`로만 남고, 장문 설명과 표는 기본 landing에서 제거 대상으로 명시돼 있어야 한다.
- primary accent가 blue token family로 통일된다고 명시돼 있어야 한다.

## Migration Notes

현재 데모 UI에서 제거 또는 축소해야 하는 요소:

- warm/orange 중심 accent
- landing 상단의 장문 comparison explanation
- feature matrix table
- dual comparison choice blocks
- hero에서 경쟁하는 다중 강조 카드

새 UI로 옮길 때 유지해야 하는 요소:

- install command 복사 흐름
- summary benchmark proof
- HWPX/DOCX format switching
- browser-local generation 메시지
