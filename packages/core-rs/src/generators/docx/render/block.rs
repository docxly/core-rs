#[path = "block_list_table.rs"]
mod list_table;
#[path = "block_text.rs"]
mod text;

use crate::error::CoreRsError;
use crate::models::block::Block;

use super::{ParagraphRenderOptions, RenderContext, heading_style_id, walk_blocks};
use list_table::{render_list, render_table};
use text::{render_block_quote, render_code_block_with_properties, render_text_paragraph, render_thematic_break};

pub(super) fn render_blocks(
    blocks: &[Block],
    context: &mut RenderContext,
) -> Result<String, CoreRsError> {
    let mut xml = String::new();
    for block in blocks {
        xml.push_str(&render_block(block, context, 0)?);
    }
    Ok(xml)
}

pub(super) fn max_heading_level(blocks: &[Block]) -> u8 {
    let mut max_level = 0;
    walk_blocks(blocks, &mut |block| {
        if let Block::Heading { level, .. } = block {
            max_level = max_level.max(*level);
        }
    });
    max_level
}

fn render_block(
    block: &Block,
    context: &mut RenderContext,
    left_indent: usize,
) -> Result<String, CoreRsError> {
    render_block_with_prefix(block, context, left_indent, None, "")
}

fn render_block_with_prefix(
    block: &Block,
    context: &mut RenderContext,
    left_indent: usize,
    prefix: Option<&str>,
    extra_paragraph_properties: &str,
) -> Result<String, CoreRsError> {
    match block {
        Block::Paragraph(content) => render_text_paragraph(
            content,
            ParagraphRenderOptions::plain(left_indent, extra_paragraph_properties, prefix),
            context,
        ),
        Block::Heading { level, content } => {
            let style_id = heading_style_id(*level);
            render_text_paragraph(
                content,
                ParagraphRenderOptions::heading(
                    &style_id,
                    left_indent,
                    extra_paragraph_properties,
                    prefix,
                ),
                context,
            )
        }
        Block::BlockQuote(blocks) => {
            render_block_quote(
                blocks,
                context,
                left_indent,
                prefix,
                extra_paragraph_properties,
                render_block_with_prefix,
            )
        }
        Block::CodeBlock { code, .. } => {
            render_code_block_with_properties(code, left_indent, prefix, extra_paragraph_properties)
        }
        Block::List(list) => render_list(
            list,
            context,
            left_indent,
            prefix,
            extra_paragraph_properties,
            render_block_with_prefix,
        ),
        Block::Table(table) => {
            render_table(table, context, left_indent, prefix, extra_paragraph_properties)
        }
        Block::ThematicBreak => render_thematic_break(left_indent, prefix, extra_paragraph_properties),
    }
}
