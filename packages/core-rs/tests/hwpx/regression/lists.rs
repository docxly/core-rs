use super::*;

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
    assert!(!section.contains("<hp:linesegarray>"));
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
    assert!(!section.contains("<hp:linesegarray>"));
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
    assert!(!section.contains("<hp:linesegarray>"));
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
    assert!(!section.contains("<hp:linesegarray>"));
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
