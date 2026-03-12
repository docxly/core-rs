use super::{BODY_FONT_FACE, HEADING_FONT_FACE, ResolvedHwpxStyle};
use crate::utils::xml_helper::escape_attr;

pub(super) fn apply_font_faces(header: &str, style: &ResolvedHwpxStyle) -> String {
    header
        .replace(
            BODY_FONT_FACE,
            &format!("face=\"{}\"", escape_attr(&style.body_font)),
        )
        .replace(
            HEADING_FONT_FACE,
            &format!("face=\"{}\"", escape_attr(&style.heading_font)),
        )
}

pub(super) fn apply_body_style(header: &str, style: &ResolvedHwpxStyle) -> String {
    let specs = [
        CharPrSpec::body(6, style.body_font_size, &style.text_color),
        CharPrSpec::bold(7, style.body_font_size, &style.text_color),
        CharPrSpec::italic(8, style.body_font_size, &style.text_color),
        CharPrSpec::code(9, style.body_font_size),
        CharPrSpec::heading(10, style.heading_sizes[0], &style.heading_colors[0]),
        CharPrSpec::heading(11, style.heading_sizes[1], &style.heading_colors[1]),
        CharPrSpec::heading(12, style.heading_sizes[2], &style.heading_colors[2]),
        CharPrSpec::heading(13, style.heading_sizes[3], &style.heading_colors[3]),
        CharPrSpec::link(14, style.body_font_size, &style.link_color),
    ];

    apply_char_pr_specs(header, &specs)
}

pub(super) fn apply_paragraph_alignment(header: &str, style: &ResolvedHwpxStyle) -> String {
    let mut updated = header.to_string();
    for id in [0_u32, 2, 3, 4, 5, 6, 7, 21, 22] {
        updated = update_para_align(&updated, id, style.paragraph_align);
    }
    updated
}

pub(super) fn additional_char_properties(style: &ResolvedHwpxStyle) -> String {
    [
        char_property_xml(CharPrSpec::bold(7, style.body_font_size, &style.text_color)),
        char_property_xml(CharPrSpec::italic(
            8,
            style.body_font_size,
            &style.text_color,
        )),
        char_property_xml(CharPrSpec::code(9, style.body_font_size)),
        char_property_xml(CharPrSpec::heading(
            10,
            style.heading_sizes[0],
            &style.heading_colors[0],
        )),
        char_property_xml(CharPrSpec::heading(
            11,
            style.heading_sizes[1],
            &style.heading_colors[1],
        )),
        char_property_xml(CharPrSpec::heading(
            12,
            style.heading_sizes[2],
            &style.heading_colors[2],
        )),
        char_property_xml(CharPrSpec::heading(
            13,
            style.heading_sizes[3],
            &style.heading_colors[3],
        )),
        char_property_xml(CharPrSpec::link(
            14,
            style.body_font_size,
            &style.link_color,
        )),
    ]
    .join("")
}

