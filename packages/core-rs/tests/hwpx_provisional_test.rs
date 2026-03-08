#[path = "support/hwpx_fixture.rs"]
mod hwpx_fixture;
#[path = "support/hwpx_runtime.rs"]
mod hwpx_runtime;

use hwpx_fixture::discover_provisional_fixtures;

#[test]
#[ignore = "provisional HWPX fixtures are compatibility snapshots, not CI release gates"]
fn provisional_hwpx_fixtures_match_current_output() {
    let fixtures = discover_provisional_fixtures().unwrap();
    if fixtures.is_empty() {
        return;
    }

    for fixture in fixtures {
        let input = hwpx_fixture::read_fixture_input(&fixture).unwrap();
        let generated = core_rs::generate_hwpx(&input, fixture.options()).unwrap();
        let generated_normalized = hwpx_fixture::normalized_entries(&generated).unwrap();

        let golden_bytes = std::fs::read(fixture.root.join("golden.hwpx")).unwrap();
        let golden_normalized = hwpx_fixture::normalized_entries(&golden_bytes).unwrap();

        assert_eq!(
            generated_normalized, golden_normalized,
            "provisional HWPX fixture drifted: {}",
            fixture.name
        );
    }
}
