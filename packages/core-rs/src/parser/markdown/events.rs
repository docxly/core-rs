use pulldown_cmark::{Event, Tag, TagEnd};

use super::data_image::{DataImageError, image_fallback_text, parse_data_uri};
use super::state::{
    Container, InlineContext, TableBuilder, TableRowBuilder, append_fallback_separator,
    code_block_language, current_table_mut, empty_to_none, flatten_inline_text, heading_level,
    push_inline_merged, table_alignment, trim_trailing_fallback_whitespace,
    unsupported_block_nesting_end, unsupported_block_nesting_tag,
};
use super::{MAX_TOTAL_DATA_URI_IMAGE_BYTES, MarkdownParser};
use crate::error::CoreRsError;
use crate::models::block::{Block, ListBlock, TableBlock, TableRow};
use crate::models::inline::{ImageData, Inline};

impl MarkdownParser {
    pub(super) fn handle_start(
        &self,
        tag: Tag<'_>,
        containers: &mut Vec<Container>,
        inline_stack: &mut Vec<InlineContext>,
    ) -> Result<(), CoreRsError> {
        match tag {
            Tag::Paragraph => containers.push(Container::Paragraph(Vec::new())),
            Tag::Heading { level, .. } => containers.push(Container::Heading {
                level: heading_level(level),
                content: Vec::new(),
            }),
            Tag::BlockQuote(_) => containers.push(Container::BlockQuote(Vec::new())),
            Tag::CodeBlock(kind) => containers.push(Container::CodeBlock {
                language: code_block_language(kind),
                code: String::new(),
            }),
            Tag::List(start) => containers.push(Container::List(ListBlock {
                ordered: start.is_some(),
                start_index: start.unwrap_or(1),
                items: Vec::new(),
            })),
            Tag::Item => containers.push(Container::ListItem(crate::models::block::ListItem {
                blocks: Vec::new(),
            })),
            Tag::Table(alignments) => containers.push(Container::Table(TableBuilder {
                aligns: alignments.into_iter().map(table_alignment).collect(),
                headers: Vec::new(),
                rows: Vec::new(),
            })),
            Tag::TableHead => containers.push(Container::TableHead(Vec::new())),
            Tag::TableRow => containers.push(Container::TableRow(TableRowBuilder { cells: Vec::new() })),
            Tag::TableCell => containers.push(Container::TableCell(Vec::new())),
            Tag::Emphasis => inline_stack.push(InlineContext::Emphasis(Vec::new())),
            Tag::Strong => inline_stack.push(InlineContext::Strong(Vec::new())),
            Tag::Link { dest_url, title, .. } => inline_stack.push(InlineContext::Link {
                url: dest_url.into_string(),
                title: empty_to_none(title.into_string()),
                content: Vec::new(),
            }),
            Tag::Image { dest_url, title, .. } => inline_stack.push(InlineContext::Image {
                url: dest_url.into_string(),
                title: empty_to_none(title.into_string()),
                alt: Vec::new(),
            }),
            Tag::DefinitionList
            | Tag::DefinitionListTitle
            | Tag::DefinitionListDefinition
            | Tag::MetadataBlock(_)
            | Tag::Strikethrough
            | Tag::HtmlBlock
            | Tag::FootnoteDefinition(_) => {
                if self.strict_mode {
                    return Err(CoreRsError::UnsupportedFeature(
                        "extended markdown syntax".to_string(),
                    ));
                }
            }
        }

        Ok(())
    }

