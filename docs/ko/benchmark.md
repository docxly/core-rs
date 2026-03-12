# 성능 비교와 포지셔닝

## 요약

`docxly`는 Pandoc을 대체하는 또 하나의 CLI가 아니라, 애플리케이션 안에 문서 생성을 직접 넣기 위한 라이브러리입니다.

- `docxly`: 제품 안에 넣어 쓰는 문서 생성 엔진
- `Pandoc`: 다양한 포맷 변환에 강한 범용 변환기

## 현재 요약 벤치마크

- 최신 수치는 루트 README의 comparison block과 `packages/npm-core-rs/demo/comparison-data.json`을 기준으로 봅니다.
- 이 문서에는 고정 숫자를 복제하지 않고, 어떤 환경에서 어떤 의미의 수치인지 해석만 남깁니다.

측정 환경은 다음과 같습니다.

- 환경: `darwin 25.2.0 / arm64`
- Node: `v23.7.0`
- Pandoc: `3.9`
- 기준: 복잡한 DOCX 벤치마크 코퍼스
- 범위: DOCX generation only

## 해석

- cold start는 초기화 비용을 포함합니다.
  - `docxly`: WASM initialization 포함
  - `Pandoc`: process startup 포함
- steady median은 warm-up 이후 반복 생성 비용을 뜻합니다.
- 이 수치는 브라우저 실측이 아니라 오프라인 Node 벤치마크 결과입니다.
- HWPX 비교나 브라우저 런타임 수치로 일반화하면 안 됩니다.

## 언제 Pandoc이 더 적합한가

- 폭넓은 문서 포맷 변환이 필요할 때
- CLI 중심 출판 파이프라인이 있을 때
- `reference.docx` 기반 커스터마이징 흐름이 필요할 때

## 언제 docxly가 더 적합한가

- 앱 안에 문서 생성 기능을 직접 넣어야 할 때
- Node와 브라우저에서 같은 코어를 재사용해야 할 때
- HWPX까지 같은 제품 흐름 안에서 함께 다뤄야 할 때
- 별도 프로세스 의존성 없이 라이브러리 형태로 배포하고 싶을 때
