use super::style::ResolvedHwpxStyle;
use crate::models::block::Block;
use crate::models::document::Document;
use crate::models::inline::Inline;
use crate::utils::xml_helper::escape_attr;

const HEADER_TEMPLATE: &str = include_str!("reference/paragraph-only/Contents/header.xml");
const CHAR_PROPERTIES_NEEDLE: &str = "</hh:charProperties>";
const CHAR_PROPERTIES_COUNT_NEEDLE: &str = "<hh:charProperties itemCnt=\"7\">";
const NUMBERINGS_COUNT_NEEDLE: &str = "<hh:numberings itemCnt=\"1\">";
const NUMBERINGS_NEEDLE: &str = "</hh:numberings>";
const PARA_PROPERTIES_COUNT_NEEDLE: &str = "<hh:paraProperties itemCnt=\"19\">";
const PARA_PROPERTIES_NEEDLE: &str = "</hh:paraProperties>";
const BORDER_FILLS_COUNT_NEEDLE: &str = "<hh:borderFills itemCnt=\"2\">";
const BORDER_FILLS_NEEDLE: &str = "</hh:borderFills>";
const BODY_FONT_FACE: &str = "face=\"함초롬바탕\"";
const HEADING_FONT_FACE: &str = "face=\"함초롬돋움\"";
const TABLE_BORDER_FILL_XML: &str = concat!(
    "<hh:borderFill id=\"3\" threeD=\"0\" shadow=\"0\" centerLine=\"NONE\" breakCellSeparateLine=\"0\">",
    "<hh:slash type=\"NONE\" Crooked=\"0\" isCounter=\"0\"/>",
    "<hh:backSlash type=\"NONE\" Crooked=\"0\" isCounter=\"0\"/>",
    "<hh:leftBorder type=\"SOLID\" width=\"0.12 mm\" color=\"#000000\"/>",
    "<hh:rightBorder type=\"SOLID\" width=\"0.12 mm\" color=\"#000000\"/>",
    "<hh:topBorder type=\"SOLID\" width=\"0.12 mm\" color=\"#000000\"/>",
    "<hh:bottomBorder type=\"SOLID\" width=\"0.12 mm\" color=\"#000000\"/>",
    "<hh:diagonal type=\"SOLID\" width=\"0.1 mm\" color=\"#000000\"/>",
    "</hh:borderFill>"
);
const LIST_BULLETS_XML: &str = concat!(
    "<hh:bullets itemCnt=\"1\">",
    "<hh:bullet id=\"1\" char=\"\" useImage=\"0\">",
    "<hh:paraHead level=\"0\" align=\"LEFT\" useInstWidth=\"0\" autoIndent=\"1\" widthAdjust=\"0\" textOffsetType=\"PERCENT\" textOffset=\"50\" numFormat=\"DIGIT\" charPrIDRef=\"4294967295\" checkable=\"0\"/>",
    "</hh:bullet>",
    "</hh:bullets>"
);
const LIST_LEVEL1_PARA_PR_XML: &str = concat!(
    "<hh:paraPr id=\"19\" tabPrIDRef=\"2\" condense=\"0\" fontLineHeight=\"0\" snapToGrid=\"1\" suppressLineNumbers=\"0\" checked=\"0\">",
    "<hh:align horizontal=\"LEFT\" vertical=\"BASELINE\"/>",
    "<hh:heading type=\"BULLET\" idRef=\"1\" level=\"0\"/>",
    "<hh:breakSetting breakLatinWord=\"KEEP_WORD\" breakNonLatinWord=\"BREAK_WORD\" widowOrphan=\"0\" keepWithNext=\"0\" keepLines=\"0\" pageBreakBefore=\"0\" lineWrap=\"BREAK\"/>",
    "<hh:autoSpacing eAsianEng=\"0\" eAsianNum=\"0\"/>",
    "<hp:switch><hp:case hp:required-namespace=\"http://www.hancom.co.kr/hwpml/2016/HwpUnitChar\"><hh:margin><hc:intent value=\"0\" unit=\"HWPUNIT\"/><hc:left value=\"1100\" unit=\"HWPUNIT\"/><hc:right value=\"0\" unit=\"HWPUNIT\"/><hc:prev value=\"0\" unit=\"HWPUNIT\"/><hc:next value=\"700\" unit=\"HWPUNIT\"/></hh:margin><hh:lineSpacing type=\"PERCENT\" value=\"160\" unit=\"HWPUNIT\"/></hp:case><hp:default><hh:margin><hc:intent value=\"0\" unit=\"HWPUNIT\"/><hc:left value=\"2200\" unit=\"HWPUNIT\"/><hc:right value=\"0\" unit=\"HWPUNIT\"/><hc:prev value=\"0\" unit=\"HWPUNIT\"/><hc:next value=\"1400\" unit=\"HWPUNIT\"/></hh:margin><hh:lineSpacing type=\"PERCENT\" value=\"160\" unit=\"HWPUNIT\"/></hp:default></hp:switch>",
    "<hh:border borderFillIDRef=\"2\" offsetLeft=\"0\" offsetRight=\"0\" offsetTop=\"0\" offsetBottom=\"0\" connect=\"0\" ignoreMargin=\"0\"/>",
    "</hh:paraPr>"
);
const LIST_LEVEL2_PARA_PR_XML: &str = concat!(
    "<hh:paraPr id=\"20\" tabPrIDRef=\"2\" condense=\"0\" fontLineHeight=\"0\" snapToGrid=\"1\" suppressLineNumbers=\"0\" checked=\"0\">",
    "<hh:align horizontal=\"LEFT\" vertical=\"BASELINE\"/>",
    "<hh:heading type=\"BULLET\" idRef=\"1\" level=\"0\"/>",
    "<hh:breakSetting breakLatinWord=\"KEEP_WORD\" breakNonLatinWord=\"BREAK_WORD\" widowOrphan=\"0\" keepWithNext=\"0\" keepLines=\"0\" pageBreakBefore=\"0\" lineWrap=\"BREAK\"/>",
    "<hh:autoSpacing eAsianEng=\"0\" eAsianNum=\"0\"/>",
    "<hp:switch><hp:case hp:required-namespace=\"http://www.hancom.co.kr/hwpml/2016/HwpUnitChar\"><hh:margin><hc:intent value=\"0\" unit=\"HWPUNIT\"/><hc:left value=\"2200\" unit=\"HWPUNIT\"/><hc:right value=\"0\" unit=\"HWPUNIT\"/><hc:prev value=\"0\" unit=\"HWPUNIT\"/><hc:next value=\"700\" unit=\"HWPUNIT\"/></hh:margin><hh:lineSpacing type=\"PERCENT\" value=\"160\" unit=\"HWPUNIT\"/></hp:case><hp:default><hh:margin><hc:intent value=\"0\" unit=\"HWPUNIT\"/><hc:left value=\"4400\" unit=\"HWPUNIT\"/><hc:right value=\"0\" unit=\"HWPUNIT\"/><hc:prev value=\"0\" unit=\"HWPUNIT\"/><hc:next value=\"1400\" unit=\"HWPUNIT\"/></hh:margin><hh:lineSpacing type=\"PERCENT\" value=\"160\" unit=\"HWPUNIT\"/></hp:default></hp:switch>",
    "<hh:border borderFillIDRef=\"2\" offsetLeft=\"0\" offsetRight=\"0\" offsetTop=\"0\" offsetBottom=\"0\" connect=\"0\" ignoreMargin=\"0\"/>",
    "</hh:paraPr>"
);

