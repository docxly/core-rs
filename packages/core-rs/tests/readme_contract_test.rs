use std::fs;
use std::path::Path;

#[test]
fn readmes_include_strict_compat_capability_contract() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let root_readme = fs::read_to_string(crate_root.join("../../README.md")).unwrap();
    let crate_readme = fs::read_to_string(crate_root.join("README.md")).unwrap();
    let npm_readme = fs::read_to_string(crate_root.join("../npm-core-rs/README.md")).unwrap();

    for readme in [&root_readme, &crate_readme, &npm_readme] {
        assert!(readme.contains("| Capability | DOCX | HWPX strict | HWPX compat |"));
    }

    assert!(root_readme.contains("Experimental API"));
    assert!(crate_readme.contains("generate_docx_with_report"));
    assert!(npm_readme.contains("experimental HWPX API"));
    assert!(npm_readme.contains("generateHwpxWithReport"));
}

#[test]
fn benchmark_docs_do_not_freeze_old_summary_numbers() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let root_readme = fs::read_to_string(crate_root.join("../../README.md")).unwrap();
    let ko_readme = fs::read_to_string(crate_root.join("../../docs/ko/README.md")).unwrap();
    let ko_benchmark = fs::read_to_string(crate_root.join("../../docs/ko/benchmark.md")).unwrap();
    let ko_design = fs::read_to_string(crate_root.join("../../docs/ko/design-system.md")).unwrap();

    for doc in [&root_readme, &ko_readme, &ko_benchmark, &ko_design] {
        assert!(!doc.contains("105x"));
        assert!(!doc.contains("80 ms cold / 2 ms steady"));
        assert!(!doc.contains("284 ms cold / 210 ms steady"));
    }

    assert!(root_readme.contains("comparison block below"));
    assert!(ko_readme.contains("comparison block"));
    assert!(ko_benchmark.contains("comparison-data.json"));
    assert!(ko_design.contains("shared benchmark dataset"));
}
