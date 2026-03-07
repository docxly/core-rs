use crate::utils::xml_helper::{escape_attr, escape_text};

use super::render::{RunStyle, preserve_space, run_properties};

pub fn wrap_document(body: &str) -> String {
    format!(
        concat!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#,
            r#"<w:document xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture">"#,
            r#"<w:body>{body}<w:sectPr><w:pgSz w:w="12240" w:h="15840"/><w:pgMar w:top="1440" w:right="1440" w:bottom="1440" w:left="1440" w:header="708" w:footer="708" w:gutter="0"/></w:sectPr></w:body></w:document>"#
        ),
        body = body
    )
}

pub fn paragraph_with_properties(properties: &str, content: &str) -> String {
    let ppr = if properties.is_empty() {
        String::new()
    } else {
        format!("<w:pPr>{}</w:pPr>", properties)
    };
    format!("<w:p>{}{}</w:p>", ppr, content)
}

pub fn text_run(text: &str, style: RunStyle) -> String {
    let escaped = escape_text(text);
    let space = if preserve_space(text) {
        " xml:space=\"preserve\""
    } else {
        ""
    };

    format!(
        "<w:r>{}<w:t{}>{}</w:t></w:r>",
        run_properties(style),
        space,
        escaped
    )
}

pub fn image_run(
    relationship_id: &str,
    alt_text: &str,
    doc_pr_id: usize,
    c_nvpr_id: usize,
    cx: i64,
    cy: i64,
) -> String {
    format!(
        concat!(
            "<w:r><w:drawing><wp:inline distT=\"0\" distB=\"0\" distL=\"0\" distR=\"0\">",
            "<wp:extent cx=\"{cx}\" cy=\"{cy}\"/>",
            "<wp:effectExtent l=\"0\" t=\"0\" r=\"0\" b=\"0\"/>",
            "<wp:docPr id=\"{doc_pr_id}\" name=\"Picture\" descr=\"{alt}\"/>",
            "<wp:cNvGraphicFramePr><a:graphicFrameLocks noChangeAspect=\"1\"/></wp:cNvGraphicFramePr>",
            "<a:graphic><a:graphicData uri=\"http://schemas.openxmlformats.org/drawingml/2006/picture\">",
            "<pic:pic><pic:nvPicPr><pic:cNvPr id=\"{c_nvpr_id}\" name=\"Image\" descr=\"{alt}\"/><pic:cNvPicPr/></pic:nvPicPr>",
            "<pic:blipFill><a:blip r:embed=\"{rid}\"/><a:stretch><a:fillRect/></a:stretch></pic:blipFill>",
            "<pic:spPr><a:xfrm><a:off x=\"0\" y=\"0\"/><a:ext cx=\"{cx}\" cy=\"{cy}\"/></a:xfrm><a:prstGeom prst=\"rect\"><a:avLst/></a:prstGeom></pic:spPr>",
            "</pic:pic></a:graphicData></a:graphic></wp:inline></w:drawing></w:r>"
        ),
        rid = escape_attr(relationship_id),
        alt = escape_attr(alt_text),
        doc_pr_id = doc_pr_id,
        c_nvpr_id = c_nvpr_id,
        cx = cx,
        cy = cy,
    )
}
