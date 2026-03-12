use roxmltree::Document as XmlDocument;

use crate::error::CoreRsError;
use crate::utils::xml_helper::normalized_node_text;

use super::RawHwpxMetadata;

pub(super) fn parse(content_hpf: &str) -> Result<RawHwpxMetadata, CoreRsError> {
    let document = XmlDocument::parse(content_hpf).map_err(|error| {
        CoreRsError::InvalidHwpx(format!("invalid Contents/content.hpf XML: {error}"))
    })?;

    let metadata = document
        .descendants()
        .find(|node| node.is_element() && node.tag_name().name() == "metadata")
        .ok_or_else(|| {
            CoreRsError::InvalidHwpx("missing opf:metadata in Contents/content.hpf".to_string())
        })?;

    Ok(RawHwpxMetadata {
        title: direct_child_text(metadata, "title"),
        language: direct_child_text(metadata, "language"),
        creator: meta_text(metadata, "creator"),
        lastsaveby: meta_text(metadata, "lastsaveby"),
    })
}

fn direct_child_text(node: roxmltree::Node<'_, '_>, name: &str) -> Option<String> {
    node.children()
        .find(|child| child.is_element() && child.tag_name().name() == name)
        .map(normalized_node_text)
        .filter(|value| !value.is_empty())
}

fn meta_text(node: roxmltree::Node<'_, '_>, meta_name: &str) -> Option<String> {
    node.children()
        .find(|child| {
            child.is_element()
                && child.tag_name().name() == "meta"
                && child.attribute("name") == Some(meta_name)
        })
        .map(normalized_node_text)
        .filter(|value| !value.is_empty())
}
