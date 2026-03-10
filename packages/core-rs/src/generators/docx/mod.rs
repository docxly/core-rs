mod render;
mod templates;
mod xml_builder;

use crate::DocxOptions;
use crate::error::CoreRsError;
use crate::generators::Generator;
use crate::models::document::Document;
use crate::utils::zip_archiver::{ArchiveEntry, zip_entries};

use self::render::{RenderContext, max_heading_level, render_blocks};

pub struct DocxGenerator {
    options: DocxOptions,
}

impl DocxGenerator {
    pub(crate) fn new(options: DocxOptions) -> Self {
        Self { options }
    }
}

impl Generator for DocxGenerator {
    fn generate(&self, document: &Document) -> Result<Vec<u8>, CoreRsError> {
        let mut context = RenderContext::default();
        let body = render_blocks(&document.blocks, &mut context)?;
        let max_heading_level = max_heading_level(&document.blocks);

        let mut entries = vec![
            ArchiveEntry::new_text(
                "[Content_Types].xml",
                templates::content_types(&context.images),
            ),
            ArchiveEntry::new_text("_rels/.rels", templates::root_relationships()),
            ArchiveEntry::new_text(
                "docProps/core.xml",
                templates::core_properties(
                    self.options.title.as_deref().unwrap_or("Untitled"),
                    self.options.author.as_deref().unwrap_or("docxly"),
                ),
            ),
            ArchiveEntry::new_text("word/document.xml", xml_builder::wrap_document(&body)),
            ArchiveEntry::new_text(
                "word/_rels/document.xml.rels",
                templates::document_relationships(&context.hyperlinks, &context.images),
            ),
            ArchiveEntry::new_text("word/styles.xml", templates::styles(max_heading_level)),
        ];

        for image in &context.images {
            entries.push(ArchiveEntry::new_bytes(&image.target, image.data.clone()));
        }

        zip_entries(&entries)
    }
}
