mod content_hpf;
mod header_xml;
mod package_xml;
mod section_xml;
mod style;

use crate::HwpxOptions;
use crate::error::CoreRsError;
use crate::generators::Generator;
use crate::models::document::Document;
use crate::utils::zip_archiver::{ArchiveEntry, zip_entries_in_order};
use style::ResolvedHwpxStyle;

const PREVIEW_IMAGE_BYTES: &[u8] = include_bytes!("assets/preview.png");

pub struct HwpxGenerator {
    options: HwpxOptions,
}

impl HwpxGenerator {
    pub(crate) fn new(options: HwpxOptions) -> Self {
        Self { options }
    }
}

impl Generator for HwpxGenerator {
    fn generate(&self, document: &Document) -> Result<Vec<u8>, CoreRsError> {
        let style = ResolvedHwpxStyle::from_options(&self.options)?;
        let header = header_xml::build_header_xml(document, &style);
        let section = section_xml::build_section_xml(document, self.options.strict_mode, &style)?;
        let content_hpf = content_hpf::build_content_hpf(self.options.title.as_deref());
        let preview_text = section_xml::build_preview_text(document, self.options.strict_mode)?;

        let entries = vec![
            ArchiveEntry::new_text("mimetype", package_xml::mimetype()),
            ArchiveEntry::new_text("version.xml", package_xml::version_xml()),
            ArchiveEntry::new_text("Contents/header.xml", header),
            ArchiveEntry::new_text("Contents/section0.xml", section),
            ArchiveEntry::new_text("Preview/PrvText.txt", preview_text),
            ArchiveEntry::new_text("settings.xml", package_xml::settings_xml()),
            ArchiveEntry::new_bytes("Preview/PrvImage.png", PREVIEW_IMAGE_BYTES),
            ArchiveEntry::new_text("META-INF/container.rdf", package_xml::container_rdf_xml()),
            ArchiveEntry::new_text("Contents/content.hpf", content_hpf),
            ArchiveEntry::new_text("META-INF/container.xml", package_xml::container_xml()),
            ArchiveEntry::new_text("META-INF/manifest.xml", package_xml::manifest_xml()),
        ];

        zip_entries_in_order(&entries)
    }
}
