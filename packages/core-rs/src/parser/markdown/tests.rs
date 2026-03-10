use super::MarkdownParser;
use crate::error::CoreRsError;
use crate::models::block::{Block, ListBlock, ListItem, TableAlignment, TableBlock, TableRow};
use crate::models::inline::{ImageData, Inline};
use base64::Engine;

const PNG_DATA_URI: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+a7mQAAAAASUVORK5CYII=";

#[test]
fn parses_heading_and_paragraph() {
    let markdown =
        "# Title\n\nHello *world* and **team** with `code` and [docs](https://example.com).";
    let document = MarkdownParser::new(true).parse(markdown).unwrap();

    assert_eq!(
        document.blocks,
        vec![
            Block::Heading {
                level: 1,
                content: vec![Inline::Text("Title".to_string())],
            },
            Block::Paragraph(vec![
                Inline::Text("Hello ".to_string()),
                Inline::Emphasis(vec![Inline::Text("world".to_string())]),
                Inline::Text(" and ".to_string()),
                Inline::Strong(vec![Inline::Text("team".to_string())]),
                Inline::Text(" with ".to_string()),
                Inline::Code("code".to_string()),
                Inline::Text(" and ".to_string()),
                Inline::Link {
                    text: vec![Inline::Text("docs".to_string())],
                    url: "https://example.com".to_string(),
                },
                Inline::Text(".".to_string()),
            ]),
        ]
    );
}

#[test]
fn parses_rich_blocks() {
    let markdown = "> quote\n\n```rust\nlet answer = 42;\n```\n\n- one\n  - two\n\n| left | right |\n| :--- | ---: |\n| a | b |\n\n---";
    let document = MarkdownParser::new(true).parse(markdown).unwrap();

    assert_eq!(
        document.blocks,
        vec![
            Block::BlockQuote(vec![Block::Paragraph(vec![Inline::Text(
                "quote".to_string()
            )])]),
            Block::CodeBlock {
                language: Some("rust".to_string()),
                code: "let answer = 42;\n".to_string(),
            },
            Block::List(ListBlock {
                ordered: false,
                start_index: 1,
                items: vec![ListItem {
                    blocks: vec![
                        Block::Paragraph(vec![Inline::Text("one".to_string())]),
                        Block::List(ListBlock {
                            ordered: false,
                            start_index: 1,
                            items: vec![ListItem {
                                blocks: vec![Block::Paragraph(vec![Inline::Text(
                                    "two".to_string()
                                )])],
                            }],
                        }),
                    ],
                }],
            }),
            Block::Table(TableBlock {
                aligns: vec![TableAlignment::Left, TableAlignment::Right],
                headers: vec![
                    vec![Inline::Text("left".to_string())],
                    vec![Inline::Text("right".to_string())],
                ],
                rows: vec![TableRow {
                    cells: vec![
                        vec![Inline::Text("a".to_string())],
                        vec![Inline::Text("b".to_string())],
                    ],
                }],
            }),
            Block::ThematicBreak,
        ]
    );
}

#[test]
fn parses_tight_list_item_with_inline_content() {
    let markdown = "- **bold** [docs](https://example.com) and `code`";
    let document = MarkdownParser::new(true).parse(markdown).unwrap();

    assert_eq!(
        document.blocks,
        vec![Block::List(ListBlock {
            ordered: false,
            start_index: 1,
            items: vec![ListItem {
                blocks: vec![Block::Paragraph(vec![
                    Inline::Strong(vec![Inline::Text("bold".to_string())]),
                    Inline::Text(" ".to_string()),
                    Inline::Link {
                        text: vec![Inline::Text("docs".to_string())],
                        url: "https://example.com".to_string(),
                    },
                    Inline::Text(" and ".to_string()),
                    Inline::Code("code".to_string()),
                ])],
            }],
        })]
    );
}

#[test]
fn parses_data_uri_image() {
    let markdown = &format!("![tiny]({PNG_DATA_URI})");
    let document = MarkdownParser::new(true).parse(markdown).unwrap();

    assert!(matches!(
        &document.blocks[0],
        Block::Paragraph(content) if matches!(
            &content[0],
            Inline::Image(ImageData { alt, mime_type, extension, data, .. })
                if alt == "tiny" && mime_type == "image/png" && extension == "png" && !data.is_empty()
        )
    ));
}

#[test]
fn falls_back_for_html_when_not_strict() {
    let markdown = "<b>raw</b>";
    let document = MarkdownParser::new(false).parse(markdown).unwrap();

    assert_eq!(
        document.blocks,
        vec![Block::Paragraph(vec![Inline::Text(
            "<b>raw</b>".to_string()
        )])]
    );
}

#[test]
fn errors_for_html_when_strict() {
    let markdown = "<b>raw</b>";
    let error = MarkdownParser::new(true).parse(markdown).unwrap_err();

    assert!(matches!(error, CoreRsError::UnsupportedFeature(_)));
}

#[test]
fn falls_back_to_alt_text_for_non_data_image_when_not_strict() {
    let markdown = "![chart](./chart.png)";
    let document = MarkdownParser::new(false).parse(markdown).unwrap();

    assert_eq!(
        document.blocks,
        vec![Block::Paragraph(vec![Inline::Text("chart".to_string())])]
    );
}

#[test]
fn errors_for_non_data_image_when_strict() {
    let markdown = "![chart](./chart.png)";
    let error = MarkdownParser::new(true).parse(markdown).unwrap_err();

    assert!(matches!(error, CoreRsError::UnsupportedFeature(_)));
}

#[test]
fn falls_back_for_thematic_break_when_not_strict() {
    let document = MarkdownParser::new(false).parse("---").unwrap();

    assert_eq!(document.blocks, vec![Block::ThematicBreak]);
}

#[test]
fn falls_back_for_deeply_nested_list_when_not_strict() {
    let markdown = "- one\n  - two\n    - three";
    let document = MarkdownParser::new(false).parse(markdown).unwrap();

    assert_eq!(
        document.blocks,
        vec![Block::List(ListBlock {
            ordered: false,
            start_index: 1,
            items: vec![ListItem {
                blocks: vec![
                    Block::Paragraph(vec![Inline::Text("one".to_string())]),
                    Block::List(ListBlock {
                        ordered: false,
                        start_index: 1,
                        items: vec![ListItem {
                            blocks: vec![
                                Block::Paragraph(vec![Inline::Text("two".to_string())]),
                                Block::Paragraph(vec![Inline::Text("three".to_string())]),
                            ],
                        }],
                    }),
                ],
            }],
        })]
    );
}

#[test]
fn errors_for_unsafe_link_scheme_when_strict() {
    let error = MarkdownParser::new(true)
        .parse("[run](javascript:alert(1))")
        .unwrap_err();

    assert!(matches!(error, CoreRsError::UnsupportedFeature(_)));
}

#[test]
fn falls_back_for_unsafe_link_scheme_when_not_strict() {
    let document = MarkdownParser::new(false)
        .parse("[run](javascript:alert(1))")
        .unwrap();

    assert_eq!(
        document.blocks,
        vec![Block::Paragraph(vec![Inline::Text("run".to_string())])]
    );
}

#[test]
fn errors_for_oversized_data_image_when_strict() {
    let payload = base64::engine::general_purpose::STANDARD.encode(vec![0_u8; 8 * 1024 * 1024 + 1]);
    let markdown = format!("![huge](data:image/png;base64,{payload})");
    let error = MarkdownParser::new(true).parse(&markdown).unwrap_err();

    assert!(matches!(error, CoreRsError::UnsupportedFeature(_)));
}
