use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;

use crate::error::CoreRsError;

const MAX_DATA_URI_IMAGE_BYTES: usize = 8 * 1024 * 1024;

#[derive(Debug, Clone, Copy)]
pub(super) enum ImageFallbackKind {
    Generic,
    Oversized,
}

#[derive(Debug)]
pub(crate) enum DataImageError {
    NotDataUrl,
    InvalidDataUri,
    NonBase64,
    UnsupportedMime { mime_type: String },
    InvalidBase64,
    TooLarge { message: String },
}

impl DataImageError {
    pub(super) fn fallback_kind(&self) -> ImageFallbackKind {
        match self {
            Self::TooLarge { .. } => ImageFallbackKind::Oversized,
            _ => ImageFallbackKind::Generic,
        }
    }

    pub(crate) fn into_core_error(self) -> CoreRsError {
        match self {
            Self::NotDataUrl => CoreRsError::UnsupportedFeature("image".to_string()),
            Self::InvalidDataUri => {
                CoreRsError::InvalidMarkdown("invalid data URI image".to_string())
            }
            Self::NonBase64 => {
                CoreRsError::UnsupportedFeature("non-base64 data URI image".to_string())
            }
            Self::UnsupportedMime { mime_type } => {
                CoreRsError::UnsupportedFeature(format!("unsupported image mime type: {mime_type}"))
            }
            Self::InvalidBase64 => {
                CoreRsError::InvalidMarkdown("invalid base64 image data".to_string())
            }
            Self::TooLarge { message } => CoreRsError::UnsupportedFeature(message),
        }
    }
}

pub(super) fn image_fallback_text(alt_text: &str, url: &str, kind: ImageFallbackKind) -> String {
    if alt_text.is_empty() {
        if url.starts_with("data:") {
            match kind {
                ImageFallbackKind::Oversized => "[image omitted]".to_string(),
                ImageFallbackKind::Generic => "[data image]".to_string(),
            }
        } else {
            url.to_string()
        }
    } else {
        alt_text.to_string()
    }
}

pub(crate) fn parse_data_uri(url: &str) -> Result<(String, String, Vec<u8>), DataImageError> {
    let Some(rest) = url.strip_prefix("data:") else {
        return Err(DataImageError::NotDataUrl);
    };
    let Some((metadata, payload)) = rest.split_once(',') else {
        return Err(DataImageError::InvalidDataUri);
    };

    let mut parts = metadata.split(';');
    let mime_type = parts.next().unwrap_or_default().to_ascii_lowercase();
    if !parts.any(|part| part.eq_ignore_ascii_case("base64")) {
        return Err(DataImageError::NonBase64);
    }

    let extension = match mime_type.as_str() {
        "image/png" => "png",
        "image/jpeg" | "image/jpg" => "jpg",
        "image/gif" => "gif",
        _ => return Err(DataImageError::UnsupportedMime { mime_type }),
    };

    let estimated_len = payload.len().div_ceil(4).saturating_mul(3);
    if estimated_len > MAX_DATA_URI_IMAGE_BYTES {
        return Err(DataImageError::TooLarge {
            message: format!(
                "image exceeds maximum supported size of {MAX_DATA_URI_IMAGE_BYTES} bytes"
            ),
        });
    }

    let data = BASE64_STANDARD
        .decode(payload)
        .map_err(|_| DataImageError::InvalidBase64)?;
    if data.len() > MAX_DATA_URI_IMAGE_BYTES {
        return Err(DataImageError::TooLarge {
            message: format!(
                "image exceeds maximum supported size of {MAX_DATA_URI_IMAGE_BYTES} bytes"
            ),
        });
    }

    Ok((mime_type, extension.to_string(), data))
}
