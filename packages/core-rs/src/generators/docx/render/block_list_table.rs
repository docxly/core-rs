use crate::error::CoreRsError;
use crate::models::block::{Block, ListBlock, ListItem, TableAlignment, TableBlock};
use crate::models::inline::Inline;

use super::{ParagraphRenderOptions, RenderContext, heading_style_id};
use super::text::{has_quote_border, render_text_paragraph};
use super::super::{RunStyle, inline::render_inlines};
use crate::generators::docx::xml_builder;

pub(super) fn render_list(
    list: &ListBlock,
    context: &mut RenderContext,
    left_indent: usize,
    prefix_override: Option<&str>,
    extra_paragraph_properties: &str,
    render_block_with_prefix: impl Fn(
        &Block,
        &mut RenderContext,
        usize,
        Option<&str>,
        &str,
    ) -> Result<String, CoreRsError>
        + Copy,
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
            render_block_with_prefix,
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
    render_block_with_prefix: impl Fn(
        &Block,
        &mut RenderContext,
        usize,
        Option<&str>,
        &str,
    ) -> Result<String, CoreRsError>
        + Copy,
) -> Result<String, CoreRsError> {
    let mut xml = String::new();
    let mut prefix_consumed = false;

    for block in &item.blocks {
        match block {
            Block::Paragraph(content) if !prefix_consumed => {
                xml.push_str(&render_text_paragraph(
                    content,
                    ParagraphRenderOptions::plain(
                        0,
                        &format!(
                            "<w:ind w:left=\"{}\" w:hanging=\"360\"/>{}",
                            left_indent, extra_paragraph_properties
                        ),
                        Some(prefix),
                    ),
                    context,
                )?);
                prefix_consumed = true;
            }
            Block::Heading { level, content } if !prefix_consumed => {
                let style_id = heading_style_id(*level);
                let extra = format!(
                    "<w:pStyle w:val=\"{}\"/><w:ind w:left=\"{}\" w:hanging=\"360\"/>{}",
                    style_id,
                    left_indent,
                    extra_paragraph_properties
                );
                xml.push_str(&render_text_paragraph(
                    content,
                    ParagraphRenderOptions::plain(0, &extra, Some(prefix)),
                    context,
                )?);
                prefix_consumed = true;
            }
            Block::List(list) if !prefix_consumed => {
                xml.push_str(&render_list_marker_paragraph(
                    prefix,
                    left_indent,
                    extra_paragraph_properties,
                ));
                xml.push_str(&render_list(
                    list,
                    context,
                    left_indent,
                    None,
                    extra_paragraph_properties,
                    render_block_with_prefix,
                )?);
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

pub(super) fn render_table(
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

    let grid = "<w:gridCol w:w=\"2400\"/>".repeat(table.aligns.len().max(table.headers.len()));

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

fn list_prefix(list: &ListBlock, left_indent: usize, index: usize) -> String {
    if list.ordered {
        format!("{}. ", list.start_index + index as u64)
    } else if left_indent == 0 {
        "• ".to_string()
    } else {
        "◦ ".to_string()
    }
}
