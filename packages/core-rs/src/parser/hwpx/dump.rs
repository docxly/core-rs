use std::fmt::Write;

use crate::models::block::{Block, ListBlock, ListItem, TableBlock, TableRow};
use crate::models::inline::Inline;

use super::{ParsedHwpx, RawHwpxMetadata};

pub(super) fn render(parsed: &ParsedHwpx) -> String {
    let mut output = String::new();
    render_metadata(&mut output, &parsed.metadata);
    output.push_str("document:\n");
    render_blocks(&mut output, &parsed.document.blocks, 1);
    output
}

fn render_metadata(output: &mut String, metadata: &RawHwpxMetadata) {
    output.push_str("metadata:\n");
    render_optional_field(output, 1, "title", metadata.title.as_deref());
    render_optional_field(output, 1, "language", metadata.language.as_deref());
    render_optional_field(output, 1, "creator", metadata.creator.as_deref());
    render_optional_field(output, 1, "lastsaveby", metadata.lastsaveby.as_deref());
}

fn render_optional_field(output: &mut String, indent: usize, name: &str, value: Option<&str>) {
    let indent = "  ".repeat(indent);
    match value {
        Some(value) => {
            let _ = writeln!(output, "{indent}{name}: {:?}", value);
        }
        None => {
            let _ = writeln!(output, "{indent}{name}: null");
        }
    }
}

fn render_blocks(output: &mut String, blocks: &[Block], indent: usize) {
    for block in blocks {
        render_block(output, block, indent);
    }
}

fn render_block(output: &mut String, block: &Block, indent: usize) {
    let indent_str = "  ".repeat(indent);
    match block {
        Block::Paragraph(inlines) => {
            let _ = writeln!(output, "{indent_str}paragraph({})", render_inlines(inlines));
        }
        Block::Heading { level, content } => {
            let _ = writeln!(
                output,
                "{indent_str}heading(level={level}, content={})",
                render_inlines(content)
            );
        }
        Block::BlockQuote(inner) => {
            let _ = writeln!(output, "{indent_str}blockquote:");
            render_blocks(output, inner, indent + 1);
        }
        Block::CodeBlock { language, code } => {
            let _ = writeln!(
                output,
                "{indent_str}code_block(language={}, code={:?})",
                render_optional_inline_value(language.as_deref()),
                code
            );
        }
        Block::List(list) => render_list(output, list, indent),
        Block::Table(table) => render_table(output, table, indent),
        Block::ThematicBreak => {
            let _ = writeln!(output, "{indent_str}thematic_break");
        }
    }
}

fn render_list(output: &mut String, list: &ListBlock, indent: usize) {
    let indent_str = "  ".repeat(indent);
    let _ = writeln!(
        output,
        "{indent_str}list(ordered={}, start_index={}):",
        list.ordered, list.start_index
    );
    for item in &list.items {
        render_list_item(output, item, indent + 1);
    }
}

fn render_list_item(output: &mut String, item: &ListItem, indent: usize) {
    let indent_str = "  ".repeat(indent);
    let _ = writeln!(output, "{indent_str}item:");
    render_blocks(output, &item.blocks, indent + 1);
}

fn render_table(output: &mut String, table: &TableBlock, indent: usize) {
    let indent_str = "  ".repeat(indent);
    let _ = writeln!(output, "{indent_str}table:");
    let _ = writeln!(
        output,
        "{indent_str}  aligns: [{}]",
        table
            .aligns
            .iter()
            .map(|align| format!("{align:?}"))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let _ = writeln!(
        output,
        "{indent_str}  headers: [{}]",
        table
            .headers
            .iter()
            .map(|cell| render_inlines(cell))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let _ = writeln!(output, "{indent_str}  rows:");
    for row in &table.rows {
        render_table_row(output, row, indent + 2);
    }
}

fn render_table_row(output: &mut String, row: &TableRow, indent: usize) {
    let indent_str = "  ".repeat(indent);
    let _ = writeln!(
        output,
        "{indent_str}[{}]",
        row.cells
            .iter()
            .map(|cell| render_inlines(cell))
            .collect::<Vec<_>>()
            .join(", ")
    );
}

fn render_inlines(inlines: &[Inline]) -> String {
    let rendered = inlines
        .iter()
        .map(render_inline)
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{rendered}]")
}

fn render_inline(inline: &Inline) -> String {
    match inline {
        Inline::Text(text) => format!("text({text:?})"),
        Inline::Emphasis(inlines) => format!("emphasis({})", render_inlines(inlines)),
        Inline::Strong(inlines) => format!("strong({})", render_inlines(inlines)),
        Inline::Code(code) => format!("code({code:?})"),
        Inline::Link { text, url } => format!("link(text={}, url={url:?})", render_inlines(text)),
        Inline::Image(image) => format!(
            "image(alt={:?}, title={}, mime_type={:?}, extension={:?}, bytes={})",
            image.alt,
            render_optional_inline_value(image.title.as_deref()),
            image.mime_type,
            image.extension,
            image.data.len()
        ),
        Inline::HardBreak => "hard_break".to_string(),
    }
}

fn render_optional_inline_value(value: Option<&str>) -> String {
    match value {
        Some(value) => format!("{value:?}"),
        None => "null".to_string(),
    }
}
