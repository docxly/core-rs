use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[path = "support/archive_fixture.rs"]
mod archive_fixture;

use archive_fixture::{
    ArchiveEntry, hash_entries, normalized_archives_equivalent, normalized_entries,
    read_expected_entries, refresh_fixture_metadata,
};

const AUXILIARY_NAMES: &[&str] = &[".DS_Store", "README.md", "provenance.md"];

#[test]
fn refresh_rebuilds_sha256_expected_tree_and_hash() {
    let temp = TestTempDir::new("hwpx-refresh-sha256");
    let fixture = copy_fixture_to_temp("core-paragraph", &temp);

    fs::remove_dir_all(fixture.join("expected")).unwrap();
    fs::write(fixture.join("hash.txt"), "stale").unwrap();

    let hash = refresh_fixture_metadata(
        &fixture,
        "golden.hwpx",
        "expected",
        "hash.txt",
        AUXILIARY_NAMES,
        AUXILIARY_NAMES,
        is_text_entry,
    )
    .unwrap();

    let golden = fs::read(fixture.join("golden.hwpx")).unwrap();
    let normalized = normalized_entries(&golden, is_text_entry).unwrap();
    let expected =
        read_expected_entries(&fixture.join("expected"), AUXILIARY_NAMES, is_text_entry).unwrap();

    assert_eq!(expected, normalized);
    assert_eq!(fs::read_to_string(fixture.join("hash.txt")).unwrap(), hash);
    assert_eq!(hash, hash_entries(&normalized));
    assert!(
        fixture
            .join("expected/Preview/PrvImage.png.sha256")
            .is_file()
    );
    assert!(!fixture.join("expected/Preview/PrvImage.png").exists());
}

#[test]
fn refresh_preserves_raw_binary_representation_when_fixture_already_uses_it() {
    let temp = TestTempDir::new("hwpx-refresh-raw");
    let fixture = copy_fixture_to_temp("list-basic", &temp);

    fs::write(fixture.join("expected/Preview/PrvImage.png"), b"corrupted").unwrap();
    fs::write(fixture.join("hash.txt"), "stale").unwrap();

    refresh_fixture_metadata(
        &fixture,
        "golden.hwpx",
        "expected",
        "hash.txt",
        AUXILIARY_NAMES,
        AUXILIARY_NAMES,
        is_text_entry,
    )
    .unwrap();

    let golden = fs::read(fixture.join("golden.hwpx")).unwrap();
    let archive = archive_fixture::archive_entries(&golden, is_text_entry).unwrap();
    let preview = archive.get("Preview/PrvImage.png").unwrap();

    match preview {
        ArchiveEntry::Binary { bytes, .. } => {
            assert_eq!(
                fs::read(fixture.join("expected/Preview/PrvImage.png")).unwrap(),
                *bytes
            );
        }
        ArchiveEntry::Text(_) => panic!("preview image unexpectedly normalized as text"),
    }

    assert!(
        !fixture
            .join("expected/Preview/PrvImage.png.sha256")
            .exists()
    );
}

#[test]
fn refresh_removes_stale_outputs_and_preserves_auxiliary_files() {
    let temp = TestTempDir::new("hwpx-refresh-stale");
    let fixture = copy_fixture_to_temp("core-paragraph", &temp);

    fs::create_dir_all(fixture.join("expected/Stale/Inner")).unwrap();
    fs::write(fixture.join("expected/Stale/Inner/ghost.txt"), "remove me").unwrap();
    fs::write(
        fixture.join("expected/Preview/PrvImage.png"),
        "wrong format",
    )
    .unwrap();
    fs::write(fixture.join("expected/README.md"), "keep me").unwrap();
    fs::write(fixture.join("expected/provenance.md"), "keep me too").unwrap();

    refresh_fixture_metadata(
        &fixture,
        "golden.hwpx",
        "expected",
        "hash.txt",
        AUXILIARY_NAMES,
        AUXILIARY_NAMES,
        is_text_entry,
    )
    .unwrap();

    assert!(!fixture.join("expected/Stale").exists());
    assert!(!fixture.join("expected/Preview/PrvImage.png").exists());
    assert!(
        fixture
            .join("expected/Preview/PrvImage.png.sha256")
            .is_file()
    );
    assert_eq!(
        fs::read_to_string(fixture.join("expected/README.md")).unwrap(),
        "keep me"
    );
    assert_eq!(
        fs::read_to_string(fixture.join("expected/provenance.md")).unwrap(),
        "keep me too"
    );
}

