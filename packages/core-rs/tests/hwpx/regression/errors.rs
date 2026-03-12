use super::*;

#[test]
fn analyze_markdown_reports_hwpx_ordered_list_contract() {
    let report = analyze_markdown("1. first\n2. second", ConversionTarget::Hwpx);
    assert_eq!(report.unsupported_count, 0);
    assert_eq!(report.fallback_count, 0);
    assert_eq!(report.issues[0].feature, "HWPX ordered list in strict mode");
    assert_eq!(report.issues[0].severity, IssueSeverity::Warning);
}

#[test]
fn strict_hwpx_report_failure_promotes_ordered_list_issue() {
    let failure =
        generate_hwpx_with_report("1. first\n2. second", HwpxOptions::default()).unwrap_err();
    assert_eq!(failure.report.unsupported_count, 1);
    assert_eq!(
        failure.report.issues[0].feature,
        "HWPX ordered list in strict mode"
    );
    assert_eq!(failure.report.issues[0].severity, IssueSeverity::Error);
    assert_eq!(
        failure.report.issues[0].message,
        "unsupported feature: HWPX approved baseline does not support ordered list"
    );
}

#[test]
fn compat_hwpx_report_marks_image_fallback() {
    let markdown = "![diagram](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO5W5V8AAAAASUVORK5CYII=)";
    let generated = generate_hwpx_with_report(
        markdown,
        HwpxOptions {
            strict_mode: false,
            ..HwpxOptions::default()
        },
    )
    .unwrap();

    assert!(generated.report.degraded);
    assert_eq!(generated.report.unsupported_count, 0);
    assert_eq!(generated.report.fallback_count, 1);
    assert_eq!(generated.report.issues[0].feature, "HWPX image");
}

#[test]
fn analyze_markdown_does_not_report_blockquote_as_hwpx_fallback() {
    let report = analyze_markdown("> quoted", ConversionTarget::Hwpx);
    assert!(report.issues.is_empty());
    assert!(!report.degraded);
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
fn invalid_hwpx_option_is_reported_in_generation_failure() {
    let failure = generate_hwpx_with_report(
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

    assert!(matches!(failure.error, CoreRsError::InvalidOption(_)));
    assert_eq!(failure.report.unsupported_count, 1);
    assert_eq!(failure.report.fallback_count, 0);
    assert_eq!(failure.report.issues[0].feature, "invalid option");
    assert_eq!(failure.report.issues[0].severity, IssueSeverity::Error);
    assert!(failure.report.issues[0].message.contains("invalid option"));
}