#[derive(Clone, Copy)]
enum ListContract {
    None,
    Unordered(u8),
    OrderedSingleLevel,
    OrderedNestedDepth2,
}

struct CharPrSpec<'a> {
    id: u32,
    height: u32,
    text_color: &'a str,
    font_ref: u32,
    bold: bool,
    italic: bool,
    underline: bool,
    spacing: Option<i32>,
}

pub fn build_header_xml(document: &Document, style: &ResolvedHwpxStyle) -> String {
    let needs_styled_header = requires_styled_header(document);
    let needs_table_border_fill = document.blocks.iter().any(block_contains_table);
    let list_contract = list_contract(document);

    if style.is_default() && !needs_styled_header {
        let header = HEADER_TEMPLATE.to_string();
        return maybe_add_list_contract(
            maybe_add_table_border_fill(header, needs_table_border_fill),
            list_contract,
        );
    }

    if style.is_default() {
        let mut header = HEADER_TEMPLATE.to_string();
        if needs_styled_header {
            header = header.replace(
                CHAR_PROPERTIES_COUNT_NEEDLE,
                "<hh:charProperties itemCnt=\"15\">",
            );
            header = header.replace(
                CHAR_PROPERTIES_NEEDLE,
                &format!(
                    "{}{}",
                    legacy_additional_char_properties(),
                    CHAR_PROPERTIES_NEEDLE
                ),
            );
        }
        return maybe_add_list_contract(
            maybe_add_table_border_fill(header, needs_table_border_fill),
            list_contract,
        );
    }

    let mut header = HEADER_TEMPLATE.to_string();
    header = apply_font_faces(&header, style);
    header = apply_body_style(&header, style);
    header = apply_paragraph_alignment(&header, style);
    header = header.replace(
        CHAR_PROPERTIES_COUNT_NEEDLE,
        "<hh:charProperties itemCnt=\"15\">",
    );
    header = header.replace(
        CHAR_PROPERTIES_NEEDLE,
        &format!(
            "{}{}",
            additional_char_properties(style),
            CHAR_PROPERTIES_NEEDLE
        ),
    );
    maybe_add_list_contract(
        maybe_add_table_border_fill(header, needs_table_border_fill),
        list_contract,
    )
}

