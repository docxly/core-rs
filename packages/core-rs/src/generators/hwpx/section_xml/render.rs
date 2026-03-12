use super::shared::{
    HyperlinkFieldSpec, hyperlink_command, render_hyperlink_run, render_section_preamble_run,
};
use super::text::flatten_inline_children;
use super::{
    BOLD_CHAR_PR, CODE_CHAR_PR, CoreRsError, H1_CHAR_PR, H2_CHAR_PR, H3_CHAR_PR, H4_CHAR_PR,
    ITALIC_CHAR_PR, Inline, LEGACY_QUOTE_HORZ_POS, LEGACY_QUOTE_HORZ_SIZE,
    LEGACY_QUOTE_LINESEG_INCREMENT, LINK_CHAR_PR, LIST_LEVEL1_HORZ_POS, LIST_LEVEL1_HORZ_SIZE,
    LIST_LEVEL2_HORZ_POS, LIST_LEVEL2_HORZ_SIZE, LIST_LINESEG_BASELINE, LIST_LINESEG_FLAGS,
    LIST_LINESEG_INCREMENT, LIST_LINESEG_SPACING, LIST_LINESEG_TEXT_HEIGHT, LIST_LINESEG_VERT_SIZE,
    LineSegProfile, NORMAL_CHAR_PR, ParagraphStyle, QUOTE_LEVEL1_HORZ_POS, QUOTE_LEVEL1_HORZ_SIZE,
    QUOTE_LEVEL2_HORZ_POS, QUOTE_LEVEL2_HORZ_SIZE, RunSpec, SectionItem,
    TABLE_CELL_INNER_WIDTH_DELTA, TABLE_HEIGHT_PER_ROW, TABLE_OUTER_BASELINE,
    TABLE_OUTER_LINESEG_SIZE, TABLE_ROW_HEIGHT, TABLE_WIDTH, TableSpec, escape_text,
};

pub(super) fn render_section_item(
    item: &SectionItem,
    is_first: bool,
    index: usize,
    list_vertpos: &mut u32,
    quote_vertpos: &mut u32,
) -> String {
    match item {
        SectionItem::Paragraph(style, runs) => {
            render_paragraph(style, runs, is_first, index, list_vertpos, quote_vertpos)
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
    quote_vertpos: &mut u32,
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
            xml.push_str(&render_run(run));
        }
    }

    if should_render_paragraph_lineseg(style) {
        xml.push_str(&render_lineseg(style.line_seg, list_vertpos, quote_vertpos));
    }
    xml.push_str("</hp:p>");
    xml
}

fn render_run(run: &RunSpec) -> String {
    match run {
        RunSpec::Text { char_pr, text } => format!(
            "<hp:run charPrIDRef=\"{char_pr}\"><hp:t>{text}</hp:t></hp:run>",
            char_pr = char_pr,
            text = escape_text(text),
        ),
        RunSpec::Hyperlink {
            char_pr,
            field_begin_id,
            field_id,
            url,
            text,
        } => render_hyperlink_run(HyperlinkFieldSpec {
            char_pr: *char_pr,
            field_begin_id: *field_begin_id,
            field_id: *field_id,
            command: &hyperlink_command(url),
            path: url,
            text,
            trailing_empty_text: false,
        }),
    }
}

fn should_render_paragraph_lineseg(style: &ParagraphStyle) -> bool {
    if style.style != 0 {
        return true;
    }

    !matches!(
        style.line_seg,
        LineSegProfile::Standard | LineSegProfile::ListLevel1 | LineSegProfile::ListLevel2
    )
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

fn render_standard_lineseg() -> &'static str {
    "<hp:linesegarray><hp:lineseg textpos=\"0\" vertpos=\"0\" vertsize=\"1000\" textheight=\"1000\" baseline=\"850\" spacing=\"600\" horzpos=\"0\" horzsize=\"42520\" flags=\"393216\"/></hp:linesegarray>"
}

fn render_lineseg(
    style: LineSegProfile,
    list_vertpos: &mut u32,
    quote_vertpos: &mut u32,
) -> String {
    match style {
        LineSegProfile::Standard => render_standard_lineseg().to_string(),
        LineSegProfile::ListLevel1 => {
            render_list_lineseg(LIST_LEVEL1_HORZ_POS, LIST_LEVEL1_HORZ_SIZE, list_vertpos)
        }
        LineSegProfile::ListLevel2 => {
            render_list_lineseg(LIST_LEVEL2_HORZ_POS, LIST_LEVEL2_HORZ_SIZE, list_vertpos)
        }
        LineSegProfile::QuoteLegacy => {
            render_quote_lineseg(LEGACY_QUOTE_HORZ_POS, LEGACY_QUOTE_HORZ_SIZE, quote_vertpos)
        }
        LineSegProfile::QuoteLevel1 => {
            render_standard_offset_lineseg(QUOTE_LEVEL1_HORZ_POS, QUOTE_LEVEL1_HORZ_SIZE)
        }
        LineSegProfile::QuoteLevel2 => {
            render_standard_offset_lineseg(QUOTE_LEVEL2_HORZ_POS, QUOTE_LEVEL2_HORZ_SIZE)
        }
    }
}

