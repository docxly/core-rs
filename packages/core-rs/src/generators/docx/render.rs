use crate::error::CoreRsError;
use crate::models::block::{Block, ListBlock, ListItem, TableAlignment, TableBlock};
use crate::models::inline::{ImageData, Inline};
use crate::utils::image_helper::image_dimensions;
use crate::utils::xml_helper::escape_attr;

use super::xml_builder;

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
pub struct ImageAsset {
    pub relationship_id: String,
    pub target: String,
    pub extension: String,
    pub mime_type: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RunStyle {
    pub bold: bool,
    pub italic: bool,
    pub code: bool,
    pub hyperlink: bool,
}

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
    for block in blocks {
        match block {
            Block::Heading { level, .. } => {
                max_level = max_level.max(*level);
            }
            Block::BlockQuote(children) => {
                max_level = max_level.max(max_heading_level(children));
            }
            Block::List(list) => {
                for item in &list.items {
                    max_level = max_level.max(max_heading_level(&item.blocks));
                }
            }
            Block::Paragraph(_) | Block::CodeBlock { .. } | Block::Table(_) | Block::ThematicBreak => {}
        }
    }
    max_level
}

pub(crate) fn run_properties(style: RunStyle) -> String {
    let mut parts = Vec::new();
    if style.bold {
        parts.push("<w:b/>".to_string());
    }
    if style.italic {
        parts.push("<w:i/>".to_string());
    }
    if style.code {
        parts.push("<w:rFonts w:ascii=\"Courier New\" w:hAnsi=\"Courier New\"/>".to_string());
    }
    if style.hyperlink {
        parts.push("<w:rStyle w:val=\"Hyperlink\"/>".to_string());
    }

    if parts.is_empty() {
        return String::new();
    }

    format!("<w:rPr>{}</w:rPr>", parts.join(""))
}

pub(crate) fn preserve_space(text: &str) -> bool {
    text.starts_with(' ') || text.ends_with(' ')
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
        Block::Paragraph(content) => {
            render_text_paragraph(content, None, left_indent, extra_paragraph_properties, context, prefix)
        }
        Block::Heading { level, content } => render_text_paragraph(
            content,
            Some(&format!("Heading{}", level)),
            left_indent,
            extra_paragraph_properties,
            context,
            prefix,
        ),
        Block::BlockQuote(blocks) => {
            render_block_quote(blocks, context, left_indent, prefix, extra_paragraph_properties)
        }
        Block::CodeBlock { code, .. } => {
            render_code_block_with_properties(code, left_indent, prefix, extra_paragraph_properties)
        }
        Block::List(list) => render_list(list, context, left_indent, prefix, extra_paragraph_properties),
        Block::Table(table) => {
            render_table(table, context, left_indent, prefix, extra_paragraph_properties)
        }
        Block::ThematicBreak => {
            let content = prefix
                .map(|value| xml_builder::text_run(value, RunStyle::default()))
                .unwrap_or_default();
            let properties = thematic_break_properties(left_indent, extra_paragraph_properties);
            Ok(xml_builder::paragraph_with_properties(&properties, &content))
        }
    }
}

fn render_text_paragraph(
    content: &[Inline],
    style_id: Option<&str>,
    left_indent: usize,
    extra_properties: &str,
    context: &mut RenderContext,
    prefix: Option<&str>,
) -> Result<String, CoreRsError> {
    let mut properties = String::new();
    if let Some(style_id) = style_id {
        properties.push_str(&format!("<w:pStyle w:val=\"{}\"/>", style_id));
    }
    if left_indent > 0 {
        properties.push_str(&format!("<w:ind w:left=\"{}\"/>", left_indent));
    }
    properties.push_str(extra_properties);

    let mut runs = String::new();
    if let Some(prefix) = prefix {
        runs.push_str(&xml_builder::text_run(prefix, RunStyle::default()));
    }
    runs.push_str(&render_inlines(content, context, RunStyle::default())?);
    Ok(xml_builder::paragraph_with_properties(&properties, &runs))
}

