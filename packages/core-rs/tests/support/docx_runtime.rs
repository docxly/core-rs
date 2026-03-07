use core_rs::{DocxOptions, generate_docx};

use super::docx_fixture::{FixtureResult, NormalizedEntry, normalized_entries};

pub struct GeneratedDocx {
    entries: std::collections::BTreeMap<String, NormalizedEntry>,
}

fn generate_normalized(markdown: &str, options: DocxOptions) -> FixtureResult<std::collections::BTreeMap<String, NormalizedEntry>> {
    let bytes = generate_docx(markdown, options)?;
    normalized_entries(&bytes)
}

pub fn generate(markdown: &str, options: DocxOptions) -> FixtureResult<GeneratedDocx> {
    Ok(GeneratedDocx {
        entries: generate_normalized(markdown, options)?,
    })
}

impl GeneratedDocx {
    pub fn text_entry(&self, path: &str) -> FixtureResult<&str> {
        match self.entries.get(path) {
            Some(NormalizedEntry::Text(text)) => Ok(text.as_str()),
            Some(NormalizedEntry::BinaryHash(_)) => {
                Err(format!("{path} was normalized as binary").into())
            }
            None => Err(format!("{path} missing from generated archive").into()),
        }
    }

    pub fn document_xml(&self) -> FixtureResult<&str> {
        self.text_entry("word/document.xml")
    }

    pub fn relationships_xml(&self) -> FixtureResult<&str> {
        self.text_entry("word/_rels/document.xml.rels")
    }
}
