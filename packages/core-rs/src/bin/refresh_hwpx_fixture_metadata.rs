use std::env;
use std::path::{Path, PathBuf};

#[path = "support/archive_fixture.rs"]
mod archive_fixture;

use archive_fixture::{FixtureResult, discover_fixture_dirs, refresh_fixture_metadata};

const IGNORED_AUXILIARY_NAMES: &[&str] = &[".DS_Store", "README.md", "provenance.md"];

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
        println!("{name}: {hash}");
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
