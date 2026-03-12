mod block;
mod inline;

use crate::error::CoreRsError;
use crate::models::block::Block;

const QUOTE_BORDER_PROPERTIES: &str =
    "<w:pBdr><w:left w:val=\"single\" w:sz=\"8\" w:space=\"8\" w:color=\"B7B7B7\"/></w:pBdr>";
const QUOTE_LEFT_BORDER_FRAGMENT: &str =
    "<w:left w:val=\"single\" w:sz=\"8\" w:space=\"8\" w:color=\"B7B7B7\"/>";

#[derive(Debug, Default)]
pub(super) struct RenderContext {
    pub hyperlinks: Vec<String>,
    pub images: Vec<ImageAsset>,
}

#[derive(Debug, Clone)]
pub(super) struct ImageAsset {
    pub relationship_id: String,
    pub target: String,
    pub extension: String,
    pub mime_type: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct RunStyle {
    pub bold: bool,
    pub italic: bool,
    pub code: bool,
    pub hyperlink: bool,
}

#[derive(Clone, Copy)]
struct ParagraphRenderOptions<'a> {
    style_id: Option<&'a str>,
    left_indent: usize,
    extra_properties: &'a str,
    prefix: Option<&'a str>,
}

impl<'a> ParagraphRenderOptions<'a> {
    fn plain(left_indent: usize, extra_properties: &'a str, prefix: Option<&'a str>) -> Self {
        Self {
            style_id: None,
            left_indent,
            extra_properties,
            prefix,
        }
    }

    fn heading(
        style_id: &'a str,
        left_indent: usize,
        extra_properties: &'a str,
        prefix: Option<&'a str>,
    ) -> Self {
        Self {
            style_id: Some(style_id),
            left_indent,
            extra_properties,
            prefix,
        }
    }
}

fn heading_style_id(level: u8) -> String {
    format!("Heading{}", level)
}

fn walk_blocks<T>(blocks: &[Block], visitor: &mut impl FnMut(&Block) -> T) {
    for block in blocks {
        visitor(block);
        match block {
            Block::BlockQuote(children) => walk_blocks(children, visitor),
            Block::List(list) => {
                for item in &list.items {
                    walk_blocks(&item.blocks, visitor);
                }
            }
            Block::Paragraph(_)
            | Block::Heading { .. }
            | Block::CodeBlock { .. }
            | Block::Table(_)
            | Block::ThematicBreak => {}
        }
    }
}

pub(super) fn render_blocks(
    blocks: &[Block],
    context: &mut RenderContext,
) -> Result<String, CoreRsError> {
    block::render_blocks(blocks, context)
}

pub(super) fn max_heading_level(blocks: &[Block]) -> u8 {
    block::max_heading_level(blocks)
}

pub(super) fn run_properties(style: RunStyle) -> String {
    inline::run_properties(style)
}

pub(super) fn preserve_space(text: &str) -> bool {
    inline::preserve_space(text)
}
