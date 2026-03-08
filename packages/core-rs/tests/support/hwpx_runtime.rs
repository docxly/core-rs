#![allow(dead_code)]

use core_rs::{HwpxOptions, generate_hwpx};

use super::hwpx_fixture::{FixtureResult, NormalizedEntry, normalized_entries};

pub struct GeneratedHwpx {
    entries: std::collections::BTreeMap<String, NormalizedEntry>,
}

pub fn generate(markdown: &str, options: HwpxOptions) -> FixtureResult<GeneratedHwpx> {
    let bytes = generate_hwpx(markdown, options)?;
    Ok(GeneratedHwpx {
        entries: normalized_entries(&bytes)?,
    })
}

impl GeneratedHwpx {
    pub fn text_entry(&self, path: &str) -> FixtureResult<&str> {
        match self.entries.get(path) {
            Some(NormalizedEntry::Text(text)) => Ok(text.as_str()),
            Some(NormalizedEntry::BinaryHash(_)) => {
                Err(format!("{path} was normalized as binary").into())
            }
            None => Err(format!("{path} missing from generated archive").into()),
        }
    }

    pub fn section_xml(&self) -> FixtureResult<&str> {
        self.text_entry("Contents/section0.xml")
    }

    pub fn content_hpf(&self) -> FixtureResult<&str> {
        self.text_entry("Contents/content.hpf")
    }

}
