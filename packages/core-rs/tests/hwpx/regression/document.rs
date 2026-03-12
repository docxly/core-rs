use super::*;

#[test]
fn non_table_documents_keep_default_header_and_settings_contract() {
    let generated = generate("hello", HwpxOptions::default()).unwrap();
    let header = generated.text_entry("Contents/header.xml").unwrap();
    let settings = generated.text_entry("settings.xml").unwrap();

    assert!(header.contains("<hh:borderFills itemCnt=\"2\">"));
    assert!(!header.contains("<hh:borderFill id=\"3\""));
    assert!(settings.contains("pos=\"45\""));
}

#[test]
fn links_are_rendered_as_visible_text() {
    let markdown = "[문서 열기](https://example.com)";
    let generated = generate(markdown, HwpxOptions::default()).unwrap();
    let section = generated.section_xml().unwrap();
    assert!(section.contains("<hp:t>문서 열기</hp:t>"));
    assert!(!section.contains("문서 열기 (https://example.com)"));
}

#[test]
fn empty_link_label_renders_url_only() {
    let markdown = "[](https://example.com)";
    let generated = generate(markdown, HwpxOptions::default()).unwrap();
    let section = generated.section_xml().unwrap();
    assert!(section.contains("https://example.com"));
}

#[test]
fn legacy_inline_links_render_as_hyperlink_fields() {
    let generated = generate(
        "# Heading\n\n본문 [링크](https://example.com/path) 입니다.",
        HwpxOptions::default(),
    )
    .unwrap();
    let section = generated.section_xml().unwrap();
    let preview = generated.text_entry("Preview/PrvText.txt").unwrap();

    assert!(section.contains("type=\"HYPERLINK\""));
    assert!(
        section.contains("<hp:stringParam name=\"Path\">https://example.com/path</hp:stringParam>")
    );
    assert!(section.contains("<hp:t>링크</hp:t>"));
    assert!(!section.contains("링크 (https://example.com/path)"));
    assert!(preview.contains("본문 링크 입니다."));
}

#[test]
fn legacy_links_use_document_unique_field_ids_across_paragraphs() {
    let generated = generate(
        "첫 문단 [링크1](https://example.com/one)\n\n둘째 문단 [링크2](https://example.com/two)",
        HwpxOptions::default(),
    )
    .unwrap();
    let section = generated.section_xml().unwrap();

    assert!(section.contains("fieldid=\"627600491\""));
    assert!(section.contains("fieldid=\"627600492\""));
    assert_eq!(section.matches("fieldid=\"627600491\"").count(), 2);
    assert_eq!(section.matches("fieldid=\"627600492\"").count(), 2);
    assert_eq!(section.matches("beginIDRef=\"2107483186\"").count(), 1);
    assert_eq!(section.matches("beginIDRef=\"2107483187\"").count(), 1);
}

#[test]
fn content_hpf_contains_metadata() {
    let generated = generate(
        "hello",
        HwpxOptions {
            title: Some("Fixture Title".to_string()),
            author: Some("Fixture Author".to_string()),
            strict_mode: true,
            ..HwpxOptions::default()
        },
    )
    .unwrap();

    let content_hpf = generated.content_hpf().unwrap();
    assert!(content_hpf.contains("<opf:title>Fixture Title</opf:title>"));
    assert!(
        content_hpf
            .contains("<opf:meta name=\"creator\" content=\"text\">(주)한글과컴퓨터</opf:meta>")
    );
    assert!(content_hpf.contains("href=\"Contents/header.xml\""));
    assert!(content_hpf.contains("href=\"Contents/section0.xml\""));
    assert!(content_hpf.contains("href=\"settings.xml\""));
    assert!(content_hpf.contains("<opf:language>ko</opf:language>"));
    assert!(content_hpf.contains("<opf:itemref idref=\"header\" linear=\"yes\"/>"));
    assert!(content_hpf.contains("<opf:itemref idref=\"section0\" linear=\"yes\"/>"));
}

#[test]
fn package_contract_paths_are_self_consistent() {
    let generated = generate("hello", HwpxOptions::default()).unwrap();
    let content_hpf = generated.content_hpf().unwrap();
    let container = generated.text_entry("META-INF/container.xml").unwrap();
    let container_rdf = generated.text_entry("META-INF/container.rdf").unwrap();

    assert!(container.contains("full-path=\"Contents/content.hpf\""));
    assert!(container.contains("full-path=\"Preview/PrvText.txt\""));
    assert!(container.contains("full-path=\"META-INF/container.rdf\""));
    assert!(content_hpf.contains("href=\"Contents/header.xml\""));
    assert!(content_hpf.contains("href=\"Contents/section0.xml\""));
    assert!(content_hpf.contains("href=\"settings.xml\""));
    assert!(container_rdf.contains("Contents/header.xml"));
    assert!(container_rdf.contains("Contents/section0.xml"));
}

