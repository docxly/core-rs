use crate::error::CoreRsError;
use crate::models::inline::ImageData;

pub(crate) fn image_dimensions(image: &ImageData) -> Result<(u32, u32), CoreRsError> {
    match image.mime_type.as_str() {
        "image/png" => png_dimensions(&image.data),
        "image/gif" => gif_dimensions(&image.data),
        "image/jpeg" | "image/jpg" => jpeg_dimensions(&image.data),
        _ => Err(CoreRsError::UnsupportedFeature(format!(
            "unsupported image mime type: {}",
            image.mime_type
        ))),
    }
}

fn png_dimensions(bytes: &[u8]) -> Result<(u32, u32), CoreRsError> {
    if bytes.len() < 24 || &bytes[..8] != b"\x89PNG\r\n\x1a\n" {
        return Err(CoreRsError::InvalidMarkdown(
            "invalid PNG image data".to_string(),
        ));
    }
    let width = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
    let height = u32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);
    Ok((width.max(1), height.max(1)))
}

fn gif_dimensions(bytes: &[u8]) -> Result<(u32, u32), CoreRsError> {
    if bytes.len() < 10 || (!bytes.starts_with(b"GIF87a") && !bytes.starts_with(b"GIF89a")) {
        return Err(CoreRsError::InvalidMarkdown(
            "invalid GIF image data".to_string(),
        ));
    }
    let width = u16::from_le_bytes([bytes[6], bytes[7]]) as u32;
    let height = u16::from_le_bytes([bytes[8], bytes[9]]) as u32;
    Ok((width.max(1), height.max(1)))
}

fn jpeg_dimensions(bytes: &[u8]) -> Result<(u32, u32), CoreRsError> {
    if bytes.len() < 4 || bytes[0] != 0xFF || bytes[1] != 0xD8 {
        return Err(CoreRsError::InvalidMarkdown(
            "invalid JPEG image data".to_string(),
        ));
    }

    let mut index = 2usize;
    while index + 8 < bytes.len() {
        while index < bytes.len() && bytes[index] != 0xFF {
            index += 1;
        }
        if index + 1 >= bytes.len() {
            break;
        }
        let marker = bytes[index + 1];
        index += 2;
        if marker == 0xD9 || marker == 0xDA {
            break;
        }
        if index + 2 > bytes.len() {
            break;
        }
        let length = u16::from_be_bytes([bytes[index], bytes[index + 1]]) as usize;
        if length < 2 || index + length > bytes.len() {
            break;
        }
        if matches!(
            marker,
            0xC0 | 0xC1
                | 0xC2
                | 0xC3
                | 0xC5
                | 0xC6
                | 0xC7
                | 0xC9
                | 0xCA
                | 0xCB
                | 0xCD
                | 0xCE
                | 0xCF
        ) && length >= 7
        {
            let height = u16::from_be_bytes([bytes[index + 3], bytes[index + 4]]) as u32;
            let width = u16::from_be_bytes([bytes[index + 5], bytes[index + 6]]) as u32;
            return Ok((width.max(1), height.max(1)));
        }
        index += length;
    }

    Err(CoreRsError::InvalidMarkdown(
        "could not determine JPEG image dimensions".to_string(),
    ))
}
