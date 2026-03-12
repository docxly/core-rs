mod style;
mod table;

use crate::generators::hwpx::document_shape::max_ordered_list_depth;
use crate::models::inline::Inline;

use self::style::{
    heading_style, normal_style, ordered_list_style, plain_paragraph_item, unordered_list_style,
    unsupported_name,
};
use self::table::build_table_spec;
use super::render::render_runs;
use super::text::flatten_block_to_paragraphs;
use super::{
    Block, CoreRsError, ListBlock, OrderedListShape, ParagraphStyle, QuoteRenderMode, SectionItem,
    TABLE_ID_SEED,
};

struct Collector {
    strict_mode: bool,
    ordered_shape: Option<OrderedListShape>,
    quote_render_mode: QuoteRenderMode,
    items: Vec<SectionItem>,
    next_table_id: u64,
    next_link_index: u64,
}

impl Collector {
    fn new(
        strict_mode: bool,
        ordered_shape: Option<OrderedListShape>,
        quote_render_mode: QuoteRenderMode,
    ) -> Self {
        Self {
            strict_mode,
            ordered_shape,
            quote_render_mode,
            items: Vec::new(),
            next_table_id: TABLE_ID_SEED,
            next_link_index: 0,
        }
    }

    fn collect_blocks(mut self, blocks: &[Block]) -> Result<Vec<SectionItem>, CoreRsError> {
        for block in blocks {
            self.collect_block_items(block, 0)?;
        }
        Ok(self.items)
    }

    fn collect_block_items(&mut self, block: &Block, quote_depth: u8) -> Result<(), CoreRsError> {
        match block {
            Block::Paragraph(inlines) => self.push_rendered_paragraph(
                normal_style(quote_depth, self.quote_render_mode),
                inlines,
            ),
            Block::Heading { level, content } => self.push_rendered_paragraph(
                heading_style(*level, quote_depth, self.quote_render_mode),
                content,
            ),
            Block::BlockQuote(blocks) => self.collect_quoted_blocks(blocks, quote_depth),
            Block::List(list) => self.collect_list_items(list, quote_depth),
            Block::Table(table) => {
                self.items.push(SectionItem::Table(build_table_spec(
                    table,
                    self.strict_mode,
                    self.next_table_id,
                )?));
                self.next_table_id += 1;
                Ok(())
            }
            unsupported => {
                if self.strict_mode {
                    return Err(CoreRsError::UnsupportedFeature(format!(
                        "HWPX core does not support {0}",
                        unsupported_name(unsupported)
                    )));
                }
                self.append_fallback_paragraphs(unsupported, quote_depth);
                Ok(())
            }
        }
    }

    fn collect_quoted_blocks(
        &mut self,
        blocks: &[Block],
        quote_depth: u8,
    ) -> Result<(), CoreRsError> {
        let next_quote_depth = quote_depth.saturating_add(1);
        for child in blocks {
            self.collect_block_items(child, next_quote_depth)?;
        }
        Ok(())
    }

    fn collect_list_items(&mut self, list: &ListBlock, quote_depth: u8) -> Result<(), CoreRsError> {
        if list.ordered {
            return self.collect_ordered_items(list, quote_depth);
        }

        self.collect_unordered_list_items(list, 1, quote_depth)
    }

    fn collect_ordered_items(
        &mut self,
        list: &ListBlock,
        quote_depth: u8,
    ) -> Result<(), CoreRsError> {
        if self.strict_mode {
            return Err(CoreRsError::UnsupportedFeature(
                "HWPX approved baseline does not support ordered list".to_string(),
            ));
        }

        if let Some(shape) = self.ordered_shape {
            self.collect_ordered_list_items(list, 1, quote_depth, shape)
        } else {
            self.append_fallback_paragraphs(&Block::List(list.clone()), quote_depth);
            Ok(())
        }
    }

    fn collect_unordered_list_items(
        &mut self,
        list: &ListBlock,
        depth: u8,
        quote_depth: u8,
    ) -> Result<(), CoreRsError> {
        if depth > 2 {
            if self.strict_mode {
                return Err(CoreRsError::UnsupportedFeature(
                    "HWPX approved baseline supports nested unordered lists up to depth 2"
                        .to_string(),
                ));
            }
            self.append_fallback_paragraphs(&Block::List(list.clone()), quote_depth);
            return Ok(());
        }

        let list_style = unordered_list_style(depth, quote_depth, self.quote_render_mode);
        for item in &list.items {
            for block in &item.blocks {
                self.collect_unordered_list_block(block, list_style, depth, quote_depth)?;
            }
        }

        Ok(())
    }

