#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use core_rs::DocxOptions;
use serde::Deserialize;
#[path = "archive_fixture.rs"]
mod archive_fixture;

pub(crate) use archive_fixture::{FixtureResult, NormalizedEntry};

#[derive(Debug, Deserialize)]
struct FixtureConfig {
    title: String,
    author: String,
    strict_mode: bool,
    determinism: bool,
}

#[derive(Debug, Clone)]
pub struct DocxFixture {
    pub name: String,
    pub root: PathBuf,
    pub title: String,
    pub author: String,
    pub strict_mode: bool,
    pub determinism: bool,
}

impl DocxFixture {
    pub fn options(&self) -> DocxOptions {
        DocxOptions {
            title: Some(self.title.clone()),
            author: Some(self.author.clone()),
            strict_mode: self.strict_mode,
        }
    }
}

pub fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("docx")
}

pub fn discover_fixtures() -> FixtureResult<Vec<DocxFixture>> {
    let root = fixtures_root();
    let fixtures = archive_fixture::discover_fixture_dirs(&root)?;

    let mut discovered = Vec::new();

    for path in fixtures {
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if name.starts_with('.') || !path.is_dir() {
            continue;
        }

        let input_path = path.join("input.md");
        let config_path = path.join("fixture.toml");
        let golden_path = path.join("golden.docx");
        let expected_path = path.join("expected");
        let hash_path = path.join("hash.txt");

        if !input_path.is_file()
            || !config_path.is_file()
            || !golden_path.is_file()
            || !expected_path.is_dir()
            || !hash_path.is_file()
        {
            continue;
        }

        let config: FixtureConfig = toml::from_str(&fs::read_to_string(config_path)?)?;
        discovered.push(DocxFixture {
            name: name.to_string(),
            root: path,
            title: config.title,
            author: config.author,
            strict_mode: config.strict_mode,
            determinism: config.determinism,
        });
    }

    Ok(discovered)
}

pub fn read_fixture_input(fixture: &DocxFixture) -> FixtureResult<String> {
    archive_fixture::read_fixture_input(&fixture.root)
}

pub fn normalized_entries(bytes: &[u8]) -> FixtureResult<BTreeMap<String, NormalizedEntry>> {
    archive_fixture::normalized_entries(bytes, is_text_entry)
}

pub fn read_expected_entries(root: &Path) -> FixtureResult<BTreeMap<String, NormalizedEntry>> {
    archive_fixture::read_expected_entries(root, &[".DS_Store"], is_text_entry)
}

pub fn hash_entries(entries: &BTreeMap<String, NormalizedEntry>) -> String {
    archive_fixture::hash_entries(entries)
}

fn is_text_entry(path: &str) -> bool {
    path.ends_with(".xml")
        || path.ends_with(".rels")
        || path.ends_with(".txt")
        || path.ends_with(".md")
}
