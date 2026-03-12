use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

use crate::models::block::Block;
use crate::models::document::Document;
use crate::models::inline::Inline;
use crate::parser::markdown::MarkdownParser;
use crate::parser::markdown::data_image::parse_data_uri;
use crate::{ConversionIssue, ConversionReport, ConversionTarget, IssueSeverity};

const HTML_FEATURE: &str = "HTML";
const FOOTNOTE_FEATURE: &str = "footnote";
const TASK_LIST_FEATURE: &str = "task list";
const MATH_FEATURE: &str = "math";
const NON_DATA_IMAGE_FEATURE: &str = "non-data image";
const UNSUPPORTED_IMAGE_MIME_FEATURE: &str = "unsupported image mime type";
const DEEP_NESTED_LIST_FEATURE: &str = "deep nested list";
const HWPX_ORDERED_LIST_FEATURE: &str = "HWPX ordered list in strict mode";
const HWPX_IMAGE_FEATURE: &str = "HWPX image";
const HWPX_UNSUPPORTED_RICH_BLOCK_FEATURE: &str = "HWPX unsupported rich block";

pub(crate) fn analyze_markdown_impl(markdown: &str, target: ConversionTarget) -> ConversionReport {
    let mut issues = collect_parser_issues(markdown);
    if let (ConversionTarget::Hwpx, Ok(document)) =
        (target, MarkdownParser::new(false).parse(markdown))
    {
        collect_hwpx_issues_from_document(&document, &mut issues);
    }
    ConversionReport::from_issues(issues)
}

pub(crate) fn report_for_mode(
    report: &ConversionReport,
    target: ConversionTarget,
    strict_mode: bool,
) -> ConversionReport {
    if !strict_mode {
        return report.clone();
    }

    let issues = report
        .issues
        .iter()
        .cloned()
        .map(|mut issue| {
            if is_strict_error_feature(target, &issue.feature) {
                issue.severity = IssueSeverity::Error;
                issue.degraded = false;
            }
            issue
        })
        .collect();
    ConversionReport::from_issues(issues)
}

fn collect_parser_issues(markdown: &str) -> Vec<ConversionIssue> {
    let parser = Parser::new_ext(markdown, Options::all());
    let mut issues = Vec::new();
    let mut list_depth = 0usize;
    let mut image_urls = Vec::new();

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::List(_) => {
                    if list_depth >= 2 {
                        issues.push(degraded_issue(
                            DEEP_NESTED_LIST_FEATURE,
                            "nested lists deeper than 2 fall back to plain text in compat mode",
                        ));
                    }
                    list_depth += 1;
                }
                Tag::Image { dest_url, .. } => image_urls.push(dest_url.into_string()),
                Tag::FootnoteDefinition(_) => issues.push(degraded_issue(
                    FOOTNOTE_FEATURE,
                    "footnotes fall back to visible text in compat mode",
                )),
                _ => {}
            },
            Event::End(TagEnd::List(_)) => {
                list_depth = list_depth.saturating_sub(1);
            }
            Event::End(TagEnd::Image) => {
                if let Some(url) = image_urls.pop() {
                    match parse_data_uri(&url) {
                        Ok(_) => {}
                        Err(error) => match error {
                            crate::parser::markdown::data_image::DataImageError::UnsupportedMime {
                                ..
                            } => issues.push(degraded_issue(
                                UNSUPPORTED_IMAGE_MIME_FEATURE,
                                &error.into_core_error().to_string(),
                            )),
                            _ => issues.push(degraded_issue(
                                NON_DATA_IMAGE_FEATURE,
                                &error.into_core_error().to_string(),
                            )),
                        },
                    }
                }
            }
            Event::Html(_) | Event::InlineHtml(_) => issues.push(degraded_issue(
                HTML_FEATURE,
                "HTML falls back to literal text in compat mode",
            )),
            Event::FootnoteReference(_) => issues.push(degraded_issue(
                FOOTNOTE_FEATURE,
                "footnotes fall back to visible text in compat mode",
            )),
            Event::TaskListMarker(_) => issues.push(degraded_issue(
                TASK_LIST_FEATURE,
                "task lists fall back to visible markers in compat mode",
            )),
            Event::InlineMath(_) | Event::DisplayMath(_) => issues.push(degraded_issue(
                MATH_FEATURE,
                "math falls back to visible text in compat mode",
            )),
            _ => {}
        }
    }

    issues
}