fn render_standard_offset_lineseg(horzpos: u32, horzsize: u32) -> String {
    format!(
        "<hp:linesegarray><hp:lineseg textpos=\"0\" vertpos=\"0\" vertsize=\"1000\" textheight=\"1000\" baseline=\"850\" spacing=\"600\" horzpos=\"{horzpos}\" horzsize=\"{horzsize}\" flags=\"393216\"/></hp:linesegarray>",
        horzpos = horzpos,
        horzsize = horzsize,
    )
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

fn render_quote_lineseg(horzpos: u32, horzsize: u32, quote_vertpos: &mut u32) -> String {
    let current_vertpos = *quote_vertpos;
    *quote_vertpos += LEGACY_QUOTE_LINESEG_INCREMENT;
    format!(
        "<hp:linesegarray><hp:lineseg textpos=\"0\" vertpos=\"{vertpos}\" vertsize=\"1000\" textheight=\"1000\" baseline=\"850\" spacing=\"600\" horzpos=\"{horzpos}\" horzsize=\"{horzsize}\" flags=\"393216\"/></hp:linesegarray>",
        vertpos = current_vertpos,
        horzpos = horzpos,
        horzsize = horzsize,
    )
}

fn render_table_lineseg() -> String {
    format!(
        "<hp:linesegarray><hp:lineseg textpos=\"0\" vertpos=\"0\" vertsize=\"{size}\" textheight=\"{size}\" baseline=\"{baseline}\" spacing=\"600\" horzpos=\"0\" horzsize=\"42520\" flags=\"393216\"/></hp:linesegarray>",
        size = TABLE_OUTER_LINESEG_SIZE,
        baseline = TABLE_OUTER_BASELINE,
    )
}

pub(super) fn render_runs(
    inlines: &[Inline],
    default_char_pr: u32,
    strict_mode: bool,
    next_link_index: &mut u64,
) -> Result<Vec<RunSpec>, CoreRsError> {
    let mut runs = Vec::new();
    for inline in inlines {
        collect_inline_runs(
            inline,
            default_char_pr,
            strict_mode,
            &mut runs,
            next_link_index,
        )?;
    }
    Ok(merge_adjacent_runs(runs))
}

fn collect_inline_runs(
    inline: &Inline,
    default_char_pr: u32,
    strict_mode: bool,
    runs: &mut Vec<RunSpec>,
    next_link_index: &mut u64,
) -> Result<(), CoreRsError> {
    match inline {
        Inline::Text(value) => push_run(runs, default_char_pr, value.clone()),
        Inline::Emphasis(children) => {
            for child in children {
                collect_inline_runs(
                    child,
                    emphasis_char_pr(default_char_pr),
                    strict_mode,
                    runs,
                    next_link_index,
                )?;
            }
        }
        Inline::Strong(children) => {
            for child in children {
                collect_inline_runs(
                    child,
                    strong_char_pr(default_char_pr),
                    strict_mode,
                    runs,
                    next_link_index,
                )?;
            }
        }
        Inline::Code(code) => push_run(runs, code_char_pr(default_char_pr), code.clone()),
        Inline::Link { text, url } => {
            let label = flatten_inline_children(text, strict_mode)?;
            let visible = if label.trim().is_empty() {
                url.clone()
            } else {
                label
            };
            runs.push(RunSpec::Hyperlink {
                char_pr: link_char_pr(default_char_pr),
                field_begin_id: 2_107_483_186 + *next_link_index,
                field_id: 627_600_491 + *next_link_index,
                url: url.clone(),
                text: visible,
            });
            *next_link_index += 1;
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
        match (merged.last_mut(), &run) {
            (
                Some(RunSpec::Text {
                    char_pr: last_char_pr,
                    text: last_text,
                }),
                RunSpec::Text { char_pr, text },
            ) if *last_char_pr == *char_pr => {
                last_text.push_str(text);
                continue;
            }
            _ => {}
        }
        merged.push(run);
    }
    merged
}

fn push_run(runs: &mut Vec<RunSpec>, char_pr: u32, text: String) {
    if !text.is_empty() {
        runs.push(RunSpec::Text { char_pr, text });
    }
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
