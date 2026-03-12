use std::collections::{BTreeMap, BTreeSet};

use roxmltree::Document as XmlDocument;

use crate::error::CoreRsError;
use crate::utils::xml_helper::required_u32_attr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CharStyle {
    Plain,
    Bold,
    Italic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ListKind {
    Unordered,
    Ordered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ListInfo {
    pub(super) kind: ListKind,
    pub(super) depth: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(super) struct ParagraphContract {
    pub(super) quote_depth: u8,
    pub(super) legacy_quote: bool,
    pub(super) list: Option<ListInfo>,
}

#[derive(Debug, Default)]
pub(super) struct Registry {
    paragraphs: BTreeMap<u32, ParagraphContract>,
    headings: BTreeMap<u32, u8>,
    chars: BTreeMap<u32, CharStyle>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HeadingType {
    None,
    Outline,
    Bullet,
    Number,
}

#[derive(Debug, Clone, Copy)]
struct ParaDefinition {
    id: u32,
    heading: HeadingType,
    heading_level: u8,
    heading_id_ref: u32,
    left_margin: Option<u32>,
    border_fill_id_ref: Option<u32>,
}

pub(super) fn parse(header_xml: &str) -> Result<Registry, CoreRsError> {
    let document = XmlDocument::parse(header_xml).map_err(|error| {
        CoreRsError::InvalidHwpx(format!("invalid Contents/header.xml XML: {error}"))
    })?;

    let para_definitions = parse_para_definitions(&document)?;
    let style_definitions = parse_style_heading_levels(&document, &para_definitions)?;
    let char_definitions = parse_char_styles(&document)?;

    let bullet_ids = collect_element_ids(&document, "bullet")?;
    let numbering_ids = collect_element_ids(&document, "numbering")?;

    let mut paragraphs = BTreeMap::new();
    for definition in &para_definitions {
        paragraphs.insert(definition.id, ParagraphContract::default());
    }

    apply_quote_contracts(&mut paragraphs, &para_definitions)?;
    apply_unordered_list_contracts(&mut paragraphs, &para_definitions, &bullet_ids)?;
    apply_ordered_list_contracts(&mut paragraphs, &para_definitions, &numbering_ids)?;

    Ok(Registry {
        paragraphs,
        headings: style_definitions,
        chars: char_definitions,
    })
}

impl Registry {
    pub(super) fn paragraph(&self, para_pr_id: u32) -> Result<ParagraphContract, CoreRsError> {
        self.paragraphs.get(&para_pr_id).copied().ok_or_else(|| {
            CoreRsError::InvalidHwpx(format!(
                "section paragraph references unknown paraPrIDRef {para_pr_id}"
            ))
        })
    }

    pub(super) fn heading_level(&self, style_id: u32) -> Option<u8> {
        self.headings.get(&style_id).copied()
    }

    pub(super) fn char_style(&self, char_pr_id: u32) -> CharStyle {
        self.chars
            .get(&char_pr_id)
            .copied()
            .unwrap_or(CharStyle::Plain)
    }
}

fn parse_para_definitions(document: &XmlDocument<'_>) -> Result<Vec<ParaDefinition>, CoreRsError> {
    let mut definitions = Vec::new();
    for node in document
        .descendants()
        .filter(|node| node.is_element() && node.tag_name().name() == "paraPr")
    {
        let id = required_u32_attr(node, "id", "hh:paraPr")?;
        let heading_node = find_descendant(node, "heading");
        let heading = match heading_node.and_then(|heading| heading.attribute("type")) {
            Some("OUTLINE") => HeadingType::Outline,
            Some("BULLET") => HeadingType::Bullet,
            Some("NUMBER") => HeadingType::Number,
            _ => HeadingType::None,
        };
        let heading_level = heading_node
            .and_then(|heading| heading.attribute("level"))
            .and_then(|value| value.parse::<u8>().ok())
            .unwrap_or(0);
        let heading_id_ref = heading_node
            .and_then(|heading| heading.attribute("idRef"))
            .and_then(|value| value.parse::<u32>().ok())
            .unwrap_or(0);
        let left_margin = find_descendant(node, "left")
            .and_then(|left| left.attribute("value"))
            .and_then(|value| value.parse::<u32>().ok());
        let border_fill_id_ref = find_descendant(node, "border")
            .and_then(|border| border.attribute("borderFillIDRef"))
            .and_then(|value| value.parse::<u32>().ok());

        definitions.push(ParaDefinition {
            id,
            heading,
            heading_level,
            heading_id_ref,
            left_margin,
            border_fill_id_ref,
        });
    }

    Ok(definitions)
}

fn parse_style_heading_levels(
    document: &XmlDocument<'_>,
    para_definitions: &[ParaDefinition],
) -> Result<BTreeMap<u32, u8>, CoreRsError> {
    let para_by_id = para_definitions
        .iter()
        .map(|definition| (definition.id, *definition))
        .collect::<BTreeMap<_, _>>();

    let mut headings = BTreeMap::new();
    for node in document
        .descendants()
        .filter(|node| node.is_element() && node.tag_name().name() == "style")
    {
        if node.attribute("type") != Some("PARA") {
            continue;
        }
        let style_id = required_u32_attr(node, "id", "hh:style")?;
        let para_pr_id_ref = required_u32_attr(node, "paraPrIDRef", "hh:style")?;
        let Some(para_definition) = para_by_id.get(&para_pr_id_ref) else {
            return Err(CoreRsError::InvalidHwpx(format!(
                "style {style_id} references unknown paraPrIDRef {para_pr_id_ref}"
            )));
        };
        if para_definition.heading == HeadingType::Outline {
            headings.insert(style_id, para_definition.heading_level.saturating_add(1));
        }
    }

    Ok(headings)
}

fn parse_char_styles(document: &XmlDocument<'_>) -> Result<BTreeMap<u32, CharStyle>, CoreRsError> {
    let mut chars = BTreeMap::new();
    for node in document
        .descendants()
        .filter(|node| node.is_element() && node.tag_name().name() == "charPr")
    {
        let id = required_u32_attr(node, "id", "hh:charPr")?;
        let style = if has_direct_child(node, "bold") {
            CharStyle::Bold
        } else if has_direct_child(node, "italic") {
            CharStyle::Italic
        } else {
            CharStyle::Plain
        };
        chars.insert(id, style);
    }
    Ok(chars)
}

fn apply_quote_contracts(
    paragraphs: &mut BTreeMap<u32, ParagraphContract>,
    para_definitions: &[ParaDefinition],
) -> Result<(), CoreRsError> {
    let mut modern_quotes = para_definitions
        .iter()
        .filter(|definition| definition.border_fill_id_ref == Some(4))
        .copied()
        .collect::<Vec<_>>();
    modern_quotes.sort_by_key(|definition| definition.left_margin.unwrap_or(u32::MAX));

    if modern_quotes.len() > 2 {
        return Err(CoreRsError::UnsupportedFeature(
            "HWPX parser supports quote depth up to 2".to_string(),
        ));
    }

    for (index, definition) in modern_quotes.iter().enumerate() {
        if let Some(contract) = paragraphs.get_mut(&definition.id) {
            contract.quote_depth = index as u8 + 1;
        }
    }

    for definition in para_definitions {
        if definition.border_fill_id_ref == Some(3)
            && definition.heading == HeadingType::None
            && definition.left_margin.unwrap_or(0) >= 2_000
        {
            if let Some(contract) = paragraphs.get_mut(&definition.id) {
                contract.quote_depth = 1;
                contract.legacy_quote = true;
            }
        }
    }

    Ok(())
}

fn apply_unordered_list_contracts(
    paragraphs: &mut BTreeMap<u32, ParagraphContract>,
    para_definitions: &[ParaDefinition],
    bullet_ids: &BTreeSet<u32>,
) -> Result<(), CoreRsError> {
    let mut unordered = para_definitions
        .iter()
        .filter(|definition| definition.heading == HeadingType::Bullet)
        .copied()
        .collect::<Vec<_>>();
    if unordered.is_empty() {
        return Ok(());
    }
    if !bullet_ids.contains(&1) {
        return Err(CoreRsError::InvalidHwpx(
            "bullet paragraph contract is missing hh:bullet id=\"1\"".to_string(),
        ));
    }

    unordered.sort_by_key(|definition| definition.left_margin.unwrap_or(u32::MAX));
    if unordered.len() > 2 {
        return Err(CoreRsError::UnsupportedFeature(
            "HWPX parser supports unordered list depth up to 2".to_string(),
        ));
    }

    for (index, definition) in unordered.iter().enumerate() {
        if let Some(contract) = paragraphs.get_mut(&definition.id) {
            contract.list = Some(ListInfo {
                kind: ListKind::Unordered,
                depth: index as u8 + 1,
            });
        }
    }

    Ok(())
}

fn apply_ordered_list_contracts(
    paragraphs: &mut BTreeMap<u32, ParagraphContract>,
    para_definitions: &[ParaDefinition],
    numbering_ids: &BTreeSet<u32>,
) -> Result<(), CoreRsError> {
    let mut ordered = para_definitions
        .iter()
        .filter(|definition| definition.heading == HeadingType::Number)
        .copied()
        .collect::<Vec<_>>();
    if ordered.is_empty() {
        return Ok(());
    }

    for definition in &ordered {
        if !numbering_ids.contains(&definition.heading_id_ref) {
            return Err(CoreRsError::InvalidHwpx(format!(
                "ordered list paragraph contract references missing numbering {}",
                definition.heading_id_ref
            )));
        }
    }

    ordered.sort_by_key(|definition| definition.left_margin.unwrap_or(u32::MAX));
    if ordered.len() > 2 {
        return Err(CoreRsError::UnsupportedFeature(
            "HWPX parser supports ordered list depth up to 2".to_string(),
        ));
    }

    for (index, definition) in ordered.iter().enumerate() {
        if let Some(contract) = paragraphs.get_mut(&definition.id) {
            contract.list = Some(ListInfo {
                kind: ListKind::Ordered,
                depth: index as u8 + 1,
            });
        }
    }

    Ok(())
}

fn collect_element_ids(
    document: &XmlDocument<'_>,
    name: &str,
) -> Result<BTreeSet<u32>, CoreRsError> {
    document
        .descendants()
        .filter(|node| node.is_element() && node.tag_name().name() == name)
        .map(|node| required_u32_attr(node, "id", name))
        .collect()
}

fn find_descendant<'a, 'input>(
    node: roxmltree::Node<'a, 'input>,
    name: &str,
) -> Option<roxmltree::Node<'a, 'input>> {
    node.descendants()
        .find(|candidate| candidate.is_element() && candidate.tag_name().name() == name)
}

fn has_direct_child(node: roxmltree::Node<'_, '_>, name: &str) -> bool {
    node.children()
        .any(|child| child.is_element() && child.tag_name().name() == name)
}
