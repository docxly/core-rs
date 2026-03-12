#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use zip::ZipArchive;

pub type FixtureResult<T> = Result<T, Box<dyn Error>>;

const NONDETERMINISTIC_ENTRY_EXCLUDES: &[&str] = &[
    "Contents/content.hpf",
    "Preview/PrvImage.png",
    "version.xml",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NormalizedEntry {
    Text(String),
    BinaryHash(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchiveEntry {
    Text(String),
    Binary { bytes: Vec<u8>, hash: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryRepresentation {
    Raw,
    Sha256Sidecar,
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

pub fn archive_entries(
    bytes: &[u8],
    is_text_entry: fn(&str) -> bool,
) -> FixtureResult<BTreeMap<String, ArchiveEntry>> {
    let reader = std::io::Cursor::new(bytes.to_vec());
    let mut archive = ZipArchive::new(reader)?;
    let mut entries = BTreeMap::new();

    for index in 0..archive.len() {
        let mut file = archive.by_index(index)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        entries.insert(
            file.name().to_string(),
            archive_entry(file.name(), &contents, is_text_entry),
        );
    }

    Ok(entries)
}

pub fn normalized_entries(
    bytes: &[u8],
    is_text_entry: fn(&str) -> bool,
) -> FixtureResult<BTreeMap<String, NormalizedEntry>> {
    Ok(normalized_entries_from_archive(&archive_entries(
        bytes,
        is_text_entry,
    )?))
}

pub fn filtered_normalized_entries(
    bytes: &[u8],
    is_text_entry: fn(&str) -> bool,
    excluded_paths: &[impl AsRef<str>],
) -> FixtureResult<BTreeMap<String, NormalizedEntry>> {
    let mut entries = normalized_entries(bytes, is_text_entry)?;
    for path in excluded_paths {
        entries.remove(path.as_ref());
    }
    Ok(entries)
}

pub fn normalized_archives_equivalent(
    left: &[u8],
    right: &[u8],
    is_text_entry: fn(&str) -> bool,
) -> FixtureResult<bool> {
    Ok(normalized_entries(left, is_text_entry)? == normalized_entries(right, is_text_entry)?)
}

pub fn fixture_is_deterministic(root: &Path) -> FixtureResult<bool> {
    Ok(read_fixture_comparison_config(root)?.determinism)
}

pub fn comparison_excludes_for_fixture(root: &Path) -> FixtureResult<Vec<String>> {
    let config = read_fixture_comparison_config(root)?;
    let mut excludes = Vec::new();

    if !config.determinism {
        excludes.extend(
            NONDETERMINISTIC_ENTRY_EXCLUDES
                .iter()
                .map(|path| (*path).to_string()),
        );
    }

    for path in config.comparison_excludes {
        if !excludes.contains(&path) {
            excludes.push(path);
        }
    }

    Ok(excludes)
}

struct FixtureComparisonConfig {
    determinism: bool,
    comparison_excludes: Vec<String>,
}

fn read_fixture_comparison_config(root: &Path) -> FixtureResult<FixtureComparisonConfig> {
    let contents = fs::read_to_string(root.join("fixture.toml"))?;
    Ok(FixtureComparisonConfig {
        determinism: !contents
            .lines()
            .any(|line| line.trim() == "determinism = false"),
        comparison_excludes: parse_comparison_excludes(&contents)?,
    })
}

fn parse_comparison_excludes(contents: &str) -> FixtureResult<Vec<String>> {
    let Some(line) = contents
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with("comparison_excludes = ["))
    else {
        return Ok(Vec::new());
    };

    let values = line
        .strip_prefix("comparison_excludes = [")
        .and_then(|line| line.strip_suffix(']'))
        .ok_or("invalid comparison_excludes syntax")?;

    if values.trim().is_empty() {
        return Ok(Vec::new());
    }

    Ok(values
        .split(',')
        .map(str::trim)
        .map(|value| value.trim_matches('"').to_string())
        .collect())
}

pub fn normalized_entries_from_archive(
    entries: &BTreeMap<String, ArchiveEntry>,
) -> BTreeMap<String, NormalizedEntry> {
    entries
        .iter()
        .map(|(path, entry)| {
            (
                path.clone(),
                match entry {
                    ArchiveEntry::Text(text) => NormalizedEntry::Text(text.clone()),
                    ArchiveEntry::Binary { hash, .. } => NormalizedEntry::BinaryHash(hash.clone()),
                },
            )
        })
        .collect()
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
            if file_name_is_ignored(&child, ignored_names) {
                continue;
            }
            if child.is_dir() {
                stack.push(child);
                continue;
            }

            let relative = relative_path(root, &child);
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
            entries.insert(
                relative.clone(),
                normalized_entry(&relative, &contents, is_text_entry),
            );
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

pub fn detect_binary_representations(
    root: &Path,
    ignored_names: &[&str],
    is_text_entry: fn(&str) -> bool,
) -> FixtureResult<BTreeMap<String, BinaryRepresentation>> {
    if !root.exists() {
        return Ok(BTreeMap::new());
    }

    let mut representations = BTreeMap::new();
    let mut stack = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let mut children = fs::read_dir(&dir)?
            .map(|entry| entry.map(|entry| entry.path()))
            .collect::<Result<Vec<_>, _>>()?;
        children.sort();

        for child in children {
            if file_name_is_ignored(&child, ignored_names) {
                continue;
            }
            if child.is_dir() {
                stack.push(child);
                continue;
            }

            let relative = relative_path(root, &child);
            if relative.ends_with(".sha256") {
                representations.insert(
                    relative.trim_end_matches(".sha256").to_string(),
                    BinaryRepresentation::Sha256Sidecar,
                );
                continue;
            }

            if !is_text_entry(&relative) {
                representations.insert(relative, BinaryRepresentation::Raw);
            }
        }
    }

    Ok(representations)
}

pub fn refresh_fixture_metadata(
    fixture_root: &Path,
    golden_file_name: &str,
    expected_dir_name: &str,
    hash_file_name: &str,
    ignored_names: &[&str],
    preserved_names: &[&str],
    is_text_entry: fn(&str) -> bool,
) -> FixtureResult<String> {
    let golden_bytes = fs::read(fixture_root.join(golden_file_name))?;
    let entries = archive_entries(&golden_bytes, is_text_entry)?;
    let normalized = normalized_entries_from_archive(&entries);
    let expected_root = fixture_root.join(expected_dir_name);
    let representations =
        detect_binary_representations(&expected_root, ignored_names, is_text_entry)?;

    write_expected_entries(
        &expected_root,
        &entries,
        &representations,
        preserved_names,
        is_text_entry,
    )?;

    let hash = hash_entries(&normalized);
    fs::write(fixture_root.join(hash_file_name), &hash)?;
    Ok(hash)
}

fn write_expected_entries(
    root: &Path,
    entries: &BTreeMap<String, ArchiveEntry>,
    representations: &BTreeMap<String, BinaryRepresentation>,
    preserved_names: &[&str],
    is_text_entry: fn(&str) -> bool,
) -> FixtureResult<()> {
    fs::create_dir_all(root)?;
    let desired_paths = desired_output_paths(entries, representations);

    remove_stale_outputs(root, &desired_paths, preserved_names)?;
    for (relative, entry) in entries {
        write_expected_entry(root, relative, entry, representations, is_text_entry)?;
    }
    prune_empty_dirs(root)?;

    Ok(())
}

fn desired_output_paths(
    entries: &BTreeMap<String, ArchiveEntry>,
    representations: &BTreeMap<String, BinaryRepresentation>,
) -> BTreeSet<String> {
    entries
        .iter()
        .map(|(relative, entry)| match entry {
            ArchiveEntry::Text(_) => relative.clone(),
            ArchiveEntry::Binary { .. } => match representations.get(relative) {
                Some(BinaryRepresentation::Raw) => relative.clone(),
                _ => format!("{relative}.sha256"),
            },
        })
        .collect()
}

fn remove_stale_outputs(
    root: &Path,
    desired_paths: &BTreeSet<String>,
    preserved_names: &[&str],
) -> FixtureResult<()> {
    if !root.exists() {
        return Ok(());
    }

    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let mut children = fs::read_dir(&dir)?
            .map(|entry| entry.map(|entry| entry.path()))
            .collect::<Result<Vec<_>, _>>()?;
        children.sort();

        for child in children {
            if file_name_is_ignored(&child, preserved_names) {
                continue;
            }
            if child.is_dir() {
                stack.push(child);
                continue;
            }

            let relative = relative_path(root, &child);
            if !desired_paths.contains(&relative) {
                fs::remove_file(child)?;
            }
        }
    }

    Ok(())
}

fn write_expected_entry(
    root: &Path,
    relative: &str,
    entry: &ArchiveEntry,
    representations: &BTreeMap<String, BinaryRepresentation>,
    is_text_entry: fn(&str) -> bool,
) -> FixtureResult<()> {
    match entry {
        ArchiveEntry::Text(text) => write_file(root, relative, text.as_bytes()),
        ArchiveEntry::Binary { bytes, hash } => match representations.get(relative) {
            Some(BinaryRepresentation::Raw) => write_file(root, relative, bytes),
            _ => {
                debug_assert!(!is_text_entry(relative));
                write_file(root, &format!("{relative}.sha256"), hash.as_bytes())
            }
        },
    }
}

fn write_file(root: &Path, relative: &str, contents: &[u8]) -> FixtureResult<()> {
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)?;
    Ok(())
}

fn prune_empty_dirs(root: &Path) -> FixtureResult<bool> {
    if !root.is_dir() {
        return Ok(false);
    }

    let mut empty = true;
    let children = fs::read_dir(root)?.collect::<Result<Vec<_>, _>>()?;
    for child in children {
        let path = child.path();
        if path.is_dir() {
            if prune_empty_dirs(&path)? {
                fs::remove_dir(&path)?;
            } else {
                empty = false;
            }
            continue;
        }
        empty = false;
    }

    Ok(empty)
}

fn file_name_is_ignored(path: &Path, ignored_names: &[&str]) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| ignored_names.contains(&name))
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn archive_entry(path: &str, contents: &[u8], is_text_entry: fn(&str) -> bool) -> ArchiveEntry {
    if is_text_entry(path) {
        let text = String::from_utf8(contents.to_vec()).expect("text fixture entry must be UTF-8");
        return ArchiveEntry::Text(normalize_content(&text));
    }

    ArchiveEntry::Binary {
        hash: hash_bytes(contents),
        bytes: contents.to_vec(),
    }
}

fn normalized_entry(
    path: &str,
    contents: &[u8],
    is_text_entry: fn(&str) -> bool,
) -> NormalizedEntry {
    match archive_entry(path, contents, is_text_entry) {
        ArchiveEntry::Text(text) => NormalizedEntry::Text(text),
        ArchiveEntry::Binary { hash, .. } => NormalizedEntry::BinaryHash(hash),
    }
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
