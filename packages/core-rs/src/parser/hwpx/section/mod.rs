mod inline;
mod table;

use crate::error::CoreRsError;
use crate::models::block::{Block, ListBlock, ListItem};
use crate::models::document::Document;
use crate::utils::xml_helper::required_u32_attr;

use roxmltree::Document as XmlDocument;

use super::registry::{ListInfo, ListKind, Registry};
use inline::{RunState, parse_run};

#[derive(Debug, Clone)]
struct ParagraphRecord {
    quote_depth: u8,
    list: Option<ListInfo>,
    block: Block,
}

pub(super) fn parse(section_xml: &str, registry: &Registry) -> Result<Document, CoreRsError> {
    let document = XmlDocument::parse(section_xml).map_err(|error| {
        CoreRsError::InvalidHwpx(format!("invalid Contents/section0.xml XML: {error}"))
    })?;
    let root = document.root_element();
    if root.tag_name().name() != "sec" {
        return Err(CoreRsError::InvalidHwpx(
            "Contents/section0.xml root element must be hs:sec".to_string(),
        ));
    }

    let mut records = Vec::new();
    for child in root.children().filter(|node| node.is_element()) {
        if child.tag_name().name() != "p" {
            return Err(CoreRsError::UnsupportedFeature(format!(
                "HWPX parser does not support section child {}",
                child.tag_name().name()
            )));
        }
        if let Some(record) = parse_paragraph(child, registry)? {
            records.push(record);
        }
    }

    let mut index = 0;
    let blocks = parse_blocks(&records, &mut index, 0)?;
    if index != records.len() {
        return Err(CoreRsError::InvalidHwpx(
            "failed to consume all parsed HWPX records".to_string(),
        ));
    }

    Ok(Document { blocks })
}

fn parse_paragraph(
    node: roxmltree::Node<'_, '_>,
    registry: &Registry,
) -> Result<Option<ParagraphRecord>, CoreRsError> {
    let para_pr_id = required_u32_attr(node, "paraPrIDRef", "hp:p")?;
    let style_id = required_u32_attr(node, "styleIDRef", "hp:p")?;
    let contract = registry.paragraph(para_pr_id)?;
    let heading_level = registry.heading_level(style_id);
    let mut state = RunState::default();

    for child in node.children().filter(|child| child.is_element()) {
        match child.tag_name().name() {
            "run" => parse_run(
                child,
                registry,
                contract,
                heading_level.is_some(),
                &mut state,
            )?,
            "linesegarray" => {}
            other => {
                return Err(CoreRsError::UnsupportedFeature(format!(
                    "HWPX parser does not support hp:p child {other}"
                )));
            }
        }
    }

    if state.active_hyperlink.is_some() {
        return Err(CoreRsError::InvalidHwpx(
            "hyperlink field is missing hp:fieldEnd".to_string(),
        ));
    }

    let block = if let Some(table) = state.table {
        if contract.list.is_some() || contract.quote_depth > 0 {
            return Err(CoreRsError::UnsupportedFeature(
                "HWPX parser does not support quoted or list tables".to_string(),
            ));
        }
        Block::Table(table)
    } else {
        if state.inlines.is_empty() {
            return Ok(None);
        }
        if let Some(level) = heading_level {
            Block::Heading {
                level,
                content: state.inlines,
            }
        } else {
            Block::Paragraph(state.inlines)
        }
    };

    Ok(Some(ParagraphRecord {
        quote_depth: contract.quote_depth,
        list: contract.list,
        block,
    }))
}

fn parse_blocks(
    records: &[ParagraphRecord],
    index: &mut usize,
    quote_depth: u8,
) -> Result<Vec<Block>, CoreRsError> {
    let mut blocks = Vec::new();
    while *index < records.len() {
        let record = &records[*index];
        if record.quote_depth < quote_depth {
            break;
        }
        if record.quote_depth > quote_depth {
            if record.quote_depth != quote_depth + 1 {
                return Err(CoreRsError::UnsupportedFeature(
                    "HWPX parser supports quote depth up to 2".to_string(),
                ));
            }
            let inner = parse_blocks(records, index, quote_depth + 1)?;
            blocks.push(Block::BlockQuote(inner));
            continue;
        }
        if let Some(list) = record.list {
            blocks.push(Block::List(parse_list(records, index, quote_depth, list)?));
            continue;
        }
        blocks.push(record.block.clone());
        *index += 1;
    }
    Ok(blocks)
}

fn parse_list(
    records: &[ParagraphRecord],
    index: &mut usize,
    quote_depth: u8,
    list: ListInfo,
) -> Result<ListBlock, CoreRsError> {
    let mut items = Vec::new();

    while *index < records.len() {
        let record = &records[*index];
        if record.quote_depth != quote_depth {
            break;
        }
        let Some(record_list) = record.list else {
            break;
        };
        if record_list.kind != list.kind {
            break;
        }
        if record_list.depth < list.depth {
            break;
        }
        if record_list.depth > list.depth {
            return Err(CoreRsError::InvalidHwpx(
                "list nesting cannot begin without a parent list item".to_string(),
            ));
        }

        let mut item = ListItem {
            blocks: vec![record.block.clone()],
        };
        *index += 1;

        while *index < records.len() {
            let next = &records[*index];
            if next.quote_depth != quote_depth {
                break;
            }
            let Some(next_list) = next.list else {
                break;
            };
            if next_list.kind != list.kind {
                break;
            }
            if next_list.depth <= list.depth {
                break;
            }
            if next_list.depth != list.depth + 1 {
                return Err(CoreRsError::UnsupportedFeature(
                    "HWPX parser supports list depth up to 2".to_string(),
                ));
            }
            item.blocks.push(Block::List(parse_list(
                records,
                index,
                quote_depth,
                next_list,
            )?));
        }

        items.push(item);
    }

    Ok(ListBlock {
        ordered: list.kind == ListKind::Ordered,
        start_index: 1,
        items,
    })
}
