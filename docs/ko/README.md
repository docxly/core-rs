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
- [로드맵](./roadmap.md)
- [지원 매트릭스](./support-matrix.md)
- [커뮤니티와 기여](./community.md)
- [자주 묻는 질문](./faq.md)

## 핵심 수치

- 최신 benchmark 수치는 루트 README의 comparison block과 npm comparison dataset을 기준으로 함께 갱신됩니다.
- 고정 수치만 따로 인용하기보다, 같은 저장소에 커밋된 비교 데이터와 caveat를 함께 보는 것이 안전합니다.
- 현재 benchmark는 DOCX generation only, offline Node 환경 기준입니다.

## 언제 docxly를 선택해야 하나

- 문서 생성을 별도 CLI 프로세스가 아니라 애플리케이션 내부 기능으로 넣고 싶을 때
- Node와 브라우저에서 같은 Rust 코어를 재사용하고 싶을 때
- DOCX뿐 아니라 HWPX까지 같은 저장소와 API 흐름으로 확장하고 싶을 때
- 예측 가능한 출력과 fixture 기반 검증 체계가 필요할 때

## 도입 판단 기준

- 가장 안전한 첫 도입 경로: npm DOCX
- 브라우저 제품에 바로 넣고 싶을 때: npm DOCX + 번들러
- HWPX가 필요할 때: 가능하지만 현재는 베타 계약으로 접근

## 초기 검토 체크리스트

- 서비스나 브라우저 안에서 바로 문서를 생성해야 한다
- 별도 Pandoc 프로세스를 띄우고 싶지 않다
- DOCX가 우선 요구사항이다
- HWPX가 필요하다면 베타 범위를 감수할 수 있다
- fixture와 실제 산출물 검증 흐름을 받아들일 수 있다

위 항목에 대부분 해당하면 `docxly`가 현재 요구사항에 적합할 가능성이 높습니다.

## 바로 가기

- [루트 README](../../README.md)
- [npm package README](../../packages/npm-core-rs/README.md)
- [기여 가이드](../../CONTRIBUTING.md)
- [행동 강령](../../CODE_OF_CONDUCT.md)
- [보안 정책](../../SECURITY.md)
- [한국어 커뮤니티 안내](./community.md)
- [한국어 FAQ](./faq.md)
- [라이브 데모](https://docxly.github.io/core-rs/)
