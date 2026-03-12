use crate::hwpx_fixture::discover_fixtures;
use crate::{assert_fixture_matches_golden, assert_same_input_same_hash};

#[test]
fn all_hwpx_fixtures_match_golden() {
    let fixtures = discover_fixtures().unwrap();
    if fixtures.is_empty() {
        eprintln!("no approved HWPX fixtures discovered; compatibility bring-up still in progress");
        return;
    }

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

    if deterministic.is_empty() {
        eprintln!("no approved deterministic HWPX fixtures configured");
        return;
    }

    for fixture in deterministic {
        assert_same_input_same_hash(fixture);
    }
}

#[test]
fn all_approved_hwpx_fixtures_are_manually_verified() {
    let fixtures = discover_fixtures().unwrap();
    for fixture in &fixtures {
        assert!(
            fixture.manual_verified,
            "approved HWPX fixture must set manual_verified = true: {}",
            fixture.name
        );
    }
}
