use std::fs;

use crate::models::block::{Block, ListBlock, ListItem, TableAlignment, TableBlock, TableRow};
use crate::models::document::Document;
use crate::models::inline::Inline;

use super::support::approved_root;

pub(super) fn expected_document(fixture: &str) -> Document {
    match fixture {
        "blockquote-basic" => doc(vec![Block::BlockQuote(vec![
            paragraph("Quoted paragraph"),
            paragraph("Another quoted line"),
        ])]),
        "code-block-basic" => doc(vec![
            paragraph("fn main() {"),
            paragraph("    println!(\"hi\");"),
            paragraph("}"),
        ]),
        "core-heading" => doc(vec![heading(1, "제목 1"), heading(2, "제목 2")]),
        "core-inline-style" => doc(vec![Block::Paragraph(vec![
            Inline::Text("일반 ".to_string()),
            strong("굵게"),
            Inline::Text(" ".to_string()),
            emphasis("기울임"),
            Inline::Text(" ".to_string()),
            Inline::Code("코드".to_string()),
            Inline::HardBreak,
            Inline::Text("다음 줄".to_string()),
        ])]),
        "core-link-text" => doc(vec![Block::Paragraph(vec![link(
            "문서 열기",
            "https://example.com/",
        )])]),
        "core-mixed" => doc(vec![
            heading(1, "HWPX Core"),
            Block::Paragraph(vec![
                Inline::Text("첫 문단은 ".to_string()),
                strong("강조"),
                Inline::Text("와 ".to_string()),
                emphasis("기울임"),
                Inline::Text(", ".to_string()),
                Inline::Code("코드".to_string()),
                Inline::Text(", 그리고 ".to_string()),
                link("링크", "https://example.com/"),
                Inline::Text("를 포함합니다.".to_string()),
            ]),
            paragraph("줄바꿈도 유지합니다."),
        ]),
        "core-paragraph" => doc(vec![
            paragraph(
                "브라우저와 Rust 코어를 공유하는 기본 문단입니다. 브라우저와 Rust 코어를 공유하는 기본 문단입니다. 브라우저와 Rust 코어를 공유하는 기본 문단입니다.",
            ),
            paragraph(
                "This is Second Contents. This is Second Contents. This is Second Contents. This is Second Contents. This is Second Contents.",
            ),
        ]),
        "list-basic" => doc(vec![unordered_list(vec![
            list_item_text("Alpha"),
            list_item_text("Beta"),
            list_item_text("Gamma"),
        ])]),
        "list-nested-depth-2" => doc(vec![unordered_list(vec![
            ListItem {
                blocks: vec![
                    paragraph("Parent"),
                    unordered_list(vec![
                        list_item_text("Child one"),
                        list_item_text("Child two"),
                    ]),
                ],
            },
            list_item_text("Sibling"),
        ])]),
        "ordered-list-basic" => doc(vec![ordered_list(vec![
            list_item_text("Alpha"),
            list_item_text("Beta"),
            list_item_text("Gamma"),
        ])]),
        "ordered-list-nested-depth-2" => doc(vec![ordered_list(vec![
            ListItem {
                blocks: vec![
                    paragraph("Parent"),
                    ordered_list(vec![
                        list_item_text("Child one"),
                        list_item_text("Child two"),
                    ]),
                ],
            },
            list_item_text("Sibling"),
        ])]),
        "style-brand-color" => doc(vec![
            heading(1, "브랜드 제목"),
            Block::Paragraph(vec![
                Inline::Text("강조 ".to_string()),
                strong("텍스트"),
                Inline::Text(" 와 ".to_string()),
                Inline::Code("코드".to_string()),
                Inline::Text(" 그리고 ".to_string()),
                link("링크", "https://example.com/"),
            ]),
        ]),
        "style-centered-layout" => doc(vec![
            heading(1, "중앙 정렬"),
            paragraph("중앙 정렬 본문입니다."),
            paragraph("두 번째 문단입니다."),
        ]),
        "style-typography" => doc(vec![
            heading(1, "제목"),
            paragraph("본문 문단입니다."),
            Block::Paragraph(vec![link("링크", "https://example.com/")]),
        ]),
        "table-alignment" | "table-basic" => doc(vec![table(
            &["Name", "Role"],
            &[&["A", "Writer"], &["B", "Reviewer"]],
        )]),
        other => panic!("missing explicit expected document for fixture {other}"),
    }
}

