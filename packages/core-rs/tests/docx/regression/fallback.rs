use core_rs::{CoreRsError, DocxOptions, generate_docx};

use crate::docx_fixture::normalized_entries;
use crate::docx_runtime::generate;
use crate::{assert_required_entries, assert_xml_is_well_formed, text_entry};

#[test]
fn returns_error_for_unsupported_html_in_strict_mode() {
    let error = generate_docx("<b>raw</b>", DocxOptions::default()).unwrap_err();
    assert!(matches!(error, CoreRsError::UnsupportedFeature(_)));
}

#[test]
fn falls_back_to_plain_text_when_not_strict() {
    let generated = generate(
        "<b>raw</b>",
        DocxOptions {
            strict_mode: false,
            ..DocxOptions::default()
        },
    )
    .unwrap();
    let document_xml = generated.document_xml().unwrap();
    assert!(document_xml.contains("&lt;b&gt;raw&lt;/b&gt;"));
}

#[test]
fn falls_back_for_unsupported_block_when_not_strict() {
    let generated = generate(
        "```rust\nlet answer = 42;\n```",
        DocxOptions {
            strict_mode: false,
            ..DocxOptions::default()
        },
    )
    .unwrap();
    let document_xml = generated.document_xml().unwrap();
    assert!(document_xml.contains("let answer = 42;"));
}

#[test]
fn generates_empty_document_without_error() {
    let bytes = generate_docx("", DocxOptions::default()).unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    assert_required_entries(&normalized);
    assert_xml_is_well_formed(&normalized);

    let document_xml = text_entry(&normalized, "word/document.xml");
    assert!(document_xml.contains("<w:sectPr>"));
}

#[test]
fn generates_multiple_hyperlink_relationships() {
    let generated = generate(
        "[one](https://example.com/one) and [two](https://example.com/two)",
        DocxOptions::default(),
    )
    .unwrap();
    let document_xml = generated.document_xml().unwrap();
    let relationships_xml = generated.relationships_xml().unwrap();

    assert!(document_xml.contains("r:id=\"rLink1\""));
    assert!(document_xml.contains("r:id=\"rLink2\""));
    assert!(relationships_xml.contains("Id=\"rLink1\""));
    assert!(relationships_xml.contains("https://example.com/one"));
    assert!(relationships_xml.contains("Id=\"rLink2\""));
    assert!(relationships_xml.contains("https://example.com/two"));
}

#[test]
fn rejects_unsafe_link_scheme_in_strict_mode() {
    let error = generate_docx("[run](javascript:alert(1))", DocxOptions::default()).unwrap_err();
    assert!(matches!(error, CoreRsError::UnsupportedFeature(_)));
}

#[test]
fn falls_back_for_unsafe_link_scheme_when_not_strict() {
    let generated = generate(
        "[run](javascript:alert(1))",
        DocxOptions {
            strict_mode: false,
            ..DocxOptions::default()
        },
    )
    .unwrap();
    let document_xml = generated.document_xml().unwrap();
    let relationships_xml = generated.relationships_xml().unwrap();

    assert!(document_xml.contains(">run<"));
    assert!(!document_xml.contains("<w:hyperlink"));
    assert!(!relationships_xml.contains("javascript:alert(1)"));
}

#[test]
fn falls_back_for_deeply_nested_list_when_not_strict() {
    let generated = generate(
        "- one\n  - two\n    - three",
        DocxOptions {
            strict_mode: false,
            ..DocxOptions::default()
        },
    )
    .unwrap();
    let document_xml = generated.document_xml().unwrap();
    assert!(document_xml.contains("three"));
}
