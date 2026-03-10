use std::io::{Cursor, Write};

use zip::CompressionMethod;
use zip::DateTime;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

use crate::error::CoreRsError;

#[derive(Debug, Clone)]
pub struct ArchiveEntry {
    pub path: String,
    pub contents: Vec<u8>,
    pub compression: CompressionMethod,
}

impl ArchiveEntry {
    pub fn new_text(path: impl Into<String>, contents: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            contents: contents.into().into_bytes(),
            compression: CompressionMethod::Stored,
        }
    }

    pub fn new_bytes(path: impl Into<String>, contents: impl Into<Vec<u8>>) -> Self {
        Self {
            path: path.into(),
            contents: contents.into(),
            compression: CompressionMethod::Stored,
        }
    }

    pub fn new_text_with_compression(
        path: impl Into<String>,
        contents: impl Into<String>,
        compression: CompressionMethod,
    ) -> Self {
        Self {
            path: path.into(),
            contents: contents.into().into_bytes(),
            compression,
        }
    }
}

pub fn zip_entries(entries: &[ArchiveEntry]) -> Result<Vec<u8>, CoreRsError> {
    let mut sorted_entries = entries.to_vec();
    sorted_entries.sort_by(|left, right| left.path.cmp(&right.path));
    zip_entries_in_order(&sorted_entries)
}

pub fn zip_entries_in_order(entries: &[ArchiveEntry]) -> Result<Vec<u8>, CoreRsError> {
    let cursor = Cursor::new(Vec::new());
    let mut writer = ZipWriter::new(cursor);

    for entry in entries {
        let options = SimpleFileOptions::default()
            .compression_method(entry.compression)
            .last_modified_time(DateTime::default());
        writer.start_file(&entry.path, options)?;
        writer.write_all(&entry.contents)?;
    }

    let cursor = writer.finish()?;
    Ok(cursor.into_inner())
}
