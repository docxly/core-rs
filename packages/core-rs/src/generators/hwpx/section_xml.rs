use crate::error::CoreRsError;
use crate::models::block::{Block, ListBlock, TableBlock};
use crate::models::document::Document;
use crate::models::inline::Inline;
use crate::utils::xml_helper::escape_text;

use super::package_xml::{set_settings_caret, set_settings_caret_pos};
use super::style::ResolvedHwpxStyle;

const NORMAL_CHAR_PR: u32 = 6;
const BOLD_CHAR_PR: u32 = 7;
const ITALIC_CHAR_PR: u32 = 8;
const CODE_CHAR_PR: u32 = 9;
const H1_CHAR_PR: u32 = 10;
const H2_CHAR_PR: u32 = 11;
const H3_CHAR_PR: u32 = 12;
const H4_CHAR_PR: u32 = 13;
const LINK_CHAR_PR: u32 = 14;
const LIST_LEVEL1_PARA_PR: u32 = 19;
const LIST_LEVEL2_PARA_PR: u32 = 20;
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

struct RunSpec {
    char_pr: u32,
    text: String,
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
    _style: &ResolvedHwpxStyle,
) -> Result<String, CoreRsError> {
    let ordered_shape = ordered_list_shape(&document.blocks);
    let items = collect_section_items(&document.blocks, strict_mode, ordered_shape)?;
    let mut body = String::new();
    let mut list_vertpos = 0;
    for (index, item) in items.iter().enumerate() {
        body.push_str(&render_section_item(
            item,
            index == 0,
            index,
            &mut list_vertpos,
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

pub fn build_preview_text(document: &Document, strict_mode: bool) -> Result<String, CoreRsError> {
    let ordered_shape = ordered_list_shape(&document.blocks);
    let items = collect_section_items(&document.blocks, strict_mode, ordered_shape)?;
    let has_table = items
        .iter()
        .any(|item| matches!(item, SectionItem::Table(_)));
    let text = visible_paragraphs(&items).join("\r\n");
    if has_table {
        set_settings_caret_pos(16);
    } else if let Some((para_id_ref, pos)) = list_caret_position(&items) {
        set_settings_caret(para_id_ref, pos);
    } else {
        set_settings_caret_pos(45);
    }
    Ok(text)
}

fn collect_section_items(
    blocks: &[Block],
    strict_mode: bool,
    ordered_shape: Option<OrderedListShape>,
) -> Result<Vec<SectionItem>, CoreRsError> {
    let mut items = Vec::new();
    let mut next_table_id = TABLE_ID_SEED;

    for block in blocks {
        match block {
            Block::Paragraph(inlines) => items.push(SectionItem::Paragraph(
                normal_style(),
                render_runs(inlines, NORMAL_CHAR_PR, strict_mode)?,
            )),
            Block::Heading { level, content } => items.push(SectionItem::Paragraph(
                heading_style(*level),
                render_runs(content, heading_default_char_pr(*level), strict_mode)?,
            )),
            Block::List(list) => {
                if list.ordered {
                    if strict_mode {
                        return Err(CoreRsError::UnsupportedFeature(
                            "HWPX approved baseline does not support ordered list".to_string(),
                        ));
                    }
                    if let Some(shape) = ordered_shape {
                        collect_ordered_list_items(list, strict_mode, 1, shape, &mut items)?;
                    } else {
                        for paragraph in flatten_block_to_paragraphs(block) {
                            items.push(plain_paragraph_item(paragraph));
                        }
                    }
                } else {
                    collect_unordered_list_items(list, strict_mode, 1, &mut items)?;
                }
            }
            Block::Table(table) => {
                items.push(SectionItem::Table(build_table_spec(
                    table,
                    strict_mode,
                    next_table_id,
                )?));
                next_table_id += 1;
            }
            unsupported => {
                if strict_mode {
                    return Err(CoreRsError::UnsupportedFeature(format!(
                        "HWPX core does not support {0}",
                        unsupported_name(unsupported)
                    )));
                }
                for paragraph in flatten_block_to_paragraphs(unsupported) {
                    items.push(plain_paragraph_item(paragraph));
                }
            }
        }
    }

    Ok(items)
}

fn collect_unordered_list_items(
    list: &ListBlock,
    strict_mode: bool,
    depth: u8,
    items: &mut Vec<SectionItem>,
) -> Result<(), CoreRsError> {
    if depth > 2 {
        if strict_mode {
            return Err(CoreRsError::UnsupportedFeature(
                "HWPX approved baseline supports nested unordered lists up to depth 2".to_string(),
            ));
        }
        for paragraph in flatten_block_to_paragraphs(&Block::List(list.clone())) {
            items.push(plain_paragraph_item(paragraph));
        }
        return Ok(());
    }

    let list_style = unordered_list_style(depth);
    for item in &list.items {
        for block in &item.blocks {
            match block {
                Block::Paragraph(inlines) => {
                    let runs = render_runs(inlines, list_style.default_char_pr, strict_mode)?;
                    items.push(SectionItem::Paragraph(list_style, runs));
                }
                Block::Heading { content, .. } => {
                    let runs = render_runs(content, list_style.default_char_pr, strict_mode)?;
                    items.push(SectionItem::Paragraph(list_style, runs));
                }
                Block::List(child) if !child.ordered => {
                    collect_unordered_list_items(child, strict_mode, depth + 1, items)?;
                }
                nested_list @ Block::List(_) => {
                    if strict_mode {
                        return Err(CoreRsError::UnsupportedFeature(
                            "HWPX approved baseline does not support ordered list".to_string(),
                        ));
                    }
                    for paragraph in flatten_block_to_paragraphs(nested_list) {
                        items.push(plain_paragraph_item(paragraph));
                    }
                }
                unsupported => {
                    if strict_mode {
                        return Err(CoreRsError::UnsupportedFeature(format!(
                            "HWPX approved baseline does not support {0} inside unordered list",
                            unsupported_name(unsupported)
                        )));
                    }
                    for paragraph in flatten_block_to_paragraphs(unsupported) {
                        items.push(plain_paragraph_item(paragraph));
                    }
                }
            }
        }
    }

    Ok(())
}

fn collect_ordered_list_items(
    list: &ListBlock,
    strict_mode: bool,
    depth: u8,
    shape: OrderedListShape,
    items: &mut Vec<SectionItem>,
) -> Result<(), CoreRsError> {
    if depth > 2 {
        for paragraph in flatten_block_to_paragraphs(&Block::List(list.clone())) {
            items.push(plain_paragraph_item(paragraph));
        }
        return Ok(());
    }

    let list_style = ordered_list_style(shape, depth);
    for item in &list.items {
        for block in &item.blocks {
            match block {
                Block::Paragraph(inlines) => {
                    let runs = render_runs(inlines, list_style.default_char_pr, strict_mode)?;
                    items.push(SectionItem::Paragraph(list_style, runs));
                }
                Block::Heading { content, .. } => {
                    let runs = render_runs(content, list_style.default_char_pr, strict_mode)?;
                    items.push(SectionItem::Paragraph(list_style, runs));
                }
                Block::List(child) if child.ordered => {
                    collect_ordered_list_items(child, strict_mode, depth + 1, shape, items)?;
                }
                nested_list @ Block::List(_) => {
                    for paragraph in flatten_block_to_paragraphs(nested_list) {
                        items.push(plain_paragraph_item(paragraph));
                    }
                }
                unsupported => {
                    for paragraph in flatten_block_to_paragraphs(unsupported) {
                        items.push(plain_paragraph_item(paragraph));
                    }
                }
            }
        }
    }

    Ok(())
}

fn build_table_spec(
    table: &TableBlock,
    strict_mode: bool,
    id: u64,
) -> Result<TableSpec, CoreRsError> {
    let mut rows = Vec::new();
    if !table.headers.is_empty() {
        rows.push(
            table
                .headers
                .iter()
                .map(|cell| flatten_inline_children(cell, strict_mode))
                .collect::<Result<Vec<_>, _>>()?,
        );
    }
    for row in &table.rows {
        rows.push(
            row.cells
                .iter()
                .map(|cell| flatten_inline_children(cell, strict_mode))
                .collect::<Result<Vec<_>, _>>()?,
        );
    }

    let col_count = rows.iter().map(Vec::len).max().unwrap_or(0);
    let col_widths = equal_widths(col_count);
    for row in &mut rows {
        row.resize(col_count, String::new());
    }

    Ok(TableSpec {
        id,
        rows,
        col_widths,
    })
}

fn render_section_item(
    item: &SectionItem,
    is_first: bool,
    index: usize,
    list_vertpos: &mut u32,
) -> String {
    match item {
        SectionItem::Paragraph(style, runs) => {
            render_paragraph(style, runs, is_first, index, list_vertpos)
        }
        SectionItem::Table(table) => render_table_paragraph(table, is_first, index),
    }
}

fn render_paragraph(
    style: &ParagraphStyle,
    runs: &[RunSpec],
    is_first: bool,
    index: usize,
    list_vertpos: &mut u32,
) -> String {
    let id = 2_757_524_817u64 + index as u64;
    let mut xml = format!(
        "<hp:p id=\"{id}\" paraPrIDRef=\"{para}\" styleIDRef=\"{style_id}\" pageBreak=\"0\" columnBreak=\"0\" merged=\"0\">",
        id = id,
        para = style.para_pr,
        style_id = style.style,
    );

    if is_first {
        xml.push_str(&render_section_preamble_run(style.default_char_pr));
    }

    if runs.is_empty() {
        xml.push_str(&format!(
            "<hp:run charPrIDRef=\"{}\"><hp:t></hp:t></hp:run>",
            style.default_char_pr
        ));
    } else {
        for run in runs {
            xml.push_str(&format!(
                "<hp:run charPrIDRef=\"{char_pr}\"><hp:t>{text}</hp:t></hp:run>",
                char_pr = run.char_pr,
                text = escape_text(&run.text)
            ));
        }
    }

    xml.push_str(&render_lineseg(style.line_seg, list_vertpos));
    xml.push_str("</hp:p>");
    xml
}

fn render_table_paragraph(table: &TableSpec, is_first: bool, index: usize) -> String {
    let id = 2_757_524_817u64 + index as u64;
    let mut xml = format!(
        "<hp:p id=\"{id}\" paraPrIDRef=\"0\" styleIDRef=\"0\" pageBreak=\"0\" columnBreak=\"0\" merged=\"0\">",
        id = id,
    );

    if is_first {
        xml.push_str(&render_section_preamble_run(NORMAL_CHAR_PR));
    }
    xml.push_str("<hp:run charPrIDRef=\"6\">");
    xml.push_str(&render_table_xml(table));
    xml.push_str("<hp:t/></hp:run>");
    xml.push_str(&render_table_lineseg());
    xml.push_str("</hp:p>");
    xml
}

fn render_table_xml(table: &TableSpec) -> String {
    let row_count = table.rows.len();
    let col_count = table.col_widths.len();
    let height = TABLE_HEIGHT_PER_ROW * row_count as u32;

    let mut xml = format!(
        concat!(
            "<hp:tbl id=\"{id}\" zOrder=\"0\" numberingType=\"TABLE\" textWrap=\"TOP_AND_BOTTOM\" textFlow=\"BOTH_SIDES\" lock=\"0\" dropcapstyle=\"None\" pageBreak=\"CELL\" repeatHeader=\"1\" rowCnt=\"{row_count}\" colCnt=\"{col_count}\" cellSpacing=\"0\" borderFillIDRef=\"3\" noAdjust=\"0\">",
            "<hp:sz width=\"{width}\" widthRelTo=\"ABSOLUTE\" height=\"{height}\" heightRelTo=\"ABSOLUTE\" protect=\"0\"/>",
            "<hp:pos treatAsChar=\"1\" affectLSpacing=\"0\" flowWithText=\"1\" allowOverlap=\"0\" holdAnchorAndSO=\"0\" vertRelTo=\"PARA\" horzRelTo=\"COLUMN\" vertAlign=\"TOP\" horzAlign=\"LEFT\" vertOffset=\"0\" horzOffset=\"0\"/>",
            "<hp:outMargin left=\"283\" right=\"283\" top=\"283\" bottom=\"283\"/>",
            "<hp:inMargin left=\"510\" right=\"510\" top=\"141\" bottom=\"141\"/>"
        ),
        id = table.id,
        row_count = row_count,
        col_count = col_count,
        width = TABLE_WIDTH,
        height = height,
    );

    for (row_index, row) in table.rows.iter().enumerate() {
        xml.push_str("<hp:tr>");
        for (col_index, cell) in row.iter().enumerate() {
            xml.push_str(&render_table_cell(
                cell,
                row_index,
                col_index,
                table.col_widths[col_index],
            ));
        }
        xml.push_str("</hp:tr>");
    }

    xml.push_str("</hp:tbl>");
    xml
}

fn render_table_cell(text: &str, row_index: usize, col_index: usize, width: u32) -> String {
    let inner_width = width.saturating_sub(TABLE_CELL_INNER_WIDTH_DELTA);
    format!(
        concat!(
            "<hp:tc name=\"\" header=\"0\" hasMargin=\"0\" protect=\"0\" editable=\"0\" dirty=\"0\" borderFillIDRef=\"3\">",
            "<hp:subList id=\"\" textDirection=\"HORIZONTAL\" lineWrap=\"BREAK\" vertAlign=\"CENTER\" linkListIDRef=\"0\" linkListNextIDRef=\"0\" textWidth=\"0\" textHeight=\"0\" hasTextRef=\"0\" hasNumRef=\"0\">",
            "<hp:p id=\"0\" paraPrIDRef=\"0\" styleIDRef=\"0\" pageBreak=\"0\" columnBreak=\"0\" merged=\"0\">",
            "<hp:run charPrIDRef=\"6\"><hp:t>{text}</hp:t></hp:run>",
            "{trailing_run}",
            "<hp:linesegarray><hp:lineseg textpos=\"0\" vertpos=\"0\" vertsize=\"1000\" textheight=\"1000\" baseline=\"850\" spacing=\"600\" horzpos=\"0\" horzsize=\"{inner_width}\" flags=\"393216\"/></hp:linesegarray>",
            "</hp:p></hp:subList>",
            "<hp:cellAddr colAddr=\"{col_index}\" rowAddr=\"{row_index}\"/>",
            "<hp:cellSpan colSpan=\"1\" rowSpan=\"1\"/>",
            "<hp:cellSz width=\"{width}\" height=\"{height}\"/>",
            "<hp:cellMargin left=\"510\" right=\"510\" top=\"141\" bottom=\"141\"/>",
            "</hp:tc>"
        ),
        text = escape_text(text),
        trailing_run = if col_index > 0 {
            "<hp:run charPrIDRef=\"6\"/>"
        } else {
            ""
        },
        inner_width = inner_width,
        col_index = col_index,
        row_index = row_index,
        width = width,
        height = TABLE_ROW_HEIGHT,
    )
}

fn render_section_preamble_run(char_pr: u32) -> String {
    format!(
        "<hp:run charPrIDRef=\"{char_pr}\">{body}</hp:run>",
        char_pr = char_pr,
        body = render_section_preamble_body(),
    )
}

fn render_section_preamble_body() -> &'static str {
    concat!(
        "<hp:secPr id=\"\" textDirection=\"HORIZONTAL\" spaceColumns=\"1134\" tabStop=\"8000\" tabStopVal=\"4000\" tabStopUnit=\"HWPUNIT\" outlineShapeIDRef=\"1\" memoShapeIDRef=\"0\" textVerticalWidthHead=\"0\" masterPageCnt=\"0\">",
        "<hp:grid lineGrid=\"0\" charGrid=\"0\" wonggojiFormat=\"0\"/>",
        "<hp:startNum pageStartsOn=\"BOTH\" page=\"0\" pic=\"0\" tbl=\"0\" equation=\"0\"/>",
        "<hp:visibility hideFirstHeader=\"0\" hideFirstFooter=\"0\" hideFirstMasterPage=\"0\" border=\"SHOW_ALL\" fill=\"SHOW_ALL\" hideFirstPageNum=\"0\" hideFirstEmptyLine=\"0\" showLineNumber=\"0\"/>",
        "<hp:lineNumberShape restartType=\"0\" countBy=\"0\" distance=\"0\" startNumber=\"0\"/>",
        "<hp:pagePr landscape=\"WIDELY\" width=\"59528\" height=\"84186\" gutterType=\"LEFT_ONLY\"><hp:margin header=\"4252\" footer=\"4252\" gutter=\"0\" left=\"8504\" right=\"8504\" top=\"5668\" bottom=\"4252\"/></hp:pagePr>",
        "<hp:footNotePr><hp:autoNumFormat type=\"DIGIT\" userChar=\"\" prefixChar=\"\" suffixChar=\")\" supscript=\"0\"/><hp:noteLine length=\"-1\" type=\"SOLID\" width=\"0.12 mm\" color=\"#000000\"/><hp:noteSpacing betweenNotes=\"283\" belowLine=\"567\" aboveLine=\"850\"/><hp:numbering type=\"CONTINUOUS\" newNum=\"1\"/><hp:placement place=\"EACH_COLUMN\" beneathText=\"0\"/></hp:footNotePr>",
        "<hp:endNotePr><hp:autoNumFormat type=\"DIGIT\" userChar=\"\" prefixChar=\"\" suffixChar=\")\" supscript=\"0\"/><hp:noteLine length=\"14692344\" type=\"SOLID\" width=\"0.12 mm\" color=\"#000000\"/><hp:noteSpacing betweenNotes=\"0\" belowLine=\"567\" aboveLine=\"850\"/><hp:numbering type=\"CONTINUOUS\" newNum=\"1\"/><hp:placement place=\"END_OF_DOCUMENT\" beneathText=\"0\"/></hp:endNotePr>",
        "<hp:pageBorderFill type=\"BOTH\" borderFillIDRef=\"1\" textBorder=\"PAPER\" headerInside=\"0\" footerInside=\"0\" fillArea=\"PAPER\"><hp:offset left=\"1417\" right=\"1417\" top=\"1417\" bottom=\"1417\"/></hp:pageBorderFill>",
        "<hp:pageBorderFill type=\"EVEN\" borderFillIDRef=\"1\" textBorder=\"PAPER\" headerInside=\"0\" footerInside=\"0\" fillArea=\"PAPER\"><hp:offset left=\"1417\" right=\"1417\" top=\"1417\" bottom=\"1417\"/></hp:pageBorderFill>",
        "<hp:pageBorderFill type=\"ODD\" borderFillIDRef=\"1\" textBorder=\"PAPER\" headerInside=\"0\" footerInside=\"0\" fillArea=\"PAPER\"><hp:offset left=\"1417\" right=\"1417\" top=\"1417\" bottom=\"1417\"/></hp:pageBorderFill>",
        "</hp:secPr><hp:ctrl><hp:colPr id=\"\" type=\"NEWSPAPER\" layout=\"LEFT\" colCount=\"1\" sameSz=\"1\" sameGap=\"0\"/></hp:ctrl>"
    )
}

fn render_standard_lineseg() -> &'static str {
    "<hp:linesegarray><hp:lineseg textpos=\"0\" vertpos=\"0\" vertsize=\"1000\" textheight=\"1000\" baseline=\"850\" spacing=\"600\" horzpos=\"0\" horzsize=\"42520\" flags=\"393216\"/></hp:linesegarray>"
}

fn render_lineseg(style: LineSegProfile, list_vertpos: &mut u32) -> String {
    match style {
        LineSegProfile::Standard => render_standard_lineseg().to_string(),
        LineSegProfile::ListLevel1 => {
            render_list_lineseg(LIST_LEVEL1_HORZ_POS, LIST_LEVEL1_HORZ_SIZE, list_vertpos)
        }
        LineSegProfile::ListLevel2 => {
            render_list_lineseg(LIST_LEVEL2_HORZ_POS, LIST_LEVEL2_HORZ_SIZE, list_vertpos)
        }
    }
}

fn render_list_lineseg(horzpos: u32, horzsize: u32, list_vertpos: &mut u32) -> String {
    let current_vertpos = *list_vertpos;
    *list_vertpos += LIST_LINESEG_INCREMENT;
    format!(
        "<hp:linesegarray><hp:lineseg textpos=\"0\" vertpos=\"{vertpos}\" vertsize=\"{vertsize}\" textheight=\"{textheight}\" baseline=\"{baseline}\" spacing=\"{spacing}\" horzpos=\"{horzpos}\" horzsize=\"{horzsize}\" flags=\"{flags}\"/></hp:linesegarray>",
        vertpos = current_vertpos,
        vertsize = LIST_LINESEG_VERT_SIZE,
        textheight = LIST_LINESEG_TEXT_HEIGHT,
        baseline = LIST_LINESEG_BASELINE,
        spacing = LIST_LINESEG_SPACING,
        horzpos = horzpos,
        horzsize = horzsize,
        flags = LIST_LINESEG_FLAGS,
    )
}

fn render_table_lineseg() -> String {
    format!(
        "<hp:linesegarray><hp:lineseg textpos=\"0\" vertpos=\"0\" vertsize=\"{size}\" textheight=\"{size}\" baseline=\"{baseline}\" spacing=\"600\" horzpos=\"0\" horzsize=\"42520\" flags=\"393216\"/></hp:linesegarray>",
        size = TABLE_OUTER_LINESEG_SIZE,
        baseline = TABLE_OUTER_BASELINE,
    )
}

fn render_runs(
    inlines: &[Inline],
    default_char_pr: u32,
    strict_mode: bool,
) -> Result<Vec<RunSpec>, CoreRsError> {
    let mut runs = Vec::new();
    for inline in inlines {
        collect_inline_runs(inline, default_char_pr, strict_mode, &mut runs)?;
    }
    Ok(merge_adjacent_runs(runs))
}

fn collect_inline_runs(
    inline: &Inline,
    default_char_pr: u32,
    strict_mode: bool,
    runs: &mut Vec<RunSpec>,
) -> Result<(), CoreRsError> {
    match inline {
        Inline::Text(value) => push_run(runs, default_char_pr, value.clone()),
        Inline::Emphasis(children) => {
            for child in children {
                collect_inline_runs(child, emphasis_char_pr(default_char_pr), strict_mode, runs)?;
            }
        }
        Inline::Strong(children) => {
            for child in children {
                collect_inline_runs(child, strong_char_pr(default_char_pr), strict_mode, runs)?;
            }
        }
        Inline::Code(code) => push_run(runs, code_char_pr(default_char_pr), code.clone()),
        Inline::Link { text, url } => {
            let label = flatten_inline_children(text, strict_mode)?;
            let visible = if label.trim().is_empty() {
                url.clone()
            } else {
                format!("{label} ({url})")
            };
            push_run(runs, link_char_pr(default_char_pr), visible);
        }
        Inline::Image(image) => {
            if strict_mode {
                return Err(CoreRsError::UnsupportedFeature(
                    "HWPX core does not support image".to_string(),
                ));
            }
            let fallback = if image.alt.trim().is_empty() {
                "[image]".to_string()
            } else {
                image.alt.clone()
            };
            push_run(runs, default_char_pr, fallback);
        }
        Inline::HardBreak => push_run(runs, default_char_pr, " ".to_string()),
    }
    Ok(())
}

fn merge_adjacent_runs(runs: Vec<RunSpec>) -> Vec<RunSpec> {
    let mut merged = Vec::new();
    for run in runs {
        if merged
            .last()
            .is_some_and(|last: &RunSpec| last.char_pr == run.char_pr)
        {
            if let Some(last) = merged.last_mut() {
                last.text.push_str(&run.text);
            }
            continue;
        }
        merged.push(run);
    }
    merged
}

fn push_run(runs: &mut Vec<RunSpec>, char_pr: u32, text: String) {
    if text.is_empty() {
        return;
    }
    runs.push(RunSpec { char_pr, text });
}

fn normal_style() -> ParagraphStyle {
    ParagraphStyle {
        para_pr: 0,
        style: 0,
        default_char_pr: NORMAL_CHAR_PR,
        line_seg: LineSegProfile::Standard,
        list_semantic: ListSemantic::None,
    }
}

fn unordered_list_style(depth: u8) -> ParagraphStyle {
    ParagraphStyle {
        para_pr: if depth == 1 {
            LIST_LEVEL1_PARA_PR
        } else {
            LIST_LEVEL2_PARA_PR
        },
        style: 0,
        default_char_pr: NORMAL_CHAR_PR,
        line_seg: if depth == 1 {
            LineSegProfile::ListLevel1
        } else {
            LineSegProfile::ListLevel2
        },
        list_semantic: ListSemantic::Unordered,
    }
}

fn ordered_list_style(shape: OrderedListShape, depth: u8) -> ParagraphStyle {
    match shape {
        OrderedListShape::SingleLevel => ParagraphStyle {
            para_pr: LIST_LEVEL1_PARA_PR,
            style: 0,
            default_char_pr: NORMAL_CHAR_PR,
            line_seg: LineSegProfile::ListLevel1,
            list_semantic: ListSemantic::OrderedSingleLevel,
        },
        OrderedListShape::NestedDepth2 => {
            if depth == 1 {
                ParagraphStyle {
                    para_pr: LIST_LEVEL2_PARA_PR,
                    style: 0,
                    default_char_pr: NORMAL_CHAR_PR,
                    line_seg: LineSegProfile::ListLevel1,
                    list_semantic: ListSemantic::OrderedTopLevel,
                }
            } else {
                ParagraphStyle {
                    para_pr: LIST_LEVEL1_PARA_PR,
                    style: 0,
                    default_char_pr: NORMAL_CHAR_PR,
                    line_seg: LineSegProfile::ListLevel2,
                    list_semantic: ListSemantic::OrderedNestedLevel,
                }
            }
        }
    }
}

fn heading_style(level: u8) -> ParagraphStyle {
    match level {
        1 => ParagraphStyle {
            para_pr: 2,
            style: 2,
            default_char_pr: H1_CHAR_PR,
            line_seg: LineSegProfile::Standard,
            list_semantic: ListSemantic::None,
        },
        2 => ParagraphStyle {
            para_pr: 3,
            style: 3,
            default_char_pr: H2_CHAR_PR,
            line_seg: LineSegProfile::Standard,
            list_semantic: ListSemantic::None,
        },
        3 => ParagraphStyle {
            para_pr: 4,
            style: 4,
            default_char_pr: H3_CHAR_PR,
            line_seg: LineSegProfile::Standard,
            list_semantic: ListSemantic::None,
        },
        4 => ParagraphStyle {
            para_pr: 5,
            style: 5,
            default_char_pr: H4_CHAR_PR,
            line_seg: LineSegProfile::Standard,
            list_semantic: ListSemantic::None,
        },
        5 => ParagraphStyle {
            para_pr: 6,
            style: 6,
            default_char_pr: BOLD_CHAR_PR,
            line_seg: LineSegProfile::Standard,
            list_semantic: ListSemantic::None,
        },
        _ => ParagraphStyle {
            para_pr: 7,
            style: 7,
            default_char_pr: BOLD_CHAR_PR,
            line_seg: LineSegProfile::Standard,
            list_semantic: ListSemantic::None,
        },
    }
}

fn heading_default_char_pr(level: u8) -> u32 {
    heading_style(level).default_char_pr
}

fn strong_char_pr(default_char_pr: u32) -> u32 {
    match default_char_pr {
        H1_CHAR_PR | H2_CHAR_PR | H3_CHAR_PR | H4_CHAR_PR | BOLD_CHAR_PR => default_char_pr,
        _ => BOLD_CHAR_PR,
    }
}

fn emphasis_char_pr(default_char_pr: u32) -> u32 {
    match default_char_pr {
        H1_CHAR_PR | H2_CHAR_PR | H3_CHAR_PR | H4_CHAR_PR => default_char_pr,
        _ => ITALIC_CHAR_PR,
    }
}

fn code_char_pr(default_char_pr: u32) -> u32 {
    match default_char_pr {
        H1_CHAR_PR | H2_CHAR_PR | H3_CHAR_PR | H4_CHAR_PR => default_char_pr,
        _ => CODE_CHAR_PR,
    }
}

fn link_char_pr(default_char_pr: u32) -> u32 {
    match default_char_pr {
        H1_CHAR_PR | H2_CHAR_PR | H3_CHAR_PR | H4_CHAR_PR => default_char_pr,
        _ => LINK_CHAR_PR,
    }
}

fn flatten_inline_children(inlines: &[Inline], strict_mode: bool) -> Result<String, CoreRsError> {
    let mut text = String::new();
    for inline in inlines {
        match inline {
            Inline::Text(value) => text.push_str(value),
            Inline::Emphasis(children) | Inline::Strong(children) => {
                text.push_str(&flatten_inline_children(children, strict_mode)?);
            }
            Inline::Code(code) => text.push_str(code),
            Inline::Link {
                text: children,
                url,
            } => {
                let label = flatten_inline_children(children, strict_mode)?;
                if label.trim().is_empty() {
                    text.push_str(url);
                } else {
                    text.push_str(&format!("{label} ({url})"));
                }
            }
            Inline::Image(image) => {
                if strict_mode {
                    return Err(CoreRsError::UnsupportedFeature(
                        "HWPX core does not support image".to_string(),
                    ));
                }
                if image.alt.trim().is_empty() {
                    text.push_str("[image]");
                } else {
                    text.push_str(&image.alt);
                }
            }
            Inline::HardBreak => text.push(' '),
        }
    }
    Ok(text)
}

fn visible_paragraphs(items: &[SectionItem]) -> Vec<String> {
    let mut paragraphs = Vec::new();
    for item in items {
        match item {
            SectionItem::Paragraph(_, runs) => {
                paragraphs.push(runs.iter().map(|run| run.text.as_str()).collect())
            }
            SectionItem::Table(table) => {
                for row in &table.rows {
                    paragraphs.push(
                        row.iter()
                            .map(|cell| format!("<{cell}>"))
                            .collect::<Vec<_>>()
                            .join(""),
                    );
                }
            }
        }
    }
    paragraphs
}

fn flatten_block_to_paragraphs(block: &Block) -> Vec<String> {
    match block {
        Block::Paragraph(content) | Block::Heading { content, .. } => {
            vec![flatten_inline_children(content, false).unwrap_or_default()]
        }
        Block::BlockQuote(blocks) => blocks
            .iter()
            .flat_map(flatten_block_to_paragraphs)
            .collect::<Vec<_>>(),
        Block::CodeBlock { code, .. } => code.lines().map(ToString::to_string).collect(),
        Block::List(list) => list
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                let prefix = if list.ordered {
                    format!("{}. ", list.start_index + index as u64)
                } else {
                    "- ".to_string()
                };
                let body = item
                    .blocks
                    .iter()
                    .flat_map(flatten_block_to_paragraphs)
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("{prefix}{body}")
            })
            .collect(),
        Block::Table(table) => {
            let mut paragraphs = Vec::new();
            if !table.headers.is_empty() {
                paragraphs.push(
                    table
                        .headers
                        .iter()
                        .map(|cell| flatten_inline_children(cell, false).unwrap_or_default())
                        .collect::<Vec<_>>()
                        .join(" | "),
                );
            }
            paragraphs.extend(table.rows.iter().map(|row| {
                row.cells
                    .iter()
                    .map(|cell| flatten_inline_children(cell, false).unwrap_or_default())
                    .collect::<Vec<_>>()
                    .join(" | ")
            }));
            paragraphs
        }
        Block::ThematicBreak => vec!["---".to_string()],
    }
}