fn requires_styled_header(document: &Document) -> bool {
    document.blocks.iter().any(block_requires_styled_header)
}

fn block_requires_styled_header(block: &Block) -> bool {
    match block {
        Block::Paragraph(inlines) => inline_slice_requires_styled_header(inlines),
        Block::Heading { .. } => true,
        Block::BlockQuote(blocks) => blocks.iter().any(block_requires_styled_header),
        Block::CodeBlock { .. } | Block::List(_) | Block::Table(_) | Block::ThematicBreak => false,
    }
}

fn block_contains_table(block: &Block) -> bool {
    match block {
        Block::Table(_) => true,
        Block::BlockQuote(blocks) => blocks.iter().any(block_contains_table),
        Block::Paragraph(_)
        | Block::Heading { .. }
        | Block::CodeBlock { .. }
        | Block::List(_)
        | Block::ThematicBreak => false,
    }
}

fn list_contract(document: &Document) -> ListContract {
    let ordered_depth = document
        .blocks
        .iter()
        .map(block_ordered_list_depth)
        .max()
        .unwrap_or(0);
    if ordered_depth > 0 {
        return if ordered_depth > 1 {
            ListContract::OrderedNestedDepth2
        } else {
            ListContract::OrderedSingleLevel
        };
    }

    let unordered_depth = document
        .blocks
        .iter()
        .map(block_unordered_list_depth)
        .max()
        .unwrap_or(0);
    if unordered_depth > 0 {
        ListContract::Unordered(unordered_depth)
    } else {
        ListContract::None
    }
}

fn block_unordered_list_depth(block: &Block) -> u8 {
    match block {
        Block::List(list) if !list.ordered => {
            let nested_depth = list
                .items
                .iter()
                .flat_map(|item| item.blocks.iter())
                .map(block_unordered_list_depth)
                .max()
                .unwrap_or(0);
            1 + nested_depth
        }
        Block::BlockQuote(blocks) => blocks
            .iter()
            .map(block_unordered_list_depth)
            .max()
            .unwrap_or(0),
        Block::Paragraph(_)
        | Block::Heading { .. }
        | Block::CodeBlock { .. }
        | Block::Table(_)
        | Block::ThematicBreak
        | Block::List(_) => 0,
    }
}

fn block_ordered_list_depth(block: &Block) -> u8 {
    match block {
        Block::List(list) if list.ordered => {
            let nested_depth = list
                .items
                .iter()
                .flat_map(|item| item.blocks.iter())
                .map(block_ordered_list_depth)
                .max()
                .unwrap_or(0);
            1 + nested_depth
        }
        Block::BlockQuote(blocks) => blocks
            .iter()
            .map(block_ordered_list_depth)
            .max()
            .unwrap_or(0),
        Block::Paragraph(_)
        | Block::Heading { .. }
        | Block::CodeBlock { .. }
        | Block::Table(_)
        | Block::ThematicBreak
        | Block::List(_) => 0,
    }
}

fn inline_slice_requires_styled_header(inlines: &[Inline]) -> bool {
    inlines.iter().any(inline_requires_styled_header)
}

fn inline_requires_styled_header(inline: &Inline) -> bool {
    match inline {
        Inline::Text(_) => false,
        Inline::Emphasis(children) | Inline::Strong(children) => {
            let _ = children;
            true
        }
        Inline::Code(_) | Inline::Link { .. } | Inline::HardBreak => true,
        Inline::Image(_) => false,
    }
}