    pub(super) fn handle_end(
        &self,
        tag: TagEnd,
        blocks: &mut Vec<Block>,
        containers: &mut Vec<Container>,
        inline_stack: &mut Vec<InlineContext>,
        total_embedded_image_bytes: &mut usize,
    ) -> Result<(), CoreRsError> {
        match tag {
            TagEnd::Paragraph => {
                let Some(Container::Paragraph(content)) = containers.pop() else {
                    return Err(CoreRsError::InvalidMarkdown(
                        "paragraph close without open".to_string(),
                    ));
                };
                self.push_block(Block::Paragraph(content), blocks, containers)?;
            }
            TagEnd::Heading(_) => {
                let Some(Container::Heading { level, content }) = containers.pop() else {
                    return Err(CoreRsError::InvalidMarkdown(
                        "heading close without open".to_string(),
                    ));
                };
                self.push_block(Block::Heading { level, content }, blocks, containers)?;
            }
            TagEnd::BlockQuote(_) => {
                let Some(Container::BlockQuote(quoted)) = containers.pop() else {
                    return Err(CoreRsError::InvalidMarkdown(
                        "blockquote close without open".to_string(),
                    ));
                };
                self.push_block(Block::BlockQuote(quoted), blocks, containers)?;
            }
            TagEnd::CodeBlock => {
                let Some(Container::CodeBlock { language, code }) = containers.pop() else {
                    return Err(CoreRsError::InvalidMarkdown(
                        "code block close without open".to_string(),
                    ));
                };
                self.push_block(Block::CodeBlock { language, code }, blocks, containers)?;
            }
            TagEnd::List(_) => {
                let Some(Container::List(list)) = containers.pop() else {
                    return Err(CoreRsError::InvalidMarkdown("list close without open".to_string()));
                };
                self.push_block(Block::List(list), blocks, containers)?;
            }
            TagEnd::Item => {
                let Some(Container::ListItem(item)) = containers.pop() else {
                    return Err(CoreRsError::InvalidMarkdown(
                        "list item close without open".to_string(),
                    ));
                };
                let Some(Container::List(list)) = containers.last_mut() else {
                    return Err(CoreRsError::InvalidMarkdown(
                        "list item close without list".to_string(),
                    ));
                };
                list.items.push(item);
            }
            TagEnd::Table => {
                let Some(Container::Table(table)) = containers.pop() else {
                    return Err(CoreRsError::InvalidMarkdown("table close without open".to_string()));
                };
                self.push_block(
                    Block::Table(TableBlock {
                        aligns: table.aligns,
                        headers: table.headers,
                        rows: table.rows,
                    }),
                    blocks,
                    containers,
                )?;
            }
            TagEnd::TableHead => {
                let Some(Container::TableHead(headers)) = containers.pop() else {
                    return Err(CoreRsError::InvalidMarkdown(
                        "table head close without open".to_string(),
                    ));
                };
                current_table_mut(containers)?.headers = headers;
            }
            TagEnd::TableRow => {
                let Some(Container::TableRow(row)) = containers.pop() else {
                    return Err(CoreRsError::InvalidMarkdown(
                        "table row close without open".to_string(),
                    ));
                };
                current_table_mut(containers)?.rows.push(TableRow { cells: row.cells });
            }
            TagEnd::TableCell => {
                let Some(Container::TableCell(cell)) = containers.pop() else {
                    return Err(CoreRsError::InvalidMarkdown(
                        "table cell close without open".to_string(),
                    ));
                };
                match containers.last_mut() {
                    Some(Container::TableRow(row)) => row.cells.push(cell),
                    Some(Container::TableHead(headers)) => headers.push(cell),
                    _ => {
                        return Err(CoreRsError::InvalidMarkdown(
                            "table cell close without row".to_string(),
                        ));
                    }
                }
            }
            TagEnd::Emphasis => {
                let Some(InlineContext::Emphasis(content)) = inline_stack.pop() else {
                    return Err(CoreRsError::InvalidMarkdown(
                        "emphasis close without open".to_string(),
                    ));
                };
                self.push_inline_or_code(
                    Inline::Emphasis(content),
                    None,
                    blocks,
                    containers,
                    inline_stack,
                )?;
            }
            TagEnd::Strong => {
                let Some(InlineContext::Strong(content)) = inline_stack.pop() else {
                    return Err(CoreRsError::InvalidMarkdown(
                        "strong close without open".to_string(),
                    ));
                };
                self.push_inline_or_code(
                    Inline::Strong(content),
                    None,
                    blocks,
                    containers,
                    inline_stack,
                )?;
            }
            TagEnd::Link => {
                let Some(InlineContext::Link { url, title, content }) = inline_stack.pop() else {
                    return Err(CoreRsError::InvalidMarkdown("link close without open".to_string()));
                };
                let _ = title;
                match validate_link_url(&url) {
                    Ok(()) => {
                        self.push_inline_or_code(
                            Inline::Link { text: content, url },
                            None,
                            blocks,
                            containers,
                            inline_stack,
                        )?;
                    }
                    Err(_) if !self.strict_mode => {
                        let fallback_text = flatten_inline_text(&content);
                        let fallback_text = if fallback_text.is_empty() {
                            url
                        } else {
                            fallback_text
                        };
                        self.push_fallback_text(
                            fallback_text,
                            blocks,
                            containers,
                            inline_stack,
                        )?;
                    }
                    Err(error) => return Err(error),
                }
            }
            TagEnd::Image => {
                let Some(InlineContext::Image { url, title, alt }) = inline_stack.pop() else {
                    return Err(CoreRsError::InvalidMarkdown("image close without open".to_string()));
                };
                let alt_text = flatten_inline_text(&alt);
                match parse_data_uri(&url) {
                    Ok((mime_type, extension, data)) => {
                        if total_embedded_image_bytes.saturating_add(data.len())
                            > MAX_TOTAL_DATA_URI_IMAGE_BYTES
                        {
                            let error = DataImageError::TooLarge {
                                message: format!(
                                    "total embedded image bytes exceed maximum supported size of {MAX_TOTAL_DATA_URI_IMAGE_BYTES} bytes"
                                ),
                            };
                            if self.strict_mode {
                                return Err(error.into_core_error());
                            }
                            self.push_fallback_text(
                                image_fallback_text(&alt_text, &url, error.fallback_kind()),
                                blocks,
                                containers,
                                inline_stack,
                            )?;
                        } else {
                            *total_embedded_image_bytes += data.len();
                            self.push_inline_or_code(
                                Inline::Image(ImageData {
                                    alt: alt_text,
                                    title,
                                    mime_type,
                                    extension,
                                    data,
                                }),
                                None,
                                blocks,
                                containers,
                                inline_stack,
                            )?;
                        }
                    }
                    Err(error) if !self.strict_mode => {
                        self.push_fallback_text(
                            image_fallback_text(&alt_text, &url, error.fallback_kind()),
                            blocks,
                            containers,
                            inline_stack,
                        )?;
                    }
                    Err(error) => return Err(error.into_core_error()),
                }
            }
            TagEnd::DefinitionList
            | TagEnd::DefinitionListTitle
            | TagEnd::DefinitionListDefinition
            | TagEnd::MetadataBlock(_)
            | TagEnd::Strikethrough
            | TagEnd::HtmlBlock
            | TagEnd::FootnoteDefinition => {}
        }

        Ok(())
    }

