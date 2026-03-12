use std::collections::BTreeSet;
use std::io::Read;

use zip::ZipArchive;

use crate::error::CoreRsError;

pub(super) struct LoadedHwpx {
    pub(super) content_hpf: String,
    pub(super) header_xml: String,
    pub(super) section_xml: String,
}

const REQUIRED_ENTRY_PATHS: &[&str] = &[
    "Contents/content.hpf",
    "Contents/header.xml",
    "Contents/section0.xml",
];
const MAX_ARCHIVE_ENTRIES: usize = 64;
const MAX_ENTRY_BYTES: usize = 8 * 1024 * 1024;
const MAX_TOTAL_ARCHIVE_BYTES: usize = 32 * 1024 * 1024;

pub(super) fn load(bytes: &[u8]) -> Result<LoadedHwpx, CoreRsError> {
    let reader = std::io::Cursor::new(bytes);
    let mut archive = ZipArchive::new(reader).map_err(|error| {
        CoreRsError::InvalidHwpx(format!("failed to open HWPX archive: {error}"))
    })?;
    if archive.len() > MAX_ARCHIVE_ENTRIES {
        return Err(CoreRsError::InvalidHwpx(format!(
            "HWPX archive contains too many entries: {}",
            archive.len()
        )));
    }

    let mut seen_paths = BTreeSet::new();
    let mut extra_sections = Vec::new();
    let mut total_bytes = 0usize;
    let mut content_hpf = None;
    let mut header_xml = None;
    let mut section_xml = None;
    for index in 0..archive.len() {
        let file = archive.by_index(index).map_err(|error| {
            CoreRsError::InvalidHwpx(format!(
                "failed to read HWPX archive entry {index}: {error}"
            ))
        })?;
        let path = file.name().to_string();
        if !seen_paths.insert(path.clone()) {
            return Err(CoreRsError::InvalidHwpx(format!(
                "HWPX archive contains duplicate entry: {}",
                path
            )));
        }
        if path.starts_with("Contents/section") && path != "Contents/section0.xml" {
            extra_sections.push(path);
            continue;
        }
        if !REQUIRED_ENTRY_PATHS.contains(&path.as_str()) {
            continue;
        }

        let mut contents = Vec::new();
        file.take(MAX_ENTRY_BYTES as u64 + 1)
            .read_to_end(&mut contents)?;
        if contents.len() > MAX_ENTRY_BYTES {
            return Err(CoreRsError::InvalidHwpx(format!(
                "HWPX archive entry {} exceeds maximum supported size of {MAX_ENTRY_BYTES} bytes",
                path
            )));
        }
        total_bytes = total_bytes
            .checked_add(contents.len())
            .ok_or_else(|| CoreRsError::InvalidHwpx("HWPX archive size overflow".to_string()))?;
        if total_bytes > MAX_TOTAL_ARCHIVE_BYTES {
            return Err(CoreRsError::InvalidHwpx(format!(
                "required HWPX contents exceed maximum supported size of {MAX_TOTAL_ARCHIVE_BYTES} bytes"
            )));
        }

        let text = String::from_utf8(contents).map_err(|error| {
            CoreRsError::InvalidHwpx(format!("{path} is not valid UTF-8 text: {error}"))
        })?;
        match path.as_str() {
            "Contents/content.hpf" => content_hpf = Some(text),
            "Contents/header.xml" => header_xml = Some(text),
            "Contents/section0.xml" => section_xml = Some(text),
            _ => {}
        }
    }

    if !extra_sections.is_empty() {
        return Err(CoreRsError::UnsupportedFeature(format!(
            "HWPX parser does not support multiple sections: {}",
            extra_sections.join(", ")
        )));
    }

    Ok(LoadedHwpx {
        content_hpf: content_hpf.ok_or_else(|| {
            CoreRsError::InvalidHwpx(
                "missing required HWPX entry: Contents/content.hpf".to_string(),
            )
        })?,
        header_xml: header_xml.ok_or_else(|| {
            CoreRsError::InvalidHwpx("missing required HWPX entry: Contents/header.xml".to_string())
        })?,
        section_xml: section_xml.ok_or_else(|| {
            CoreRsError::InvalidHwpx(
                "missing required HWPX entry: Contents/section0.xml".to_string(),
            )
        })?,
    })
}
