use super::super::document_shape::{
    max_blockquote_depth, max_ordered_list_depth, max_unordered_list_depth,
    uses_legacy_quote_only_contract,
};
use super::{
    BORDER_FILLS_NEEDLE, CHAR_PROPERTIES_NEEDLE, Document, LEGACY_QUOTE_BORDER_FILL_XML,
    LEGACY_QUOTE_CHAR_PR_XML, LEGACY_QUOTE_PARA_PR_XML, LIST_BULLETS_XML, LIST_LEVEL1_PARA_PR_XML,
    LIST_LEVEL2_PARA_PR_XML, ListContract, NUMBERINGS_COUNT_NEEDLE, NUMBERINGS_NEEDLE,
    PARA_PROPERTIES_NEEDLE, QUOTE_BORDER_FILL_XML, QUOTE_LEVEL1_PARA_PR_XML,
    QUOTE_LEVEL2_PARA_PR_XML, QuoteContract, TABLE_BORDER_FILL_XML,
};

pub(super) fn list_contract(document: &Document) -> ListContract {
    let ordered_depth = max_ordered_list_depth(&document.blocks);
    if ordered_depth > 0 {
        return if ordered_depth > 1 {
            ListContract::OrderedNestedDepth2
        } else {
            ListContract::OrderedSingleLevel
        };
    }

    let unordered_depth = max_unordered_list_depth(&document.blocks);
    if unordered_depth > 0 {
        ListContract::Unordered(unordered_depth)
    } else {
        ListContract::None
    }
}

pub(super) fn quote_contract(document: &Document) -> QuoteContract {
    match max_blockquote_depth(&document.blocks) {
        0 => QuoteContract::None,
        1 => QuoteContract::SingleLevel,
        _ => QuoteContract::NestedDepth2,
    }
}

pub(super) fn uses_legacy_quote_contract(document: &Document) -> bool {
    uses_legacy_quote_only_contract(&document.blocks)
}

pub(super) fn maybe_add_table_border_fill(header: String, needs_table_border_fill: bool) -> String {
    if !needs_table_border_fill || header.contains("<hh:borderFill id=\"3\"") {
        return header;
    }

    let header = increment_item_count(&header, "<hh:borderFills itemCnt=\"", 1);
    header.replace(
        BORDER_FILLS_NEEDLE,
        &format!("{TABLE_BORDER_FILL_XML}{BORDER_FILLS_NEEDLE}"),
    )
}

pub(super) fn maybe_add_list_contract(header: String, contract: ListContract) -> String {
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

    let list_para_properties = if list_depth > 1 {
        format!("{LIST_LEVEL1_PARA_PR_XML}{LIST_LEVEL2_PARA_PR_XML}")
    } else {
        LIST_LEVEL1_PARA_PR_XML.to_string()
    };
    let para_increment = if list_depth > 1 { 2 } else { 1 };

    increment_item_count(&header, "<hh:paraProperties itemCnt=\"", para_increment)
        .replacen(
            NUMBERINGS_NEEDLE,
            &format!("{NUMBERINGS_NEEDLE}{LIST_BULLETS_XML}"),
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
    increment_item_count(
        &header,
        "<hh:paraProperties itemCnt=\"",
        if nested { 2 } else { 1 },
    )
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
        PARA_PROPERTIES_NEEDLE,
        &format!("{para_properties}{PARA_PROPERTIES_NEEDLE}"),
        1,
    )
}

pub(super) fn maybe_add_quote_contract(
    header: String,
    contract: QuoteContract,
    use_legacy_quote_contract: bool,
) -> String {
    if use_legacy_quote_contract && contract == QuoteContract::SingleLevel {
        return maybe_add_legacy_quote_contract(header);
    }

    match contract {
        QuoteContract::None => header,
        QuoteContract::SingleLevel => maybe_add_quote_paragraphs(header, 1),
        QuoteContract::NestedDepth2 => maybe_add_quote_paragraphs(header, 2),
    }
}

fn maybe_add_legacy_quote_contract(header: String) -> String {
    if header.contains("<hh:paraPr id=\"19\"") {
        return header;
    }

    let header = increment_item_count(&header, "<hh:borderFills itemCnt=\"", 1).replace(
        BORDER_FILLS_NEEDLE,
        &format!("{LEGACY_QUOTE_BORDER_FILL_XML}{BORDER_FILLS_NEEDLE}"),
    );
    let header = increment_item_count(&header, "<hh:charProperties itemCnt=\"", 1).replacen(
        CHAR_PROPERTIES_NEEDLE,
        &format!("{LEGACY_QUOTE_CHAR_PR_XML}{CHAR_PROPERTIES_NEEDLE}"),
        1,
    );

    increment_item_count(&header, "<hh:paraProperties itemCnt=\"", 1).replacen(
        PARA_PROPERTIES_NEEDLE,
        &format!("{LEGACY_QUOTE_PARA_PR_XML}{PARA_PROPERTIES_NEEDLE}"),
        1,
    )
}

fn maybe_add_quote_paragraphs(header: String, quote_depth: u8) -> String {
    if header.contains("<hh:paraPr id=\"21\"") {
        return header;
    }

    let para_properties = if quote_depth > 1 {
        format!("{QUOTE_LEVEL1_PARA_PR_XML}{QUOTE_LEVEL2_PARA_PR_XML}")
    } else {
        QUOTE_LEVEL1_PARA_PR_XML.to_string()
    };
    let header = if header.contains("<hh:borderFill id=\"4\"") {
        header
    } else {
        increment_item_count(&header, "<hh:borderFills itemCnt=\"", 1).replace(
            BORDER_FILLS_NEEDLE,
            &format!("{QUOTE_BORDER_FILL_XML}{BORDER_FILLS_NEEDLE}"),
        )
    };

    increment_item_count(
        &header,
        "<hh:paraProperties itemCnt=\"",
        if quote_depth > 1 { 2 } else { 1 },
    )
    .replacen(
        PARA_PROPERTIES_NEEDLE,
        &format!("{para_properties}{PARA_PROPERTIES_NEEDLE}"),
        1,
    )
}

fn increment_item_count(xml: &str, prefix: &str, delta: u32) -> String {
    let Some(start) = xml.find(prefix) else {
        return xml.to_string();
    };
    let count_start = start + prefix.len();
    let Some(rel_end) = xml[count_start..].find('"') else {
        return xml.to_string();
    };
    let count_end = count_start + rel_end;
    let Ok(current) = xml[count_start..count_end].parse::<u32>() else {
        return xml.to_string();
    };

    let mut updated = String::with_capacity(xml.len());
    updated.push_str(&xml[..count_start]);
    updated.push_str(&(current + delta).to_string());
    updated.push_str(&xml[count_end..]);
    updated
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
