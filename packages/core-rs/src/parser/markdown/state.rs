use pulldown_cmark::{Alignment, CodeBlockKind, HeadingLevel, Tag, TagEnd};

use crate::error::CoreRsError;
use crate::models::block::{ListBlock, ListItem, TableAlignment, TableRow};
use crate::models::inline::{ImageData, Inline};

#[derive(Debug)]
pub(super) enum Container {
    Paragraph(Vec<Inline>),
    Heading {
        level: u8,
        content: Vec<Inline>,
    },
    BlockQuote(Vec<crate::models::block::Block>),
    CodeBlock {
        language: Option<String>,
        code: String,
    },
    List(ListBlock),
    ListItem(ListItem),
    Table(TableBuilder),
    TableHead(Vec<Vec<Inline>>),
    TableRow(TableRowBuilder),
    TableCell(Vec<Inline>),
}

#[derive(Debug)]
pub(super) struct TableBuilder {
    pub aligns: Vec<TableAlignment>,
    pub headers: Vec<Vec<Inline>>,
    pub rows: Vec<TableRow>,
}

#[derive(Debug)]
pub(super) struct TableRowBuilder {
    pub cells: Vec<Vec<Inline>>,
}

#[derive(Debug)]
pub(super) enum InlineContext {
    Emphasis(Vec<Inline>),
    Strong(Vec<Inline>),
    Link {
        url: String,
        title: Option<String>,
        content: Vec<Inline>,
    },
    Image {
        url: String,
        title: Option<String>,
        alt: Vec<Inline>,
    },
}

impl InlineContext {
    pub(super) fn push(&mut self, inline: Inline) {
        match self {
            Self::Emphasis(content) | Self::Strong(content) | Self::Link { content, .. } => {
                push_inline_merged(content, inline)
            }
            Self::Image { alt, .. } => push_inline_merged(alt, inline),
        }
    }
}

pub(super) fn push_inline_merged(content: &mut Vec<Inline>, inline: Inline) {
    match (content.last_mut(), inline) {
        (Some(Inline::Text(existing)), Inline::Text(text)) => existing.push_str(&text),
        (_, inline) => content.push(inline),
    }
}

pub(super) fn append_fallback_separator(content: &mut Vec<Inline>) {
    match content.last_mut() {
        Some(Inline::Text(existing)) if !existing.ends_with(char::is_whitespace) => {
            existing.push(' ')
        }
        Some(Inline::HardBreak) | None => {}
        _ => content.push(Inline::Text(" ".to_string())),
    }
}

pub(super) fn trim_trailing_fallback_whitespace(content: &mut Vec<Inline>) {
    if let Some(Inline::Text(text)) = content.last_mut() {
        let trimmed = text.trim_end().to_string();
        *text = trimmed;
        if text.is_empty() {
            content.pop();
        }
    }
}

pub(super) fn current_list_depth(containers: &[Container]) -> usize {
    containers
        .iter()
        .filter(|container| matches!(container, Container::List(_)))
        .count()
}

pub(super) fn current_table_mut(
    containers: &mut [Container],
) -> Result<&mut TableBuilder, CoreRsError> {
    containers
        .iter_mut()
        .rev()
        .find_map(|container| match container {
            Container::Table(table) => Some(table),
            _ => None,
        })
        .ok_or_else(|| CoreRsError::InvalidMarkdown("table tag without open table".to_string()))
}

pub(super) fn unsupported_block_feature(tag: &Tag<'_>) -> Option<&'static str> {
    match tag {
        Tag::HtmlBlock => Some("html block"),
        Tag::FootnoteDefinition(_) => Some("footnote"),
        Tag::DefinitionList | Tag::DefinitionListTitle | Tag::DefinitionListDefinition => {
            Some("definition list")
        }
        _ => None,
    }
}

pub(super) fn unsupported_block_nesting_tag(tag: &Tag<'_>) -> bool {
    matches!(
        tag,
        Tag::List(_)
            | Tag::BlockQuote(_)
            | Tag::CodeBlock(_)
            | Tag::Table(_)
            | Tag::HtmlBlock
            | Tag::FootnoteDefinition(_)
            | Tag::DefinitionList
            | Tag::DefinitionListTitle
            | Tag::DefinitionListDefinition
    )
}

pub(super) fn unsupported_block_nesting_end(tag: &TagEnd) -> bool {
    matches!(
        tag,
        TagEnd::List(_)
            | TagEnd::BlockQuote(_)
            | TagEnd::CodeBlock
            | TagEnd::Table
            | TagEnd::HtmlBlock
            | TagEnd::FootnoteDefinition
            | TagEnd::DefinitionList
            | TagEnd::DefinitionListTitle
            | TagEnd::DefinitionListDefinition
    )
}

pub(super) fn table_alignment(alignment: Alignment) -> TableAlignment {
    match alignment {
        Alignment::None => TableAlignment::None,
        Alignment::Left => TableAlignment::Left,
        Alignment::Center => TableAlignment::Center,
        Alignment::Right => TableAlignment::Right,
    }
}

pub(super) fn code_block_language(kind: CodeBlockKind<'_>) -> Option<String> {
    match kind {
        CodeBlockKind::Indented => None,
        CodeBlockKind::Fenced(language) => empty_to_none(language.trim().to_string()),
    }
}

pub(super) fn flatten_inline_text(inlines: &[Inline]) -> String {
    let mut text = String::new();
    for inline in inlines {
        match inline {
            Inline::Text(value) | Inline::Code(value) => text.push_str(value),
            Inline::Emphasis(content) | Inline::Strong(content) => {
                text.push_str(&flatten_inline_text(content));
            }
            Inline::Link { text: content, .. } => text.push_str(&flatten_inline_text(content)),
            Inline::Image(ImageData { alt, .. }) => text.push_str(alt),
            Inline::HardBreak => text.push(' '),
        }
    }
    text
}

pub(super) fn heading_level(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

pub(super) fn empty_to_none(value: String) -> Option<String> {
    if value.is_empty() { None } else { Some(value) }
}
