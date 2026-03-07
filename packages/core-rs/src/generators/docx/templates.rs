use std::collections::BTreeMap;

use crate::utils::xml_helper::{escape_attr, escape_text};

use super::render::ImageAsset;

pub fn content_types(images: &[ImageAsset]) -> String {
    let mut parts = vec![
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.to_string(),
        r#"<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">"#.to_string(),
        r#"<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>"#.to_string(),
        r#"<Default Extension="xml" ContentType="application/xml"/>"#.to_string(),
    ];

    let mut image_types = BTreeMap::new();
    for image in images {
        image_types.insert(image.extension.clone(), image.mime_type.clone());
    }
    for (extension, mime_type) in image_types {
        parts.push(format!(
            r#"<Default Extension="{}" ContentType="{}"/>"#,
            escape_attr(&extension),
            escape_attr(&mime_type)
        ));
    }

    parts.extend([
        r#"<Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>"#.to_string(),
        r#"<Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>"#.to_string(),
        r#"<Override PartName="/word/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"/>"#.to_string(),
        r#"</Types>"#.to_string(),
    ]);

    parts.join("")
}

pub fn root_relationships() -> String {
    concat!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#,
        r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">"#,
        r#"<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>"#,
        r#"<Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>"#,
        r#"</Relationships>"#
    )
    .to_string()
}

pub fn core_properties(title: &str, author: &str) -> String {
    format!(
        concat!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#,
            r#"<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" xmlns:dcmitype="http://purl.org/dc/dcmitype/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">"#,
            r#"<dc:title>{}</dc:title><dc:creator>{}</dc:creator><cp:lastModifiedBy>{}</cp:lastModifiedBy><dcterms:created xsi:type="dcterms:W3CDTF">2024-01-01T00:00:00Z</dcterms:created><dcterms:modified xsi:type="dcterms:W3CDTF">2024-01-01T00:00:00Z</dcterms:modified></cp:coreProperties>"#
        ),
        escape_text(title),
        escape_text(author),
        escape_text(author)
    )
}

pub fn document_relationships(hyperlinks: &[String], images: &[ImageAsset]) -> String {
    let mut relationships = vec![
        r#"<Relationship Id="rIdStyles" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/>"#.to_string(),
    ];

    relationships.extend(hyperlinks.iter().enumerate().map(|(index, url)| {
        format!(
            r#"<Relationship Id="rLink{}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink" Target="{}" TargetMode="External"/>"#,
            index + 1,
            escape_attr(url)
        )
    }));

    relationships.extend(images.iter().map(|image| {
        format!(
            r#"<Relationship Id="{}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="{}"/>"#,
            escape_attr(&image.relationship_id),
            escape_attr(image.target.trim_start_matches("word/"))
        )
    }));

    format!(
        concat!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#,
            r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">{}</Relationships>"#
        ),
        relationships.join("")
    )
}

pub fn styles(max_heading_level: u8) -> String {
    let mut xml = String::from(concat!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#,
        r#"<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">"#,
        r#"<w:docDefaults><w:rPrDefault><w:rPr><w:rFonts w:asciiTheme="minorHAnsi" w:hAnsiTheme="minorHAnsi" w:eastAsiaTheme="minorHAnsi" w:cstheme="minorBidi"/><w:sz w:val="22"/><w:szCs w:val="22"/><w:lang w:val="en-US" w:eastAsia="ko-KR" w:bidi="ar-SA"/></w:rPr></w:rPrDefault><w:pPrDefault/></w:docDefaults>"#,
        r#"<w:latentStyles w:defLockedState="0" w:defUIPriority="99" w:defSemiHidden="0" w:defUnhideWhenUsed="0" w:defQFormat="0" w:count="0"/>"#,
        r#"<w:style w:type="paragraph" w:default="1" w:styleId="Normal"><w:name w:val="Normal"/><w:qFormat/></w:style>"#,
        r#"<w:style w:type="character" w:default="1" w:styleId="DefaultParagraphFont"><w:name w:val="Default Paragraph Font"/><w:uiPriority w:val="1"/><w:semiHidden/><w:unhideWhenUsed/></w:style>"#,
        r#"<w:style w:type="paragraph" w:styleId="Heading1"><w:name w:val="heading 1"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/><w:uiPriority w:val="9"/><w:unhideWhenUsed/><w:qFormat/><w:rPr><w:b/><w:sz w:val="32"/></w:rPr></w:style>"#,
        r#"<w:style w:type="paragraph" w:styleId="Heading2"><w:name w:val="heading 2"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/><w:uiPriority w:val="9"/><w:unhideWhenUsed/><w:qFormat/><w:rPr><w:b/><w:sz w:val="28"/></w:rPr></w:style>"#,
        r#"<w:style w:type="paragraph" w:styleId="Heading3"><w:name w:val="heading 3"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/><w:uiPriority w:val="9"/><w:unhideWhenUsed/><w:qFormat/><w:rPr><w:b/><w:sz w:val="26"/></w:rPr></w:style>"#,
    ));
    if max_heading_level >= 4 {
        xml.push_str(r#"<w:style w:type="paragraph" w:styleId="Heading4"><w:name w:val="heading 4"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/><w:uiPriority w:val="9"/><w:unhideWhenUsed/><w:qFormat/><w:rPr><w:b/><w:sz w:val="24"/></w:rPr></w:style>"#);
    }
    if max_heading_level >= 5 {
        xml.push_str(r#"<w:style w:type="paragraph" w:styleId="Heading5"><w:name w:val="heading 5"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/><w:uiPriority w:val="9"/><w:unhideWhenUsed/><w:qFormat/><w:rPr><w:b/><w:sz w:val="22"/></w:rPr></w:style>"#);
    }
    if max_heading_level >= 6 {
        xml.push_str(r#"<w:style w:type="paragraph" w:styleId="Heading6"><w:name w:val="heading 6"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/><w:uiPriority w:val="9"/><w:unhideWhenUsed/><w:qFormat/><w:rPr><w:b/><w:sz w:val="22"/></w:rPr></w:style>"#);
    }
    xml.push_str(r#"<w:style w:type="character" w:styleId="Hyperlink"><w:name w:val="Hyperlink"/><w:basedOn w:val="DefaultParagraphFont"/><w:uiPriority w:val="99"/><w:unhideWhenUsed/><w:rPr><w:color w:val="0563C1"/><w:u w:val="single"/></w:rPr></w:style>"#);
    xml.push_str(r#"</w:styles>"#);
    xml
}
