use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use core_rs::DocxOptions;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use zip::ZipArchive;

type FixtureResult<T> = Result<T, Box<dyn Error>>;

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
    let mut fixtures = fs::read_dir(&root)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()?;
    fixtures.sort();

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
    Ok(fs::read_to_string(fixture.root.join("input.md"))?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NormalizedEntry {
    Text(String),
    BinaryHash(String),
}

pub fn normalized_entries(bytes: &[u8]) -> FixtureResult<BTreeMap<String, NormalizedEntry>> {
    let reader = std::io::Cursor::new(bytes.to_vec());
    let mut archive = ZipArchive::new(reader)?;
    let mut entries = BTreeMap::new();

    for index in 0..archive.len() {
        let mut file = archive.by_index(index)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        entries.insert(file.name().to_string(), normalize_entry(file.name(), &contents));
    }

    Ok(entries)
}

pub fn read_expected_entries(root: &Path) -> FixtureResult<BTreeMap<String, NormalizedEntry>> {
    let mut entries = BTreeMap::new();
    let mut stack = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let mut children = fs::read_dir(&dir)?
            .map(|entry| entry.map(|entry| entry.path()))
            .collect::<Result<Vec<_>, _>>()?;
        children.sort();

        for child in children {
            if child
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name == ".DS_Store")
            {
                continue;
            }
            if child.is_dir() {
                stack.push(child);
                continue;
            }

            let relative = child
                .strip_prefix(root)
                .unwrap_or(&child)
                .to_string_lossy()
                .replace('\\', "/");
            if relative.ends_with(".sha256") {
                let normalized_path = relative.trim_end_matches(".sha256").to_string();
                let contents = fs::read_to_string(&child)?;
                entries.insert(
                    normalized_path,
                    NormalizedEntry::BinaryHash(contents.trim().to_string()),
                );
                continue;
            }

            let contents = fs::read(&child)?;
            entries.insert(relative.clone(), normalize_entry(&relative, &contents));
        }
    }

    Ok(entries)
}

pub fn hash_entries(entries: &BTreeMap<String, NormalizedEntry>) -> String {
    let mut hasher = Sha256::new();
    for (path, contents) in entries {
        hasher.update(path.as_bytes());
        hasher.update([0]);
        match contents {
            NormalizedEntry::Text(text) => {
                hasher.update([0]);
                hasher.update(text.as_bytes());
            }
            NormalizedEntry::BinaryHash(hash) => {
                hasher.update([1]);
                hasher.update(hash.as_bytes());
            }
        }
        hasher.update([0]);
    }
    format!("{:x}", hasher.finalize())
}

fn normalize_entry(path: &str, contents: &[u8]) -> NormalizedEntry {
    if is_text_entry(path) {
        let text = String::from_utf8(contents.to_vec()).expect("text fixture entry must be UTF-8");
        return NormalizedEntry::Text(normalize_content(&text));
    }

    NormalizedEntry::BinaryHash(hash_bytes(contents))
}

fn is_text_entry(path: &str) -> bool {
    path.ends_with(".xml") || path.ends_with(".rels") || path.ends_with(".txt") || path.ends_with(".md")
}

fn normalize_content(contents: &str) -> String {
    contents
        .replace("\r\n", "\n")
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
