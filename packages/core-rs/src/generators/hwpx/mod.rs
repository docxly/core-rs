mod content_hpf;
mod document_shape;
mod header_xml;
mod package_xml;
mod profile;
mod section_xml;
mod style;

use crate::HwpxOptions;
use crate::error::CoreRsError;
use crate::generators::Generator;
use crate::models::document::Document;
use crate::utils::zip_archiver::{ArchiveEntry, zip_entries_in_order};
use profile::resolve_compatibility_profile;
use style::ResolvedHwpxStyle;
use zip::CompressionMethod;

const PREVIEW_IMAGE_BYTES: &[u8] = include_bytes!("assets/preview.png");
const CORE_PARAGRAPH_PREVIEW_IMAGE_BYTES: &[u8] =
    include_bytes!("assets/core-paragraph-preview.png");

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
        let profile = resolve_compatibility_profile(document, &style);
        let header = header_xml::build_header_xml(document, &style, profile);
        let section =
            section_xml::build_section_xml(document, self.options.strict_mode, &style, profile)?;
        let content_hpf = content_hpf::build_content_hpf(self.options.title.as_deref(), profile);
        let preview_text =
            section_xml::build_preview_text(document, self.options.strict_mode, profile, &style)?;
        let preview_image_bytes = match profile {
            profile::ResolvedHwpxCompatibilityProfile::CoreParagraphFixture => {
                CORE_PARAGRAPH_PREVIEW_IMAGE_BYTES
            }
            _ => PREVIEW_IMAGE_BYTES,
        };

        let entries = vec![
            ArchiveEntry::new_text("mimetype", package_xml::mimetype()),
            ArchiveEntry::new_text_with_compression(
                "version.xml",
                package_xml::version_xml(profile),
                CompressionMethod::Deflated,
            ),
            ArchiveEntry::new_text_with_compression(
                "Contents/header.xml",
                header,
                CompressionMethod::Deflated,
            ),
            ArchiveEntry::new_text_with_compression(
                "Contents/section0.xml",
                section,
                CompressionMethod::Deflated,
            ),
            ArchiveEntry::new_text_with_compression(
                "Preview/PrvText.txt",
                preview_text,
                CompressionMethod::Deflated,
            ),
            ArchiveEntry::new_text_with_compression(
                "settings.xml",
                package_xml::settings_xml(),
                CompressionMethod::Deflated,
            ),
            ArchiveEntry::new_bytes("Preview/PrvImage.png", preview_image_bytes),
            ArchiveEntry::new_text_with_compression(
                "META-INF/container.rdf",
                package_xml::container_rdf_xml(),
                CompressionMethod::Deflated,
            ),
            ArchiveEntry::new_text_with_compression(
                "Contents/content.hpf",
                content_hpf,
                CompressionMethod::Deflated,
            ),
            ArchiveEntry::new_text_with_compression(
                "META-INF/container.xml",
                package_xml::container_xml(),
                CompressionMethod::Deflated,
            ),
            ArchiveEntry::new_text_with_compression(
                "META-INF/manifest.xml",
                package_xml::manifest_xml(),
                CompressionMethod::Deflated,
            ),
        ];

        zip_entries_in_order(&entries)
    }
}