pub(super) fn fixture_options(fixture: &str) -> crate::HwpxOptions {
    let root = approved_root().join(fixture);
    let contents = fs::read_to_string(root.join("fixture.toml")).unwrap();
    let mut title = None;
    let mut author = None;
    let mut strict_mode = true;
    let mut in_style = false;
    let mut style = crate::HwpxStyleOptions::default();

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            in_style = line == "[style]";
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();

        if in_style {
            match key {
                "body_font" => style.body_font = Some(parse_toml_value(value)),
                "heading_font" => style.heading_font = Some(parse_toml_value(value)),
                "body_font_size" => style.body_font_size = Some(value.parse().unwrap()),
                "heading_font_size" => style.heading_font_size = Some(value.parse().unwrap()),
                "text_color" => style.text_color = Some(parse_toml_value(value)),
                "heading_color" => style.heading_color = Some(parse_toml_value(value)),
                "link_color" => style.link_color = Some(parse_toml_value(value)),
                "paragraph_align" => {
                    style.paragraph_align = Some(parse_paragraph_align(&parse_toml_value(value)))
                }
                _ => {}
            }
            continue;
        }

        match key {
            "title" => title = Some(parse_toml_value(value)),
            "author" => author = Some(parse_toml_value(value)),
            "strict_mode" => strict_mode = value.parse().unwrap(),
            _ => {}
        }
    }

    crate::HwpxOptions {
        title,
        author,
        strict_mode,
        style,
    }
}

fn parse_toml_value(value: &str) -> String {
    value.trim().trim_matches('"').to_string()
}

fn parse_paragraph_align(value: &str) -> crate::HwpxParagraphAlign {
    match value {
        "left" => crate::HwpxParagraphAlign::Left,
        "center" => crate::HwpxParagraphAlign::Center,
        "right" => crate::HwpxParagraphAlign::Right,
        "justify" => crate::HwpxParagraphAlign::Justify,
        other => panic!("unsupported paragraph_align fixture value: {other}"),
    }
}

pub(super) fn doc(blocks: Vec<Block>) -> Document {
    Document { blocks }
}

fn paragraph(text: &str) -> Block {
    Block::Paragraph(vec![Inline::Text(text.to_string())])
}

fn heading(level: u8, text: &str) -> Block {
    Block::Heading {
        level,
        content: vec![Inline::Text(text.to_string())],
    }
}

fn strong(text: &str) -> Inline {
    Inline::Strong(vec![Inline::Text(text.to_string())])
}

fn emphasis(text: &str) -> Inline {
    Inline::Emphasis(vec![Inline::Text(text.to_string())])
}

pub(super) fn link(text: &str, url: &str) -> Inline {
    Inline::Link {
        text: vec![Inline::Text(text.to_string())],
        url: url.to_string(),
    }
}

fn unordered_list(items: Vec<ListItem>) -> Block {
    Block::List(ListBlock {
        ordered: false,
        start_index: 1,
        items,
    })
}

fn ordered_list(items: Vec<ListItem>) -> Block {
    Block::List(ListBlock {
        ordered: true,
        start_index: 1,
        items,
    })
}

fn table(headers: &[&str], rows: &[&[&str]]) -> Block {
    Block::Table(TableBlock {
        aligns: vec![TableAlignment::None; headers.len()],
        headers: headers
            .iter()
            .map(|cell| vec![Inline::Text((*cell).to_string())])
            .collect(),
        rows: rows
            .iter()
            .map(|row| TableRow {
                cells: row
                    .iter()
                    .map(|cell| vec![Inline::Text((*cell).to_string())])
                    .collect(),
            })
            .collect(),
    })
}

fn list_item_text(text: &str) -> ListItem {
    ListItem {
        blocks: vec![Block::Paragraph(vec![Inline::Text(text.to_string())])],
    }
}
