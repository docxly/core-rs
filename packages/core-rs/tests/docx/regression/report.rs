use core_rs::{
    ConversionTarget, DocxOptions, IssueSeverity, analyze_markdown, generate_docx_with_report,
};

#[test]
fn analyze_markdown_reports_docx_fallback_issues() {
    let report = analyze_markdown("<b>raw</b>", ConversionTarget::Docx);
    assert!(report.degraded);
    assert_eq!(report.unsupported_count, 0);
    assert_eq!(report.fallback_count, 1);
    assert_eq!(report.issues[0].feature, "HTML");
    assert_eq!(report.issues[0].severity, IssueSeverity::Warning);
}

#[test]
fn strict_docx_report_failure_promotes_unsupported_issue() {
    let failure = generate_docx_with_report("<b>raw</b>", DocxOptions::default()).unwrap_err();
    assert_eq!(failure.report.unsupported_count, 1);
    assert_eq!(failure.report.fallback_count, 0);
    assert_eq!(failure.report.issues[0].feature, "HTML");
    assert_eq!(failure.report.issues[0].severity, IssueSeverity::Error);
    assert_eq!(
        failure.report.issues[0].message,
        "unsupported feature: html"
    );
}

#[test]
fn compat_docx_report_marks_degraded_success() {
    let generated = generate_docx_with_report(
        "<b>raw</b>",
        DocxOptions {
            strict_mode: false,
            ..DocxOptions::default()
        },
    )
    .unwrap();

    assert!(generated.report.degraded);
    assert_eq!(generated.report.unsupported_count, 0);
    assert_eq!(generated.report.fallback_count, 1);
    assert!(!generated.bytes.is_empty());
}
