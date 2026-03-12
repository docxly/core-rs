use crate::error::CoreRsError;
use crate::models::block::{TableAlignment, TableBlock, TableRow};
use crate::models::inline::Inline;

use super::inline::flatten_text_node;

pub(super) fn parse_table(node: roxmltree::Node<'_, '_>) -> Result<TableBlock, CoreRsError> {
    let rows = node
        .children()
        .filter(|child| child.is_element() && child.tag_name().name() == "tr")
        .map(parse_table_row)
        .collect::<Result<Vec<_>, _>>()?;
    if rows.is_empty() {
        return Err(CoreRsError::InvalidHwpx(
            "table must contain at least one row".to_string(),
        ));
    }

    let headers = rows[0].cells.clone();
    let body_rows = rows.into_iter().skip(1).collect::<Vec<_>>();
    let aligns = vec![TableAlignment::None; headers.len()];

    Ok(TableBlock {
        aligns,
        headers,
        rows: body_rows,
    })
}

fn parse_table_row(node: roxmltree::Node<'_, '_>) -> Result<TableRow, CoreRsError> {
    let cells = node
        .children()
        .filter(|child| child.is_element() && child.tag_name().name() == "tc")
        .map(parse_table_cell)
        .collect::<Result<Vec<_>, _>>()?;
    if cells.is_empty() {
        return Err(CoreRsError::InvalidHwpx(
            "table row must contain at least one cell".to_string(),
        ));
    }
    Ok(TableRow { cells })
}

fn parse_table_cell(node: roxmltree::Node<'_, '_>) -> Result<Vec<Inline>, CoreRsError> {
    let mut sublist = None;
    for child in node.children().filter(|child| child.is_element()) {
        match child.tag_name().name() {
            "subList" => {
                if sublist.replace(child).is_some() {
                    return Err(CoreRsError::UnsupportedFeature(
                        "HWPX parser does not support multiple hp:subList elements in a table cell"
                            .to_string(),
                    ));
                }
            }
            "cellAddr" | "cellSpan" | "cellSz" | "cellMargin" => {}
            other => {
                return Err(CoreRsError::UnsupportedFeature(format!(
                    "HWPX parser does not support hp:tc child {other}"
                )));
            }
        }
    }

    let sublist = sublist
        .ok_or_else(|| CoreRsError::InvalidHwpx("table cell is missing hp:subList".to_string()))?;
    let paragraph = parse_cell_paragraph(sublist)?;
    let mut inlines = Vec::new();
    for run in paragraph
        .children()
        .filter(|child| child.is_element() && child.tag_name().name() == "run")
    {
        for child in run.children().filter(|child| child.is_element()) {
            match child.tag_name().name() {
                "t" => {
                    let value = flatten_text_node(child)?;
                    if !value.is_empty() {
                        match inlines.last_mut() {
                            Some(Inline::Text(existing)) => existing.push_str(&value),
                            _ => inlines.push(Inline::Text(value)),
                        }
                    }
                }
                other => {
                    return Err(CoreRsError::UnsupportedFeature(format!(
                        "HWPX parser does not support hp:run child {other} inside table cells"
                    )));
                }
            }
        }
    }
    Ok(inlines)
}

fn parse_cell_paragraph<'a, 'input>(
    sublist: roxmltree::Node<'a, 'input>,
) -> Result<roxmltree::Node<'a, 'input>, CoreRsError> {
    let mut paragraph = None;
    for child in sublist.children().filter(|child| child.is_element()) {
        match child.tag_name().name() {
            "p" => {
                if paragraph.replace(child).is_some() {
                    return Err(CoreRsError::UnsupportedFeature(
                        "HWPX parser does not support multiple paragraphs inside a table cell"
                            .to_string(),
                    ));
                }
            }
            other => {
                return Err(CoreRsError::UnsupportedFeature(format!(
                    "HWPX parser does not support hp:subList child {other} inside table cells"
                )));
            }
        }
    }

    paragraph.ok_or_else(|| CoreRsError::InvalidHwpx("table cell is missing hp:p".to_string()))
}
