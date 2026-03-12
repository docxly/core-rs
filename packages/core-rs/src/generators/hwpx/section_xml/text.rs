use super::{Block, CoreRsError, Inline, ListSemantic, SectionItem, TableSpec};

pub(super) fn flatten_inline_children(
    inlines: &[Inline],
    strict_mode: bool,
) -> Result<String, CoreRsError> {
    let mut text = String::new();
    for inline in inlines {
        match inline {
            Inline::Text(value) => text.push_str(value),
            Inline::Emphasis(children) | Inline::Strong(children) => {
                text.push_str(&flatten_inline_children(children, strict_mode)?);
            }
            Inline::Code(code) => text.push_str(code),
            Inline::Link {
                text: children,
                url,
            } => {
                let label = flatten_inline_children(children, strict_mode)?;
                if label.trim().is_empty() {
                    text.push_str(url);
                } else {
                    text.push_str(&label);
                }
            }
            Inline::Image(image) => {
                if strict_mode {
                    return Err(CoreRsError::UnsupportedFeature(
                        "HWPX core does not support image".to_string(),
                    ));
                }
                if image.alt.trim().is_empty() {
                    text.push_str("[image]");
                } else {
                    text.push_str(&image.alt);
                }
            }
            Inline::HardBreak => text.push(' '),
        }
    }
    Ok(text)
}

pub(super) fn visible_paragraphs(items: &[SectionItem]) -> Vec<String> {
    let mut paragraphs = Vec::new();
    for item in items {
        match item {
            SectionItem::Paragraph(_, runs) => {
                paragraphs.push(runs.iter().map(run_visible_text).collect())
            }
            SectionItem::Table(table) => append_table_preview_paragraphs(table, &mut paragraphs),
        }
    }
    paragraphs
}

fn append_table_preview_paragraphs(table: &TableSpec, paragraphs: &mut Vec<String>) {
    for row in &table.rows {
        paragraphs.push(
            row.iter()
                .map(|cell| format!("<{cell}>"))
                .collect::<Vec<_>>()
                .join(""),
        );
    }
}

pub(super) fn flatten_block_to_paragraphs(block: &Block) -> Vec<String> {
    match block {
        Block::Paragraph(content) | Block::Heading { content, .. } => {
            vec![flatten_inline_children(content, false).unwrap_or_default()]
        }
        Block::BlockQuote(blocks) => blocks
            .iter()
            .flat_map(flatten_block_to_paragraphs)
            .collect::<Vec<_>>(),
        Block::CodeBlock { code, .. } => code.lines().map(ToString::to_string).collect(),
        Block::List(list) => list
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                let prefix = if list.ordered {
                    format!("{}. ", list.start_index + index as u64)
                } else {
                    "- ".to_string()
                };
                let body = item
                    .blocks
                    .iter()
                    .flat_map(flatten_block_to_paragraphs)
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("{prefix}{body}")
            })
            .collect(),
        Block::Table(table) => flatten_table_to_paragraphs(table),
        Block::ThematicBreak => vec!["---".to_string()],
    }
}

fn flatten_table_to_paragraphs(table: &super::TableBlock) -> Vec<String> {
    let mut paragraphs = Vec::new();
    if !table.headers.is_empty() {
        paragraphs.push(
            table
                .headers
                .iter()
                .map(|cell| flatten_inline_children(cell, false).unwrap_or_default())
                .collect::<Vec<_>>()
                .join(" | "),
        );
    }
    paragraphs.extend(table.rows.iter().map(|row| {
        row.cells
            .iter()
            .map(|cell| flatten_inline_children(cell, false).unwrap_or_default())
            .collect::<Vec<_>>()
            .join(" | ")
    }));
    paragraphs
}

pub(super) fn list_caret_position(items: &[SectionItem]) -> Option<(usize, usize)> {
    let mut paragraph_index = 0;
    let mut last_unordered_caret = None;
    let mut ordered_carets = Vec::new();

    for item in items {
        match item {
            SectionItem::Paragraph(style, runs) => {
                let text = runs.iter().map(run_visible_text).collect::<String>();
                let caret = (paragraph_index, text.chars().count());
                match style.list_semantic {
                    ListSemantic::None => {}
                    ListSemantic::Unordered => last_unordered_caret = Some(caret),
                    ListSemantic::OrderedSingleLevel
                    | ListSemantic::OrderedTopLevel
                    | ListSemantic::OrderedNestedLevel => {
                        ordered_carets.push((style.list_semantic, caret));
                    }
                }
                paragraph_index += 1;
            }
            SectionItem::Table(table) => {
                paragraph_index += table.rows.len();
            }
        }
    }

    if !ordered_carets.is_empty() {
        let only_single_level = ordered_carets
            .iter()
            .all(|(semantic, _)| *semantic == ListSemantic::OrderedSingleLevel);
        if only_single_level && ordered_carets.len() > 1 {
            return ordered_carets
                .get(ordered_carets.len() - 2)
                .map(|(_, caret)| *caret);
        }
        return ordered_carets.last().map(|(_, caret)| *caret);
    }

    last_unordered_caret
}

pub(super) fn last_paragraph_caret(items: &[SectionItem]) -> Option<(usize, usize)> {
    let mut paragraph_index = 0;
    let mut last_caret = None;

    for item in items {
        match item {
            SectionItem::Paragraph(_, runs) => {
                let text = runs.iter().map(run_visible_text).collect::<String>();
                last_caret = Some((paragraph_index, text.chars().count()));
                paragraph_index += 1;
            }
            SectionItem::Table(table) => {
                paragraph_index += table.rows.len();
            }
        }
    }

    last_caret
}

fn run_visible_text(run: &super::RunSpec) -> &str {
    match run {
        super::RunSpec::Text { text, .. } | super::RunSpec::Hyperlink { text, .. } => text,
    }
}
