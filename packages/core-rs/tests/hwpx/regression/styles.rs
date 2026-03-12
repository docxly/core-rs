use super::*;

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
    assert!(header.contains(
        "<hh:charPr id=\"9\" height=\"1000\" textColor=\"#333333\" shadeColor=\"none\" useFontSpace=\"0\" useKerning=\"0\" symMark=\"NONE\" borderFillIDRef=\"2\"><hh:fontRef hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:ratio hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:spacing hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/>"
    ));
    assert!(header.contains("<hh:charPr id=\"14\" height=\"1000\" textColor=\"#0563C1\""));
    assert!(section.contains("<hp:run charPrIDRef=\"6\"><hp:t>일반 </hp:t></hp:run>"));
    assert!(section.contains("<hp:run charPrIDRef=\"7\"><hp:t>굵게</hp:t></hp:run>"));
    assert!(section.contains("<hp:run charPrIDRef=\"8\"><hp:t>기울임</hp:t></hp:run>"));
    assert!(section.contains("<hp:run charPrIDRef=\"9\"><hp:t>코드</hp:t></hp:run>"));
    assert!(section.contains("type=\"HYPERLINK\""));
    assert!(section.contains("<hp:stringParam name=\"Path\">https://example.com</hp:stringParam>"));
    assert!(section.contains("<hp:t>링크</hp:t>"));
    assert!(!section.contains("링크 (https://example.com)"));
}

#[test]
fn default_header_registry_does_not_leave_negative_tracking() {
    let generated = generate("본문", HwpxOptions::default()).unwrap();
    let header = generated.text_entry("Contents/header.xml").unwrap();

    assert!(!header.contains("hangul=\"-5\""));
    assert!(header.contains("<hh:charProperties itemCnt=\"6\">"));
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
    assert!(section.contains("type=\"HYPERLINK\""));
    assert!(section.contains("<hp:stringParam name=\"Path\">https://example.com</hp:stringParam>"));
    assert!(section.contains("<hp:t>링크</hp:t>"));
    assert!(!section.contains("링크 (https://example.com)"));
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
