use wasm_bindgen::prelude::*;

use crate::{DocxOptions, generate_docx};

#[wasm_bindgen(js_name = generateDocxBytes)]
pub fn generate_docx_bytes(
    markdown: &str,
    title: Option<String>,
    author: Option<String>,
    strict_mode: bool,
) -> Result<Vec<u8>, JsValue> {
    generate_docx(
        markdown,
        DocxOptions {
            title,
            author,
            strict_mode,
        },
    )
    .map_err(|error| JsValue::from_str(&error.to_string()))
}
