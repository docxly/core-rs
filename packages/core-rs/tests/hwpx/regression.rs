use std::path::Path;

use core_rs::{
    CoreRsError, HwpxOptions, HwpxParagraphAlign, HwpxStyleOptions, generate_hwpx,
};

use crate::hwpx_contract::{compare_contracts, snapshot_from_bytes, snapshot_from_expected_tree};
use crate::hwpx_runtime::generate;

#[test]
fn lists_fail_in_strict_mode() {
    let markdown = "- item";
    let error = generate_hwpx(markdown, HwpxOptions::default()).unwrap_err();
    assert!(matches!(error, CoreRsError::UnsupportedFeature(_)));
}

#[test]
fn lists_fall_back_to_plain_text_in_non_strict_mode() {
    let markdown = "- first\n- second";
    let generated = generate(
        markdown,
        HwpxOptions {
            strict_mode: false,
            ..HwpxOptions::default()
        },
    )
    .unwrap();

    let section = generated.section_xml().unwrap();
    assert!(section.contains("- first"));
    assert!(section.contains("- second"));
}

#[test]
fn links_are_rendered_as_visible_text() {
    let markdown = "[문서 열기](https://example.com)";
    let generated = generate(markdown, HwpxOptions::default()).unwrap();
    let section = generated.section_xml().unwrap();
    assert!(section.contains("문서 열기 (https://example.com)"));
}

