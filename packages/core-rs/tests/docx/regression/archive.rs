use std::io::Write;

use core_rs::{DocxOptions, generate_docx};
use zip::CompressionMethod;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

use crate::docx_fixture::{NormalizedEntry, normalized_entries};
use crate::{assert_xml_is_well_formed, text_entry};

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
fn normalizes_binary_entries_without_panicking() {
    let mut writer = ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    writer.start_file("word/document.xml", options).unwrap();
    writer.write_all(b"<root/>").unwrap();
    writer.start_file("word/media/image1.png", options).unwrap();
    writer
        .write_all(&[0x89, b'P', b'N', b'G', 0x00, 0x01])
        .unwrap();
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