    pub(super) fn handle_unsupported_event(
        &self,
        event: Event<'_>,
        blocks: &mut Vec<Block>,
        containers: &mut [Container],
        unsupported_block_depth: &mut usize,
        unsupported_block_content: &mut Vec<Inline>,
    ) -> Result<(), CoreRsError> {
        match event {
            Event::Start(tag) => {
                if matches!(tag, Tag::Item) && !unsupported_block_content.is_empty() {
                    append_fallback_separator(unsupported_block_content);
                }
                if unsupported_block_nesting_tag(&tag) {
                    *unsupported_block_depth += 1;
                }
            }
            Event::End(tag) => {
                if unsupported_block_nesting_end(&tag) {
                    *unsupported_block_depth -= 1;
                    if *unsupported_block_depth == 0 {
                        trim_trailing_fallback_whitespace(unsupported_block_content);
                        if !unsupported_block_content.is_empty() {
                            self.push_block(
                                Block::Paragraph(std::mem::take(unsupported_block_content)),
                                blocks,
                                containers,
                            )?;
                        }
                    }
                }
            }
            Event::Text(text)
            | Event::Code(text)
            | Event::Html(text)
            | Event::InlineHtml(text)
            | Event::InlineMath(text)
            | Event::DisplayMath(text) => {
                push_inline_merged(unsupported_block_content, Inline::Text(text.into_string()));
            }
            Event::SoftBreak => {
                push_inline_merged(unsupported_block_content, Inline::Text(" ".to_string()));
            }
            Event::HardBreak => unsupported_block_content.push(Inline::HardBreak),
            Event::Rule => {
                push_inline_merged(unsupported_block_content, Inline::Text("---".to_string()));
            }
            Event::FootnoteReference(reference) => {
                push_inline_merged(
                    unsupported_block_content,
                    Inline::Text(format!("[^{reference}]")),
                );
            }
            Event::TaskListMarker(checked) => {
                push_inline_merged(
                    unsupported_block_content,
                    Inline::Text(if checked {
                        "[x] ".to_string()
                    } else {
                        "[ ] ".to_string()
                    }),
                );
            }
        }

        Ok(())
    }
}

fn validate_link_url(url: &str) -> Result<(), CoreRsError> {
    let Some((scheme, _)) = url.split_once(':') else {
        return Err(CoreRsError::UnsupportedFeature(
            "relative link targets".to_string(),
        ));
    };
    let normalized = scheme.to_ascii_lowercase();
    if matches!(normalized.as_str(), "http" | "https" | "mailto") {
        Ok(())
    } else {
        Err(CoreRsError::UnsupportedFeature(format!(
            "unsupported link scheme: {scheme}"
        )))
    }
}
