mod error;
mod generators;
mod models;
mod parser;
mod utils;

#[cfg(target_arch = "wasm32")]
mod wasm;

pub use error::CoreRsError;

use generators::Generator;
use generators::docx::DocxGenerator;
use generators::hwpx::HwpxGenerator;
use parser::markdown::MarkdownParser;

fn default_document_options() -> (Option<String>, Option<String>, bool) {
    (None, None, true)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HwpxParagraphAlign {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HwpxStyleOptions {
    pub body_font: Option<String>,
    pub heading_font: Option<String>,
    pub body_font_size: Option<u32>,
    pub heading_font_size: Option<u32>,
    pub text_color: Option<String>,
    pub heading_color: Option<String>,
    pub link_color: Option<String>,
    pub paragraph_align: Option<HwpxParagraphAlign>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocxOptions {
    pub title: Option<String>,
    pub author: Option<String>,
    pub strict_mode: bool,
}

impl Default for DocxOptions {
    fn default() -> Self {
        let (title, author, strict_mode) = default_document_options();
        Self {
            title,
            author,
            strict_mode,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HwpxOptions {
    pub title: Option<String>,
    pub author: Option<String>,
    pub strict_mode: bool,
    pub style: HwpxStyleOptions,
}

impl Default for HwpxOptions {
    fn default() -> Self {
        let (title, author, strict_mode) = default_document_options();
        Self {
            title,
            author,
            strict_mode,
            style: HwpxStyleOptions::default(),
        }
    }
}

pub fn generate_docx(markdown: &str, options: DocxOptions) -> Result<Vec<u8>, CoreRsError> {
    let document = MarkdownParser::new(options.strict_mode).parse(markdown)?;
    DocxGenerator::new(options).generate(&document)
}

pub fn generate_hwpx(markdown: &str, options: HwpxOptions) -> Result<Vec<u8>, CoreRsError> {
    let normalized_markdown;
    let markdown = if options.strict_mode {
        markdown
    } else {
        normalized_markdown = normalize_hwpx_markdown(markdown);
        normalized_markdown.as_str()
    };
    let document = MarkdownParser::new(options.strict_mode).parse(markdown)?;
    HwpxGenerator::new(options).generate(&document)
}

fn normalize_hwpx_markdown(markdown: &str) -> String {
    let mut normalized = String::with_capacity(markdown.len());
    for segment in markdown.split_inclusive('\n') {
        let (line, newline) = if let Some(line) = segment.strip_suffix('\n') {
            (line, "\n")
        } else {
            (segment, "")
        };

        let line = if line.starts_with("  ") && is_ordered_list_marker(&line[2..]) {
            format!(" {line}")
        } else {
            line.to_string()
        };

        normalized.push_str(&line);
        normalized.push_str(newline);
    }
    normalized
}

fn is_ordered_list_marker(value: &str) -> bool {
    let digits = value
        .bytes()
        .take_while(|byte| byte.is_ascii_digit())
        .count();
    digits > 0
        && value
            .as_bytes()
            .get(digits..digits + 2)
            .is_some_and(|suffix| suffix == b". ")
}
