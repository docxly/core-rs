mod collect;
mod compat;
mod render;
mod shared;
mod text;

use crate::error::CoreRsError;
use crate::generators::hwpx::document_shape::uses_legacy_quote_only_contract as uses_legacy_quote_only_shape;
use crate::models::block::{Block, ListBlock, TableBlock};
use crate::models::document::Document;
use crate::models::inline::Inline;
use crate::utils::xml_helper::escape_text;

use super::package_xml::{set_settings_caret, set_settings_caret_pos};
use super::profile::ResolvedHwpxCompatibilityProfile;
use super::style::ResolvedHwpxStyle;

const NORMAL_CHAR_PR: u32 = 6;
const BOLD_CHAR_PR: u32 = 7;
const ITALIC_CHAR_PR: u32 = 8;
const LEGACY_QUOTE_CHAR_PR: u32 = 7;
const CODE_CHAR_PR: u32 = 9;
const H1_CHAR_PR: u32 = 10;
const H2_CHAR_PR: u32 = 11;
const H3_CHAR_PR: u32 = 12;
const H4_CHAR_PR: u32 = 13;
const LINK_CHAR_PR: u32 = 14;
const LIST_LEVEL1_PARA_PR: u32 = 19;
const LIST_LEVEL2_PARA_PR: u32 = 20;
const QUOTE_LEVEL1_PARA_PR: u32 = 21;
const QUOTE_LEVEL2_PARA_PR: u32 = 22;
const LIST_LINESEG_INCREMENT: u32 = 2_300;
const LIST_LINESEG_VERT_SIZE: u32 = 1_000;
const LIST_LINESEG_TEXT_HEIGHT: u32 = 1_000;
const LIST_LINESEG_BASELINE: u32 = 850;
const LIST_LINESEG_SPACING: u32 = 600;
const LIST_LINESEG_FLAGS: u32 = 2_490_368;
const LIST_LEVEL1_HORZ_POS: u32 = 1_100;
const LIST_LEVEL1_HORZ_SIZE: u32 = 41_420;
const LIST_LEVEL2_HORZ_POS: u32 = 2_200;
const LIST_LEVEL2_HORZ_SIZE: u32 = 40_320;
const QUOTE_LEVEL1_HORZ_POS: u32 = 1_100;
const QUOTE_LEVEL1_HORZ_SIZE: u32 = 41_420;
const QUOTE_LEVEL2_HORZ_POS: u32 = 2_200;
const QUOTE_LEVEL2_HORZ_SIZE: u32 = 40_320;
const LEGACY_QUOTE_PARA_PR: u32 = 19;
const LEGACY_QUOTE_LINESEG_INCREMENT: u32 = 1_600;
const LEGACY_QUOTE_HORZ_POS: u32 = 2_000;
const LEGACY_QUOTE_HORZ_SIZE: u32 = 40_520;

const TABLE_ID_SEED: u64 = 2_104_760_845;
const TABLE_WIDTH: u32 = 41_954;
const TABLE_OUTER_LINESEG_SIZE: u32 = 4_412;
const TABLE_OUTER_BASELINE: u32 = 3_750;
const TABLE_ROW_HEIGHT: u32 = 282;
const TABLE_HEIGHT_PER_ROW: u32 = 1_282;
const TABLE_CELL_INNER_WIDTH_DELTA: u32 = 1_021;

#[derive(Clone, Copy)]
struct ParagraphStyle {
    para_pr: u32,
    style: u32,
    default_char_pr: u32,
    line_seg: LineSegProfile,
    list_semantic: ListSemantic,
}

