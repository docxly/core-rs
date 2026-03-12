#![allow(dead_code)]

mod dump;
mod metadata;
mod package;
mod registry;
mod section;

#[cfg(test)]
mod tests;

use crate::error::CoreRsError;
use crate::models::document::Document;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RawHwpxMetadata {
    pub(crate) title: Option<String>,
    pub(crate) language: Option<String>,
    pub(crate) creator: Option<String>,
    pub(crate) lastsaveby: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParsedHwpx {
    pub(crate) document: Document,
    pub(crate) metadata: RawHwpxMetadata,
}

pub(crate) fn parse_hwpx(bytes: &[u8]) -> Result<ParsedHwpx, CoreRsError> {
    let package = package::load(bytes)?;
    let metadata = metadata::parse(&package.content_hpf)?;
    let registry = registry::parse(&package.header_xml)?;
    let document = section::parse(&package.section_xml, &registry)?;

    Ok(ParsedHwpx { document, metadata })
}

pub(crate) fn debug_dump(bytes: &[u8]) -> Result<String, CoreRsError> {
    let parsed = parse_hwpx(bytes)?;
    Ok(dump::render(&parsed))
}
