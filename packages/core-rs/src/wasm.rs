use base64::Engine;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::{
    ConversionTarget, DocxOptions, GenerationFailure, GenerationResult, HwpxOptions,
    HwpxParagraphAlign, HwpxStyleOptions, analyze_markdown, generate_docx,
    generate_docx_with_report, generate_hwpx, generate_hwpx_with_report,
};

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
    style_json: Option<String>,
) -> Result<Vec<u8>, JsValue> {
    generate_hwpx(
        markdown,
        parse_hwpx_options(title, author, strict_mode, style_json)?,
    )
    .map_err(|error| JsValue::from_str(&error.to_string()))
}

#[wasm_bindgen(js_name = analyzeMarkdownJson)]
pub fn analyze_markdown_json(markdown: &str, target: &str) -> Result<String, JsValue> {
    let target = match target {
        "docx" => ConversionTarget::Docx,
        "hwpx" => ConversionTarget::Hwpx,
        _ => return Err(JsValue::from_str("target must be `docx` or `hwpx`")),
    };
    serde_json::to_string(&analyze_markdown(markdown, target))
        .map_err(|error| JsValue::from_str(&error.to_string()))
}

#[derive(Serialize)]
struct WasmGenerationResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    bytes_base64: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    report: crate::ConversionReport,
}

#[wasm_bindgen(js_name = generateDocxWithReportJson)]
pub fn generate_docx_with_report_json(
    markdown: &str,
    title: Option<String>,
    author: Option<String>,
    strict_mode: bool,
) -> Result<String, JsValue> {
    let payload = match generate_docx_with_report(
        markdown,
        DocxOptions {
            title,
            author,
            strict_mode,
        },
    ) {
        Ok(result) => WasmGenerationResponse::success(result),
        Err(failure) => WasmGenerationResponse::failure(failure),
    };
    serde_json::to_string(&payload).map_err(|error| JsValue::from_str(&error.to_string()))
}

#[wasm_bindgen(js_name = generateHwpxWithReportJson)]
pub fn generate_hwpx_with_report_json(
    markdown: &str,
    title: Option<String>,
    author: Option<String>,
    strict_mode: bool,
    style_json: Option<String>,
) -> Result<String, JsValue> {
    let payload = match generate_hwpx_with_report(
        markdown,
        parse_hwpx_options(title, author, strict_mode, style_json)?,
    ) {
        Ok(result) => WasmGenerationResponse::success(result),
        Err(failure) => WasmGenerationResponse::failure(failure),
    };
    serde_json::to_string(&payload).map_err(|error| JsValue::from_str(&error.to_string()))
}

impl WasmGenerationResponse {
    fn success(result: GenerationResult) -> Self {
        Self {
            ok: true,
            bytes_base64: Some(base64::engine::general_purpose::STANDARD.encode(result.bytes)),
            error: None,
            report: result.report,
        }
    }

    fn failure(failure: GenerationFailure) -> Self {
        Self {
            ok: false,
            bytes_base64: None,
            error: Some(failure.error.to_string()),
            report: failure.report,
        }
    }
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WasmHwpxStyleOptions {
    body_font: Option<String>,
    heading_font: Option<String>,
    body_font_size: Option<u32>,
    heading_font_size: Option<u32>,
    text_color: Option<String>,
    heading_color: Option<String>,
    link_color: Option<String>,
    paragraph_align: Option<String>,
}

fn parse_hwpx_options(
    title: Option<String>,
    author: Option<String>,
    strict_mode: bool,
    style_json: Option<String>,
) -> Result<HwpxOptions, JsValue> {
    Ok(HwpxOptions {
        title,
        author,
        strict_mode,
        style: parse_hwpx_style(style_json)?,
    })
}

fn parse_hwpx_style(style_json: Option<String>) -> Result<HwpxStyleOptions, JsValue> {
    let Some(style_json) = style_json else {
        return Ok(HwpxStyleOptions::default());
    };

    let style = serde_json::from_str::<WasmHwpxStyleOptions>(&style_json)
        .map_err(|error| JsValue::from_str(&format!("invalid HWPX style options: {error}")))?;

    Ok(HwpxStyleOptions {
        body_font: style.body_font,
        heading_font: style.heading_font,
        body_font_size: style.body_font_size,
        heading_font_size: style.heading_font_size,
        text_color: style.text_color,
        heading_color: style.heading_color,
        link_color: style.link_color,
        paragraph_align: style
            .paragraph_align
            .map(|value| parse_hwpx_paragraph_align(&value))
            .transpose()?,
    })
}

fn parse_hwpx_paragraph_align(value: &str) -> Result<HwpxParagraphAlign, JsValue> {
    match value.to_ascii_lowercase().as_str() {
        "left" => Ok(HwpxParagraphAlign::Left),
        "center" => Ok(HwpxParagraphAlign::Center),
        "right" => Ok(HwpxParagraphAlign::Right),
        "justify" => Ok(HwpxParagraphAlign::Justify),
        other => Err(JsValue::from_str(&format!(
            "invalid HWPX paragraph alignment: {other}"
        ))),
    }
}
