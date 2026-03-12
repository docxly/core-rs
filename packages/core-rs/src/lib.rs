mod diagnostics;
mod error;
mod generators;
mod models;
mod parser;
mod utils;

#[cfg(target_arch = "wasm32")]
mod wasm;

pub use error::CoreRsError;

use diagnostics::{analyze_markdown_impl, report_for_mode};
use generators::Generator;
use generators::docx::DocxGenerator;
use generators::hwpx::HwpxGenerator;
use parser::markdown::MarkdownParser;
use serde::Serialize;
use thiserror::Error;

fn default_document_options() -> (Option<String>, Option<String>, bool) {
    (None, None, true)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ConversionTarget {
    Docx,
    Hwpx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum IssueSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConversionIssue {
    pub feature: String,
    pub message: String,
    pub severity: IssueSeverity,
    pub degraded: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConversionReport {
    pub issues: Vec<ConversionIssue>,
    pub degraded: bool,
    pub unsupported_count: usize,
    pub fallback_count: usize,
}

impl ConversionReport {
    pub(crate) fn from_issues(issues: Vec<ConversionIssue>) -> Self {
        let mut deduped = Vec::with_capacity(issues.len());
        for issue in issues {
            if !deduped.contains(&issue) {
                deduped.push(issue);
            }
        }

        let unsupported_count = deduped
            .iter()
            .filter(|issue| issue.severity == IssueSeverity::Error)
            .count();
        let fallback_count = deduped.iter().filter(|issue| issue.degraded).count();
        let degraded = deduped.iter().any(|issue| issue.degraded);
        Self {
            issues: deduped,
            degraded,
            unsupported_count,
            fallback_count,
        }
    }

    pub(crate) fn with_error_issue(&self, error: &CoreRsError) -> Self {
        let mut issues = self.issues.clone();
        if let Some(existing) = issues
            .iter_mut()
            .find(|existing| issue_matches_error(existing, error))
        {
            existing.message = error.to_string();
            existing.severity = IssueSeverity::Error;
            existing.degraded = false;
            return Self::from_issues(issues);
        }

        issues.push(error_issue(error));
        Self::from_issues(issues)
    }
}

fn error_issue(error: &CoreRsError) -> ConversionIssue {
    match error {
        CoreRsError::UnsupportedFeature(_) => ConversionIssue {
            feature: "unsupported feature".to_string(),
            message: error.to_string(),
            severity: IssueSeverity::Error,
            degraded: false,
        },
        CoreRsError::InvalidMarkdown(_) => ConversionIssue {
            feature: "invalid markdown".to_string(),
            message: error.to_string(),
            severity: IssueSeverity::Error,
            degraded: false,
        },
        CoreRsError::InvalidHwpx(_) => ConversionIssue {
            feature: "invalid HWPX".to_string(),
            message: error.to_string(),
            severity: IssueSeverity::Error,
            degraded: false,
        },
        CoreRsError::InvalidOption(_) => ConversionIssue {
            feature: "invalid option".to_string(),
            message: error.to_string(),
            severity: IssueSeverity::Error,
            degraded: false,
        },
        CoreRsError::Zip(_) => ConversionIssue {
            feature: "zip packaging failed".to_string(),
            message: error.to_string(),
            severity: IssueSeverity::Error,
            degraded: false,
        },
        CoreRsError::Io(_) => ConversionIssue {
            feature: "io failure".to_string(),
            message: error.to_string(),
            severity: IssueSeverity::Error,
            degraded: false,
        },
    }
}

fn issue_matches_error(issue: &ConversionIssue, error: &CoreRsError) -> bool {
    match error {
        CoreRsError::UnsupportedFeature(feature) => match feature.as_str() {
            "html" => issue.feature == "HTML",
            "footnote" => issue.feature == "footnote",
            "task list" => issue.feature == "task list",
            "math" => issue.feature == "math",
            "nested list depth > 2" => issue.feature == "deep nested list",
            value if value.contains("ordered list") => {
                issue.feature == "HWPX ordered list in strict mode"
            }
            "HWPX core does not support image" => issue.feature == "HWPX image",
            value if value.starts_with("HWPX core does not support ") => {
                issue.feature == "HWPX unsupported rich block"
            }
            _ => false,
        },
        CoreRsError::InvalidMarkdown(_)
        | CoreRsError::InvalidHwpx(_)
        | CoreRsError::InvalidOption(_)
        | CoreRsError::Zip(_)
        | CoreRsError::Io(_) => false,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GenerationResult {
    pub bytes: Vec<u8>,
    pub report: ConversionReport,
}

#[derive(Debug, Error, Serialize)]
#[error("{error}")]
pub struct GenerationFailure {
    #[serde(skip_serializing)]
    pub error: CoreRsError,
    pub report: ConversionReport,
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

pub fn analyze_markdown(markdown: &str, target: ConversionTarget) -> ConversionReport {
    analyze_markdown_impl(markdown, target)
}

pub fn generate_docx_with_report(
    markdown: &str,
    options: DocxOptions,
) -> Result<GenerationResult, GenerationFailure> {
    let report = report_for_mode(
        &analyze_markdown(markdown, ConversionTarget::Docx),
        ConversionTarget::Docx,
        options.strict_mode,
    );
    match generate_docx(markdown, options) {
        Ok(bytes) => Ok(GenerationResult { bytes, report }),
        Err(error) => Err(GenerationFailure {
            report: report.with_error_issue(&error),
            error,
        }),
    }
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

pub fn generate_hwpx_with_report(
    markdown: &str,
    options: HwpxOptions,
) -> Result<GenerationResult, GenerationFailure> {
    let report = report_for_mode(
        &analyze_markdown(markdown, ConversionTarget::Hwpx),
        ConversionTarget::Hwpx,
        options.strict_mode,
    );
    match generate_hwpx(markdown, options) {
        Ok(bytes) => Ok(GenerationResult { bytes, report }),
        Err(error) => Err(GenerationFailure {
            report: report.with_error_issue(&error),
            error,
        }),
    }
}

#[doc(hidden)]
pub fn debug_parse_hwpx(bytes: &[u8]) -> Result<String, CoreRsError> {
    parser::hwpx::debug_dump(bytes)
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
