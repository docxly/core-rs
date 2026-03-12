use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[path = "support/archive_fixture.rs"]
mod archive_fixture;

use archive_fixture::{
    FixtureResult, comparison_excludes_for_fixture, discover_fixture_dirs,
    filtered_normalized_entries, refresh_fixture_metadata,
};

const IGNORED_AUXILIARY_NAMES: &[&str] = &[".DS_Store", "README.md", "provenance.md"];
const GENERATED_OUTPUTS_ROOT: &str = "/tmp/hwpx-generator-outputs";
const REFRESH_EMOJI: &str = "📦";
const EQUIVALENT_EMOJI: &str = "✅";
const DIFFERENT_EMOJI: &str = "❌";

fn main() {
    if let Err(error) = run(env::args().skip(1).collect()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(names: Vec<String>) -> FixtureResult<()> {
    let fixtures_root = approved_fixtures_root();
    let fixtures = resolve_fixture_dirs(&fixtures_root, &names)?;

    for fixture in fixtures {
        let hash = refresh_fixture_metadata(
            &fixture,
            "golden.hwpx",
            "expected",
            "hash.txt",
            IGNORED_AUXILIARY_NAMES,
            IGNORED_AUXILIARY_NAMES,
            is_text_entry,
        )?;
        let name = fixture
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or("invalid fixture directory name")?;
        let golden_path = fixture.join("golden.hwpx");
        println!("{REFRESH_EMOJI} fixture: {name}");
        println!("   golden: {}", golden_path.display());
        println!("   hash:   {hash}");
        if let Some(line) = generated_output_comparison_line(&fixture)? {
            println!("{line}");
        }
    }

    Ok(())
}

fn approved_fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("hwpx")
        .join("approved")
}

fn generated_output_comparison_line(fixture: &Path) -> FixtureResult<Option<String>> {
    let name = fixture
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or("invalid fixture directory name")?;
    let generated_path = Path::new(GENERATED_OUTPUTS_ROOT)
        .join(name)
        .join("generated.hwpx");
    if !generated_path.is_file() {
        return Ok(None);
    }

    let generated = fs::read(&generated_path)?;
    let golden = fs::read(fixture.join("golden.hwpx"))?;
    let excludes = comparison_excludes_for_fixture(fixture)?;
    let generated_entries = filtered_normalized_entries(&generated, is_text_entry, &excludes)?;
    let golden_entries = filtered_normalized_entries(&golden, is_text_entry, &excludes)?;
    let (emoji, status) = if generated_entries == golden_entries {
        (EQUIVALENT_EMOJI, "equivalent")
    } else {
        (DIFFERENT_EMOJI, "different")
    };

    let golden_path = fixture.join("golden.hwpx");
    Ok(Some(format!(
        "   compare: {emoji} {} {status} {}",
        generated_path.display(),
        golden_path.display()
    )))
}

fn resolve_fixture_dirs(root: &Path, names: &[String]) -> FixtureResult<Vec<PathBuf>> {
    if names.is_empty() {
        return Ok(discover_fixture_dirs(root)?
            .into_iter()
            .filter(|path| is_refreshable_fixture_dir(path))
            .collect());
    }

    let mut resolved = Vec::with_capacity(names.len());
    for name in names {
        let path = root.join(name);
        if !is_refreshable_fixture_dir(&path) {
            return Err(format!("unknown HWPX approved fixture: {name}").into());
        }
        resolved.push(path);
    }

    Ok(resolved)
}

fn is_refreshable_fixture_dir(path: &Path) -> bool {
    path.is_dir()
        && path.join("input.md").is_file()
        && path.join("fixture.toml").is_file()
        && path.join("golden.hwpx").is_file()
}

fn is_text_entry(path: &str) -> bool {
    path == "mimetype"
        || path.ends_with(".xml")
        || path.ends_with(".rdf")
        || path.ends_with(".rels")
        || path.ends_with(".txt")
        || path.ends_with(".hpf")
}
