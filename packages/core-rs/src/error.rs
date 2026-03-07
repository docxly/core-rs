use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreRsError {
    #[error("unsupported feature: {0}")]
    UnsupportedFeature(String),
    #[error("invalid markdown: {0}")]
    InvalidMarkdown(String),
    #[error("zip packaging failed: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("io failure: {0}")]
    Io(#[from] std::io::Error),
}
