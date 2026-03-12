use crate::error::CoreRsError;
use crate::models::block::Block;
use crate::models::inline::Inline;

use super::super::{
    QUOTE_BORDER_PROPERTIES, QUOTE_LEFT_BORDER_FRAGMENT, RunStyle, inline::render_inlines,
};
use super::{ParagraphRenderOptions, RenderContext, heading_style_id};
use crate::generators::docx::xml_builder;

pub(super) fn render_text_paragraph(
    content: &[Inline],
    options: ParagraphRenderOptions<'_>,
    context: &mut RenderContext,
) -> Result<String, CoreRsError> {
    let mut properties = String::new();
    if let Some(style_id) = options.style_id {
        properties.push_str(&format!("<w:pStyle w:val=\"{}\"/>", style_id));
    }
    if options.left_indent > 0 {
        properties.push_str(&format!("<w:ind w:left=\"{}\"/>", options.left_indent));
    }
    properties.push_str(options.extra_properties);

    let mut runs = String::new();
    if let Some(prefix) = options.prefix {
        runs.push_str(&xml_builder::text_run(prefix, RunStyle::default()));
    }
    runs.push_str(&render_inlines(content, context, RunStyle::default())?);
    Ok(xml_builder::paragraph_with_properties(&properties, &runs))
}

pub(super) fn render_block_quote(
    blocks: &[Block],
    context: &mut RenderContext,
    left_indent: usize,
    prefix: Option<&str>,
    inherited_paragraph_properties: &str,
    render_block_with_prefix: impl Fn(
        &Block,
        &mut RenderContext,
        usize,
        Option<&str>,
        &str,
    ) -> Result<String, CoreRsError>,
) -> Result<String, CoreRsError> {
    let mut xml = String::new();
    let quote_indent = left_indent + 720;
    let quote_props = join_properties(inherited_paragraph_properties, QUOTE_BORDER_PROPERTIES);
    let mut prefix = prefix;

    for block in blocks {
        match block {
            Block::Paragraph(content) => {
                xml.push_str(&render_text_paragraph(
                    content,
                    ParagraphRenderOptions::plain(quote_indent, &quote_props, prefix),
                    context,
                )?);
                prefix = None;
            }
            Block::Heading { level, content } => {
                let style_id = heading_style_id(*level);
                xml.push_str(&render_text_paragraph(
                    content,
                    ParagraphRenderOptions::heading(&style_id, quote_indent, &quote_props, prefix),
                    context,
                )?);
                prefix = None;
            }
            _ => {
                xml.push_str(&render_block_with_prefix(
                    block,
                    context,
                    quote_indent,
                    prefix,
                    &quote_props,
                )?);
                prefix = None;
            }
        }
    }

    Ok(xml)
}

pub(super) fn render_code_block_with_properties(
    code: &str,
    left_indent: usize,
    prefix: Option<&str>,
    extra_properties: &str,
) -> Result<String, CoreRsError> {
    let mut content = String::new();
    if let Some(prefix) = prefix {
        content.push_str(&xml_builder::text_run(prefix, RunStyle::default()));
    }
    for (index, line) in code.lines().enumerate() {
        if index > 0 || prefix.is_some() {
            content.push_str("<w:r><w:br/></w:r>");
        }
        content.push_str(&xml_builder::text_run(
            line,
            RunStyle {
                code: true,
                ..RunStyle::default()
            },
        ));
    }

    let properties = format!(
        "<w:ind w:left=\"{}\"/>{}<w:shd w:val=\"clear\" w:color=\"auto\" w:fill=\"F4F4F4\"/><w:spacing w:before=\"120\" w:after=\"120\"/>",
        left_indent + 360,
        extra_properties
    );
    Ok(xml_builder::paragraph_with_properties(
        &properties,
        &content,
    ))
}

pub(super) fn render_thematic_break(
    left_indent: usize,
    prefix: Option<&str>,
    extra_paragraph_properties: &str,
) -> Result<String, CoreRsError> {
    let content = prefix
        .map(|value| xml_builder::text_run(value, RunStyle::default()))
        .unwrap_or_default();
    let properties = thematic_break_properties(left_indent, extra_paragraph_properties);
    Ok(xml_builder::paragraph_with_properties(
        &properties,
        &content,
    ))
}

pub(super) fn join_properties(first: &str, second: &str) -> String {
    if has_quote_border(first) && has_quote_border(second) {
        first.to_string()
    } else {
        let mut result = String::new();
        result.push_str(first);
        result.push_str(second);
        result
    }
}

fn thematic_break_properties(left_indent: usize, extra_paragraph_properties: &str) -> String {
    let mut properties = String::new();
    if left_indent > 0 {
        properties.push_str(&format!("<w:ind w:left=\"{}\"/>", left_indent));
    }
    if has_quote_border(extra_paragraph_properties) {
        properties.push_str(
            "<w:pBdr><w:left w:val=\"single\" w:sz=\"8\" w:space=\"8\" w:color=\"B7B7B7\"/><w:bottom w:val=\"single\" w:sz=\"6\" w:space=\"1\" w:color=\"auto\"/></w:pBdr>",
        );
    } else {
        properties.push_str(extra_paragraph_properties);
        properties.push_str(
            "<w:pBdr><w:bottom w:val=\"single\" w:sz=\"6\" w:space=\"1\" w:color=\"auto\"/></w:pBdr>",
        );
    }
    properties
}

pub(super) fn has_quote_border(properties: &str) -> bool {
    properties.contains(QUOTE_LEFT_BORDER_FRAGMENT)
}
