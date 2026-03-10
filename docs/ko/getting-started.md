# 빠른 시작

## 설치

가장 빠르게 시작하는 방법은 npm 패키지를 사용하는 것입니다.

```bash
npm install @docxly/core-rs
```

Rust crate는 이 저장소의 워크스페이스에서 바로 참조할 수 있습니다.

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

저장소 루트에서 아래 명령을 실행하면 로컬 데모를 바로 띄울 수 있습니다.

```bash
npm install
npm run demo
```

GitHub Pages용 정적 산출물만 만들려면 다음 명령을 사용합니다.

```bash
npm run build:pages
```