#[test]
fn empty_link_label_renders_url_only() {
    let markdown = "[](https://example.com)";
    let generated = generate(markdown, HwpxOptions::default()).unwrap();
    let section = generated.section_xml().unwrap();
    assert!(section.contains("https://example.com"));
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
    assert!(content_hpf.contains("<opf:meta name=\"creator\" content=\"text\">(주)한글과컴퓨터</opf:meta>"));
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
fn hard_breaks_flatten_to_visible_space_in_phase_a() {
    let generated = generate("hello  \nworld", HwpxOptions::default()).unwrap();
    let section = generated.section_xml().unwrap();
    assert!(section.contains("<hp:t>hello world</hp:t>"));
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

    assert!(diff.missing_entries.is_empty(), "missing entries: {:?}", diff.missing_entries);
    assert!(diff.extra_entries.is_empty(), "extra entries: {:?}", diff.extra_entries);
    assert!(diff.mismatched_roots.is_empty(), "root mismatch: {:?}", diff.mismatched_roots);

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
fn headings_use_outline_style_refs_and_heading_char_properties() {
    let generated = generate("# Heading", HwpxOptions::default()).unwrap();
    let header = generated.text_entry("Contents/header.xml").unwrap();
    let section = generated.section_xml().unwrap();

    assert!(header.contains("<hh:charProperties itemCnt=\"15\">"));
    assert!(header.contains("<hh:charPr id=\"10\" height=\"1400\""));
    assert!(section.contains("paraPrIDRef=\"2\""));
    assert!(section.contains("styleIDRef=\"2\""));
    assert!(section.contains("charPrIDRef=\"10\""));
}

#[test]
fn inline_styles_render_as_distinct_runs() {
    let generated = generate(
        "일반 **굵게** *기울임* `코드` [링크](https://example.com)",
        HwpxOptions::default(),
    )
    .unwrap();
    let header = generated.text_entry("Contents/header.xml").unwrap();
    let section = generated.section_xml().unwrap();

    assert!(header.contains("<hh:charPr id=\"7\" height=\"1000\""));
    assert!(header.contains("<hh:charPr id=\"8\" height=\"1000\""));
    assert!(header.contains("<hh:charPr id=\"9\" height=\"1000\""));
    assert!(header.contains("<hh:charPr id=\"14\" height=\"1000\" textColor=\"#0563C1\""));
    assert!(section.contains("<hp:run charPrIDRef=\"6\"><hp:t>일반 </hp:t></hp:run>"));
    assert!(section.contains("<hp:run charPrIDRef=\"7\"><hp:t>굵게</hp:t></hp:run>"));
    assert!(section.contains("<hp:run charPrIDRef=\"8\"><hp:t>기울임</hp:t></hp:run>"));
    assert!(section.contains("<hp:run charPrIDRef=\"9\"><hp:t>코드</hp:t></hp:run>"));
    assert!(section.contains("<hp:run charPrIDRef=\"14\"><hp:t>링크 (https://example.com)</hp:t></hp:run>"));
}

#[test]
fn custom_style_overrides_header_fonts_colors_and_sizes() {
    let generated = generate(
        "# Heading\n\n본문 [링크](https://example.com)",
        HwpxOptions {
            style: HwpxStyleOptions {
                body_font: Some("Pretendard".to_string()),
                heading_font: Some("MaruBuri".to_string()),
                body_font_size: Some(1120),
                heading_font_size: Some(1560),
                text_color: Some("444444".to_string()),
                heading_color: Some("AA2200".to_string()),
                link_color: Some("0088CC".to_string()),
                paragraph_align: Some(HwpxParagraphAlign::Center),
            },
            ..HwpxOptions::default()
        },
    )
    .unwrap();
    let header = generated.text_entry("Contents/header.xml").unwrap();

    assert!(header.contains("face=\"Pretendard\""));
    assert!(header.contains("face=\"MaruBuri\""));
    assert!(header.contains("<hh:charPr id=\"6\" height=\"1120\" textColor=\"#444444\""));
    assert!(header.contains("<hh:charPr id=\"10\" height=\"1560\" textColor=\"#AA2200\""));
    assert!(header.contains("<hh:charPr id=\"11\" height=\"1460\" textColor=\"#AA2200\""));
    assert!(header.contains("<hh:charPr id=\"14\" height=\"1120\" textColor=\"#0088CC\""));
    assert!(header.contains("<hh:align horizontal=\"CENTER\" vertical=\"BASELINE\"/>"));
    assert!(header.contains("<hh:paraPr id=\"2\""));
    assert!(header.contains("<hh:paraPr id=\"3\""));
}

#[test]
fn custom_style_keeps_run_ids_but_uses_updated_header_contract() {
    let generated = generate(
        "일반 **굵게** *기울임* `코드` [링크](https://example.com)",
        HwpxOptions {
            style: HwpxStyleOptions {
                body_font_size: Some(1080),
                text_color: Some("#222222".to_string()),
                link_color: Some("#CC3300".to_string()),
                ..HwpxStyleOptions::default()
            },
            ..HwpxOptions::default()
        },
    )
    .unwrap();
    let header = generated.text_entry("Contents/header.xml").unwrap();
    let section = generated.section_xml().unwrap();

    assert!(header.contains("<hh:charPr id=\"7\" height=\"1080\" textColor=\"#222222\""));
    assert!(header.contains("<hh:charPr id=\"8\" height=\"1080\" textColor=\"#222222\""));
    assert!(header.contains("<hh:charPr id=\"14\" height=\"1080\" textColor=\"#CC3300\""));
    assert!(section.contains("<hp:run charPrIDRef=\"7\"><hp:t>굵게</hp:t></hp:run>"));
    assert!(section.contains("<hp:run charPrIDRef=\"8\"><hp:t>기울임</hp:t></hp:run>"));
    assert!(section.contains("<hp:run charPrIDRef=\"14\"><hp:t>링크 (https://example.com)</hp:t></hp:run>"));
}

#[test]
fn custom_heading_font_does_not_leak_into_body_inline_runs() {
    let generated = generate(
        "본문 **굵게** *기울임* [링크](https://example.com)",
        HwpxOptions {
            style: HwpxStyleOptions {
                heading_font: Some("MaruBuri".to_string()),
                ..HwpxStyleOptions::default()
            },
            ..HwpxOptions::default()
        },
    )
    .unwrap();
    let header = generated.text_entry("Contents/header.xml").unwrap();

    assert!(header.contains("face=\"함초롬바탕\""));
    assert!(header.contains("face=\"MaruBuri\""));
    assert!(header.contains("<hh:charPr id=\"7\""));
    assert!(header.contains("<hh:fontRef hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/>"));

    let body_bold = header
        .split("<hh:charPr id=\"7\"")
        .nth(1)
        .and_then(|rest| rest.split("</hh:charPr>").next())
        .unwrap();
    let body_italic = header
        .split("<hh:charPr id=\"8\"")
        .nth(1)
        .and_then(|rest| rest.split("</hh:charPr>").next())
        .unwrap();
    let body_link = header
        .split("<hh:charPr id=\"14\"")
        .nth(1)
        .and_then(|rest| rest.split("</hh:charPr>").next())
        .unwrap();

    assert!(body_bold.contains("hangul=\"0\""));
    assert!(body_italic.contains("hangul=\"0\""));
    assert!(body_link.contains("hangul=\"0\""));
}

#[test]
fn paragraph_alignment_applies_to_heading_paragraph_styles() {
    let generated = generate(
        "# Heading\n\n본문",
        HwpxOptions {
            style: HwpxStyleOptions {
                paragraph_align: Some(HwpxParagraphAlign::Right),
                ..HwpxStyleOptions::default()
            },
            ..HwpxOptions::default()
        },
    )
    .unwrap();
    let header = generated.text_entry("Contents/header.xml").unwrap();

    for para_id in [0, 2, 3, 4, 5, 6, 7] {
        let marker = format!("<hh:paraPr id=\"{para_id}\"");
        let segment = header
            .split(&marker)
            .nth(1)
            .and_then(|rest| rest.split("</hh:paraPr>").next())
            .unwrap();
        assert!(segment.contains("<hh:align horizontal=\"RIGHT\" vertical=\"BASELINE\"/>"));
    }
}

#[test]
fn invalid_hwpx_color_option_fails_fast() {
    let error = generate_hwpx(
        "본문",
        HwpxOptions {
            style: HwpxStyleOptions {
                text_color: Some("not-a-color".to_string()),
                ..HwpxStyleOptions::default()
            },
            ..HwpxOptions::default()
        },
    )
    .unwrap_err();

    assert!(matches!(error, CoreRsError::InvalidOption(_)));
}

#[test]
fn invalid_hwpx_font_name_fails_fast() {
    let error = generate_hwpx(
        "본문",
        HwpxOptions {
            style: HwpxStyleOptions {
                body_font: Some("   ".to_string()),
                ..HwpxStyleOptions::default()
            },
            ..HwpxOptions::default()
        },
    )
    .unwrap_err();

    assert!(matches!(error, CoreRsError::InvalidOption(_)));
}

#[test]
fn invalid_hwpx_font_size_fails_fast() {
    let error = generate_hwpx(
        "본문",
        HwpxOptions {
            style: HwpxStyleOptions {
                body_font_size: Some(5000),
                ..HwpxStyleOptions::default()
            },
            ..HwpxOptions::default()
        },
    )
    .unwrap_err();

    assert!(matches!(error, CoreRsError::InvalidOption(_)));
}
