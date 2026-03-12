mod document;
mod errors;
mod lists;
mod styles;

use std::path::Path;

use core_rs::{
    ConversionTarget, CoreRsError, HwpxOptions, HwpxParagraphAlign, HwpxStyleOptions,
    IssueSeverity, analyze_markdown, generate_hwpx, generate_hwpx_with_report,
};
use roxmltree::Document as XmlDocument;

use crate::hwpx_contract::{compare_contracts, snapshot_from_bytes, snapshot_from_expected_tree};
use crate::hwpx_fixture::{discover_fixtures, read_expected_entries};
use crate::hwpx_runtime::generate;

fn approved_fixture(name: &str) -> crate::hwpx_fixture::HwpxFixture {
    discover_fixtures()
        .unwrap()
        .into_iter()
        .find(|fixture| fixture.name == name)
        .unwrap_or_else(|| panic!("missing approved HWPX fixture: {name}"))
}

fn text_entry<'a>(
    entries: &'a std::collections::BTreeMap<String, crate::NormalizedEntry>,
    path: &str,
) -> &'a str {
    match entries
        .get(path)
        .unwrap_or_else(|| panic!("missing entry: {path}"))
    {
        crate::NormalizedEntry::Text(text) => text.as_str(),
        crate::NormalizedEntry::BinaryHash(_) => panic!("{path} was normalized as binary"),
    }
}

type HeaderStyleContract = (Vec<u32>, Vec<u32>, Vec<(u32, u32, u32)>);

fn header_style_contract(xml: &str) -> HeaderStyleContract {
    let doc = XmlDocument::parse(xml).unwrap();
    let mut char_ids = Vec::new();
    let mut para_ids = Vec::new();
    let mut style_triplets = Vec::new();

    for node in doc.descendants().filter(|node| node.is_element()) {
        match node.tag_name().name() {
            "charPr" if node.tag_name().namespace().is_some() => {
                if let Some(id) = node
                    .attribute("id")
                    .and_then(|value| value.parse::<u32>().ok())
                {
                    char_ids.push(id);
                }
            }
            "paraPr" if node.tag_name().namespace().is_some() => {
                if let Some(id) = node
                    .attribute("id")
                    .and_then(|value| value.parse::<u32>().ok())
                {
                    para_ids.push(id);
                }
            }
            "style" if node.tag_name().namespace().is_some() => {
                let Some(id) = node
                    .attribute("id")
                    .and_then(|value| value.parse::<u32>().ok())
                else {
                    continue;
                };
                let Some(para_pr) = node
                    .attribute("paraPrIDRef")
                    .and_then(|value| value.parse::<u32>().ok())
                else {
                    continue;
                };
                let Some(char_pr) = node
                    .attribute("charPrIDRef")
                    .and_then(|value| value.parse::<u32>().ok())
                else {
                    continue;
                };
                style_triplets.push((id, para_pr, char_pr));
            }
            _ => {}
        }
    }

    char_ids.sort_unstable();
    char_ids.dedup();
    para_ids.sort_unstable();
    para_ids.dedup();
    style_triplets.sort_unstable();

    (char_ids, para_ids, style_triplets)
}

fn section_style_refs(xml: &str) -> Vec<(u32, u32, Vec<u32>)> {
    let doc = XmlDocument::parse(xml).unwrap();
    let mut paragraphs = Vec::new();

    for paragraph in doc
        .descendants()
        .filter(|node| node.is_element() && node.tag_name().name() == "p")
    {
        let Some(para_pr) = paragraph
            .attribute("paraPrIDRef")
            .and_then(|value| value.parse::<u32>().ok())
        else {
            continue;
        };
        let Some(style_id) = paragraph
            .attribute("styleIDRef")
            .and_then(|value| value.parse::<u32>().ok())
        else {
            continue;
        };

        let mut char_refs = paragraph
            .children()
            .filter(|node| node.is_element() && node.tag_name().name() == "run")
            .filter_map(|run| run.attribute("charPrIDRef"))
            .filter_map(|value| value.parse::<u32>().ok())
            .collect::<Vec<_>>();
        char_refs.sort_unstable();
        char_refs.dedup();
        paragraphs.push((para_pr, style_id, char_refs));
    }

    paragraphs
}