    fn collect_unordered_list_block(
        &mut self,
        block: &Block,
        list_style: ParagraphStyle,
        depth: u8,
        quote_depth: u8,
    ) -> Result<(), CoreRsError> {
        match block {
            Block::Paragraph(inlines) => self.push_rendered_paragraph(list_style, inlines),
            Block::Heading { content, .. } => self.push_rendered_paragraph(list_style, content),
            Block::List(child) if !child.ordered => {
                self.collect_unordered_list_items(child, depth + 1, quote_depth)
            }
            nested_list @ Block::List(_) => {
                if self.strict_mode {
                    return Err(CoreRsError::UnsupportedFeature(
                        "HWPX approved baseline does not support ordered list".to_string(),
                    ));
                }
                self.append_fallback_paragraphs(nested_list, quote_depth);
                Ok(())
            }
            unsupported => {
                if self.strict_mode {
                    return Err(CoreRsError::UnsupportedFeature(format!(
                        "HWPX approved baseline does not support {0} inside unordered list",
                        unsupported_name(unsupported)
                    )));
                }
                self.append_fallback_paragraphs(unsupported, quote_depth);
                Ok(())
            }
        }
    }

    fn collect_ordered_list_items(
        &mut self,
        list: &ListBlock,
        depth: u8,
        quote_depth: u8,
        shape: OrderedListShape,
    ) -> Result<(), CoreRsError> {
        if depth > 2 {
            self.append_fallback_paragraphs(&Block::List(list.clone()), quote_depth);
            return Ok(());
        }

        let list_style = ordered_list_style(shape, depth, quote_depth, self.quote_render_mode);
        for item in &list.items {
            for block in &item.blocks {
                self.collect_ordered_list_block(block, list_style, depth, quote_depth, shape)?;
            }
        }

        Ok(())
    }

    fn collect_ordered_list_block(
        &mut self,
        block: &Block,
        list_style: ParagraphStyle,
        depth: u8,
        quote_depth: u8,
        shape: OrderedListShape,
    ) -> Result<(), CoreRsError> {
        match block {
            Block::Paragraph(inlines) => self.push_rendered_paragraph(list_style, inlines),
            Block::Heading { content, .. } => self.push_rendered_paragraph(list_style, content),
            Block::List(child) if child.ordered => {
                self.collect_ordered_list_items(child, depth + 1, quote_depth, shape)
            }
            nested_list @ Block::List(_) => {
                self.append_fallback_paragraphs(nested_list, quote_depth);
                Ok(())
            }
            unsupported => {
                self.append_fallback_paragraphs(unsupported, quote_depth);
                Ok(())
            }
        }
    }

    fn push_rendered_paragraph(
        &mut self,
        style: ParagraphStyle,
        inlines: &[Inline],
    ) -> Result<(), CoreRsError> {
        let runs = render_runs(
            inlines,
            style.default_char_pr,
            self.strict_mode,
            &mut self.next_link_index,
        )?;
        self.items.push(SectionItem::Paragraph(style, runs));
        Ok(())
    }

    fn append_fallback_paragraphs(&mut self, block: &Block, quote_depth: u8) {
        for paragraph in flatten_block_to_paragraphs(block) {
            self.items.push(plain_paragraph_item(
                paragraph,
                quote_depth,
                self.quote_render_mode,
            ));
        }
    }
}

pub(super) fn collect_section_items(
    blocks: &[Block],
    strict_mode: bool,
    ordered_shape: Option<OrderedListShape>,
    quote_render_mode: QuoteRenderMode,
) -> Result<Vec<SectionItem>, CoreRsError> {
    Collector::new(strict_mode, ordered_shape, quote_render_mode).collect_blocks(blocks)
}

pub(super) fn ordered_list_shape(blocks: &[Block]) -> Option<OrderedListShape> {
    match max_ordered_list_depth(blocks) {
        0 => None,
        1 => Some(OrderedListShape::SingleLevel),
        2 => Some(OrderedListShape::NestedDepth2),
        _ => None,
    }
}