fn collect_hwpx_issues_from_document(document: &Document, issues: &mut Vec<ConversionIssue>) {
    collect_hwpx_block_issues(&document.blocks, issues);
}

fn collect_hwpx_block_issues(blocks: &[Block], issues: &mut Vec<ConversionIssue>) {
    for block in blocks {
        match block {
            Block::Paragraph(inlines) => collect_hwpx_inline_issues(inlines, issues),
            Block::Heading { content, .. } => collect_hwpx_inline_issues(content, issues),
            Block::BlockQuote(inner) => collect_hwpx_block_issues(inner, issues),
            Block::CodeBlock { .. } => issues.push(degraded_issue(
                HWPX_UNSUPPORTED_RICH_BLOCK_FEATURE,
                "HWPX strict mode does not support code blocks; compat mode falls back to visible text",
            )),
            Block::ThematicBreak => issues.push(degraded_issue(
                HWPX_UNSUPPORTED_RICH_BLOCK_FEATURE,
                "HWPX strict mode does not support thematic breaks; compat mode falls back to visible text",
            )),
            Block::List(list) => {
                if list.ordered {
                    issues.push(non_degraded_issue(
                        HWPX_ORDERED_LIST_FEATURE,
                        "HWPX strict mode rejects ordered lists; compat mode uses the semantic ordered-list contract",
                    ));
                }
                for item in &list.items {
                    collect_hwpx_block_issues(&item.blocks, issues);
                }
            }
            Block::Table(table) => {
                for cell in &table.headers {
                    collect_hwpx_inline_issues(cell, issues);
                }
                for row in &table.rows {
                    for cell in &row.cells {
                        collect_hwpx_inline_issues(cell, issues);
                    }
                }
            }
        }
    }
}

fn collect_hwpx_inline_issues(inlines: &[Inline], issues: &mut Vec<ConversionIssue>) {
    for inline in inlines {
        match inline {
            Inline::Emphasis(children) | Inline::Strong(children) => {
                collect_hwpx_inline_issues(children, issues);
            }
            Inline::Link { text, .. } => collect_hwpx_inline_issues(text, issues),
            Inline::Image(_) => issues.push(degraded_issue(
                HWPX_IMAGE_FEATURE,
                "HWPX strict mode does not support images; compat mode falls back to visible alt text",
            )),
            Inline::Text(_) | Inline::Code(_) | Inline::HardBreak => {}
        }
    }
}

fn degraded_issue(feature: &str, message: &str) -> ConversionIssue {
    ConversionIssue {
        feature: feature.to_string(),
        message: message.to_string(),
        severity: IssueSeverity::Warning,
        degraded: true,
    }
}

fn non_degraded_issue(feature: &str, message: &str) -> ConversionIssue {
    ConversionIssue {
        feature: feature.to_string(),
        message: message.to_string(),
        severity: IssueSeverity::Warning,
        degraded: false,
    }
}

fn is_strict_error_feature(target: ConversionTarget, feature: &str) -> bool {
    matches!(
        feature,
        HTML_FEATURE
            | FOOTNOTE_FEATURE
            | TASK_LIST_FEATURE
            | MATH_FEATURE
            | NON_DATA_IMAGE_FEATURE
            | UNSUPPORTED_IMAGE_MIME_FEATURE
            | DEEP_NESTED_LIST_FEATURE
    ) || matches!(target, ConversionTarget::Hwpx)
        && matches!(
            feature,
            HWPX_ORDERED_LIST_FEATURE | HWPX_IMAGE_FEATURE | HWPX_UNSUPPORTED_RICH_BLOCK_FEATURE
        )
}