#[derive(Clone, Copy)]
enum LineSegProfile {
    Standard,
    ListLevel1,
    ListLevel2,
    QuoteLegacy,
    QuoteLevel1,
    QuoteLevel2,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum QuoteRenderMode {
    Modern,
    LegacyQuoteOnly,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ListSemantic {
    None,
    Unordered,
    OrderedSingleLevel,
    OrderedTopLevel,
    OrderedNestedLevel,
}

#[derive(Clone, Copy)]
enum OrderedListShape {
    SingleLevel,
    NestedDepth2,
}

#[derive(Clone)]
enum RunSpec {
    Text {
        char_pr: u32,
        text: String,
    },
    Hyperlink {
        char_pr: u32,
        field_begin_id: u64,
        field_id: u64,
        url: String,
        text: String,
    },
}

enum SectionItem {
    Paragraph(ParagraphStyle, Vec<RunSpec>),
    Table(TableSpec),
}

struct TableSpec {
    id: u64,
    rows: Vec<Vec<String>>,
    col_widths: Vec<u32>,
}

pub fn build_section_xml(
    document: &Document,
    strict_mode: bool,
    style: &ResolvedHwpxStyle,
    profile: ResolvedHwpxCompatibilityProfile,
) -> Result<String, CoreRsError> {
    if profile != ResolvedHwpxCompatibilityProfile::LegacyDefault {
        return compat::build_section_xml(document, profile, style);
    }
    let quote_render_mode = quote_render_mode(document);
    let ordered_shape = collect::ordered_list_shape(&document.blocks);
    let items = collect::collect_section_items(
        &document.blocks,
        strict_mode,
        ordered_shape,
        quote_render_mode,
    )?;
    let mut body = String::new();
    let mut list_vertpos = 0;
    let mut quote_vertpos = 0;
    for (index, item) in items.iter().enumerate() {
        body.push_str(&render::render_section_item(
            item,
            index == 0,
            index,
            &mut list_vertpos,
            &mut quote_vertpos,
        ));
    }

    Ok(format!(
        concat!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\" ?>",
            "<hs:sec xmlns:ha=\"http://www.hancom.co.kr/hwpml/2011/app\" xmlns:hp=\"http://www.hancom.co.kr/hwpml/2011/paragraph\" xmlns:hp10=\"http://www.hancom.co.kr/hwpml/2016/paragraph\" xmlns:hs=\"http://www.hancom.co.kr/hwpml/2011/section\" xmlns:hc=\"http://www.hancom.co.kr/hwpml/2011/core\" xmlns:hh=\"http://www.hancom.co.kr/hwpml/2011/head\" xmlns:hhs=\"http://www.hancom.co.kr/hwpml/2011/history\" xmlns:hm=\"http://www.hancom.co.kr/hwpml/2011/master-page\" xmlns:hpf=\"http://www.hancom.co.kr/schema/2011/hpf\" xmlns:dc=\"http://purl.org/dc/elements/1.1/\" xmlns:opf=\"http://www.idpf.org/2007/opf/\" xmlns:ooxmlchart=\"http://www.hancom.co.kr/hwpml/2016/ooxmlchart\" xmlns:hwpunitchar=\"http://www.hancom.co.kr/hwpml/2016/HwpUnitChar\" xmlns:epub=\"http://www.idpf.org/2007/ops\" xmlns:config=\"urn:oasis:names:tc:opendocument:xmlns:config:1.0\">",
            "{body}",
            "</hs:sec>"
        ),
        body = body
    ))
}

pub fn build_preview_text(
    document: &Document,
    strict_mode: bool,
    profile: ResolvedHwpxCompatibilityProfile,
    style: &ResolvedHwpxStyle,
) -> Result<String, CoreRsError> {
    if profile != ResolvedHwpxCompatibilityProfile::LegacyDefault {
        return compat::build_preview_text(document, profile, style);
    }
    let quote_render_mode = quote_render_mode(document);
    let ordered_shape = collect::ordered_list_shape(&document.blocks);
    let items = collect::collect_section_items(
        &document.blocks,
        strict_mode,
        ordered_shape,
        quote_render_mode,
    )?;
    let has_table = items
        .iter()
        .any(|item| matches!(item, SectionItem::Table(_)));
    let text = text::visible_paragraphs(&items).join("\r\n");
    if has_table {
        set_settings_caret_pos(16);
    } else if quote_render_mode == QuoteRenderMode::LegacyQuoteOnly {
        if let Some((para_id_ref, pos)) = text::last_paragraph_caret(&items) {
            set_settings_caret(para_id_ref, pos);
        } else {
            set_settings_caret_pos(45);
        }
    } else if let Some((para_id_ref, pos)) = text::list_caret_position(&items) {
        set_settings_caret(para_id_ref, pos);
    } else {
        set_settings_caret_pos(45);
    }
    Ok(text)
}

fn quote_render_mode(document: &Document) -> QuoteRenderMode {
    if uses_legacy_quote_only_shape(&document.blocks) {
        QuoteRenderMode::LegacyQuoteOnly
    } else {
        QuoteRenderMode::Modern
    }
}