fn unsupported_name(block: &Block) -> &'static str {
    match block {
        Block::Paragraph(_) => "paragraph",
        Block::Heading { .. } => "heading",
        Block::BlockQuote(_) => "blockquote",
        Block::CodeBlock { .. } => "code block",
        Block::List(_) => "list",
        Block::Table(_) => "table",
        Block::ThematicBreak => "thematic break",
    }
}

fn plain_paragraph_item(text: String) -> SectionItem {
    SectionItem::Paragraph(
        normal_style(),
        vec![RunSpec {
            char_pr: NORMAL_CHAR_PR,
            text,
        }],
    )
}

fn list_caret_position(items: &[SectionItem]) -> Option<(usize, usize)> {
    let mut paragraph_index = 0;
    let mut last_unordered_caret = None;
    let mut ordered_carets = Vec::new();

    for item in items {
        match item {
            SectionItem::Paragraph(style, runs) => {
                let text = runs.iter().map(|run| run.text.as_str()).collect::<String>();
                let caret = (paragraph_index, text.chars().count());
                match style.list_semantic {
                    ListSemantic::None => {}
                    ListSemantic::Unordered => last_unordered_caret = Some(caret),
                    ListSemantic::OrderedSingleLevel
                    | ListSemantic::OrderedTopLevel
                    | ListSemantic::OrderedNestedLevel => {
                        ordered_carets.push((style.list_semantic, caret));
                    }
                }
                paragraph_index += 1;
            }
            SectionItem::Table(table) => {
                paragraph_index += table.rows.len();
            }
        }
    }

    if !ordered_carets.is_empty() {
        let only_single_level = ordered_carets
            .iter()
            .all(|(semantic, _)| *semantic == ListSemantic::OrderedSingleLevel);
        if only_single_level && ordered_carets.len() > 1 {
            return ordered_carets
                .get(ordered_carets.len() - 2)
                .map(|(_, caret)| *caret);
        }
        return ordered_carets.last().map(|(_, caret)| *caret);
    }

    last_unordered_caret
}

fn ordered_list_shape(blocks: &[Block]) -> Option<OrderedListShape> {
    match max_ordered_list_depth(blocks) {
        0 => None,
        1 => Some(OrderedListShape::SingleLevel),
        2 => Some(OrderedListShape::NestedDepth2),
        _ => None,
    }
}

fn max_ordered_list_depth(blocks: &[Block]) -> u8 {
    blocks
        .iter()
        .map(block_ordered_list_depth)
        .max()
        .unwrap_or(0)
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

fn equal_widths(count: usize) -> Vec<u32> {
    if count == 0 {
        return Vec::new();
    }

    let base = TABLE_WIDTH / count as u32;
    let remainder = TABLE_WIDTH % count as u32;
    let mut widths = vec![base; count];
    if let Some(last) = widths.last_mut() {
        *last += remainder;
    }
    widths
}
