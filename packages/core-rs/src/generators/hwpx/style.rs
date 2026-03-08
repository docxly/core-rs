use crate::{CoreRsError, HwpxOptions, HwpxParagraphAlign};

const DEFAULT_BODY_FONT: &str = "함초롬바탕";
const DEFAULT_HEADING_FONT: &str = "함초롬돋움";
const DEFAULT_TEXT_COLOR: &str = "#000000";
const DEFAULT_H1_COLOR: &str = "#2E74B5";
const DEFAULT_HEADING_COLOR: &str = "#1F1F1F";
const DEFAULT_LINK_COLOR: &str = "#0563C1";
const DEFAULT_BODY_FONT_SIZE: u32 = 1000;
const DEFAULT_H1_FONT_SIZE: u32 = 1400;
const MIN_FONT_SIZE: u32 = 600;
const MAX_FONT_SIZE: u32 = 4000;

#[derive(Debug, Clone)]
pub(crate) struct ResolvedHwpxStyle {
    pub body_font: String,
    pub heading_font: String,
    pub body_font_size: u32,
    pub heading_sizes: [u32; 4],
    pub text_color: String,
    pub heading_colors: [String; 4],
    pub link_color: String,
    pub paragraph_align: &'static str,
    pub custom_body_font: bool,
    pub custom_heading_font: bool,
    pub custom_body_font_size: bool,
    pub custom_heading_font_size: bool,
    pub custom_text_color: bool,
    pub custom_heading_color: bool,
    pub custom_link_color: bool,
    pub custom_paragraph_align: bool,
}

impl ResolvedHwpxStyle {
    pub(crate) fn from_options(options: &HwpxOptions) -> Result<Self, CoreRsError> {
        let custom_body_font = options.style.body_font.is_some();
        let custom_heading_font = options.style.heading_font.is_some();
        let custom_body_font_size = options.style.body_font_size.is_some();
        let custom_heading_font_size = options.style.heading_font_size.is_some();
        let custom_text_color = options.style.text_color.is_some();
        let custom_heading_color = options.style.heading_color.is_some();
        let custom_link_color = options.style.link_color.is_some();
        let custom_paragraph_align = options.style.paragraph_align.is_some();
        let body_font = options
            .style
            .body_font
            .as_deref()
            .map(validate_font_name)
            .transpose()?
            .unwrap_or_else(|| DEFAULT_BODY_FONT.to_string());
        let heading_font = options
            .style
            .heading_font
            .as_deref()
            .map(validate_font_name)
            .transpose()?
            .unwrap_or_else(|| DEFAULT_HEADING_FONT.to_string());
        let body_font_size = options
            .style
            .body_font_size
            .map(validate_font_size)
            .transpose()?
            .unwrap_or(DEFAULT_BODY_FONT_SIZE);
        let h1_font_size = options
            .style
            .heading_font_size
            .map(validate_font_size)
            .transpose()?
            .unwrap_or(DEFAULT_H1_FONT_SIZE);
        let heading_color_override = options
            .style
            .heading_color
            .as_deref()
            .map(validate_and_normalize_color)
            .transpose()?;

        Ok(Self {
            body_font,
            heading_font,
            body_font_size,
            heading_sizes: heading_sizes(h1_font_size, body_font_size),
            text_color: options
                .style
                .text_color
                .as_deref()
                .map(validate_and_normalize_color)
                .transpose()?
                .unwrap_or_else(|| DEFAULT_TEXT_COLOR.to_string()),
            heading_colors: [
                heading_color_override
                    .clone()
                    .unwrap_or_else(|| DEFAULT_H1_COLOR.to_string()),
                heading_color_override
                    .clone()
                    .unwrap_or_else(|| DEFAULT_HEADING_COLOR.to_string()),
                heading_color_override
                    .clone()
                    .unwrap_or_else(|| DEFAULT_HEADING_COLOR.to_string()),
                heading_color_override.unwrap_or_else(|| DEFAULT_HEADING_COLOR.to_string()),
            ],
            link_color: options
                .style
                .link_color
                .as_deref()
                .map(validate_and_normalize_color)
                .transpose()?
                .unwrap_or_else(|| DEFAULT_LINK_COLOR.to_string()),
            paragraph_align: match options.style.paragraph_align.unwrap_or(HwpxParagraphAlign::Justify)
            {
                HwpxParagraphAlign::Left => "LEFT",
                HwpxParagraphAlign::Center => "CENTER",
                HwpxParagraphAlign::Right => "RIGHT",
                HwpxParagraphAlign::Justify => "JUSTIFY",
            },
            custom_body_font,
            custom_heading_font,
            custom_body_font_size,
            custom_heading_font_size,
            custom_text_color,
            custom_heading_color,
            custom_link_color,
            custom_paragraph_align,
        })
    }

    pub(crate) fn is_default(&self) -> bool {
        !self.custom_body_font
            && !self.custom_heading_font
            && !self.custom_body_font_size
            && !self.custom_heading_font_size
            && !self.custom_text_color
            && !self.custom_heading_color
            && !self.custom_link_color
            && !self.custom_paragraph_align
    }
}

fn validate_font_name(value: &str) -> Result<String, CoreRsError> {
    let trimmed = value.trim();
    let is_valid = !trimmed.is_empty() && !trimmed.chars().any(char::is_control);

    if is_valid {
        Ok(trimmed.to_string())
    } else {
        Err(CoreRsError::InvalidOption(format!(
            "HWPX font names must be non-empty plain text, got `{value}`"
        )))
    }
}

fn validate_font_size(value: u32) -> Result<u32, CoreRsError> {
    if (MIN_FONT_SIZE..=MAX_FONT_SIZE).contains(&value) {
        Ok(value)
    } else {
        Err(CoreRsError::InvalidOption(format!(
            "HWPX font sizes must be between {MIN_FONT_SIZE} and {MAX_FONT_SIZE}, got `{value}`"
        )))
    }
}

fn validate_and_normalize_color(value: &str) -> Result<String, CoreRsError> {
    let trimmed = value.trim();
    let normalized = if trimmed.starts_with('#') {
        trimmed.to_uppercase()
    } else {
        format!("#{}", trimmed.to_uppercase())
    };

    let is_valid = normalized.len() == 7
        && normalized.starts_with('#')
        && normalized[1..].chars().all(|ch| ch.is_ascii_hexdigit());

    if is_valid {
        Ok(normalized)
    } else {
        Err(CoreRsError::InvalidOption(format!(
            "HWPX color values must be 6-digit hex, got `{trimmed}`"
        )))
    }
}

fn heading_sizes(h1_size: u32, body_size: u32) -> [u32; 4] {
    let h2 = h1_size.saturating_sub(100).max(body_size);
    let h3 = h1_size.saturating_sub(200).max(body_size);
    let h4 = h1_size.saturating_sub(300).max(body_size);
    [h1_size, h2, h3, h4]
}
