mod expected;
mod support;

use std::fs;

use super::{debug_dump, parse_hwpx};
use crate::error::CoreRsError;
use crate::models::block::Block;
use crate::models::inline::Inline;

use expected::{expected_document, fixture_options};
use support::{
    add_section_entry, approved_fixture_names, approved_root, build_hwpx_archive,
    expected_metadata_from_content_hpf, mutate_fixture_section, provisional_root,
};

#[test]
fn parses_all_approved_hwpx_golden_fixtures() {
    for fixture in approved_fixture_names() {
        let bytes = fs::read(approved_root().join(fixture).join("golden.hwpx")).unwrap();
        let parsed = parse_hwpx(&bytes).unwrap_or_else(|error| {
            panic!("failed to parse approved fixture {fixture}: {error}");
        });
        assert_eq!(
            parsed.document,
            expected_document(fixture),
            "document mismatch for fixture {fixture}"
        );

        let content_hpf = fs::read_to_string(
            approved_root()
                .join(fixture)
                .join("expected")
                .join("Contents")
                .join("content.hpf"),
        )
        .unwrap();
        let expected_metadata = expected_metadata_from_content_hpf(&content_hpf);
        assert_eq!(
            parsed.metadata, expected_metadata,
            "metadata mismatch for fixture {fixture}"
        );
    }
}

#[test]
fn parses_generated_hwpx_for_all_approved_fixture_inputs() {
    for fixture in approved_fixture_names() {
        let input = fs::read_to_string(approved_root().join(fixture).join("input.md")).unwrap();
        let options = fixture_options(fixture);
        let bytes = crate::generate_hwpx(&input, options).unwrap();
        let parsed = parse_hwpx(&bytes).unwrap_or_else(|error| {
            panic!("failed to parse generated fixture {fixture}: {error}");
        });
        assert_eq!(
            parsed.document,
            expected_document(fixture),
            "generated document mismatch for fixture {fixture}"
        );
    }
}

#[test]
fn rejects_non_zip_input() {
    let error = parse_hwpx(b"not a zip archive").unwrap_err();
    assert!(matches!(error, CoreRsError::InvalidHwpx(_)));
}

#[test]
fn rejects_missing_required_entry() {
    let bytes = build_hwpx_archive(std::collections::BTreeMap::from([(
        "Contents/header.xml".to_string(),
        "<hh:head/>".as_bytes().to_vec(),
    )]));
    let error = parse_hwpx(&bytes).unwrap_err();
    assert!(matches!(error, CoreRsError::InvalidHwpx(_)));
}

#[test]
fn rejects_oversized_archive_entries() {
    let bytes = build_hwpx_archive(std::collections::BTreeMap::from([
        ("mimetype".to_string(), b"application/hwp+zip".to_vec()),
        (
            "Contents/content.hpf".to_string(),
            vec![b'x'; 8 * 1024 * 1024 + 1],
        ),
        ("Contents/header.xml".to_string(), b"<hh:head/>".to_vec()),
        ("Contents/section0.xml".to_string(), b"<hs:sec/>".to_vec()),
    ]));
    let error = parse_hwpx(&bytes).unwrap_err();
    assert!(matches!(error, CoreRsError::InvalidHwpx(_)));
}

#[test]
fn ignores_oversized_irrelevant_archive_entries() {
    let root = approved_root().join("core-paragraph").join("expected");
    let mut entries = support::expected_entries(&root);
    entries.insert(
        "Preview/PrvImage.png".to_string(),
        vec![0_u8; 8 * 1024 * 1024 + 1],
    );
    let parsed = parse_hwpx(&build_hwpx_archive(entries)).unwrap();
    assert_eq!(parsed.document, expected_document("core-paragraph"));
}

#[test]
fn rejects_malformed_xml() {
    let bytes = mutate_fixture_section("core-paragraph", |xml| xml.replace("</hs:sec>", ""));
    let error = parse_hwpx(&bytes).unwrap_err();
    assert!(matches!(error, CoreRsError::InvalidHwpx(_)));
}

#[test]
fn rejects_missing_registry_reference() {
    let bytes = mutate_fixture_section("core-paragraph", |xml| {
        xml.replacen("paraPrIDRef=\"0\"", "paraPrIDRef=\"999\"", 1)
    });
    let error = parse_hwpx(&bytes).unwrap_err();
    assert!(matches!(error, CoreRsError::InvalidHwpx(_)));
}

#[test]
fn rejects_unbalanced_hyperlink_fields() {
    let bytes = mutate_fixture_section("core-link-text", |xml| {
        xml.replacen("<hp:ctrl><hp:fieldEnd", "<hp:ctrl><hp:fieldFinish", 1)
    });
    let error = parse_hwpx(&bytes).unwrap_err();
    assert!(matches!(error, CoreRsError::InvalidHwpx(_)));
}

#[test]
fn parses_hyperlink_with_trailing_text_after_field_end_in_same_run() {
    let bytes = mutate_fixture_section("core-link-text", |xml| {
        xml.replacen("<hp:t/></hp:run>", "<hp:t> 이후</hp:t></hp:run>", 1)
    });
    let parsed = parse_hwpx(&bytes).unwrap();
    assert_eq!(
        parsed.document,
        expected::doc(vec![Block::Paragraph(vec![
            expected::link("문서 열기", "https://example.com/"),
            Inline::Text(" 이후".to_string()),
        ])])
    );
}

#[test]
fn renders_stable_semantic_dump_for_debugging() {
    let bytes = fs::read(approved_root().join("core-paragraph").join("golden.hwpx")).unwrap();
    let dump = debug_dump(&bytes).unwrap();
    assert!(dump.contains("metadata:\n"));
    assert!(dump.contains("document:\n"));
    assert!(dump.contains("paragraph([text("));
}

#[test]
fn rejects_unsupported_image_fixture() {
    let bytes = fs::read(
        provisional_root()
            .join("image-data-uri-basic")
            .join("golden.hwpx"),
    )
    .unwrap();
    let error = parse_hwpx(&bytes).unwrap_err();
    assert!(matches!(error, CoreRsError::UnsupportedFeature(_)));
}

#[test]
fn rejects_multiple_sections() {
    let bytes = add_section_entry("core-paragraph");
    let error = parse_hwpx(&bytes).unwrap_err();
    assert!(matches!(error, CoreRsError::UnsupportedFeature(_)));
}

#[test]
fn rejects_unsupported_control_shape() {
    let bytes = mutate_fixture_section("code-block-basic", |xml| {
        xml.replacen(
            "<hp:run charPrIDRef=\"6\">",
            "<hp:run charPrIDRef=\"6\"><hp:ctrl><hp:pic/></hp:ctrl>",
            1,
        )
    });
    let error = parse_hwpx(&bytes).unwrap_err();
    assert!(matches!(error, CoreRsError::UnsupportedFeature(_)));
}

#[test]
fn rejects_multiple_paragraphs_inside_a_table_cell() {
    let bytes = mutate_fixture_section("table-basic", |xml| {
        xml.replacen(
            "</hp:subList>",
            "<hp:p id=\"1\" paraPrIDRef=\"0\" styleIDRef=\"0\" pageBreak=\"0\" columnBreak=\"0\" merged=\"0\"><hp:run charPrIDRef=\"6\"><hp:t>Extra</hp:t></hp:run></hp:p></hp:subList>",
            1,
        )
    });
    let error = parse_hwpx(&bytes).unwrap_err();
    assert!(matches!(error, CoreRsError::UnsupportedFeature(_)));
}
