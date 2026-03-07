use std::collections::BTreeMap;
use std::fs;

#[path = "support/docx_fixture.rs"]
mod docx_fixture;
#[path = "support/docx_runtime.rs"]
mod docx_runtime;
#[path = "docx/fixture_suite.rs"]
mod fixture_suite;
#[path = "docx/regression/mod.rs"]
mod regression;

use docx_fixture::{
    DocxFixture, NormalizedEntry, hash_entries, normalized_entries, read_expected_entries,
    read_fixture_input,
};
use roxmltree::Document as XmlDocument;

pub(crate) fn assert_fixture_matches_golden(fixture: &DocxFixture) {
    let input = read_fixture_input(fixture).unwrap();

    let generated = core_rs::generate_docx(&input, fixture.options()).unwrap();
    let generated_normalized = normalized_entries(&generated).unwrap();
    assert_required_entries(&generated_normalized);
    assert_xml_is_well_formed(&generated_normalized);

    let golden_bytes = fs::read(fixture.root.join("golden.docx")).unwrap();
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
        generated_normalized, golden_normalized,
        "generated golden mismatch: {}",
        fixture.name
    );
}

pub(crate) fn assert_same_input_same_hash(fixture: &DocxFixture) {
    let input = read_fixture_input(fixture).unwrap();

    let first = core_rs::generate_docx(&input, fixture.options()).unwrap();
    let second = core_rs::generate_docx(&input, fixture.options()).unwrap();

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
        "[Content_Types].xml",
        "_rels/.rels",
        "docProps/core.xml",
        "word/document.xml",
        "word/_rels/document.xml.rels",
        "word/styles.xml",
    ] {
        assert!(entries.contains_key(path), "missing required entry: {path}");
    }
}

pub(crate) fn assert_xml_is_well_formed(entries: &BTreeMap<String, NormalizedEntry>) {
    for (path, contents) in entries {
        if !(path.ends_with(".xml") || path.ends_with(".rels")) {
            continue;
        }
        if let NormalizedEntry::Text(contents) = contents {
            XmlDocument::parse(contents).unwrap_or_else(|error| {
                panic!("entry {path} is not valid XML: {error}");
            });
        }
    }
}

pub(crate) fn text_entry<'a>(entries: &'a BTreeMap<String, NormalizedEntry>, path: &str) -> &'a str {
    match entries.get(path).unwrap() {
        NormalizedEntry::Text(text) => text,
        NormalizedEntry::BinaryHash(_) => panic!("entry {path} is binary"),
    }
}
