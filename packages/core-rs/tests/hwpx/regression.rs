use std::path::Path;

use core_rs::{CoreRsError, HwpxOptions, HwpxParagraphAlign, HwpxStyleOptions, generate_hwpx};
use roxmltree::Document as XmlDocument;

use crate::hwpx_contract::{compare_contracts, snapshot_from_bytes, snapshot_from_expected_tree};
use crate::hwpx_fixture::{discover_fixtures, read_expected_entries};
use crate::hwpx_runtime::generate;

#[test]
fn lists_fail_in_strict_mode() {
    let markdown = "1. item";
    let error = generate_hwpx(markdown, HwpxOptions::default()).unwrap_err();
    assert!(matches!(error, CoreRsError::UnsupportedFeature(_)));
}

#[test]
fn unordered_lists_use_list_paragraph_contract_in_strict_mode() {
    let markdown = "- Alpha\n- Beta";
    let generated = generate(markdown, HwpxOptions::default()).unwrap();

    let section = generated.section_xml().unwrap();
    assert!(section.contains("paraPrIDRef=\"19\""));
    assert!(section.contains("styleIDRef=\"0\""));
    assert!(section.contains("<hp:t>Alpha</hp:t>"));
    assert!(section.contains("<hp:t>Beta</hp:t>"));
    assert!(section.contains("vertpos=\"0\""));
    assert!(section.contains("vertpos=\"2300\""));
    assert!(section.contains("horzpos=\"1100\" horzsize=\"41420\" flags=\"2490368\""));
    assert!(!section.contains("<hp:t>- Alpha</hp:t>"));
    assert!(!section.contains("<hp:t>- Beta</hp:t>"));
    assert!(!section.contains('•'));
    assert!(!section.contains("paraPrIDRef=\"14\""));
}

#[test]
fn nested_unordered_lists_use_second_list_level() {
    let markdown = "- Parent\n  - Child one\n  - Child two\n- Sibling";
    let generated = generate(markdown, HwpxOptions::default()).unwrap();

    let section = generated.section_xml().unwrap();
    assert!(section.contains("paraPrIDRef=\"19\""));
    assert!(section.contains("paraPrIDRef=\"20\""));
    assert!(section.contains("styleIDRef=\"0\""));
    assert!(section.contains("<hp:t>Parent</hp:t>"));
    assert!(section.contains("<hp:t>Child one</hp:t>"));
    assert!(section.contains("<hp:t>Child two</hp:t>"));
    assert!(section.contains("<hp:t>Sibling</hp:t>"));
    assert!(section.contains("vertpos=\"6900\""));
    assert!(section.contains("horzpos=\"1100\" horzsize=\"41420\" flags=\"2490368\""));
    assert!(section.contains("horzpos=\"2200\" horzsize=\"40320\" flags=\"2490368\""));
    assert!(!section.contains("<hp:t>- Parent</hp:t>"));
    assert!(!section.contains("<hp:t>- Child one</hp:t>"));
    assert!(!section.contains("<hp:t>- Child two</hp:t>"));
    assert!(!section.contains("<hp:t>- Sibling</hp:t>"));
    assert!(!section.contains('•'));
    assert!(!section.contains('◦'));
    assert!(!section.contains("paraPrIDRef=\"14\""));
    assert!(!section.contains("paraPrIDRef=\"15\""));
}

#[test]
fn header_reuses_existing_list_paragraph_contract() {
    let generated = generate("- Alpha\n  - Child", HwpxOptions::default()).unwrap();
    let header = generated.text_entry("Contents/header.xml").unwrap();

    assert!(header.contains("<hh:paraProperties itemCnt=\"21\">"));
    assert!(header.contains("<hh:bullets itemCnt=\"1\">"));
    assert!(header.contains("<hh:bullet id=\"1\""));
    assert!(header.contains("<hh:paraPr id=\"19\""));
    assert!(header.contains("<hh:paraPr id=\"20\""));
    assert!(header.contains("<hh:heading type=\"BULLET\" idRef=\"1\" level=\"0\"/>"));
    assert!(header.contains("<hh:style id=\"19\""));
    assert!(header.contains("<hh:style id=\"20\""));
}

#[test]
fn preview_text_omits_literal_markers_for_unordered_lists() {
    let markdown = "- Alpha\n- Beta\n- Gamma";
    let generated = generate(markdown, HwpxOptions::default()).unwrap();
    let preview = generated.text_entry("Preview/PrvText.txt").unwrap();
    let settings = generated.text_entry("settings.xml").unwrap();

    assert_eq!(preview, "Alpha\nBeta\nGamma");
    assert!(!preview.contains("- "));
    assert!(!preview.contains('•'));
    assert!(settings.contains("paraIDRef=\"2\""));
    assert!(settings.contains("pos=\"5\""));
}