fn render_block_quote(
    blocks: &[Block],
    context: &mut RenderContext,
    left_indent: usize,
    prefix: Option<&str>,
    inherited_paragraph_properties: &str,
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
                    None,
                    quote_indent,
                    &quote_props,
                    context,
                    prefix,
                )?);
                prefix = None;
            }
            Block::Heading { level, content } => {
                xml.push_str(&render_text_paragraph(
                    content,
                    Some(&format!("Heading{}", level)),
                    quote_indent,
                    &quote_props,
                    context,
                    prefix,
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

fn render_code_block_with_properties(
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
    Ok(xml_builder::paragraph_with_properties(&properties, &content))
}

fn render_list(
    list: &ListBlock,
    context: &mut RenderContext,
    left_indent: usize,
    prefix_override: Option<&str>,
    extra_paragraph_properties: &str,
) -> Result<String, CoreRsError> {
    let mut xml = String::new();
    for (index, item) in list.items.iter().enumerate() {
        let prefix = if index == 0 {
            prefix_override
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| list_prefix(list, left_indent, index))
        } else {
            list_prefix(list, left_indent, index)
        };
        xml.push_str(&render_list_item(
            item,
            &prefix,
            context,
            left_indent + 720,
            extra_paragraph_properties,
        )?);
    }
    Ok(xml)
}

fn render_list_item(
    item: &ListItem,
    prefix: &str,
    context: &mut RenderContext,
    left_indent: usize,
    extra_paragraph_properties: &str,
) -> Result<String, CoreRsError> {
    let mut xml = String::new();
    let mut prefix_consumed = false;

    for block in &item.blocks {
        match block {
            Block::Paragraph(content) if !prefix_consumed => {
                xml.push_str(&render_text_paragraph(
                    content,
                    None,
                    0,
                    &format!(
                        "<w:ind w:left=\"{}\" w:hanging=\"360\"/>{}",
                        left_indent, extra_paragraph_properties
                    ),
                    context,
                    Some(prefix),
                )?);
                prefix_consumed = true;
            }
            Block::Heading { level, content } if !prefix_consumed => {
                let extra = format!(
                    "<w:pStyle w:val=\"Heading{}\"/><w:ind w:left=\"{}\" w:hanging=\"360\"/>{}",
                    level, left_indent, extra_paragraph_properties
                );
                xml.push_str(&render_text_paragraph(content, None, 0, &extra, context, Some(prefix))?);
                prefix_consumed = true;
            }
            Block::List(list) if !prefix_consumed => {
                xml.push_str(&render_list_marker_paragraph(
                    prefix,
                    left_indent,
                    extra_paragraph_properties,
                ));
                xml.push_str(&render_list(list, context, left_indent, None, extra_paragraph_properties)?);
                prefix_consumed = true;
            }
            other => {
                xml.push_str(&render_block_with_prefix(
                    other,
                    context,
                    left_indent,
                    if prefix_consumed { None } else { Some(prefix) },
                    extra_paragraph_properties,
                )?);
                prefix_consumed = true;
            }
        }
    }

    Ok(xml)
}

fn render_table(
    table: &TableBlock,
    context: &mut RenderContext,
    left_indent: usize,
    prefix: Option<&str>,
    extra_paragraph_properties: &str,
) -> Result<String, CoreRsError> {
    let mut xml = String::new();
    if let Some(prefix) = prefix {
        xml.push_str(&render_list_marker_paragraph(
            prefix,
            left_indent,
            extra_paragraph_properties,
        ));
    }

    let mut table_xml_rows = String::new();
    table_xml_rows.push_str(&render_table_row(&table.headers, &table.aligns, true, context)?);
    for row in &table.rows {
        table_xml_rows.push_str(&render_table_row(&row.cells, &table.aligns, false, context)?);
    }

    let mut grid = String::new();
    for _ in 0..table.aligns.len().max(table.headers.len()) {
        grid.push_str("<w:gridCol w:w=\"2400\"/>");
    }

    let left_border = if has_quote_border(extra_paragraph_properties) {
        "<w:left w:val=\"single\" w:sz=\"8\" w:space=\"8\" w:color=\"B7B7B7\"/>"
    } else {
        "<w:left w:val=\"single\" w:sz=\"4\" w:space=\"0\" w:color=\"A0A0A0\"/>"
    };
    let mut table_properties = format!(
        "<w:tblW w:w=\"0\" w:type=\"auto\"/><w:tblBorders><w:top w:val=\"single\" w:sz=\"4\" w:space=\"0\" w:color=\"A0A0A0\"/>{left_border}<w:bottom w:val=\"single\" w:sz=\"4\" w:space=\"0\" w:color=\"A0A0A0\"/><w:right w:val=\"single\" w:sz=\"4\" w:space=\"0\" w:color=\"A0A0A0\"/><w:insideH w:val=\"single\" w:sz=\"4\" w:space=\"0\" w:color=\"D0D0D0\"/><w:insideV w:val=\"single\" w:sz=\"4\" w:space=\"0\" w:color=\"D0D0D0\"/></w:tblBorders>",
    );
    if left_indent > 0 {
        table_properties.push_str(&format!("<w:tblInd w:w=\"{}\" w:type=\"dxa\"/>", left_indent));
    }

    xml.push_str(&format!(
        "<w:tbl><w:tblPr>{properties}</w:tblPr><w:tblGrid>{grid}</w:tblGrid>{rows}</w:tbl>",
        properties = table_properties,
        grid = grid,
        rows = table_xml_rows
    ));

    Ok(xml)
}

fn render_table_row(
    cells: &[Vec<Inline>],
    aligns: &[TableAlignment],
    header: bool,
    context: &mut RenderContext,
) -> Result<String, CoreRsError> {
    let mut xml = String::new();
    for (index, cell) in cells.iter().enumerate() {
        let alignment = aligns.get(index).copied().unwrap_or(TableAlignment::None);
        let paragraph_props = match alignment {
            TableAlignment::None => String::new(),
            TableAlignment::Left => "<w:jc w:val=\"left\"/>".to_string(),
            TableAlignment::Center => "<w:jc w:val=\"center\"/>".to_string(),
            TableAlignment::Right => "<w:jc w:val=\"right\"/>".to_string(),
        };
        let runs = render_inlines(
            cell,
            context,
            RunStyle {
                bold: header,
                ..RunStyle::default()
            },
        )?;
        let paragraph = xml_builder::paragraph_with_properties(&paragraph_props, &runs);
        let cell_props = if header {
            "<w:shd w:val=\"clear\" w:color=\"auto\" w:fill=\"EFEFEF\"/>"
        } else {
            ""
        };
        xml.push_str(&format!("<w:tc><w:tcPr>{}</w:tcPr>{}</w:tc>", cell_props, paragraph));
    }
    Ok(format!("<w:tr>{}</w:tr>", xml))
}

fn render_inlines(
    inlines: &[Inline],
    context: &mut RenderContext,
    inherited: RunStyle,
) -> Result<String, CoreRsError> {
    let mut xml = String::new();
    for inline in inlines {
        match inline {
            Inline::Text(text) => xml.push_str(&xml_builder::text_run(text, inherited)),
            Inline::Code(code) => xml.push_str(&xml_builder::text_run(
                code,
                RunStyle {
                    code: true,
                    ..inherited
                },
            )),
            Inline::Emphasis(content) => xml.push_str(&render_inlines(
                content,
                context,
                RunStyle {
                    italic: true,
                    ..inherited
                },
            )?),
            Inline::Strong(content) => xml.push_str(&render_inlines(
                content,
                context,
                RunStyle {
                    bold: true,
                    ..inherited
                },
            )?),
            Inline::Link { text, url } => {
                let relationship_id = format!("rLink{}", context.hyperlinks.len() + 1);
                context.hyperlinks.push(url.clone());
                let content_xml = render_inlines(
                    text,
                    context,
                    RunStyle {
                        hyperlink: true,
                        ..inherited
                    },
                )?;
                xml.push_str(&format!(
                    "<w:hyperlink r:id=\"{}\" w:history=\"1\">{}</w:hyperlink>",
                    escape_attr(&relationship_id),
                    content_xml
                ));
            }
            Inline::Image(image) => xml.push_str(&render_image(image, context)?),
            Inline::HardBreak => xml.push_str("<w:r><w:br/></w:r>"),
        }
    }
    Ok(xml)
}

fn render_list_marker_paragraph(
    prefix: &str,
    left_indent: usize,
    extra_paragraph_properties: &str,
) -> String {
    let mut properties = String::new();
    if left_indent > 0 {
        properties.push_str(&format!("<w:ind w:left=\"{}\" w:hanging=\"360\"/>", left_indent));
    }
    properties.push_str(extra_paragraph_properties);
    let marker = format!("{prefix}\u{200B}");
    let runs = xml_builder::text_run(&marker, RunStyle::default());
    xml_builder::paragraph_with_properties(&properties, &runs)
}

fn join_properties(first: &str, second: &str) -> String {
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

fn has_quote_border(properties: &str) -> bool {
    properties.contains(QUOTE_LEFT_BORDER_FRAGMENT)
}

fn render_image(image: &ImageData, context: &mut RenderContext) -> Result<String, CoreRsError> {
    let image_index = context.images.len() + 1;
    let relationship_id = format!("rImage{}", context.images.len() + 1);
    let target = format!("word/media/image{}.{}", image_index, image.extension);
    let (width, height) = image_dimensions(image)?;
    context.images.push(ImageAsset {
        relationship_id: relationship_id.clone(),
        target: target.clone(),
        extension: image.extension.clone(),
        mime_type: image.mime_type.clone(),
        data: image.data.clone(),
    });

    Ok(xml_builder::image_run(
        &relationship_id,
        &image.alt,
        image_index,
        image_index,
        width as i64 * 9_525,
        height as i64 * 9_525,
    ))
}

fn list_prefix(list: &ListBlock, left_indent: usize, index: usize) -> String {
    if list.ordered {
        format!("{}. ", list.start_index + index as u64)
    } else if left_indent == 0 {
        "• ".to_string()
    } else {
        "◦ ".to_string()
    }
}