#[test]
fn headings_render_as_plain_hwpx_paragraphs() {
    let generated = generate("# Heading", HwpxOptions::default()).unwrap();
    let section = generated.section_xml().unwrap();
    assert!(section.contains("<hs:sec"));
    assert!(section.contains("<hp:p id="));
    assert!(section.contains("<hp:t>Heading</hp:t>"));
}

#[test]
fn blockquotes_render_with_quote_paragraph_contract_in_strict_mode() {
    let generated = generate(
        "> Quoted paragraph\n>\n> Another quoted line",
        HwpxOptions::default(),
    )
    .unwrap();
    let section = generated.section_xml().unwrap();
    let header = generated.text_entry("Contents/header.xml").unwrap();
    let settings = generated.text_entry("settings.xml").unwrap();
    let preview = generated.text_entry("Preview/PrvText.txt").unwrap();

    assert!(section.contains("paraPrIDRef=\"19\""));
    assert!(section.contains("charPrIDRef=\"7\""));
    assert!(section.contains("horzpos=\"2000\" horzsize=\"40520\" flags=\"393216\""));
    assert!(section.contains("vertpos=\"1600\""));
    assert!(section.contains("<hp:t>Quoted paragraph</hp:t>"));
    assert!(section.contains("<hp:t>Another quoted line</hp:t>"));
    assert!(header.contains("<hh:borderFill id=\"3\""));
    assert!(header.contains("<hh:charPr id=\"7\""));
    assert!(header.contains("<hh:paraPr id=\"19\""));
    assert!(settings.contains("paraIDRef=\"1\""));
    assert!(settings.contains("pos=\"19\""));
    assert_eq!(preview, "Quoted paragraph\nAnother quoted line");
}

#[test]
fn hard_breaks_flatten_to_visible_space_in_phase_a() {
    let generated = generate("hello  \nworld", HwpxOptions::default()).unwrap();
    let section = generated.section_xml().unwrap();
    assert!(section.contains("<hp:lineBreak/>"));
    assert!(!section.contains("<hp:t>hello world</hp:t>"));
}

#[test]
fn hwpx_xml_uses_hancom_2011_namespaces() {
    let generated = generate("hello", HwpxOptions::default()).unwrap();
    let section = generated.section_xml().unwrap();
    let content_hpf = generated.content_hpf().unwrap();

    assert!(section.contains("http://www.hancom.co.kr/hwpml/2011/paragraph"));
    assert!(section.contains("<hs:sec"));
    assert!(content_hpf.contains("http://www.idpf.org/2007/opf/"));
}

#[test]
fn images_fail_in_strict_mode() {
    let markdown = "![alt](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO5W5V8AAAAASUVORK5CYII=)";
    let error = generate_hwpx(markdown, HwpxOptions::default()).unwrap_err();
    assert!(matches!(error, CoreRsError::UnsupportedFeature(_)));
}

#[test]
fn images_fall_back_to_alt_text_in_non_strict_mode() {
    let markdown = "![diagram](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO5W5V8AAAAASUVORK5CYII=)";
    let generated = generate(
        markdown,
        HwpxOptions {
            strict_mode: false,
            ..HwpxOptions::default()
        },
    )
    .unwrap();

    let section = generated.section_xml().unwrap();
    assert!(section.contains("diagram"));
}

#[test]
fn paragraph_reference_contract_matches_hancom_sample_shape() {
    let generated_bytes = generate_hwpx(
        "안녕하세요",
        HwpxOptions {
            title: Some("Generated Document".to_string()),
            strict_mode: true,
            ..HwpxOptions::default()
        },
    )
    .unwrap();
    let reference_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("generators")
        .join("hwpx")
        .join("reference")
        .join("paragraph-only");

    let generated = snapshot_from_bytes(&generated_bytes).unwrap();
    let reference = snapshot_from_expected_tree(&reference_root).unwrap();
    let diff = compare_contracts(&reference, &generated);

    assert!(
        diff.missing_entries.is_empty(),
        "missing entries: {:?}",
        diff.missing_entries
    );
    assert!(
        diff.extra_entries.is_empty(),
        "extra entries: {:?}",
        diff.extra_entries
    );
    assert!(
        diff.mismatched_roots.is_empty(),
        "root mismatch: {:?}",
        diff.mismatched_roots
    );

    let runtime = generate(
        "안녕하세요",
        HwpxOptions {
            title: Some("Generated Document".to_string()),
            strict_mode: true,
            ..HwpxOptions::default()
        },
    )
    .unwrap();
    let content_hpf = runtime.content_hpf().unwrap();
    let section = runtime.section_xml().unwrap();
    assert!(content_hpf.contains("<opf:title>Generated Document</opf:title>"));
    assert!(section.contains("<hs:sec"));
    assert!(section.contains("<hp:t>안녕하세요</hp:t>"));
}

#[test]
fn preview_text_tracks_visible_paragraph_text() {
    let generated = generate(
        "# 제목\n\n문단",
        HwpxOptions {
            strict_mode: true,
            ..HwpxOptions::default()
        },
    )
    .unwrap();
    let preview = generated.text_entry("Preview/PrvText.txt").unwrap();
    assert!(preview.contains("제목"));
    assert!(preview.contains("문단"));
    assert_eq!(preview, "제목\n문단");
}

