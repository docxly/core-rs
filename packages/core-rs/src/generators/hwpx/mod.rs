mod content_hpf;
mod section_xml;

use crate::HwpxOptions;
use crate::error::CoreRsError;
use crate::generators::Generator;
use crate::models::document::Document;

pub struct HwpxGenerator {
    options: HwpxOptions,
}

impl HwpxGenerator {
    pub(crate) fn new(options: HwpxOptions) -> Self {
        Self { options }
    }
}

impl Generator for HwpxGenerator {
    fn generate(&self, _document: &Document) -> Result<Vec<u8>, CoreRsError> {
        let _ = (
            &self.options,
            content_hpf::CONTENT_HPF_PLACEHOLDER,
            section_xml::SECTION_XML_PLACEHOLDER,
        );
        Err(CoreRsError::UnsupportedFeature(
            "hwpx generation is planned but not implemented yet".to_string(),
        ))
    }
}