#[test]
fn cli_fails_for_unknown_fixture_name() {
    let output = Command::new(env!("CARGO_BIN_EXE_refresh_hwpx_fixture_metadata"))
        .arg("does-not-exist")
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("unknown HWPX approved fixture: does-not-exist"));
}

#[test]
fn cli_prints_equivalent_status_for_matching_generated_output() {
    let _guard = GeneratedOutputGuard::backup("core-paragraph");
    let generated = generated_output_root("core-paragraph");
    fs::create_dir_all(generated.parent().unwrap()).unwrap();
    fs::copy(
        fixture_root().join("core-paragraph/golden.hwpx"),
        &generated,
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_refresh_hwpx_fixture_metadata"))
        .arg("core-paragraph")
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let golden_path = fixture_root().join("core-paragraph/golden.hwpx");
    let generated_path = generated_output_root("core-paragraph");
    assert!(stdout.contains("📦 fixture: core-paragraph"));
    assert!(stdout.contains(&format!("golden: {}", golden_path.display())));
    assert!(stdout.contains(&format!(
        "compare: ✅ {} equivalent {}",
        generated_path.display(),
        golden_path.display()
    )));
}

#[test]
fn normalized_archive_comparison_detects_differences() {
    let left = fs::read(fixture_root().join("core-paragraph/golden.hwpx")).unwrap();
    let right = fs::read(fixture_root().join("table-basic/golden.hwpx")).unwrap();

    assert!(!normalized_archives_equivalent(&left, &right, is_text_entry).unwrap());
}

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("hwpx")
        .join("approved")
}

fn copy_fixture_to_temp(name: &str, temp: &TestTempDir) -> PathBuf {
    let source = fixture_root().join(name);
    let destination = temp.path().join(name);
    copy_dir_recursive(&source, &destination);
    destination
}

fn copy_dir_recursive(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if source_path.is_dir() {
            copy_dir_recursive(&source_path, &destination_path);
        } else {
            fs::copy(&source_path, &destination_path).unwrap();
        }
    }
}

fn is_text_entry(path: &str) -> bool {
    path == "mimetype"
        || path.ends_with(".xml")
        || path.ends_with(".rdf")
        || path.ends_with(".rels")
        || path.ends_with(".txt")
        || path.ends_with(".hpf")
}

struct TestTempDir {
    path: PathBuf,
}

impl TestTempDir {
    fn new(prefix: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("{prefix}-{nanos}-{}", std::process::id()));
        fs::create_dir_all(&path).unwrap();
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestTempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn generated_output_root(name: &str) -> PathBuf {
    Path::new("/tmp")
        .join("hwpx-generator-outputs")
        .join(name)
        .join("generated.hwpx")
}

struct GeneratedOutputGuard {
    target_dir: PathBuf,
    backup_dir: Option<PathBuf>,
}

impl GeneratedOutputGuard {
    fn backup(name: &str) -> Self {
        let target_dir = Path::new("/tmp").join("hwpx-generator-outputs").join(name);
        let backup_dir = if target_dir.exists() {
            let backup = std::env::temp_dir().join(format!(
                "hwpx-generator-outputs-backup-{name}-{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            if let Some(parent) = backup.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::rename(&target_dir, &backup).unwrap();
            Some(backup)
        } else {
            None
        };

        Self {
            target_dir,
            backup_dir,
        }
    }
}

impl Drop for GeneratedOutputGuard {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.target_dir);
        if let Some(backup_dir) = &self.backup_dir {
            if let Some(parent) = self.target_dir.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::rename(backup_dir, &self.target_dir);
        }
    }
}
