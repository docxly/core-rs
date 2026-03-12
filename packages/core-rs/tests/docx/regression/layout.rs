use core_rs::{DocxOptions, generate_docx};

use crate::docx_fixture::normalized_entries;
use crate::text_entry;

#[test]
fn list_item_starting_with_blockquote_does_not_create_empty_marker_paragraph() {
    let bytes = generate_docx("- > quote", DocxOptions::default()).unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");

    assert!(!document_xml.contains(
        "<w:t xml:space=\"preserve\">• </w:t></w:r></w:p><w:p><w:pPr><w:ind w:left=\"1440\""
    ));
    assert!(document_xml.contains("<w:t xml:space=\"preserve\">• </w:t>"));
    assert!(document_xml.contains("<w:t>quote</w:t>"));
}

#[test]
fn list_item_starting_with_code_block_does_not_create_empty_marker_paragraph() {
    let bytes = generate_docx("- ```\ncode\n```", DocxOptions::default()).unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");

    assert!(!document_xml.contains(
        "<w:t xml:space=\"preserve\">• </w:t></w:r></w:p><w:p><w:pPr><w:ind w:left=\"1080\""
    ));
    assert!(document_xml.contains("<w:t xml:space=\"preserve\">• </w:t>"));
    assert!(document_xml.contains("<w:t>code</w:t>"));
}

#[test]
fn list_item_starting_with_table_preserves_table_cell_content() {
    let bytes = generate_docx(
        "- | left | right |\n  | --- | --- |\n  | a | b |",
        DocxOptions::default(),
    )
    .unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");

    assert!(document_xml.contains("<w:tbl>"));
    assert!(document_xml.contains("<w:t>left</w:t>"));
    assert!(!document_xml.contains("• left"));
    assert!(!document_xml.contains(">• left<"));
}

#[test]
fn blockquote_list_keeps_quote_border_on_list_items() {
    let bytes = generate_docx("> - quoted item", DocxOptions::default()).unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");

    assert!(
        document_xml
            .contains("<w:left w:val=\"single\" w:sz=\"8\" w:space=\"8\" w:color=\"B7B7B7\"/>")
    );
    assert!(document_xml.contains("quoted item"));
    assert!(
        document_xml.contains("xml:space=\"preserve\">◦ </w:t>")
            || document_xml.contains("xml:space=\"preserve\">• </w:t>")
    );
}

#[test]
fn blockquote_code_block_keeps_quote_border() {
    let bytes = generate_docx("> ```\ncode\n```", DocxOptions::default()).unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");

    assert!(
        document_xml
            .contains("<w:left w:val=\"single\" w:sz=\"8\" w:space=\"8\" w:color=\"B7B7B7\"/>")
    );
    assert!(document_xml.contains("<w:t>code</w:t>"));
}

#[test]
fn nested_list_as_first_child_preserves_parent_and_child_markers() {
    let bytes = generate_docx("-\n  1. child one\n  2. child two", DocxOptions::default()).unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");

    assert!(document_xml.contains(">• \u{200B}<") || document_xml.contains(">• \u{200b}<"));
    assert!(document_xml.contains("xml:space=\"preserve\">1. "));
    assert!(document_xml.contains("xml:space=\"preserve\">2. "));
    assert!(document_xml.contains("child one"));
    assert!(document_xml.contains("child two"));
}

#[test]
fn quoted_table_does_not_inject_hidden_anchor_paragraph() {
    let bytes = generate_docx(
        "> | left | right |\n> | --- | --- |\n> | a | b |",
        DocxOptions::default(),
    )
    .unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");

    assert!(document_xml.contains("<w:tbl>"));
    assert!(!document_xml.contains("\u{200B}"));
    assert!(
        document_xml
            .contains("<w:left w:val=\"single\" w:sz=\"8\" w:space=\"8\" w:color=\"B7B7B7\"/>")
    );
}

#[test]
fn nested_blockquote_does_not_duplicate_quote_border_properties() {
    let bytes = generate_docx("> > nested", DocxOptions::default()).unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");
    let quote_border_count = document_xml.matches("w:color=\"B7B7B7\"").count();

    assert_eq!(quote_border_count, 1);
}

#[test]
fn includes_heading_styles_up_to_level_six() {
    let bytes = generate_docx(
        "#### Four\n\n##### Five\n\n###### Six",
        DocxOptions::default(),
    )
    .unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");
    let styles_xml = text_entry(&normalized, "word/styles.xml");

    assert!(document_xml.contains("Heading4"));
    assert!(document_xml.contains("Heading5"));
    assert!(document_xml.contains("Heading6"));
    assert!(styles_xml.contains("styleId=\"Heading4\""));
    assert!(styles_xml.contains("styleId=\"Heading5\""));
    assert!(styles_xml.contains("styleId=\"Heading6\""));
}
