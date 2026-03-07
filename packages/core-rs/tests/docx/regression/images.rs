use core_rs::{CoreRsError, DocxOptions, generate_docx};

use crate::docx_fixture::{discover_fixtures, normalized_entries, read_fixture_input};
use crate::text_entry;

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
