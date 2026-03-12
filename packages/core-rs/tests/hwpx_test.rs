use std::collections::{BTreeMap, BTreeSet};
use std::fs;

#[path = "hwpx/fixture_suite.rs"]
mod fixture_suite;
#[path = "support/hwpx_contract.rs"]
mod hwpx_contract;
#[path = "support/hwpx_fixture.rs"]
mod hwpx_fixture;
#[path = "support/hwpx_runtime.rs"]
mod hwpx_runtime;
#[path = "hwpx/regression/mod.rs"]
mod regression;

use hwpx_fixture::{
    HwpxFixture, NormalizedEntry, archive_entry_names, hash_entries, normalized_entries,
    read_expected_entries, read_fixture_input,
};
use roxmltree::Document as XmlDocument;

pub(crate) fn assert_fixture_matches_golden(fixture: &HwpxFixture) {
    let input = read_fixture_input(fixture).unwrap();

    let generated = core_rs::generate_hwpx(&input, fixture.options()).unwrap();
    assert_entry_order(&generated);
    let generated_normalized = normalized_entries(&generated).unwrap();
    assert_required_entries(&generated_normalized);
    assert_xml_is_well_formed(&generated_normalized);

    let golden_bytes = fs::read(fixture.root.join("golden.hwpx")).unwrap();
    assert_entry_order(&golden_bytes);
    let golden_normalized = normalized_entries(&golden_bytes).unwrap();
    assert_required_entries(&golden_normalized);
    assert_xml_is_well_formed(&golden_normalized);

    let expected = read_expected_entries(&fixture.root.join("expected")).unwrap();
    assert_eq!(
        golden_normalized, expected,
        "golden expected mismatch: {}",
        fixture.name
    );

    let expected_hash = fs::read_to_string(fixture.root.join("hash.txt")).unwrap();
    assert_eq!(
        hash_entries(&golden_normalized),
        expected_hash.trim(),
        "golden hash mismatch: {}",
        fixture.name
    );

    assert_eq!(
        comparable_entries(fixture, &generated_normalized),
        comparable_entries(fixture, &golden_normalized),
        "generated golden mismatch: {}",
        fixture.name
    );
}

pub(crate) fn assert_same_input_same_hash(fixture: &HwpxFixture) {
    let input = read_fixture_input(fixture).unwrap();

    let first = core_rs::generate_hwpx(&input, fixture.options()).unwrap();
    let second = core_rs::generate_hwpx(&input, fixture.options()).unwrap();

    let first_hash = hash_entries(&normalized_entries(&first).unwrap());
    let second_hash = hash_entries(&normalized_entries(&second).unwrap());
    assert_eq!(
        first_hash, second_hash,
        "non-deterministic fixture: {}",
        fixture.name
    );
}

pub(crate) fn assert_required_entries(entries: &BTreeMap<String, NormalizedEntry>) {
    for path in [
        "mimetype",
        "version.xml",
        "settings.xml",
        "META-INF/container.xml",
        "META-INF/container.rdf",
        "META-INF/manifest.xml",
        "Contents/content.hpf",
        "Contents/header.xml",
        "Contents/section0.xml",
        "Preview/PrvText.txt",
        "Preview/PrvImage.png",
    ] {
        assert!(entries.contains_key(path), "missing required entry: {path}");
    }
}

pub(crate) fn assert_entry_order(bytes: &[u8]) {
    let names = archive_entry_names(bytes).unwrap();
    assert_eq!(
        names,
        vec![
            "mimetype",
            "version.xml",
            "Contents/header.xml",
            "Contents/section0.xml",
            "Preview/PrvText.txt",
            "settings.xml",
            "Preview/PrvImage.png",
            "META-INF/container.rdf",
            "Contents/content.hpf",
            "META-INF/container.xml",
            "META-INF/manifest.xml",
        ],
    );
}

pub(crate) fn assert_xml_is_well_formed(entries: &BTreeMap<String, NormalizedEntry>) {
    for (path, contents) in entries {
        if !(path.ends_with(".xml") || path.ends_with(".rels") || path.ends_with(".hpf")) {
            continue;
        }
        if let NormalizedEntry::Text(contents) = contents {
            XmlDocument::parse(contents).unwrap_or_else(|error| {
                panic!("entry {path} is not valid XML: {error}");
            });
        }
    }
}

fn comparable_entries(
    fixture: &HwpxFixture,
    entries: &BTreeMap<String, NormalizedEntry>,
) -> BTreeMap<String, NormalizedEntry> {
    let excluded_paths = fixture_comparison_excludes(fixture);
    if excluded_paths.is_empty() {
        return entries.clone();
    }

    entries
        .iter()
        .filter(|(path, _)| !excluded_paths.contains(path.as_str()))
        .map(|(path, entry)| (path.clone(), entry.clone()))
        .collect()
}

fn fixture_comparison_excludes(fixture: &HwpxFixture) -> BTreeSet<&str> {
    let mut excluded_paths = BTreeSet::new();
    if !fixture.determinism {
        excluded_paths.extend(NONDETERMINISTIC_ENTRY_EXCLUDES.iter().copied());
    }
    excluded_paths.extend(fixture.comparison_excludes.iter().map(String::as_str));
    excluded_paths
}

const NONDETERMINISTIC_ENTRY_EXCLUDES: &[&str] = &[
    "Contents/content.hpf",
    "Preview/PrvImage.png",
    "version.xml",
];
