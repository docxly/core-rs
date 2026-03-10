<div align="center">
  <img src="../../packages/npm-core-rs/demo/assets/logo.png" alt="docxly logo" width="96">
</div>

# docxly 문서

`docxly`는 DOCX와 HWPX 생성을 애플리케이션 내부에 직접 내장하기 위한 Rust/WASM 문서 생성 엔진입니다.

Pandoc이 범용 문서 변환기라면, `docxly`는 Node 서비스, 브라우저 워크플로, 제품 UI 안에 직접 들어가는 라이브러리 경로에 초점을 맞춥니다.

## 시작하기

- [빠른 시작](./getting-started.md)
- [런타임 가이드](./runtime.md)
- [성능 비교와 포지셔닝](./benchmark.md)
- [디자인 시스템](./design-system.md)

## 핵심 수치

- 현재 summary benchmark 기준 `docxly`는 `80 ms cold / 2 ms steady`
- 같은 코퍼스에서 Pandoc은 `284 ms cold / 210 ms steady`
- steady 기준으로 `105x` 빠른 수치가 측정됨

## 언제 docxly를 선택해야 하나

- 문서 생성을 별도 CLI 프로세스가 아니라 애플리케이션 내부 라이브러리로 붙이고 싶을 때
- Node와 브라우저에서 같은 Rust 코어를 재사용하고 싶을 때
- DOCX뿐 아니라 HWPX까지 같은 저장소와 API 흐름으로 확장하고 싶을 때
- 테스트 가능한 결정적 출력과 fixture 기반 검증이 필요할 때

## 바로 가기

- [루트 README](../../README.md)
- [npm package README](../../packages/npm-core-rs/README.md)
- [라이브 데모](https://docxly.github.io/core-rs/)