#[test]
fn legacy_body_paragraphs_omit_linesegarray_to_allow_viewer_reflow() {
    let generated = generate(
        "# Heading\n\nThis paragraph should be wrapped by the viewer instead of using a fixed single lineseg.",
        HwpxOptions {
            strict_mode: true,
            ..HwpxOptions::default()
        },
    )
    .unwrap();
    let section = generated.section_xml().unwrap();

    assert_eq!(section.matches("<hp:linesegarray>").count(), 1);
    assert!(section.contains("<hp:t>Heading</hp:t>"));
    assert!(section.contains(
        "<hp:t>This paragraph should be wrapped by the viewer instead of using a fixed single lineseg.</hp:t>"
    ));
}

#[test]
fn legacy_list_items_omit_linesegarray_to_avoid_spacing_compression() {
    let generated = generate(
        "- 첫 번째 항목은 viewer가 줄바꿈을 다시 계산해야 합니다.\n- 두 번째 항목도 동일한 규칙을 따라야 합니다.",
        HwpxOptions {
            strict_mode: false,
            ..HwpxOptions::default()
        },
    )
    .unwrap();
    let section = generated.section_xml().unwrap();

    assert!(!section.contains("<hp:linesegarray>"));
    assert!(section.contains("paraPrIDRef=\"19\""));
}

#[test]
fn plain_multi_paragraph_profile_does_not_fall_back_to_fixture_metadata() {
    let markdown = "브라우저와 Rust 코어를 공유하는 기본 문단입니다. 브라우저와 Rust 코어를 공유하는 기본 문단입니다. 브라우저와 Rust 코어를 공유하는 기본 문단입니다.\n\nThis is Second Contents. This is Second Contents. This is Second Contents. This is Second Contents. This is Second Contents.";
    let generated = generate(
        markdown,
        HwpxOptions {
            title: Some("Runtime Title".to_string()),
            ..HwpxOptions::default()
        },
    )
    .unwrap();

    let content_hpf = generated.content_hpf().unwrap();
    let version = generated.text_entry("version.xml").unwrap();
    let settings = generated.text_entry("settings.xml").unwrap();
    let section = generated.section_xml().unwrap();
    let doc = XmlDocument::parse(section).unwrap();
    let paragraph_ids = doc
        .descendants()
        .filter(|node| node.is_element() && node.tag_name().name() == "p")
        .filter_map(|node| node.attribute("id"))
        .collect::<Vec<_>>();
    let unique_ids = paragraph_ids
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();

    assert!(content_hpf.contains("<opf:title>Runtime Title</opf:title>"));
    assert!(version.contains("version=\"1.0\""));
    assert!(settings.contains("paraIDRef=\"2\""));
    assert!(settings.contains("pos=\"0\""));
    assert_eq!(paragraph_ids.len(), 3);
    assert_eq!(unique_ids.len(), paragraph_ids.len());
}

#[test]
fn legacy_report_like_documents_keep_hyperlinks_and_viewer_reflow_contract() {
    let markdown = "# Adoption Report\n\n\
이 문서는 [저장소](https://github.com/docxly/core-rs)와 [데모](https://docxly.github.io/core-rs/)를 함께 안내합니다.\n\n\
- 첫 번째 항목은 [가이드](https://example.com/guide) 링크를 포함합니다.\n\
- 두 번째 항목도 viewer가 줄바꿈을 다시 계산해야 하는 긴 설명을 유지합니다.";
    let generated = generate(markdown, HwpxOptions::default()).unwrap();
    let section = generated.section_xml().unwrap();
    let preview = generated.text_entry("Preview/PrvText.txt").unwrap();

    assert_eq!(section.matches("type=\"HYPERLINK\"").count(), 3);
    assert!(section.contains(
        "<hp:stringParam name=\"Path\">https://github.com/docxly/core-rs</hp:stringParam>"
    ));
    assert!(section.contains(
        "<hp:stringParam name=\"Path\">https://docxly.github.io/core-rs/</hp:stringParam>"
    ));
    assert!(
        section
            .contains("<hp:stringParam name=\"Path\">https://example.com/guide</hp:stringParam>")
    );
    assert!(section.contains("<hp:t>저장소</hp:t>"));
    assert!(section.contains("<hp:t>데모</hp:t>"));
    assert!(section.contains("<hp:t>가이드</hp:t>"));
    assert!(!section.contains("저장소 (https://github.com/docxly/core-rs)"));
    assert!(!section.contains("데모 (https://docxly.github.io/core-rs/)"));
    assert!(!section.contains("가이드 (https://example.com/guide)"));
    assert_eq!(section.matches("<hp:linesegarray>").count(), 1);
    assert_eq!(
        preview,
        "Adoption Report\n이 문서는 저장소와 데모를 함께 안내합니다.\n첫 번째 항목은 가이드 링크를 포함합니다.\n두 번째 항목도 viewer가 줄바꿈을 다시 계산해야 하는 긴 설명을 유지합니다."
    );
}
