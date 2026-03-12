#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use core_rs::{HwpxOptions, HwpxParagraphAlign, HwpxStyleOptions};
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
    #[serde(default)]
    comparison_excludes: Vec<String>,
    #[serde(default)]
    manual_verified: bool,
    #[serde(default)]
    verified_at: Option<String>,
    #[serde(default)]
    style: FixtureStyleConfig,
}

#[derive(Debug, Deserialize, Default, Clone)]
struct FixtureStyleConfig {
    body_font: Option<String>,
    heading_font: Option<String>,
    body_font_size: Option<u32>,
    heading_font_size: Option<u32>,
    text_color: Option<String>,
    heading_color: Option<String>,
    link_color: Option<String>,
    paragraph_align: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HwpxFixture {
    pub name: String,
    pub root: PathBuf,
    pub title: String,
    pub author: String,
    pub strict_mode: bool,
    pub determinism: bool,
    pub comparison_excludes: Vec<String>,
    pub manual_verified: bool,
    pub verified_at: Option<String>,
    pub style: HwpxStyleOptions,
}

impl HwpxFixture {
    pub fn options(&self) -> HwpxOptions {
        HwpxOptions {
            title: Some(self.title.clone()),
            author: Some(self.author.clone()),
            strict_mode: self.strict_mode,
            style: self.style.clone(),
        }
    }
}

pub fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("hwpx")
        .join("approved")
}

pub fn discover_fixtures() -> FixtureResult<Vec<HwpxFixture>> {
    discover_fixtures_in(fixtures_root())
}

pub fn provisional_fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("hwpx")
        .join("provisional")
}

pub fn discover_provisional_fixtures() -> FixtureResult<Vec<HwpxFixture>> {
    discover_fixtures_in(provisional_fixtures_root())
}

fn discover_fixtures_in(root: PathBuf) -> FixtureResult<Vec<HwpxFixture>> {
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
        let golden_path = path.join("golden.hwpx");
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
        discovered.push(HwpxFixture {
            name: name.to_string(),
            root: path,
            title: config.title,
            author: config.author,
            strict_mode: config.strict_mode,
            determinism: config.determinism,
            comparison_excludes: config.comparison_excludes,
            manual_verified: config.manual_verified,
            verified_at: config.verified_at,
            style: HwpxStyleOptions {
                body_font: config.style.body_font,
                heading_font: config.style.heading_font,
                body_font_size: config.style.body_font_size,
                heading_font_size: config.style.heading_font_size,
                text_color: config.style.text_color,
                heading_color: config.style.heading_color,
                link_color: config.style.link_color,
                paragraph_align: config
                    .style
                    .paragraph_align
                    .as_deref()
                    .map(parse_paragraph_align)
                    .transpose()?,
            },
        });
    }

    Ok(discovered)
}

pub fn read_fixture_input(fixture: &HwpxFixture) -> FixtureResult<String> {
    archive_fixture::read_fixture_input(&fixture.root)
}

pub fn normalized_entries(bytes: &[u8]) -> FixtureResult<BTreeMap<String, NormalizedEntry>> {
    archive_fixture::normalized_entries(bytes, is_text_entry)
}

pub fn archive_entry_names(bytes: &[u8]) -> FixtureResult<Vec<String>> {
    archive_fixture::archive_entry_names(bytes)
}

pub fn read_expected_entries(root: &Path) -> FixtureResult<BTreeMap<String, NormalizedEntry>> {
    archive_fixture::read_expected_entries(
        root,
        &[".DS_Store", "README.md", "provenance.md"],
        is_text_entry,
    )
}

pub fn hash_entries(entries: &BTreeMap<String, NormalizedEntry>) -> String {
    archive_fixture::hash_entries(entries)
}

fn is_text_entry(path: &str) -> bool {
    path == "mimetype"
        || path.ends_with(".xml")
        || path.ends_with(".rdf")
        || path.ends_with(".rels")
        || path.ends_with(".txt")
        || path.ends_with(".hpf")
}

fn parse_paragraph_align(value: &str) -> FixtureResult<HwpxParagraphAlign> {
    match value.to_ascii_lowercase().as_str() {
        "left" => Ok(HwpxParagraphAlign::Left),
        "center" => Ok(HwpxParagraphAlign::Center),
        "right" => Ok(HwpxParagraphAlign::Right),
        "justify" => Ok(HwpxParagraphAlign::Justify),
        other => Err(format!("unsupported HWPX paragraph alignment: {other}").into()),
    }
}