#[test]
fn strict_mode_tables_render_as_hwpx_tables() {
    let markdown = "| Name | Role |\n| --- | --- |\n| A | Writer |\n| B | Reviewer |";
    let generated = generate(markdown, HwpxOptions::default()).unwrap();
    let section = generated.section_xml().unwrap();

    assert!(section.contains("<hp:tbl id=\"2104760845\""));
    assert!(section.contains("rowCnt=\"3\""));
    assert!(section.contains("colCnt=\"2\""));
    assert!(section.contains("<hp:t>Name</hp:t>"));
    assert!(section.contains("<hp:t>Reviewer</hp:t>"));
    assert!(!section.contains("<hp:t>Name | Role</hp:t>"));
}

#[test]
fn table_preview_text_matches_golden_contract() {
    let markdown = "| Name | Role |\n| --- | --- |\n| A | Writer |\n| B | Reviewer |";
    let generated = generate(markdown, HwpxOptions::default()).unwrap();
    let preview = generated.text_entry("Preview/PrvText.txt").unwrap();
    let settings = generated.text_entry("settings.xml").unwrap();

    assert_eq!(preview, "<Name><Role>\n<A><Writer>\n<B><Reviewer>");
    assert!(settings.contains("pos=\"16\""));
}

#[test]
fn tables_add_table_border_fill_to_header() {
    let markdown = "| Name | Role |\n| --- | --- |\n| A | Writer |";
    let generated = generate(markdown, HwpxOptions::default()).unwrap();
    let header = generated.text_entry("Contents/header.xml").unwrap();

    assert!(header.contains("<hh:borderFills itemCnt=\"3\">"));
    assert!(header.contains("<hh:borderFill id=\"3\""));
}

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
fn ordered_lists_use_semantic_paragraph_contract_in_non_strict_mode() {
    let markdown = "1. first\n2. second";
    let generated = generate(
        markdown,
        HwpxOptions {
            strict_mode: false,
            ..HwpxOptions::default()
        },
    )
    .unwrap();

    let section = generated.section_xml().unwrap();
    let preview = generated.text_entry("Preview/PrvText.txt").unwrap();
    let settings = generated.text_entry("settings.xml").unwrap();

    assert!(section.contains("paraPrIDRef=\"19\""));
    assert!(section.contains("horzpos=\"1100\" horzsize=\"41420\" flags=\"2490368\""));
    assert!(section.contains("<hp:t>first</hp:t>"));
    assert!(section.contains("<hp:t>second</hp:t>"));
    assert!(!section.contains("<hp:t>1. first</hp:t>"));
    assert!(!section.contains("<hp:t>2. second</hp:t>"));
    assert_eq!(preview, "first\nsecond");
    assert!(settings.contains("paraIDRef=\"0\""));
    assert!(settings.contains("pos=\"5\""));
}

#[test]
fn nested_ordered_lists_use_shape_specific_contract_in_non_strict_mode() {
    let markdown = "1. Parent\n  1. Child one\n  2. Child two\n2. Sibling";
    let generated = generate(
        markdown,
        HwpxOptions {
            strict_mode: false,
            ..HwpxOptions::default()
        },
    )
    .unwrap();

    let section = generated.section_xml().unwrap();
    let preview = generated.text_entry("Preview/PrvText.txt").unwrap();
    let settings = generated.text_entry("settings.xml").unwrap();

    assert!(section.contains("paraPrIDRef=\"20\""));
    assert!(section.contains("paraPrIDRef=\"19\""));
    assert!(section.contains("<hp:t>Parent</hp:t>"));
    assert!(section.contains("<hp:t>Child one</hp:t>"));
    assert!(section.contains("<hp:t>Child two</hp:t>"));
    assert!(section.contains("<hp:t>Sibling</hp:t>"));
    assert!(section.contains("horzpos=\"1100\" horzsize=\"41420\" flags=\"2490368\""));
    assert!(section.contains("horzpos=\"2200\" horzsize=\"40320\" flags=\"2490368\""));
    assert!(!section.contains("<hp:t>1. Parent</hp:t>"));
    assert!(!section.contains("<hp:t>1. Child one</hp:t>"));
    assert!(!section.contains("<hp:t>2. Child two</hp:t>"));
    assert!(!section.contains("<hp:t>2. Sibling</hp:t>"));
    assert_eq!(preview, "Parent\nChild one\nChild two\nSibling");
    assert!(settings.contains("paraIDRef=\"3\""));
    assert!(settings.contains("pos=\"7\""));
}

