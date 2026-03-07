use core_rs::{CoreRsError, HwpxOptions, generate_hwpx};

#[test]
fn returns_not_implemented_for_hwpx_generation() {
    let error = generate_hwpx("hello", HwpxOptions::default()).unwrap_err();
    assert!(matches!(error, CoreRsError::UnsupportedFeature(_)));
}