pub(super) fn legacy_additional_char_properties() -> &'static str {
    concat!(
        "<hh:charPr id=\"7\" height=\"1000\" textColor=\"#000000\" shadeColor=\"none\" useFontSpace=\"0\" useKerning=\"0\" symMark=\"NONE\" borderFillIDRef=\"2\"><hh:fontRef hangul=\"1\" latin=\"1\" hanja=\"1\" japanese=\"1\" other=\"1\" symbol=\"1\" user=\"1\"/><hh:ratio hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:spacing hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:relSz hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:offset hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:bold/><hh:underline type=\"NONE\" shape=\"SOLID\" color=\"#000000\"/><hh:strikeout shape=\"NONE\" color=\"#000000\"/><hh:outline type=\"NONE\"/><hh:shadow type=\"NONE\" color=\"#C0C0C0\" offsetX=\"10\" offsetY=\"10\"/></hh:charPr>",
        "<hh:charPr id=\"8\" height=\"1000\" textColor=\"#000000\" shadeColor=\"none\" useFontSpace=\"0\" useKerning=\"0\" symMark=\"NONE\" borderFillIDRef=\"2\"><hh:fontRef hangul=\"1\" latin=\"1\" hanja=\"1\" japanese=\"1\" other=\"1\" symbol=\"1\" user=\"1\"/><hh:ratio hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:spacing hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:relSz hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:offset hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:italic/><hh:underline type=\"NONE\" shape=\"SOLID\" color=\"#000000\"/><hh:strikeout shape=\"NONE\" color=\"#000000\"/><hh:outline type=\"NONE\"/><hh:shadow type=\"NONE\" color=\"#C0C0C0\" offsetX=\"10\" offsetY=\"10\"/></hh:charPr>",
        "<hh:charPr id=\"9\" height=\"1000\" textColor=\"#333333\" shadeColor=\"none\" useFontSpace=\"0\" useKerning=\"0\" symMark=\"NONE\" borderFillIDRef=\"2\"><hh:fontRef hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:ratio hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:spacing hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:relSz hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:offset hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:underline type=\"NONE\" shape=\"SOLID\" color=\"#000000\"/><hh:strikeout shape=\"NONE\" color=\"#000000\"/><hh:outline type=\"NONE\"/><hh:shadow type=\"NONE\" color=\"#C0C0C0\" offsetX=\"10\" offsetY=\"10\"/></hh:charPr>",
        "<hh:charPr id=\"10\" height=\"1400\" textColor=\"#2E74B5\" shadeColor=\"none\" useFontSpace=\"0\" useKerning=\"0\" symMark=\"NONE\" borderFillIDRef=\"2\"><hh:fontRef hangul=\"1\" latin=\"1\" hanja=\"1\" japanese=\"1\" other=\"1\" symbol=\"1\" user=\"1\"/><hh:ratio hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:spacing hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:relSz hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:offset hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:bold/><hh:underline type=\"NONE\" shape=\"SOLID\" color=\"#2E74B5\"/><hh:strikeout shape=\"NONE\" color=\"#2E74B5\"/><hh:outline type=\"NONE\"/><hh:shadow type=\"NONE\" color=\"#C0C0C0\" offsetX=\"10\" offsetY=\"10\"/></hh:charPr>",
        "<hh:charPr id=\"11\" height=\"1300\" textColor=\"#1F1F1F\" shadeColor=\"none\" useFontSpace=\"0\" useKerning=\"0\" symMark=\"NONE\" borderFillIDRef=\"2\"><hh:fontRef hangul=\"1\" latin=\"1\" hanja=\"1\" japanese=\"1\" other=\"1\" symbol=\"1\" user=\"1\"/><hh:ratio hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:spacing hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:relSz hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:offset hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:bold/><hh:underline type=\"NONE\" shape=\"SOLID\" color=\"#1F1F1F\"/><hh:strikeout shape=\"NONE\" color=\"#1F1F1F\"/><hh:outline type=\"NONE\"/><hh:shadow type=\"NONE\" color=\"#C0C0C0\" offsetX=\"10\" offsetY=\"10\"/></hh:charPr>",
        "<hh:charPr id=\"12\" height=\"1200\" textColor=\"#1F1F1F\" shadeColor=\"none\" useFontSpace=\"0\" useKerning=\"0\" symMark=\"NONE\" borderFillIDRef=\"2\"><hh:fontRef hangul=\"1\" latin=\"1\" hanja=\"1\" japanese=\"1\" other=\"1\" symbol=\"1\" user=\"1\"/><hh:ratio hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:spacing hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:relSz hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:offset hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:bold/><hh:underline type=\"NONE\" shape=\"SOLID\" color=\"#1F1F1F\"/><hh:strikeout shape=\"NONE\" color=\"#1F1F1F\"/><hh:outline type=\"NONE\"/><hh:shadow type=\"NONE\" color=\"#C0C0C0\" offsetX=\"10\" offsetY=\"10\"/></hh:charPr>",
        "<hh:charPr id=\"13\" height=\"1100\" textColor=\"#1F1F1F\" shadeColor=\"none\" useFontSpace=\"0\" useKerning=\"0\" symMark=\"NONE\" borderFillIDRef=\"2\"><hh:fontRef hangul=\"1\" latin=\"1\" hanja=\"1\" japanese=\"1\" other=\"1\" symbol=\"1\" user=\"1\"/><hh:ratio hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:spacing hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:relSz hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:offset hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:bold/><hh:underline type=\"NONE\" shape=\"SOLID\" color=\"#1F1F1F\"/><hh:strikeout shape=\"NONE\" color=\"#1F1F1F\"/><hh:outline type=\"NONE\"/><hh:shadow type=\"NONE\" color=\"#C0C0C0\" offsetX=\"10\" offsetY=\"10\"/></hh:charPr>",
        "<hh:charPr id=\"14\" height=\"1000\" textColor=\"#0563C1\" shadeColor=\"none\" useFontSpace=\"0\" useKerning=\"0\" symMark=\"NONE\" borderFillIDRef=\"2\"><hh:fontRef hangul=\"1\" latin=\"1\" hanja=\"1\" japanese=\"1\" other=\"1\" symbol=\"1\" user=\"1\"/><hh:ratio hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:spacing hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:relSz hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:offset hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:underline type=\"SOLID\" shape=\"SOLID\" color=\"#0563C1\"/><hh:strikeout shape=\"NONE\" color=\"#0563C1\"/><hh:outline type=\"NONE\"/><hh:shadow type=\"NONE\" color=\"#C0C0C0\" offsetX=\"10\" offsetY=\"10\"/></hh:charPr>"
    )
}

