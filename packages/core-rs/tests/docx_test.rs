use std::collections::BTreeMap;
use std::fs;
use std::io::Write;

#[path = "support/docx_fixture.rs"]
mod docx_fixture;

use core_rs::{CoreRsError, DocxOptions, generate_docx};
use docx_fixture::{
    DocxFixture, NormalizedEntry, discover_fixtures, hash_entries, normalized_entries,
    read_expected_entries, read_fixture_input,
};
use roxmltree::Document as XmlDocument;
use zip::CompressionMethod;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

#[test]
fn all_docx_fixtures_match_golden() {
    let fixtures = discover_fixtures().unwrap();
    assert!(!fixtures.is_empty(), "no DOCX fixtures discovered");

    for fixture in &fixtures {
        assert_fixture_matches_golden(fixture);
    }
}

#[test]
fn deterministic_fixtures_generate_same_hash() {
    let fixtures = discover_fixtures().unwrap();
    let deterministic = fixtures
        .iter()
        .filter(|fixture| fixture.determinism)
        .collect::<Vec<_>>();

    assert!(
        !deterministic.is_empty(),
        "no deterministic DOCX fixtures configured"
    );

    for fixture in deterministic {
        assert_same_input_same_hash(fixture);
    }
}

#[test]
fn returns_error_for_unsupported_html_in_strict_mode() {
    let error = generate_docx("<b>raw</b>", DocxOptions::default()).unwrap_err();
    assert!(matches!(error, CoreRsError::UnsupportedFeature(_)));
}

#[test]
fn falls_back_to_plain_text_when_not_strict() {
    let bytes = generate_docx(
        "<b>raw</b>",
        DocxOptions {
            strict_mode: false,
            ..DocxOptions::default()
        },
    )
    .unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");
    assert!(document_xml.contains("&lt;b&gt;raw&lt;/b&gt;"));
}

#[test]
fn falls_back_for_unsupported_block_when_not_strict() {
    let bytes = generate_docx(
        "```rust\nlet answer = 42;\n```",
        DocxOptions {
            strict_mode: false,
            ..DocxOptions::default()
        },
    )
    .unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");
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
    let bytes = generate_docx(
        "[one](https://example.com/one) and [two](https://example.com/two)",
        DocxOptions::default(),
    )
    .unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");
    let relationships_xml = text_entry(&normalized, "word/_rels/document.xml.rels");

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
    let bytes = generate_docx(
        "[run](javascript:alert(1))",
        DocxOptions {
            strict_mode: false,
            ..DocxOptions::default()
        },
    )
    .unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");
    let relationships_xml = text_entry(&normalized, "word/_rels/document.xml.rels");

    assert!(document_xml.contains(">run<"));
    assert!(!document_xml.contains("<w:hyperlink"));
    assert!(!relationships_xml.contains("javascript:alert(1)"));
}

#[test]
fn errors_for_non_data_image_in_strict_mode() {
    let error = generate_docx("![chart](./chart.png)", DocxOptions::default()).unwrap_err();
    assert!(matches!(error, CoreRsError::UnsupportedFeature(_)));
}

#[test]
fn falls_back_to_alt_text_for_non_data_image_when_not_strict() {
    let bytes = generate_docx(
        "![chart](./chart.png)",
        DocxOptions {
            strict_mode: false,
            ..DocxOptions::default()
        },
    )
    .unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");
    let relationship_xml = text_entry(&normalized, "word/_rels/document.xml.rels");

    assert!(document_xml.contains(">chart<"));
    assert!(!relationship_xml.contains("relationships/image"));
}

#[test]
fn falls_back_to_image_url_when_non_data_image_has_empty_alt() {
    let bytes = generate_docx(
        "![](./chart.png)",
        DocxOptions {
            strict_mode: false,
            ..DocxOptions::default()
        },
    )
    .unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");

    assert!(document_xml.contains("./chart.png"));
}

