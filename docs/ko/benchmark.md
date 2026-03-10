# 성능 비교와 포지셔닝

## 요약

`docxly`는 Pandoc의 대체 CLI가 아니라, 앱 내부에 문서 생성을 임베드하기 위한 라이브러리 경로를 제공합니다.

- `docxly`: embeddable generation engine
- `Pandoc`: general-purpose converter

## 현재 summary benchmark

- `docxly`: `80 ms cold / 2 ms steady`
- `Pandoc`: `284 ms cold / 210 ms steady`
- steady median 기준 `105x`

측정 조건:

- 환경: `darwin 25.2.0 / arm64`
- Node: `v23.7.0`
- Pandoc: `3.9`
- 기준: complex DOCX benchmark corpus

## 해석

- cold start는 초기화 비용을 포함합니다.
  - `docxly`: WASM initialization 포함
  - `Pandoc`: process startup 포함
- steady median은 warm-up 이후 반복 생성 비용을 의미합니다.
- 이 수치는 브라우저 실측이 아니라 offline Node benchmark입니다.

## 언제 Pandoc이 더 적합한가

- 폭넓은 문서 포맷 변환이 필요할 때
- CLI 중심 출판 파이프라인이 있을 때
- `reference.docx` 기반 커스터마이징 흐름이 필요할 때

## 언제 docxly가 더 적합한가

- 앱 내부에 직접 문서 생성 기능을 붙여야 할 때
- Node와 브라우저에서 같은 코어를 재사용해야 할 때
- HWPX까지 같은 제품 흐름 안에서 다뤄야 할 때
- 별도 프로세스 의존성 없이 라이브러리 형태로 배포하고 싶을 때
