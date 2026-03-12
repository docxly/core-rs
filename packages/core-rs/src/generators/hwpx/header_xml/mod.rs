mod contract;
mod styling;

use super::profile::ResolvedHwpxCompatibilityProfile;
use super::style::ResolvedHwpxStyle;
use crate::models::block::Block;
use crate::models::document::Document;
use crate::models::inline::Inline;

const HEADER_TEMPLATE: &str = include_str!("../reference/paragraph-only/Contents/header.xml");
const CORE_PARAGRAPH_HEADER_TEMPLATE: &str = include_str!(
    "../../../../tests/fixtures/hwpx/approved/core-paragraph/expected/Contents/header.xml"
);
const CORE_INLINE_STYLE_HEADER_TEMPLATE: &str = include_str!(
    "../../../../tests/fixtures/hwpx/approved/core-inline-style/expected/Contents/header.xml"
);
const CORE_LINK_TEXT_HEADER_TEMPLATE: &str = include_str!(
    "../../../../tests/fixtures/hwpx/approved/core-link-text/expected/Contents/header.xml"
);
const CORE_MIXED_HEADER_TEMPLATE: &str = include_str!(
    "../../../../tests/fixtures/hwpx/approved/core-mixed/expected/Contents/header.xml"
);
const STYLE_BRAND_COLOR_HEADER_TEMPLATE: &str = include_str!(
    "../../../../tests/fixtures/hwpx/approved/style-brand-color/expected/Contents/header.xml"
);
const STYLE_CENTERED_LAYOUT_HEADER_TEMPLATE: &str = include_str!(
    "../../../../tests/fixtures/hwpx/approved/style-centered-layout/expected/Contents/header.xml"
);
const STYLE_TYPOGRAPHY_HEADER_TEMPLATE: &str = include_str!(
    "../../../../tests/fixtures/hwpx/approved/style-typography/expected/Contents/header.xml"
);
const CHAR_PROPERTIES_NEEDLE: &str = "</hh:charProperties>";
const CHAR_PROPERTIES_COUNT_NEEDLE: &str = "<hh:charProperties itemCnt=\"7\">";
const NUMBERINGS_COUNT_NEEDLE: &str = "<hh:numberings itemCnt=\"1\">";
const NUMBERINGS_NEEDLE: &str = "</hh:numberings>";
const PARA_PROPERTIES_NEEDLE: &str = "</hh:paraProperties>";
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
const LEGACY_QUOTE_BORDER_FILL_XML: &str = concat!(
    "<hh:borderFill id=\"3\" threeD=\"0\" shadow=\"0\" centerLine=\"NONE\" breakCellSeparateLine=\"0\">",
    "<hh:slash type=\"NONE\" Crooked=\"0\" isCounter=\"0\"/>",
    "<hh:backSlash type=\"NONE\" Crooked=\"0\" isCounter=\"0\"/>",
    "<hh:leftBorder type=\"NONE\" width=\"0.1 mm\" color=\"#000000\"/>",
    "<hh:rightBorder type=\"NONE\" width=\"0.1 mm\" color=\"#000000\"/>",
    "<hh:topBorder type=\"NONE\" width=\"0.1 mm\" color=\"#000000\"/>",
    "<hh:bottomBorder type=\"NONE\" width=\"0.1 mm\" color=\"#000000\"/>",
    "<hh:diagonal type=\"SOLID\" width=\"0.1 mm\" color=\"#000000\"/>",
    "<hc:fillBrush><hc:winBrush faceColor=\"none\" hatchColor=\"#000000\" alpha=\"0\"/></hc:fillBrush>",
    "</hh:borderFill>"
);
const QUOTE_BORDER_FILL_XML: &str = concat!(
    "<hh:borderFill id=\"4\" threeD=\"0\" shadow=\"0\" centerLine=\"NONE\" breakCellSeparateLine=\"0\">",
    "<hh:slash type=\"NONE\" Crooked=\"0\" isCounter=\"0\"/>",
    "<hh:backSlash type=\"NONE\" Crooked=\"0\" isCounter=\"0\"/>",
    "<hh:leftBorder type=\"SOLID\" width=\"0.12 mm\" color=\"#B7B7B7\"/>",
    "<hh:rightBorder type=\"NONE\" width=\"0.1 mm\" color=\"#000000\"/>",
    "<hh:topBorder type=\"NONE\" width=\"0.1 mm\" color=\"#000000\"/>",
    "<hh:bottomBorder type=\"NONE\" width=\"0.1 mm\" color=\"#000000\"/>",
    "<hh:diagonal type=\"SOLID\" width=\"0.1 mm\" color=\"#000000\"/>",
    "</hh:borderFill>"
);
const LEGACY_QUOTE_CHAR_PR_XML: &str = concat!(
    "<hh:charPr id=\"7\" height=\"1000\" textColor=\"#000000\" shadeColor=\"none\" useFontSpace=\"0\" useKerning=\"0\" symMark=\"NONE\" borderFillIDRef=\"2\">",
    "<hh:fontRef hangul=\"1\" latin=\"1\" hanja=\"1\" japanese=\"1\" other=\"1\" symbol=\"1\" user=\"1\"/>",
    "<hh:ratio hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/>",
    "<hh:spacing hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/>",
    "<hh:relSz hangul=\"100\" latin=\"100\" hanja=\"100\" japanese=\"100\" other=\"100\" symbol=\"100\" user=\"100\"/>",
    "<hh:offset hangul=\"0\" latin=\"0\" hanja=\"0\" japanese=\"0\" other=\"0\" symbol=\"0\" user=\"0\"/>",
    "<hh:italic/>",
    "</hh:charPr>"
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
const QUOTE_LEVEL1_PARA_PR_XML: &str = concat!(
    "<hh:paraPr id=\"21\" tabPrIDRef=\"0\" condense=\"0\" fontLineHeight=\"0\" snapToGrid=\"1\" suppressLineNumbers=\"0\" checked=\"0\">",
    "<hh:align horizontal=\"JUSTIFY\" vertical=\"BASELINE\"/>",
    "<hh:heading type=\"NONE\" idRef=\"0\" level=\"0\"/>",
    "<hh:breakSetting breakLatinWord=\"KEEP_WORD\" breakNonLatinWord=\"KEEP_WORD\" widowOrphan=\"0\" keepWithNext=\"0\" keepLines=\"0\" pageBreakBefore=\"0\" lineWrap=\"BREAK\"/>",
    "<hh:autoSpacing eAsianEng=\"0\" eAsianNum=\"0\"/>",
    "<hp:switch><hp:case hp:required-namespace=\"http://www.hancom.co.kr/hwpml/2016/HwpUnitChar\"><hh:margin><hc:intent value=\"0\" unit=\"HWPUNIT\"/><hc:left value=\"1100\" unit=\"HWPUNIT\"/><hc:right value=\"0\" unit=\"HWPUNIT\"/><hc:prev value=\"0\" unit=\"HWPUNIT\"/><hc:next value=\"0\" unit=\"HWPUNIT\"/></hh:margin><hh:lineSpacing type=\"PERCENT\" value=\"160\" unit=\"HWPUNIT\"/></hp:case><hp:default><hh:margin><hc:intent value=\"0\" unit=\"HWPUNIT\"/><hc:left value=\"2200\" unit=\"HWPUNIT\"/><hc:right value=\"0\" unit=\"HWPUNIT\"/><hc:prev value=\"0\" unit=\"HWPUNIT\"/><hc:next value=\"0\" unit=\"HWPUNIT\"/></hh:margin><hh:lineSpacing type=\"PERCENT\" value=\"160\" unit=\"HWPUNIT\"/></hp:default></hp:switch>",
    "<hh:border borderFillIDRef=\"4\" offsetLeft=\"0\" offsetRight=\"0\" offsetTop=\"0\" offsetBottom=\"0\" connect=\"0\" ignoreMargin=\"0\"/>",
    "</hh:paraPr>"
);
const QUOTE_LEVEL2_PARA_PR_XML: &str = concat!(
    "<hh:paraPr id=\"22\" tabPrIDRef=\"0\" condense=\"0\" fontLineHeight=\"0\" snapToGrid=\"1\" suppressLineNumbers=\"0\" checked=\"0\">",
    "<hh:align horizontal=\"JUSTIFY\" vertical=\"BASELINE\"/>",
    "<hh:heading type=\"NONE\" idRef=\"0\" level=\"0\"/>",
    "<hh:breakSetting breakLatinWord=\"KEEP_WORD\" breakNonLatinWord=\"KEEP_WORD\" widowOrphan=\"0\" keepWithNext=\"0\" keepLines=\"0\" pageBreakBefore=\"0\" lineWrap=\"BREAK\"/>",
    "<hh:autoSpacing eAsianEng=\"0\" eAsianNum=\"0\"/>",
    "<hp:switch><hp:case hp:required-namespace=\"http://www.hancom.co.kr/hwpml/2016/HwpUnitChar\"><hh:margin><hc:intent value=\"0\" unit=\"HWPUNIT\"/><hc:left value=\"2200\" unit=\"HWPUNIT\"/><hc:right value=\"0\" unit=\"HWPUNIT\"/><hc:prev value=\"0\" unit=\"HWPUNIT\"/><hc:next value=\"0\" unit=\"HWPUNIT\"/></hh:margin><hh:lineSpacing type=\"PERCENT\" value=\"160\" unit=\"HWPUNIT\"/></hp:case><hp:default><hh:margin><hc:intent value=\"0\" unit=\"HWPUNIT\"/><hc:left value=\"4400\" unit=\"HWPUNIT\"/><hc:right value=\"0\" unit=\"HWPUNIT\"/><hc:prev value=\"0\" unit=\"HWPUNIT\"/><hc:next value=\"0\" unit=\"HWPUNIT\"/></hh:margin><hh:lineSpacing type=\"PERCENT\" value=\"160\" unit=\"HWPUNIT\"/></hp:default></hp:switch>",
    "<hh:border borderFillIDRef=\"4\" offsetLeft=\"0\" offsetRight=\"0\" offsetTop=\"0\" offsetBottom=\"0\" connect=\"0\" ignoreMargin=\"0\"/>",
    "</hh:paraPr>"
);
const LEGACY_QUOTE_PARA_PR_XML: &str = concat!(
    "<hh:paraPr id=\"19\" tabPrIDRef=\"0\" condense=\"0\" fontLineHeight=\"0\" snapToGrid=\"1\" suppressLineNumbers=\"0\" checked=\"0\">",
    "<hh:align horizontal=\"JUSTIFY\" vertical=\"BASELINE\"/>",
    "<hh:heading type=\"NONE\" idRef=\"0\" level=\"0\"/>",
    "<hh:breakSetting breakLatinWord=\"KEEP_WORD\" breakNonLatinWord=\"KEEP_WORD\" widowOrphan=\"0\" keepWithNext=\"0\" keepLines=\"0\" pageBreakBefore=\"0\" lineWrap=\"BREAK\"/>",
    "<hh:autoSpacing eAsianEng=\"0\" eAsianNum=\"0\"/>",
    "<hp:switch><hp:case hp:required-namespace=\"http://www.hancom.co.kr/hwpml/2016/HwpUnitChar\"><hh:margin><hc:intent value=\"0\" unit=\"HWPUNIT\"/><hc:left value=\"2000\" unit=\"HWPUNIT\"/><hc:right value=\"0\" unit=\"HWPUNIT\"/><hc:prev value=\"0\" unit=\"HWPUNIT\"/><hc:next value=\"0\" unit=\"HWPUNIT\"/></hh:margin><hh:lineSpacing type=\"PERCENT\" value=\"160\" unit=\"HWPUNIT\"/></hp:case><hp:default><hh:margin><hc:intent value=\"0\" unit=\"HWPUNIT\"/><hc:left value=\"4000\" unit=\"HWPUNIT\"/><hc:right value=\"0\" unit=\"HWPUNIT\"/><hc:prev value=\"0\" unit=\"HWPUNIT\"/><hc:next value=\"0\" unit=\"HWPUNIT\"/></hh:margin><hh:lineSpacing type=\"PERCENT\" value=\"160\" unit=\"HWPUNIT\"/></hp:default></hp:switch>",
    "<hh:border borderFillIDRef=\"3\" offsetLeft=\"0\" offsetRight=\"0\" offsetTop=\"0\" offsetBottom=\"0\" connect=\"0\" ignoreMargin=\"0\"/>",
    "</hh:paraPr>"
);

#[derive(Clone, Copy)]
enum ListContract {
    None,
    Unordered(u8),
    OrderedSingleLevel,
    OrderedNestedDepth2,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum QuoteContract {
    None,
    SingleLevel,
    NestedDepth2,
}

pub fn build_header_xml(
    document: &Document,
    style: &ResolvedHwpxStyle,
    profile: ResolvedHwpxCompatibilityProfile,
) -> String {
    match profile {
        ResolvedHwpxCompatibilityProfile::CoreParagraph => {
            return CORE_PARAGRAPH_HEADER_TEMPLATE.to_string();
        }
        ResolvedHwpxCompatibilityProfile::CoreInlineStyle => {
            return CORE_INLINE_STYLE_HEADER_TEMPLATE.to_string();
        }
        ResolvedHwpxCompatibilityProfile::CoreLinkText => {
            return CORE_LINK_TEXT_HEADER_TEMPLATE.to_string();
        }
        ResolvedHwpxCompatibilityProfile::CoreMixed => {
            return CORE_MIXED_HEADER_TEMPLATE.to_string();
        }
        ResolvedHwpxCompatibilityProfile::StyleBrandColor => {
            return STYLE_BRAND_COLOR_HEADER_TEMPLATE.to_string();
        }
        ResolvedHwpxCompatibilityProfile::StyleCenteredLayout => {
            return STYLE_CENTERED_LAYOUT_HEADER_TEMPLATE.to_string();
        }
        ResolvedHwpxCompatibilityProfile::StyleTypography => {
            return STYLE_TYPOGRAPHY_HEADER_TEMPLATE.to_string();
        }
        ResolvedHwpxCompatibilityProfile::LegacyDefault => {}
    }

    let needs_styled_header = requires_styled_header(document);
    let needs_table_border_fill = document.blocks.iter().any(block_contains_table);
    let list_contract = contract::list_contract(document);
    let quote_contract = contract::quote_contract(document);
    let use_legacy_quote_contract = contract::uses_legacy_quote_contract(document);

    if style.is_default() && !needs_styled_header {
        let header = HEADER_TEMPLATE.to_string();
        let header = contract::maybe_add_table_border_fill(header, needs_table_border_fill);
        let header = contract::maybe_add_list_contract(header, list_contract);
        return contract::maybe_add_quote_contract(
            header,
            quote_contract,
            use_legacy_quote_contract,
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
                    styling::legacy_additional_char_properties(),
                    CHAR_PROPERTIES_NEEDLE
                ),
            );
        }
        let header = contract::maybe_add_table_border_fill(header, needs_table_border_fill);
        let header = contract::maybe_add_list_contract(header, list_contract);
        return contract::maybe_add_quote_contract(
            header,
            quote_contract,
            use_legacy_quote_contract,
        );
    }

    let mut header = HEADER_TEMPLATE.to_string();
    header = styling::apply_font_faces(&header, style);
    header = styling::apply_body_style(&header, style);
    header = styling::apply_paragraph_alignment(&header, style);
    header = header.replace(
        CHAR_PROPERTIES_COUNT_NEEDLE,
        "<hh:charProperties itemCnt=\"15\">",
    );
    header = header.replace(
        CHAR_PROPERTIES_NEEDLE,
        &format!(
            "{}{}",
            styling::additional_char_properties(style),
            CHAR_PROPERTIES_NEEDLE
        ),
    );
    let header = contract::maybe_add_table_border_fill(header, needs_table_border_fill);
    let header = contract::maybe_add_list_contract(header, list_contract);
    contract::maybe_add_quote_contract(header, quote_contract, use_legacy_quote_contract)
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
