# 빠른 시작

## 설치

가장 빠르게 시작하는 방법은 npm 패키지를 사용하는 것입니다.

```bash
npm install @docxly/core-rs
```

처음 평가할 때는 npm DOCX 경로부터 확인하는 편이 좋습니다.

## 초기 성공 경로

가장 권장하는 첫 성공 경로는 npm DOCX입니다.

1. `npm install @docxly/core-rs`
2. 아래 Node 또는 브라우저 예제를 그대로 실행
3. 실제 파일이 열리는지 확인
4. 그 다음에만 HWPX 또는 Rust 경로로 확장

## Node 예제

```js
import { writeFile } from "node:fs/promises";
import { generateDocx } from "@docxly/core-rs";

const bytes = await generateDocx("# Hello\n\nThis is **docxly**.");
await writeFile("output.docx", bytes);
```

## 브라우저 예제

브라우저에서는 `.wasm` 자산을 내보낼 수 있는 번들러가 필요합니다.

```js
import { generateDocx } from "@docxly/core-rs";

const bytes = await generateDocx("# 브라우저에서 만든 문서");
const blob = new Blob([bytes], {
  type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
});
```

다운로드까지 확인하려면:

```js
const url = URL.createObjectURL(blob);
const anchor = document.createElement("a");
anchor.href = url;
anchor.download = "docxly-browser.docx";
anchor.click();
URL.revokeObjectURL(url);
```

## HWPX 빠른 메모

- `generateHwpx`는 npm과 Rust에서 모두 사용할 수 있습니다.
- 다만 HWPX는 현재 베타이며, DOCX보다 좁은 승인 fixture 기준 안에서 사용하는 것이 안전합니다.
- 기본 프로덕션 진입은 DOCX부터 시작하는 편이 좋습니다.

간단한 HWPX 예제:

```js
import { generateHwpx } from "@docxly/core-rs";

const bytes = await generateHwpx("# 제목\n\n본문 **강조**");
```

## Rust 예제

Rust 통합이 필요하다면 이 저장소의 워크스페이스 크레이트를 바로 참조할 수 있습니다.

```toml
[dependencies]
core-rs = { path = "packages/core-rs" }
```

```rust
use std::fs;

use core_rs::{DocxOptions, generate_docx};

let docx = generate_docx("# Hello\n\nThis is **docxly**.", DocxOptions::default())?;
fs::write("output.docx", docx)?;
```

## 로컬 데모 실행

저장소 루트에서 아래 명령을 실행하면 로컬 데모를 바로 띄울 수 있습니다.

```bash
npm install
npm run demo
```

GitHub Pages용 정적 산출물만 만들려면 다음 명령을 사용합니다.

```bash
npm run build:pages
```

다음 단계:

- 지원 범위 확인: [지원 매트릭스](./support-matrix.md)
- 런타임 제약 확인: [런타임 가이드](./runtime.md)
- 기여 흐름 확인: [커뮤니티와 기여](./community.md)
