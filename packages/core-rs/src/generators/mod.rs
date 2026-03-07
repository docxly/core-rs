pub(crate) mod docx;
pub(crate) mod hwpx;

use crate::error::CoreRsError;
use crate::models::document::Document;

pub(crate) trait Generator {
    fn generate(&self, document: &Document) -> Result<Vec<u8>, CoreRsError>;
}
