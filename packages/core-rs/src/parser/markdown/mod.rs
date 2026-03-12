pub(crate) mod data_image;
mod events;
mod push;
mod state;

#[cfg(test)]
mod tests;

use pulldown_cmark::{Event, Options, Parser, Tag};

use self::state::{Container, current_list_depth, unsupported_block_feature};
use crate::error::CoreRsError;
use crate::models::block::Block;
use crate::models::document::Document;
use crate::models::inline::Inline;

const MAX_TOTAL_DATA_URI_IMAGE_BYTES: usize = 16 * 1024 * 1024;

pub struct MarkdownParser {
    strict_mode: bool,
}

impl MarkdownParser {
    pub fn new(strict_mode: bool) -> Self {
        Self { strict_mode }
    }

    pub fn parse(&self, markdown: &str) -> Result<Document, CoreRsError> {
        let parser = Parser::new_ext(markdown, Options::all());
        let mut blocks = Vec::new();
        let mut containers = Vec::new();
        let mut inline_stack = Vec::new();
        let mut unsupported_block_depth = 0usize;
        let mut unsupported_block_content = Vec::new();
        let mut total_embedded_image_bytes = 0usize;

        for event in parser {
            if unsupported_block_depth > 0 {
                self.handle_unsupported_event(
                    event,
                    &mut blocks,
                    &mut containers,
                    &mut unsupported_block_depth,
                    &mut unsupported_block_content,
                )?;
                continue;
            }

            match event {
                Event::Start(tag) => {
                    if matches!(tag, Tag::List(_)) && current_list_depth(&containers) >= 2 {
                        if self.strict_mode {
                            return Err(CoreRsError::UnsupportedFeature(
                                "nested list depth > 2".to_string(),
                            ));
                        }
                        unsupported_block_depth = 1;
                        continue;
                    }

                    if let Some(feature) = unsupported_block_feature(&tag) {
                        if self.strict_mode {
                            return Err(CoreRsError::UnsupportedFeature(feature.to_string()));
                        }
                        unsupported_block_depth = 1;
                        continue;
                    }

                    self.handle_start(tag, &mut containers, &mut inline_stack)?;
                }
                Event::End(tag) => {
                    self.handle_end(
                        tag,
                        &mut blocks,
                        &mut containers,
                        &mut inline_stack,
                        &mut total_embedded_image_bytes,
                    )?;
                }
                Event::Text(text) => {
                    self.push_text(
                        text.into_string(),
                        &mut blocks,
                        &mut containers,
                        &mut inline_stack,
                    )?;
                }
                Event::Code(code) => {
                    self.push_inline_or_code(
                        Inline::Code(code.into_string()),
                        None,
                        &mut blocks,
                        &mut containers,
                        &mut inline_stack,
                    )?;
                }
                Event::SoftBreak => {
                    self.push_soft_break(&mut blocks, &mut containers, &mut inline_stack)?;
                }
                Event::HardBreak => {
                    self.push_hard_break(&mut blocks, &mut containers, &mut inline_stack)?;
                }
                Event::Rule => {
                    self.push_block(Block::ThematicBreak, &mut blocks, &mut containers)?;
                }
                Event::Html(html) | Event::InlineHtml(html) => {
                    if self.strict_mode {
                        return Err(CoreRsError::UnsupportedFeature("html".to_string()));
                    }
                    self.push_fallback_text(
                        html.into_string(),
                        &mut blocks,
                        &mut containers,
                        &mut inline_stack,
                    )?;
                }
                Event::FootnoteReference(reference) => {
                    if self.strict_mode {
                        return Err(CoreRsError::UnsupportedFeature("footnote".to_string()));
                    }
                    self.push_fallback_text(
                        format!("[^{reference}]"),
                        &mut blocks,
                        &mut containers,
                        &mut inline_stack,
                    )?;
                }
                Event::TaskListMarker(checked) => {
                    if self.strict_mode {
                        return Err(CoreRsError::UnsupportedFeature("task list".to_string()));
                    }
                    self.push_fallback_text(
                        if checked {
                            "[x] ".to_string()
                        } else {
                            "[ ] ".to_string()
                        },
                        &mut blocks,
                        &mut containers,
                        &mut inline_stack,
                    )?;
                }
                Event::InlineMath(math) | Event::DisplayMath(math) => {
                    if self.strict_mode {
                        return Err(CoreRsError::UnsupportedFeature("math".to_string()));
                    }
                    self.push_fallback_text(
                        math.into_string(),
                        &mut blocks,
                        &mut containers,
                        &mut inline_stack,
                    )?;
                }
            }
        }

        if !inline_stack.is_empty() || !containers.is_empty() {
            return Err(CoreRsError::InvalidMarkdown(
                "unbalanced markdown structure".to_string(),
            ));
        }

        Ok(Document { blocks })
    }

    fn push_inline_to_implicit_paragraph(containers: &mut [Container], inline: Inline) -> bool {
        match containers.last_mut() {
            Some(Container::ListItem(item)) => {
                Self::append_inline_to_paragraph_block(&mut item.blocks, inline);
                true
            }
            Some(Container::BlockQuote(blocks)) => {
                Self::append_inline_to_paragraph_block(blocks, inline);
                true
            }
            _ => false,
        }
    }

    fn append_inline_to_paragraph_block(blocks: &mut Vec<Block>, inline: Inline) {
        match blocks.last_mut() {
            Some(Block::Paragraph(content)) => state::push_inline_merged(content, inline),
            _ => blocks.push(Block::Paragraph(vec![inline])),
        }
    }
}
