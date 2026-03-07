use super::state::{Container, InlineContext, push_inline_merged};
use super::MarkdownParser;
use crate::error::CoreRsError;
use crate::models::block::Block;
use crate::models::inline::Inline;

impl MarkdownParser {
    pub(super) fn push_text(
        &self,
        text: String,
        blocks: &mut Vec<Block>,
        containers: &mut Vec<Container>,
        inline_stack: &mut Vec<InlineContext>,
    ) -> Result<(), CoreRsError> {
        self.push_inline_or_code(Inline::Text(text), None, blocks, containers, inline_stack)
    }

    pub(super) fn push_soft_break(
        &self,
        blocks: &mut Vec<Block>,
        containers: &mut Vec<Container>,
        inline_stack: &mut Vec<InlineContext>,
    ) -> Result<(), CoreRsError> {
        self.push_inline_or_code(
            Inline::Text(" ".to_string()),
            Some('\n'),
            blocks,
            containers,
            inline_stack,
        )
    }

    pub(super) fn push_hard_break(
        &self,
        blocks: &mut Vec<Block>,
        containers: &mut Vec<Container>,
        inline_stack: &mut Vec<InlineContext>,
    ) -> Result<(), CoreRsError> {
        self.push_inline_or_code(Inline::HardBreak, Some('\n'), blocks, containers, inline_stack)
    }

    pub(super) fn push_inline_or_code(
        &self,
        inline: Inline,
        code_break: Option<char>,
        blocks: &mut Vec<Block>,
        containers: &mut Vec<Container>,
        inline_stack: &mut Vec<InlineContext>,
    ) -> Result<(), CoreRsError> {
        if let Some(last_inline) = inline_stack.last_mut() {
            last_inline.push(inline);
            return Ok(());
        }

        if let Some(Container::CodeBlock { code, .. }) = containers.last_mut() {
            match (inline, code_break) {
                (Inline::Text(text), _) | (Inline::Code(text), _) => code.push_str(&text),
                (Inline::HardBreak, Some(break_char)) => code.push(break_char),
                (Inline::HardBreak, None) => code.push('\n'),
                _ => {
                    return Err(CoreRsError::InvalidMarkdown(
                        "invalid inline inside code block".to_string(),
                    ));
                }
            }
            return Ok(());
        }

        if let Some(
            Container::Paragraph(content)
            | Container::TableCell(content)
            | Container::Heading { content, .. },
        ) = containers.last_mut()
        {
            push_inline_merged(content, inline);
            return Ok(());
        }

        match inline {
            Inline::Text(text) if !text.is_empty() => {
                self.push_fallback_text(text, blocks, containers, inline_stack)
            }
            Inline::HardBreak => Err(CoreRsError::InvalidMarkdown(
                "hard break appeared outside a text container".to_string(),
            )),
            _ => Err(CoreRsError::InvalidMarkdown(
                "inline content appeared outside a text container".to_string(),
            )),
        }
    }

    pub(super) fn push_fallback_text(
        &self,
        text: String,
        blocks: &mut Vec<Block>,
        containers: &mut Vec<Container>,
        inline_stack: &mut Vec<InlineContext>,
    ) -> Result<(), CoreRsError> {
        if text.is_empty() {
            return Ok(());
        }

        if !inline_stack.is_empty() {
            self.push_inline_or_code(Inline::Text(text), None, blocks, containers, inline_stack)
        } else if let Some(
            Container::Paragraph(_)
            | Container::Heading { .. }
            | Container::TableCell(_)
            | Container::CodeBlock { .. },
        ) = containers.last()
        {
            self.push_inline_or_code(Inline::Text(text), None, blocks, containers, inline_stack)
        } else {
            self.push_block(Block::Paragraph(vec![Inline::Text(text)]), blocks, containers)
        }
    }

    pub(super) fn push_block(
        &self,
        block: Block,
        blocks: &mut Vec<Block>,
        containers: &mut [Container],
    ) -> Result<(), CoreRsError> {
        if let Some(container) = containers.last_mut() {
            match container {
                Container::BlockQuote(quoted) => quoted.push(block),
                Container::ListItem(item) => item.blocks.push(block),
                _ => {
                    return Err(CoreRsError::InvalidMarkdown(
                        "block appeared in an invalid container".to_string(),
                    ));
                }
            }
        } else {
            blocks.push(block);
        }

        Ok(())
    }
}