#[test]
fn ordered_list_headers_use_numbering_contract() {
    let basic = generate(
        "1. Alpha\n2. Beta\n3. Gamma",
        HwpxOptions {
            strict_mode: false,
            ..HwpxOptions::default()
        },
    )
    .unwrap();
    let basic_header = basic.text_entry("Contents/header.xml").unwrap();
    assert!(basic_header.contains("<hh:numberings itemCnt=\"2\">"));
    assert!(basic_header.contains("<hh:numbering id=\"2\" start=\"0\">"));
    assert!(basic_header.contains("<hh:paraProperties itemCnt=\"20\">"));
    assert!(basic_header.contains("<hh:paraPr id=\"19\""));
    assert!(basic_header.contains("<hh:heading type=\"NUMBER\" idRef=\"2\" level=\"0\"/>"));

    let nested = generate(
        "1. Parent\n  1. Child\n2. Sibling",
        HwpxOptions {
            strict_mode: false,
            ..HwpxOptions::default()
        },
    )
    .unwrap();
    let nested_header = nested.text_entry("Contents/header.xml").unwrap();
    assert!(nested_header.contains("<hh:numberings itemCnt=\"3\">"));
    assert!(nested_header.contains("<hh:numbering id=\"2\" start=\"1\">"));
    assert!(nested_header.contains("<hh:numbering id=\"3\" start=\"0\">"));
    assert!(nested_header.contains("<hh:paraProperties itemCnt=\"21\">"));
    assert!(nested_header.contains("<hh:paraPr id=\"19\""));
    assert!(nested_header.contains("<hh:paraPr id=\"20\""));
    assert!(nested_header.contains("<hh:heading type=\"NUMBER\" idRef=\"2\" level=\"0\"/>"));
    assert!(nested_header.contains("<hh:heading type=\"NUMBER\" idRef=\"3\" level=\"0\"/>"));
}

#[test]
fn nested_lists_over_depth_two_fail_in_strict_mode() {
    let markdown = "- A\n  - B\n    - C";
    let error = generate_hwpx(markdown, HwpxOptions::default()).unwrap_err();
    assert!(matches!(error, CoreRsError::UnsupportedFeature(_)));
}

#[test]
fn nested_lists_over_depth_two_fall_back_in_non_strict_mode() {
    let markdown = "- A\n  - B\n    - C";
    let generated = generate(
        markdown,
        HwpxOptions {
            strict_mode: false,
            ..HwpxOptions::default()
        },
    )
    .unwrap();

    let section = generated.section_xml().unwrap();
    assert!(section.contains("<hp:t>A</hp:t>"));
    assert!(section.contains("<hp:t>B</hp:t>"));
    assert!(section.contains("<hp:t>C</hp:t>"));
    assert!(section.contains("paraPrIDRef=\"20\""));
    assert!(!section.contains('•'));
    assert!(!section.contains('◦'));
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
    assert!(
        section.contains(
            "<hp:run charPrIDRef=\"14\"><hp:t>링크 (https://example.com)</hp:t></hp:run>"
        )
    );
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
    assert!(
        section.contains(
            "<hp:run charPrIDRef=\"14\"><hp:t>링크 (https://example.com)</hp:t></hp:run>"
        )
    );
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

#[test]
fn approved_style_fixtures_preserve_header_style_registry_contract() {
    for fixture_name in [
        "core-heading",
        "core-inline-style",
        "core-link-text",
        "style-typography",
        "style-centered-layout",
        "style-brand-color",
    ] {
        let fixture = approved_fixture(fixture_name);
        let input = crate::read_fixture_input(&fixture).unwrap();
        let generated = generate_hwpx(&input, fixture.options()).unwrap();
        let generated_entries = crate::normalized_entries(&generated).unwrap();
        let expected_entries = read_expected_entries(&fixture.root.join("expected")).unwrap();

        let generated_header = text_entry(&generated_entries, "Contents/header.xml");
        let expected_header = text_entry(&expected_entries, "Contents/header.xml");

        assert_eq!(
            header_style_contract(generated_header),
            header_style_contract(expected_header),
            "header style contract drifted for fixture {fixture_name}",
        );
    }
}

#[test]
fn approved_style_fixtures_preserve_section_style_references() {
    for fixture_name in [
        "core-heading",
        "core-inline-style",
        "core-link-text",
        "style-typography",
        "style-centered-layout",
        "style-brand-color",
    ] {
        let fixture = approved_fixture(fixture_name);
        let input = crate::read_fixture_input(&fixture).unwrap();
        let generated = generate_hwpx(&input, fixture.options()).unwrap();
        let generated_entries = crate::normalized_entries(&generated).unwrap();
        let expected_entries = read_expected_entries(&fixture.root.join("expected")).unwrap();

        let generated_section = text_entry(&generated_entries, "Contents/section0.xml");
        let expected_section = text_entry(&expected_entries, "Contents/section0.xml");

        assert_eq!(
            section_style_refs(generated_section),
            section_style_refs(expected_section),
            "section style references drifted for fixture {fixture_name}",
        );
    }
}

fn approved_fixture(name: &str) -> crate::hwpx_fixture::HwpxFixture {
    discover_fixtures()
        .unwrap()
        .into_iter()
        .find(|fixture| fixture.name == name)
        .unwrap_or_else(|| panic!("missing approved HWPX fixture: {name}"))
}

fn text_entry<'a>(
    entries: &'a std::collections::BTreeMap<String, crate::NormalizedEntry>,
    path: &str,
) -> &'a str {
    match entries
        .get(path)
        .unwrap_or_else(|| panic!("missing entry: {path}"))
    {
        crate::NormalizedEntry::Text(text) => text.as_str(),
        crate::NormalizedEntry::BinaryHash(_) => panic!("{path} was normalized as binary"),
    }
}

