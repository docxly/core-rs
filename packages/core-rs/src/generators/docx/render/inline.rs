use crate::error::CoreRsError;
use crate::models::inline::{ImageData, Inline};
use crate::utils::image_helper::image_dimensions;
use crate::utils::xml_helper::escape_attr;

use super::super::xml_builder;
use super::{ImageAsset, RenderContext, RunStyle};

pub(super) fn run_properties(style: RunStyle) -> String {
    let mut parts = Vec::new();
    if style.bold {
        parts.push("<w:b/>".to_string());
    }
    if style.italic {
        parts.push("<w:i/>".to_string());
    }
    if style.code {
        parts.push("<w:rFonts w:ascii=\"Courier New\" w:hAnsi=\"Courier New\"/>".to_string());
    }
    if style.hyperlink {
        parts.push("<w:rStyle w:val=\"Hyperlink\"/>".to_string());
    }

    if parts.is_empty() {
        return String::new();
    }

    format!("<w:rPr>{}</w:rPr>", parts.join(""))
}

pub(super) fn preserve_space(text: &str) -> bool {
    text.starts_with(' ') || text.ends_with(' ')
}

pub(super) fn render_inlines(
    inlines: &[Inline],
    context: &mut RenderContext,
    inherited: RunStyle,
) -> Result<String, CoreRsError> {
    let mut xml = String::new();
    for inline in inlines {
        match inline {
            Inline::Text(text) => xml.push_str(&xml_builder::text_run(text, inherited)),
            Inline::Code(code) => xml.push_str(&xml_builder::text_run(
                code,
                RunStyle {
                    code: true,
                    ..inherited
                },
            )),
            Inline::Emphasis(content) => xml.push_str(&render_inlines(
                content,
                context,
                RunStyle {
                    italic: true,
                    ..inherited
                },
            )?),
            Inline::Strong(content) => xml.push_str(&render_inlines(
                content,
                context,
                RunStyle {
                    bold: true,
                    ..inherited
                },
            )?),
            Inline::Link { text, url } => {
                let relationship_id = format!("rLink{}", context.hyperlinks.len() + 1);
                context.hyperlinks.push(url.clone());
                let content_xml = render_inlines(
                    text,
                    context,
                    RunStyle {
                        hyperlink: true,
                        ..inherited
                    },
                )?;
                xml.push_str(&format!(
                    "<w:hyperlink r:id=\"{}\" w:history=\"1\">{}</w:hyperlink>",
                    escape_attr(&relationship_id),
                    content_xml
                ));
            }
            Inline::Image(image) => xml.push_str(&render_image(image, context)?),
            Inline::HardBreak => xml.push_str("<w:r><w:br/></w:r>"),
        }
    }
    Ok(xml)
}

fn render_image(image: &ImageData, context: &mut RenderContext) -> Result<String, CoreRsError> {
    let image_index = context.images.len() + 1;
    let relationship_id = format!("rImage{}", context.images.len() + 1);
    let target = format!("word/media/image{}.{}", image_index, image.extension);
    let (width, height) = image_dimensions(image)?;
    context.images.push(ImageAsset {
        relationship_id: relationship_id.clone(),
        target: target.clone(),
        extension: image.extension.clone(),
        mime_type: image.mime_type.clone(),
        data: image.data.clone(),
    });

    Ok(xml_builder::image_run(
        &relationship_id,
        &image.alt,
        image_index,
        image_index,
        width as i64 * 9_525,
        height as i64 * 9_525,
    ))
}