#[test]
fn falls_back_to_placeholder_when_oversized_data_image_has_empty_alt() {
    let payload = {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.encode(vec![0_u8; 8 * 1024 * 1024 + 1])
    };
    let markdown = format!("![](data:image/png;base64,{payload})");
    let bytes = generate_docx(
        &markdown,
        DocxOptions {
            strict_mode: false,
            ..DocxOptions::default()
        },
    )
    .unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");

    assert!(document_xml.contains("[image omitted]"));
    assert!(!document_xml.contains("data:image/png;base64"));
}

#[test]
fn falls_back_to_data_image_placeholder_for_invalid_data_image_with_empty_alt() {
    let bytes = generate_docx(
        "![](data:image/svg+xml;base64,PHN2Zz4=)",
        DocxOptions {
            strict_mode: false,
            ..DocxOptions::default()
        },
    )
    .unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");

    assert!(document_xml.contains("[data image]"));
    assert!(!document_xml.contains("image/svg+xml"));
    assert!(!document_xml.contains("PHN2Zz4="));
}

#[test]
fn falls_back_to_data_image_placeholder_for_invalid_base64_data_image() {
    let bytes = generate_docx(
        "![](data:image/png;base64,%%%%)",
        DocxOptions {
            strict_mode: false,
            ..DocxOptions::default()
        },
    )
    .unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");

    assert!(document_xml.contains("[data image]"));
    assert!(!document_xml.contains("%%%%"));
}

#[test]
fn falls_back_to_data_image_placeholder_for_long_metadata_data_image() {
    let long_mime = format!("image/{}", "x".repeat(4096));
    let markdown = format!("![](data:{long_mime};base64,QQ==)");
    let bytes = generate_docx(
        &markdown,
        DocxOptions {
            strict_mode: false,
            ..DocxOptions::default()
        },
    )
    .unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");

    assert!(document_xml.contains("[data image]"));
    assert!(!document_xml.contains(&long_mime));
    assert!(!document_xml.contains("QQ=="));
}

#[test]
fn falls_back_for_deeply_nested_list_when_not_strict() {
    let bytes = generate_docx(
        "- one\n  - two\n    - three",
        DocxOptions {
            strict_mode: false,
            ..DocxOptions::default()
        },
    )
    .unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");
    assert!(document_xml.contains("three"));
}

#[test]
fn rejects_oversized_data_image_in_strict_mode() {
    let payload = {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.encode(vec![0_u8; 8 * 1024 * 1024 + 1])
    };
    let markdown = format!("![huge](data:image/png;base64,{payload})");

    let error = generate_docx(&markdown, DocxOptions::default()).unwrap_err();

    assert!(matches!(error, CoreRsError::UnsupportedFeature(_)));
}

#[test]
fn rejects_documents_with_excessive_total_embedded_image_bytes() {
    let payload = {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.encode(vec![0_u8; 6 * 1024 * 1024])
    };
    let markdown = format!(
        "![one](data:image/png;base64,{payload})\n\n![two](data:image/png;base64,{payload})\n\n![three](data:image/png;base64,{payload})"
    );

    let error = generate_docx(&markdown, DocxOptions::default()).unwrap_err();

    assert!(matches!(error, CoreRsError::UnsupportedFeature(_)));
}

#[test]
fn assigns_unique_ids_for_multiple_images() {
    let fixture = discover_fixtures()
        .unwrap()
        .into_iter()
        .find(|fixture| fixture.name == "image")
        .unwrap();
    let image_input = read_fixture_input(&fixture).unwrap();
    let markdown = format!("{image_input}\n\n{image_input}");
    let bytes = generate_docx(&markdown, DocxOptions::default()).unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");

    assert!(document_xml.contains("wp:docPr id=\"1\""));
    assert!(document_xml.contains("wp:docPr id=\"2\""));
    assert!(document_xml.contains("pic:cNvPr id=\"1\""));
    assert!(document_xml.contains("pic:cNvPr id=\"2\""));
}

