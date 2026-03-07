use crate::docx_fixture::discover_fixtures;
use crate::{assert_fixture_matches_golden, assert_same_input_same_hash};

#[test]
fn all_docx_fixtures_match_golden() {
    let fixtures = discover_fixtures().unwrap();
    assert!(!fixtures.is_empty(), "no DOCX fixtures discovered");

    for fixture in &fixtures {
        assert_fixture_matches_golden(fixture);
    }
}

#[test]
fn deterministic_fixtures_generate_same_hash() {
    let fixtures = discover_fixtures().unwrap();
    let deterministic = fixtures
        .iter()
        .filter(|fixture| fixture.determinism)
        .collect::<Vec<_>>();

    assert!(
        !deterministic.is_empty(),
        "no deterministic DOCX fixtures configured"
    );

    for fixture in deterministic {
        assert_same_input_same_hash(fixture);
    }
}
