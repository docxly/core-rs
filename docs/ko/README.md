<div align="center">
  <img src="../../packages/npm-core-rs/demo/assets/logo.png" alt="docxly logo" width="96">
</div>

# docxly 문서

`docxly`는 애플리케이션 안에서 DOCX와 HWPX를 직접 만들 수 있게 해주는 Rust/WASM 기반 문서 생성 엔진입니다.

Pandoc이 다양한 형식 사이를 변환하는 범용 도구라면, `docxly`는 Node 서비스, 브라우저 워크플로, 제품 UI에 바로 넣어 쓸 수 있는 라이브러리 경험에 초점을 둡니다.

## 시작하기

- [빠른 시작](./getting-started.md)
- [런타임 가이드](./runtime.md)
- [성능 비교와 포지셔닝](./benchmark.md)
- [디자인 시스템](./design-system.md)

## 핵심 수치

- 현재 요약 벤치마크 기준 `docxly`는 `80 ms cold / 2 ms steady`
- 같은 코퍼스에서 Pandoc은 `284 ms cold / 210 ms steady`
- 반복 실행 구간에서는 `105x` 빠른 수치가 측정됐습니다.

## 언제 docxly를 선택해야 하나

- 문서 생성을 별도 CLI 프로세스가 아니라 애플리케이션 내부 기능으로 넣고 싶을 때
- Node와 브라우저에서 같은 Rust 코어를 재사용하고 싶을 때
- DOCX뿐 아니라 HWPX까지 같은 저장소와 API 흐름으로 확장하고 싶을 때
- 예측 가능한 출력과 fixture 기반 검증 체계가 필요할 때

## 바로 가기

- [루트 README](../../README.md)
- [npm package README](../../packages/npm-core-rs/README.md)
- [라이브 데모](https://docxly.github.io/core-rs/)