#[test]
fn sanitizes_invalid_xml_characters_in_document_and_metadata() {
    let bytes = generate_docx(
        "Hello \u{0000}docx \u{000B}world",
        DocxOptions {
            title: Some("Ti\u{0000}tle".to_string()),
            author: Some("Au\u{001F}thor".to_string()),
            strict_mode: true,
        },
    )
    .unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    assert_xml_is_well_formed(&normalized);

    let document_xml = text_entry(&normalized, "word/document.xml");
    let core_xml = text_entry(&normalized, "docProps/core.xml");

    assert!(document_xml.contains("Hello �docx �world"));
    assert!(core_xml.contains("Ti�tle"));
    assert!(core_xml.contains("Au�thor"));
    assert!(!document_xml.contains('\u{0000}'));
    assert!(!document_xml.contains('\u{000B}'));
}

#[test]
fn list_item_starting_with_blockquote_does_not_create_empty_marker_paragraph() {
    let bytes = generate_docx("- > quote", DocxOptions::default()).unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");

    assert!(!document_xml.contains("<w:t xml:space=\"preserve\">• </w:t></w:r></w:p><w:p><w:pPr><w:ind w:left=\"1440\""));
    assert!(document_xml.contains("<w:t xml:space=\"preserve\">• </w:t>"));
    assert!(document_xml.contains("<w:t>quote</w:t>"));
}

#[test]
fn list_item_starting_with_code_block_does_not_create_empty_marker_paragraph() {
    let bytes = generate_docx("- ```\ncode\n```", DocxOptions::default()).unwrap();

    let normalized = normalized_entries(&bytes).unwrap();
    let document_xml = text_entry(&normalized, "word/document.xml");

    assert!(!document_xml.contains("<w:t xml:space=\"preserve\">• </w:t></w:r></w:p><w:p><w:pPr><w:ind w:left=\"1080\""));
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

    assert!(document_xml.contains("<w:left w:val=\"single\" w:sz=\"8\" w:space=\"8\" w:color=\"B7B7B7\"/>"));
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

    assert!(document_xml.contains("<w:left w:val=\"single\" w:sz=\"8\" w:space=\"8\" w:color=\"B7B7B7\"/>"));
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
    assert!(document_xml.contains("<w:left w:val=\"single\" w:sz=\"8\" w:space=\"8\" w:color=\"B7B7B7\"/>"));
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
    let bytes = generate_docx("#### Four\n\n##### Five\n\n###### Six", DocxOptions::default()).unwrap();

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

#[test]
fn normalizes_binary_entries_without_panicking() {
    let mut writer = ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    writer.start_file("word/document.xml", options).unwrap();
    writer.write_all(b"<root/>").unwrap();
    writer.start_file("word/media/image1.png", options).unwrap();
    writer.write_all(&[0x89, b'P', b'N', b'G', 0x00, 0x01]).unwrap();
    let bytes = writer.finish().unwrap().into_inner();

    let normalized = normalized_entries(&bytes).unwrap();

    assert_eq!(
        normalized.get("word/document.xml"),
        Some(&NormalizedEntry::Text("<root/>".to_string()))
    );
    assert!(matches!(
        normalized.get("word/media/image1.png"),
        Some(NormalizedEntry::BinaryHash(hash)) if !hash.is_empty()
    ));
}

fn assert_fixture_matches_golden(fixture: &DocxFixture) {
    let input = read_fixture_input(fixture).unwrap();

    let generated = generate_docx(&input, fixture.options()).unwrap();
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

fn assert_same_input_same_hash(fixture: &DocxFixture) {
    let input = read_fixture_input(fixture).unwrap();

    let first = generate_docx(&input, fixture.options()).unwrap();
    let second = generate_docx(&input, fixture.options()).unwrap();

    let first_hash = hash_entries(&normalized_entries(&first).unwrap());
    let second_hash = hash_entries(&normalized_entries(&second).unwrap());
    assert_eq!(
        first_hash, second_hash,
        "non-deterministic fixture: {}",
        fixture.name
    );
}

fn assert_required_entries(entries: &BTreeMap<String, NormalizedEntry>) {
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

fn assert_xml_is_well_formed(entries: &BTreeMap<String, NormalizedEntry>) {
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

fn text_entry<'a>(entries: &'a BTreeMap<String, NormalizedEntry>, path: &str) -> &'a str {
    match entries.get(path).unwrap() {
        NormalizedEntry::Text(text) => text,
        NormalizedEntry::BinaryHash(_) => panic!("entry {path} is binary"),
    }
}
