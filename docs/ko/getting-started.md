# 빠른 시작

## 설치

가장 빠른 시작 경로는 npm 패키지입니다.

```bash
npm install @docxly/core-rs
```

Rust crate는 이 저장소의 워크스페이스에서 바로 사용할 수 있습니다.

```toml
[dependencies]
core-rs = { path = "packages/core-rs" }
```

## Node 예제

```js
import { writeFile } from "node:fs/promises";
import { generateDocx } from "@docxly/core-rs";

const bytes = await generateDocx("# Hello\n\nThis is **docxly**.");
await writeFile("output.docx", bytes);
```

## Rust 예제

```rust
use std::fs;

use core_rs::{DocxOptions, generate_docx};

let docx = generate_docx("# Hello\n\nThis is **docxly**.", DocxOptions::default())?;
fs::write("output.docx", docx)?;
```

## 로컬 데모 실행

저장소 루트에서 아래 명령을 실행합니다.

```bash
npm install
npm run demo
```

정적 Pages 산출물만 생성하려면:

```bash
npm run build:pages
```