fn apply_font_faces(header: &str, style: &ResolvedHwpxStyle) -> String {
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

fn apply_body_style(header: &str, style: &ResolvedHwpxStyle) -> String {
    let mut updated = update_char_pr(
        header,
        CharPrSpec {
            id: 6,
            height: style.body_font_size,
            text_color: &style.text_color,
            font_ref: 0,
            bold: false,
            italic: false,
            underline: false,
            spacing: None,
        },
    );
    updated = update_char_pr(
        &updated,
        CharPrSpec {
            id: 7,
            height: style.body_font_size,
            text_color: &style.text_color,
            font_ref: 0,
            bold: true,
            italic: false,
            underline: false,
            spacing: None,
        },
    );
    updated = update_char_pr(
        &updated,
        CharPrSpec {
            id: 8,
            height: style.body_font_size,
            text_color: &style.text_color,
            font_ref: 0,
            bold: false,
            italic: true,
            underline: false,
            spacing: None,
        },
    );
    updated = update_char_pr(
        &updated,
        CharPrSpec {
            id: 9,
            height: style.body_font_size,
            text_color: "#333333",
            font_ref: 0,
            bold: false,
            italic: false,
            underline: false,
            spacing: Some(-5),
        },
    );
    updated = update_char_pr(
        &updated,
        CharPrSpec {
            id: 10,
            height: style.heading_sizes[0],
            text_color: &style.heading_colors[0],
            font_ref: 1,
            bold: true,
            italic: false,
            underline: false,
            spacing: None,
        },
    );
    updated = update_char_pr(
        &updated,
        CharPrSpec {
            id: 11,
            height: style.heading_sizes[1],
            text_color: &style.heading_colors[1],
            font_ref: 1,
            bold: true,
            italic: false,
            underline: false,
            spacing: None,
        },
    );
    updated = update_char_pr(
        &updated,
        CharPrSpec {
            id: 12,
            height: style.heading_sizes[2],
            text_color: &style.heading_colors[2],
            font_ref: 1,
            bold: true,
            italic: false,
            underline: false,
            spacing: None,
        },
    );
    updated = update_char_pr(
        &updated,
        CharPrSpec {
            id: 13,
            height: style.heading_sizes[3],
            text_color: &style.heading_colors[3],
            font_ref: 1,
            bold: true,
            italic: false,
            underline: false,
            spacing: None,
        },
    );
    update_char_pr(
        &updated,
        CharPrSpec {
            id: 14,
            height: style.body_font_size,
            text_color: &style.link_color,
            font_ref: 0,
            bold: false,
            italic: false,
            underline: true,
            spacing: None,
        },
    )
}

fn apply_paragraph_alignment(header: &str, style: &ResolvedHwpxStyle) -> String {
    let mut updated = header.to_string();
    for id in [0_u32, 2, 3, 4, 5, 6, 7] {
        updated = update_para_align(&updated, id, style.paragraph_align);
    }
    updated
}

fn additional_char_properties(style: &ResolvedHwpxStyle) -> String {
    [
        char_property_xml(CharPrSpec {
            id: 7,
            height: style.body_font_size,
            text_color: &style.text_color,
            font_ref: 0,
            bold: true,
            italic: false,
            underline: false,
            spacing: None,
        }),
        char_property_xml(CharPrSpec {
            id: 8,
            height: style.body_font_size,
            text_color: &style.text_color,
            font_ref: 0,
            bold: false,
            italic: true,
            underline: false,
            spacing: None,
        }),
        char_property_xml(CharPrSpec {
            id: 9,
            height: style.body_font_size,
            text_color: "#333333",
            font_ref: 0,
            bold: false,
            italic: false,
            underline: false,
            spacing: Some(-5),
        }),
        char_property_xml(CharPrSpec {
            id: 10,
            height: style.heading_sizes[0],
            text_color: &style.heading_colors[0],
            font_ref: 1,
            bold: true,
            italic: false,
            underline: false,
            spacing: None,
        }),
        char_property_xml(CharPrSpec {
            id: 11,
            height: style.heading_sizes[1],
            text_color: &style.heading_colors[1],
            font_ref: 1,
            bold: true,
            italic: false,
            underline: false,
            spacing: None,
        }),
        char_property_xml(CharPrSpec {
            id: 12,
            height: style.heading_sizes[2],
            text_color: &style.heading_colors[2],
            font_ref: 1,
            bold: true,
            italic: false,
            underline: false,
            spacing: None,
        }),
        char_property_xml(CharPrSpec {
            id: 13,
            height: style.heading_sizes[3],
            text_color: &style.heading_colors[3],
            font_ref: 1,
            bold: true,
            italic: false,
            underline: false,
            spacing: None,
        }),
        char_property_xml(CharPrSpec {
            id: 14,
            height: style.body_font_size,
            text_color: &style.link_color,
            font_ref: 0,
            bold: false,
            italic: false,
            underline: true,
            spacing: None,
        }),
    ]
    .join("")
}

fn legacy_additional_char_properties() -> &'static str {
    concat!(
        "<hh:charPr id=\"7\" height=\"1000\" textColor=\"#000000\" shadeColor=\"none\" useFontSpace=\"0\" useKerning=\"0\" symMark=\"NONE\" borderFillIDRef=\"2\"><hh:fontRef hangul=\"1\" latin=\"1\" hanja=\"1\" japanese=\"1\" other=\"1\" symbol=\"1\" user=\"1\"/><hh:ratio hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:spacing hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:relSz hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:offset hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:bold/><hh:underline type=\"NONE\" shape=\"SOLID\" color=\"#000000\"/><hh:strikeout shape=\"NONE\" color=\"#000000\"/><hh:outline type=\"NONE\"/><hh:shadow type=\"NONE\" color=\"#C0C0C0\" offsetX=\"10\" offsetY=\"10\"/></hh:charPr>",
        "<hh:charPr id=\"8\" height=\"1000\" textColor=\"#000000\" shadeColor=\"none\" useFontSpace=\"0\" useKerning=\"0\" symMark=\"NONE\" borderFillIDRef=\"2\"><hh:fontRef hangul=\"1\" latin=\"1\" hanja=\"1\" japanese=\"1\" other=\"1\" symbol=\"1\" user=\"1\"/><hh:ratio hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:spacing hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:relSz hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:offset hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:italic/><hh:underline type=\"NONE\" shape=\"SOLID\" color=\"#000000\"/><hh:strikeout shape=\"NONE\" color=\"#000000\"/><hh:outline type=\"NONE\"/><hh:shadow type=\"NONE\" color=\"#C0C0C0\" offsetX=\"10\" offsetY=\"10\"/></hh:charPr>",
        "<hh:charPr id=\"9\" height=\"1000\" textColor=\"#333333\" shadeColor=\"none\" useFontSpace=\"0\" useKerning=\"0\" symMark=\"NONE\" borderFillIDRef=\"2\"><hh:fontRef hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:ratio hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:spacing hangul=\"-5\" latin=\"-5\" hanja=\"-5\" japanese=\"-5\" other=\"-5\" symbol=\"-5\" user=\"-5\"/><hh:relSz hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:offset hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:underline type=\"NONE\" shape=\"SOLID\" color=\"#000000\"/><hh:strikeout shape=\"NONE\" color=\"#000000\"/><hh:outline type=\"NONE\"/><hh:shadow type=\"NONE\" color=\"#C0C0C0\" offsetX=\"10\" offsetY=\"10\"/></hh:charPr>",
        "<hh:charPr id=\"10\" height=\"1400\" textColor=\"#2E74B5\" shadeColor=\"none\" useFontSpace=\"0\" useKerning=\"0\" symMark=\"NONE\" borderFillIDRef=\"2\"><hh:fontRef hangul=\"1\" latin=\"1\" hanja=\"1\" japanese=\"1\" other=\"1\" symbol=\"1\" user=\"1\"/><hh:ratio hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:spacing hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:relSz hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:offset hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:bold/><hh:underline type=\"NONE\" shape=\"SOLID\" color=\"#2E74B5\"/><hh:strikeout shape=\"NONE\" color=\"#2E74B5\"/><hh:outline type=\"NONE\"/><hh:shadow type=\"NONE\" color=\"#C0C0C0\" offsetX=\"10\" offsetY=\"10\"/></hh:charPr>",
        "<hh:charPr id=\"11\" height=\"1300\" textColor=\"#1F1F1F\" shadeColor=\"none\" useFontSpace=\"0\" useKerning=\"0\" symMark=\"NONE\" borderFillIDRef=\"2\"><hh:fontRef hangul=\"1\" latin=\"1\" hanja=\"1\" japanese=\"1\" other=\"1\" symbol=\"1\" user=\"1\"/><hh:ratio hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:spacing hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:relSz hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:offset hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:bold/><hh:underline type=\"NONE\" shape=\"SOLID\" color=\"#1F1F1F\"/><hh:strikeout shape=\"NONE\" color=\"#1F1F1F\"/><hh:outline type=\"NONE\"/><hh:shadow type=\"NONE\" color=\"#C0C0C0\" offsetX=\"10\" offsetY=\"10\"/></hh:charPr>",
        "<hh:charPr id=\"12\" height=\"1200\" textColor=\"#1F1F1F\" shadeColor=\"none\" useFontSpace=\"0\" useKerning=\"0\" symMark=\"NONE\" borderFillIDRef=\"2\"><hh:fontRef hangul=\"1\" latin=\"1\" hanja=\"1\" japanese=\"1\" other=\"1\" symbol=\"1\" user=\"1\"/><hh:ratio hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:spacing hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:relSz hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:offset hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:bold/><hh:underline type=\"NONE\" shape=\"SOLID\" color=\"#1F1F1F\"/><hh:strikeout shape=\"NONE\" color=\"#1F1F1F\"/><hh:outline type=\"NONE\"/><hh:shadow type=\"NONE\" color=\"#C0C0C0\" offsetX=\"10\" offsetY=\"10\"/></hh:charPr>",
        "<hh:charPr id=\"13\" height=\"1100\" textColor=\"#1F1F1F\" shadeColor=\"none\" useFontSpace=\"0\" useKerning=\"0\" symMark=\"NONE\" borderFillIDRef=\"2\"><hh:fontRef hangul=\"1\" latin=\"1\" hanja=\"1\" japanese=\"1\" other=\"1\" symbol=\"1\" user=\"1\"/><hh:ratio hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:spacing hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:relSz hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:offset hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:bold/><hh:underline type=\"NONE\" shape=\"SOLID\" color=\"#1F1F1F\"/><hh:strikeout shape=\"NONE\" color=\"#1F1F1F\"/><hh:outline type=\"NONE\"/><hh:shadow type=\"NONE\" color=\"#C0C0C0\" offsetX=\"10\" offsetY=\"10\"/></hh:charPr>",
        "<hh:charPr id=\"14\" height=\"1000\" textColor=\"#0563C1\" shadeColor=\"none\" useFontSpace=\"0\" useKerning=\"0\" symMark=\"NONE\" borderFillIDRef=\"2\"><hh:fontRef hangul=\"1\" latin=\"1\" hanja=\"1\" japanese=\"1\" other=\"1\" symbol=\"1\" user=\"1\"/><hh:ratio hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:spacing hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:relSz hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/><hh:offset hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/><hh:underline type=\"SOLID\" shape=\"SOLID\" color=\"#0563C1\"/><hh:strikeout shape=\"NONE\" color=\"#0563C1\"/><hh:outline type=\"NONE\"/><hh:shadow type=\"NONE\" color=\"#C0C0C0\" offsetX=\"10\" offsetY=\"10\"/></hh:charPr>"
    )
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
    let id = spec.id;
    replace_char_pr_segment(xml, id, &char_property_xml(spec))
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

fn maybe_add_table_border_fill(header: String, needs_table_border_fill: bool) -> String {
    if !needs_table_border_fill || header.contains("<hh:borderFill id=\"3\"") {
        return header;
    }

    header
        .replace(BORDER_FILLS_COUNT_NEEDLE, "<hh:borderFills itemCnt=\"3\">")
        .replace(
            BORDER_FILLS_NEEDLE,
            &format!("{TABLE_BORDER_FILL_XML}{BORDER_FILLS_NEEDLE}"),
        )
}

fn maybe_add_list_contract(header: String, contract: ListContract) -> String {
    match contract {
        ListContract::None => header,
        ListContract::Unordered(list_depth) => {
            maybe_add_unordered_list_contract(header, list_depth)
        }
        ListContract::OrderedSingleLevel => maybe_add_ordered_list_contract(header, false),
        ListContract::OrderedNestedDepth2 => maybe_add_ordered_list_contract(header, true),
    }
}

fn maybe_add_unordered_list_contract(header: String, list_depth: u8) -> String {
    if header.contains("<hh:paraPr id=\"19\"") {
        return header;
    }

    let new_count = if list_depth > 1 { 21 } else { 20 };
    let list_para_properties = if list_depth > 1 {
        format!("{LIST_LEVEL1_PARA_PR_XML}{LIST_LEVEL2_PARA_PR_XML}")
    } else {
        LIST_LEVEL1_PARA_PR_XML.to_string()
    };

    header
        .replacen(
            NUMBERINGS_NEEDLE,
            &format!("{NUMBERINGS_NEEDLE}{LIST_BULLETS_XML}"),
            1,
        )
        .replacen(
            PARA_PROPERTIES_COUNT_NEEDLE,
            &format!("<hh:paraProperties itemCnt=\"{new_count}\">"),
            1,
        )
        .replacen(
            PARA_PROPERTIES_NEEDLE,
            &format!("{list_para_properties}{PARA_PROPERTIES_NEEDLE}"),
            1,
        )
}

fn maybe_add_ordered_list_contract(header: String, nested: bool) -> String {
    if header.contains("<hh:heading type=\"NUMBER\"") {
        return header;
    }

    let added_numberings = if nested {
        format!(
            "{}{}",
            ordered_numbering_xml(2, 1),
            ordered_numbering_xml(3, 0)
        )
    } else {
        ordered_numbering_xml(2, 0)
    };
    let para_properties = if nested {
        format!(
            "{}{}",
            ordered_para_pr_xml(19, 2, 2_200),
            ordered_para_pr_xml(20, 3, 1_100)
        )
    } else {
        ordered_para_pr_xml(19, 2, 1_100)
    };
    let numbering_count = if nested { 3 } else { 2 };
    let para_count = if nested { 21 } else { 20 };

    header
        .replacen(
            NUMBERINGS_COUNT_NEEDLE,
            &format!("<hh:numberings itemCnt=\"{numbering_count}\">"),
            1,
        )
        .replacen(
            NUMBERINGS_NEEDLE,
            &format!("{added_numberings}{NUMBERINGS_NEEDLE}"),
            1,
        )
        .replacen(
            PARA_PROPERTIES_COUNT_NEEDLE,
            &format!("<hh:paraProperties itemCnt=\"{para_count}\">"),
            1,
        )
        .replacen(
            PARA_PROPERTIES_NEEDLE,
            &format!("{para_properties}{PARA_PROPERTIES_NEEDLE}"),
            1,
        )
}

fn ordered_numbering_xml(id: u32, start: u32) -> String {
    format!(
        concat!(
            "<hh:numbering id=\"{id}\" start=\"{start}\">",
            "<hh:paraHead start=\"1\" level=\"1\" align=\"LEFT\" useInstWidth=\"1\" autoIndent=\"1\" widthAdjust=\"0\" textOffsetType=\"PERCENT\" textOffset=\"50\" numFormat=\"DIGIT\" charPrIDRef=\"4294967295\" checkable=\"0\">^1.</hh:paraHead>",
            "<hh:paraHead start=\"1\" level=\"2\" align=\"LEFT\" useInstWidth=\"1\" autoIndent=\"1\" widthAdjust=\"0\" textOffsetType=\"PERCENT\" textOffset=\"50\" numFormat=\"HANGUL_SYLLABLE\" charPrIDRef=\"4294967295\" checkable=\"0\">^2.</hh:paraHead>",
            "<hh:paraHead start=\"1\" level=\"3\" align=\"LEFT\" useInstWidth=\"1\" autoIndent=\"1\" widthAdjust=\"0\" textOffsetType=\"PERCENT\" textOffset=\"50\" numFormat=\"DIGIT\" charPrIDRef=\"4294967295\" checkable=\"0\">^3)</hh:paraHead>",
            "<hh:paraHead start=\"1\" level=\"4\" align=\"LEFT\" useInstWidth=\"1\" autoIndent=\"1\" widthAdjust=\"0\" textOffsetType=\"PERCENT\" textOffset=\"50\" numFormat=\"HANGUL_SYLLABLE\" charPrIDRef=\"4294967295\" checkable=\"0\">^4)</hh:paraHead>",
            "<hh:paraHead start=\"1\" level=\"5\" align=\"LEFT\" useInstWidth=\"1\" autoIndent=\"1\" widthAdjust=\"0\" textOffsetType=\"PERCENT\" textOffset=\"50\" numFormat=\"DIGIT\" charPrIDRef=\"4294967295\" checkable=\"0\">(^5)</hh:paraHead>",
            "<hh:paraHead start=\"1\" level=\"6\" align=\"LEFT\" useInstWidth=\"1\" autoIndent=\"1\" widthAdjust=\"0\" textOffsetType=\"PERCENT\" textOffset=\"50\" numFormat=\"HANGUL_SYLLABLE\" charPrIDRef=\"4294967295\" checkable=\"0\">(^6)</hh:paraHead>",
            "<hh:paraHead start=\"1\" level=\"7\" align=\"LEFT\" useInstWidth=\"1\" autoIndent=\"1\" widthAdjust=\"0\" textOffsetType=\"PERCENT\" textOffset=\"50\" numFormat=\"CIRCLED_DIGIT\" charPrIDRef=\"4294967295\" checkable=\"1\">^7</hh:paraHead>",
            "<hh:paraHead start=\"1\" level=\"8\" align=\"LEFT\" useInstWidth=\"1\" autoIndent=\"1\" widthAdjust=\"0\" textOffsetType=\"PERCENT\" textOffset=\"50\" numFormat=\"CIRCLED_HANGUL_SYLLABLE\" charPrIDRef=\"4294967295\" checkable=\"1\">^8</hh:paraHead>",
            "<hh:paraHead start=\"1\" level=\"9\" align=\"LEFT\" useInstWidth=\"1\" autoIndent=\"1\" widthAdjust=\"0\" textOffsetType=\"PERCENT\" textOffset=\"50\" numFormat=\"DIGIT\" charPrIDRef=\"4294967295\" checkable=\"0\"/>",
            "<hh:paraHead start=\"1\" level=\"10\" align=\"LEFT\" useInstWidth=\"1\" autoIndent=\"1\" widthAdjust=\"0\" textOffsetType=\"PERCENT\" textOffset=\"50\" numFormat=\"DIGIT\" charPrIDRef=\"4294967295\" checkable=\"0\"/>",
            "</hh:numbering>"
        ),
        id = id,
        start = start,
    )
}

fn ordered_para_pr_xml(id: u32, numbering_id_ref: u32, left_margin: u32) -> String {
    format!(
        concat!(
            "<hh:paraPr id=\"{id}\" tabPrIDRef=\"2\" condense=\"0\" fontLineHeight=\"0\" snapToGrid=\"1\" suppressLineNumbers=\"0\" checked=\"0\">",
            "<hh:align horizontal=\"LEFT\" vertical=\"BASELINE\"/>",
            "<hh:heading type=\"NUMBER\" idRef=\"{numbering_id_ref}\" level=\"0\"/>",
            "<hh:breakSetting breakLatinWord=\"KEEP_WORD\" breakNonLatinWord=\"BREAK_WORD\" widowOrphan=\"0\" keepWithNext=\"0\" keepLines=\"0\" pageBreakBefore=\"0\" lineWrap=\"BREAK\"/>",
            "<hh:autoSpacing eAsianEng=\"0\" eAsianNum=\"0\"/>",
            "<hp:switch><hp:case hp:required-namespace=\"http://www.hancom.co.kr/hwpml/2016/HwpUnitChar\"><hh:margin><hc:intent value=\"0\" unit=\"HWPUNIT\"/><hc:left value=\"{left_margin}\" unit=\"HWPUNIT\"/><hc:right value=\"0\" unit=\"HWPUNIT\"/><hc:prev value=\"0\" unit=\"HWPUNIT\"/><hc:next value=\"700\" unit=\"HWPUNIT\"/></hh:margin><hh:lineSpacing type=\"PERCENT\" value=\"160\" unit=\"HWPUNIT\"/></hp:case><hp:default><hh:margin><hc:intent value=\"0\" unit=\"HWPUNIT\"/><hc:left value=\"{default_left_margin}\" unit=\"HWPUNIT\"/><hc:right value=\"0\" unit=\"HWPUNIT\"/><hc:prev value=\"0\" unit=\"HWPUNIT\"/><hc:next value=\"1400\" unit=\"HWPUNIT\"/></hh:margin><hh:lineSpacing type=\"PERCENT\" value=\"160\" unit=\"HWPUNIT\"/></hp:default></hp:switch>",
            "<hh:border borderFillIDRef=\"2\" offsetLeft=\"0\" offsetRight=\"0\" offsetTop=\"0\" offsetBottom=\"0\" connect=\"0\" ignoreMargin=\"0\"/>",
            "</hh:paraPr>"
        ),
        id = id,
        numbering_id_ref = numbering_id_ref,
        left_margin = left_margin,
        default_left_margin = left_margin * 2,
    )
}
