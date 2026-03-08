#![allow(dead_code)]

use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use zip::ZipArchive;

pub(crate) type FixtureResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NormalizedEntry {
    Text(String),
    BinaryHash(String),
}

pub fn discover_fixture_dirs(root: &Path) -> FixtureResult<Vec<PathBuf>> {
    let mut fixtures = fs::read_dir(root)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()?;
    fixtures.sort();
    Ok(fixtures)
}

pub fn read_fixture_input(root: &Path) -> FixtureResult<String> {
    Ok(fs::read_to_string(root.join("input.md"))?)
}

pub fn normalized_entries(
    bytes: &[u8],
    is_text_entry: fn(&str) -> bool,
) -> FixtureResult<BTreeMap<String, NormalizedEntry>> {
    let reader = std::io::Cursor::new(bytes.to_vec());
    let mut archive = ZipArchive::new(reader)?;
    let mut entries = BTreeMap::new();

    for index in 0..archive.len() {
        let mut file = archive.by_index(index)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        entries.insert(
            file.name().to_string(),
            normalize_entry(file.name(), &contents, is_text_entry),
        );
    }

    Ok(entries)
}

pub fn archive_entry_names(bytes: &[u8]) -> FixtureResult<Vec<String>> {
    let reader = std::io::Cursor::new(bytes.to_vec());
    let mut archive = ZipArchive::new(reader)?;
    let mut names = Vec::with_capacity(archive.len());

    for index in 0..archive.len() {
        let file = archive.by_index(index)?;
        names.push(file.name().to_string());
    }

    Ok(names)
}

pub fn read_expected_entries(
    root: &Path,
    ignored_names: &[&str],
    is_text_entry: fn(&str) -> bool,
) -> FixtureResult<BTreeMap<String, NormalizedEntry>> {
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
                .is_some_and(|name| ignored_names.contains(&name))
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
            entries.insert(relative.clone(), normalize_entry(&relative, &contents, is_text_entry));
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

fn normalize_entry(
    path: &str,
    contents: &[u8],
    is_text_entry: fn(&str) -> bool,
) -> NormalizedEntry {
    if is_text_entry(path) {
        let text = String::from_utf8(contents.to_vec()).expect("text fixture entry must be UTF-8");
        return NormalizedEntry::Text(normalize_content(&text));
    }

    NormalizedEntry::BinaryHash(hash_bytes(contents))
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