fn apply_char_pr_specs(header: &str, specs: &[CharPrSpec<'_>]) -> String {
    specs.iter().fold(header.to_string(), |current, spec| {
        update_char_pr(&current, *spec)
    })
}

fn char_property_xml(spec: CharPrSpec<'_>) -> String {
    let underline_type = if spec.underline { "SOLID" } else { "NONE" };
    let color = escape_attr(spec.text_color);
    let spacing = spec.spacing.unwrap_or(0);
    format!(
        concat!(
            "<hh:charPr id=\"{id}\" height=\"{height}\" textColor=\"{color}\" shadeColor=\"none\" useFontSpace=\"0\" useKerning=\"0\" symMark=\"NONE\" borderFillIDRef=\"2\">",
            "<hh:fontRef hangul=\"{font_ref}\" latin=\"{font_ref}\" hanja=\"{font_ref}\" japanese=\"{font_ref}\" other=\"{font_ref}\" symbol=\"{font_ref}\" user=\"{font_ref}\"/>",
            "<hh:ratio hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/>",
            "<hh:spacing hangul=\"{spacing}\" latin=\"{spacing}\" hanja=\"{spacing}\" japanese=\"{spacing}\" other=\"{spacing}\" symbol=\"{spacing}\" user=\"{spacing}\"/>",
            "<hh:relSz hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/>",
            "<hh:offset hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/>",
            "{bold_tag}{italic_tag}",
            "<hh:underline type=\"{underline_type}\" shape=\"SOLID\" color=\"{color}\"/>",
            "<hh:strikeout shape=\"NONE\" color=\"{color}\"/>",
            "<hh:outline type=\"NONE\"/>",
            "<hh:shadow type=\"NONE\" color=\"#C0C0C0\" offsetX=\"10\" offsetY=\"10\"/>",
            "</hh:charPr>"
        ),
        id = spec.id,
        height = spec.height,
        color = color,
        font_ref = spec.font_ref,
        spacing = spacing,
        bold_tag = if spec.bold { "<hh:bold/>" } else { "" },
        italic_tag = if spec.italic { "<hh:italic/>" } else { "" },
        underline_type = underline_type,
    )
}

fn update_char_pr(xml: &str, spec: CharPrSpec<'_>) -> String {
    replace_char_pr_segment(xml, spec.id, &char_property_xml(spec))
}

fn replace_char_pr_segment(xml: &str, id: u32, replacement: &str) -> String {
    let start_tag = format!("<hh:charPr id=\"{id}\"");
    let Some(start) = xml.find(&start_tag) else {
        return xml.to_string();
    };
    let tail = &xml[start..];
    let Some(end_offset) = tail.find("</hh:charPr>") else {
        return xml.to_string();
    };
    let end = start + end_offset + "</hh:charPr>".len();
    let mut updated = String::with_capacity(xml.len() + replacement.len());
    updated.push_str(&xml[..start]);
    updated.push_str(replacement);
    updated.push_str(&xml[end..]);
    updated
}

fn update_para_align(xml: &str, id: u32, align: &str) -> String {
    let start_tag = format!("<hh:paraPr id=\"{id}\"");
    let Some(start) = xml.find(&start_tag) else {
        return xml.to_string();
    };
    let tail = &xml[start..];
    let Some(end_offset) = tail.find("</hh:paraPr>") else {
        return xml.to_string();
    };
    let end = start + end_offset + "</hh:paraPr>".len();
    let segment = &xml[start..end];
    let replaced = segment.replace(
        "<hh:align horizontal=\"JUSTIFY\"",
        &format!("<hh:align horizontal=\"{align}\""),
    );
    let mut updated = String::with_capacity(xml.len());
    updated.push_str(&xml[..start]);
    updated.push_str(&replaced);
    updated.push_str(&xml[end..]);
    updated
}

#[derive(Clone, Copy)]
pub(super) struct CharPrSpec<'a> {
    id: u32,
    height: u32,
    text_color: &'a str,
    font_ref: u32,
    bold: bool,
    italic: bool,
    underline: bool,
    spacing: Option<i32>,
}

impl<'a> CharPrSpec<'a> {
    fn body(id: u32, height: u32, text_color: &'a str) -> Self {
        Self {
            id,
            height,
            text_color,
            font_ref: 0,
            bold: false,
            italic: false,
            underline: false,
            spacing: None,
        }
    }

    fn bold(id: u32, height: u32, text_color: &'a str) -> Self {
        Self {
            bold: true,
            ..Self::body(id, height, text_color)
        }
    }

    fn italic(id: u32, height: u32, text_color: &'a str) -> Self {
        Self {
            italic: true,
            ..Self::body(id, height, text_color)
        }
    }

    fn code(id: u32, height: u32) -> Self {
        Self {
            id,
            height,
            text_color: "#333333",
            font_ref: 0,
            bold: false,
            italic: false,
            underline: false,
            spacing: None,
        }
    }

    fn heading(id: u32, height: u32, text_color: &'a str) -> Self {
        Self {
            id,
            height,
            text_color,
            font_ref: 1,
            bold: true,
            italic: false,
            underline: false,
            spacing: None,
        }
    }

    fn link(id: u32, height: u32, text_color: &'a str) -> Self {
        Self {
            underline: true,
            ..Self::body(id, height, text_color)
        }
    }
}
