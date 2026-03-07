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
}

impl Default for HwpxOptions {
    fn default() -> Self {
        let (title, author, strict_mode) = default_document_options();
        Self {
            title,
            author,
            strict_mode,
        }
    }
}

pub fn generate_docx(markdown: &str, options: DocxOptions) -> Result<Vec<u8>, CoreRsError> {
    let document = MarkdownParser::new(options.strict_mode).parse(markdown)?;
    DocxGenerator::new(options).generate(&document)
}

pub fn generate_hwpx(markdown: &str, options: HwpxOptions) -> Result<Vec<u8>, CoreRsError> {
    let document = MarkdownParser::new(options.strict_mode).parse(markdown)?;
    HwpxGenerator::new(options).generate(&document)
}
