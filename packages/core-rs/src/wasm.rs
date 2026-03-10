use wasm_bindgen::prelude::*;

use crate::{DocxOptions, HwpxOptions, generate_docx, generate_hwpx};

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

#[wasm_bindgen(js_name = generateHwpxBytes)]
pub fn generate_hwpx_bytes(
    markdown: &str,
    title: Option<String>,
    author: Option<String>,
    strict_mode: bool,
) -> Result<Vec<u8>, JsValue> {
    generate_hwpx(
        markdown,
        HwpxOptions {
            title,
            author,
            strict_mode,
            style: Default::default(),
        },
    )
    .map_err(|error| JsValue::from_str(&error.to_string()))
}