type HeaderStyleContract = (Vec<u32>, Vec<u32>, Vec<(u32, u32, u32)>);

fn header_style_contract(xml: &str) -> HeaderStyleContract {
    let doc = XmlDocument::parse(xml).unwrap();
    let mut char_ids = Vec::new();
    let mut para_ids = Vec::new();
    let mut style_triplets = Vec::new();

    for node in doc.descendants().filter(|node| node.is_element()) {
        match node.tag_name().name() {
            "charPr" if node.tag_name().namespace().is_some() => {
                if let Some(id) = node
                    .attribute("id")
                    .and_then(|value| value.parse::<u32>().ok())
                {
                    char_ids.push(id);
                }
            }
            "paraPr" if node.tag_name().namespace().is_some() => {
                if let Some(id) = node
                    .attribute("id")
                    .and_then(|value| value.parse::<u32>().ok())
                {
                    para_ids.push(id);
                }
            }
            "style" if node.tag_name().namespace().is_some() => {
                let Some(id) = node
                    .attribute("id")
                    .and_then(|value| value.parse::<u32>().ok())
                else {
                    continue;
                };
                let Some(para_pr) = node
                    .attribute("paraPrIDRef")
                    .and_then(|value| value.parse::<u32>().ok())
                else {
                    continue;
                };
                let Some(char_pr) = node
                    .attribute("charPrIDRef")
                    .and_then(|value| value.parse::<u32>().ok())
                else {
                    continue;
                };
                style_triplets.push((id, para_pr, char_pr));
            }
            _ => {}
        }
    }

    char_ids.sort_unstable();
    char_ids.dedup();
    para_ids.sort_unstable();
    para_ids.dedup();
    style_triplets.sort_unstable();

    (char_ids, para_ids, style_triplets)
}

fn section_style_refs(xml: &str) -> Vec<(u32, u32, Vec<u32>)> {
    let doc = XmlDocument::parse(xml).unwrap();
    let mut paragraphs = Vec::new();

    for paragraph in doc
        .descendants()
        .filter(|node| node.is_element() && node.tag_name().name() == "p")
    {
        let Some(para_pr) = paragraph
            .attribute("paraPrIDRef")
            .and_then(|value| value.parse::<u32>().ok())
        else {
            continue;
        };
        let Some(style_id) = paragraph
            .attribute("styleIDRef")
            .and_then(|value| value.parse::<u32>().ok())
        else {
            continue;
        };

        let mut char_refs = paragraph
            .children()
            .filter(|node| node.is_element() && node.tag_name().name() == "run")
            .filter_map(|run| run.attribute("charPrIDRef"))
            .filter_map(|value| value.parse::<u32>().ok())
            .collect::<Vec<_>>();
        char_refs.sort_unstable();
        char_refs.dedup();
        paragraphs.push((para_pr, style_id, char_refs));
    }

    paragraphs
}
