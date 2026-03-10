#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use roxmltree::Document as XmlDocument;

use crate::hwpx_fixture::{FixtureResult, NormalizedEntry, normalized_entries};

#[derive(Debug)]
pub struct HwpxContractSnapshot {
    pub entry_names: Vec<String>,
    pub entries: BTreeMap<String, NormalizedEntry>,
    pub xml_roots: BTreeMap<String, XmlRoot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlRoot {
    pub name: String,
    pub namespace: Option<String>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct HwpxContractDiff {
    pub missing_entries: Vec<String>,
    pub extra_entries: Vec<String>,
    pub entry_order_differs: bool,
    pub mismatched_roots: Vec<String>,
}

pub fn snapshot_from_bytes(bytes: &[u8]) -> FixtureResult<HwpxContractSnapshot> {
    let entry_names = crate::hwpx_fixture::archive_entry_names(bytes)?;
    let entries = normalized_entries(bytes)?;
    Ok(HwpxContractSnapshot {
        xml_roots: xml_roots(&entries)?,
        entry_names,
        entries,
    })
}

pub fn snapshot_from_expected_tree(root: &Path) -> FixtureResult<HwpxContractSnapshot> {
    let entries = crate::hwpx_fixture::read_expected_entries(root)?;
    let mut entry_names = entries.keys().cloned().collect::<Vec<_>>();
    entry_names.sort();
    Ok(HwpxContractSnapshot {
        xml_roots: xml_roots(&entries)?,
        entry_names,
        entries,
    })
}

pub fn snapshot_from_stale_fixture(root: &Path) -> FixtureResult<HwpxContractSnapshot> {
    let bytes = fs::read(root.join("golden.hwpx"))?;
    snapshot_from_bytes(&bytes)
}

pub fn compare_contracts(
    expected: &HwpxContractSnapshot,
    actual: &HwpxContractSnapshot,
) -> HwpxContractDiff {
    let expected_names = expected
        .entry_names
        .iter()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    let actual_names = actual
        .entry_names
        .iter()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();

    let missing_entries = expected_names
        .difference(&actual_names)
        .cloned()
        .collect::<Vec<_>>();
    let extra_entries = actual_names
        .difference(&expected_names)
        .cloned()
        .collect::<Vec<_>>();

    let mut mismatched_roots = Vec::new();
    for path in expected_names.intersection(&actual_names) {
        let Some(expected_root) = expected.xml_roots.get(path.as_str()) else {
            continue;
        };
        let Some(actual_root) = actual.xml_roots.get(path.as_str()) else {
            continue;
        };
        if expected_root != actual_root {
            mismatched_roots.push(path.clone());
        }
    }

    HwpxContractDiff {
        missing_entries,
        extra_entries,
        entry_order_differs: expected.entry_names != actual.entry_names,
        mismatched_roots,
    }
}

fn xml_roots(
    entries: &BTreeMap<String, NormalizedEntry>,
) -> FixtureResult<BTreeMap<String, XmlRoot>> {
    let mut roots = BTreeMap::new();

    for (path, contents) in entries {
        if !(path.ends_with(".xml") || path.ends_with(".hpf") || path.ends_with(".rdf")) {
            continue;
        }
        let NormalizedEntry::Text(text) = contents else {
            continue;
        };
        let doc = XmlDocument::parse(text)?;
        let root = doc.root_element();
        roots.insert(
            path.clone(),
            XmlRoot {
                name: root.tag_name().name().to_string(),
                namespace: root.tag_name().namespace().map(ToString::to_string),
            },
        );
    }

    Ok(roots)
}
