use crate::error::CoreRsError;
use crate::models::block::Block;
use crate::models::document::Document;
use crate::models::inline::Inline;
use crate::utils::xml_helper::escape_text;
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

struct ParagraphStyle {
    para_pr: u32,
    style: u32,
    default_char_pr: u32,
}

struct RunSpec {
    char_pr: u32,
    text: String,
}

pub fn build_section_xml(
    document: &Document,
    strict_mode: bool,
    _style: &ResolvedHwpxStyle,
) -> Result<String, CoreRsError> {
    let paragraphs = collect_paragraph_specs(&document.blocks, strict_mode)?;
    let mut body = String::new();
    for (index, paragraph) in paragraphs.iter().enumerate() {
        body.push_str(&render_paragraph(paragraph, index == 0, index));
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
    let paragraphs = visible_paragraphs(&document.blocks, strict_mode)?;
    let text = paragraphs.join("\r\n");
    Ok(text)
}

fn collect_paragraph_specs(blocks: &[Block], strict_mode: bool) -> Result<Vec<(ParagraphStyle, Vec<RunSpec>)>, CoreRsError> {
    let mut paragraphs = Vec::new();
    for block in blocks {
        match block {
            Block::Paragraph(inlines) => paragraphs.push((
                normal_style(),
                render_runs(inlines, NORMAL_CHAR_PR, strict_mode)?,
            )),
            Block::Heading { level, content } => paragraphs.push((
                heading_style(*level),
                render_runs(content, heading_default_char_pr(*level), strict_mode)?,
            )),
            unsupported => {
                if strict_mode {
                    return Err(CoreRsError::UnsupportedFeature(format!(
                        "HWPX core does not support {0}",
                        unsupported_name(unsupported)
                    )));
                }
                for paragraph in flatten_block_to_paragraphs(unsupported) {
                    paragraphs.push((
                        normal_style(),
                        vec![RunSpec {
                            char_pr: NORMAL_CHAR_PR,
                            text: paragraph,
                        }],
                    ));
                }
            }
        }
    }
    Ok(paragraphs)
}

fn render_paragraph((style, runs): &(ParagraphStyle, Vec<RunSpec>), is_first: bool, index: usize) -> String {
    let id = 2_757_524_817u64 + index as u64;
    let mut xml = format!(
        "<hp:p id=\"{id}\" paraPrIDRef=\"{para}\" styleIDRef=\"{style_id}\" pageBreak=\"0\" columnBreak=\"0\" merged=\"0\">",
        id = id,
        para = style.para_pr,
        style_id = style.style,
    );

    if is_first {
        xml.push_str(&format!(
            concat!(
                "<hp:run charPrIDRef=\"{char_pr}\">",
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
                "</hp:secPr><hp:ctrl><hp:colPr id=\"\" type=\"NEWSPAPER\" layout=\"LEFT\" colCount=\"1\" sameSz=\"1\" sameGap=\"0\"/></hp:ctrl></hp:run>"
            ),
            char_pr = style.default_char_pr,
        ));
    }

    if runs.is_empty() {
        xml.push_str(&format!("<hp:run charPrIDRef=\"{}\"><hp:t></hp:t></hp:run>", style.default_char_pr));
    } else {
        for run in runs {
            xml.push_str(&format!(
                "<hp:run charPrIDRef=\"{char_pr}\"><hp:t>{text}</hp:t></hp:run>",
                char_pr = run.char_pr,
                text = escape_text(&run.text)
            ));
        }
    }

    xml.push_str("<hp:linesegarray><hp:lineseg textpos=\"0\" vertpos=\"0\" vertsize=\"1000\" textheight=\"1000\" baseline=\"850\" spacing=\"600\" horzpos=\"0\" horzsize=\"42520\" flags=\"393216\"/></hp:linesegarray></hp:p>");
    xml
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
    merge_adjacent_runs(runs)
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

fn merge_adjacent_runs(runs: Vec<RunSpec>) -> Result<Vec<RunSpec>, CoreRsError> {
    let mut merged: Vec<RunSpec> = Vec::new();
    for run in runs {
        if merged.last().is_some_and(|last| last.char_pr == run.char_pr) {
            if let Some(last) = merged.last_mut() {
                last.text.push_str(&run.text);
            }
            continue;
        }
        merged.push(run);
    }
    Ok(merged)
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
    }
}

fn heading_style(level: u8) -> ParagraphStyle {
    match level {
        1 => ParagraphStyle { para_pr: 2, style: 2, default_char_pr: H1_CHAR_PR },
        2 => ParagraphStyle { para_pr: 3, style: 3, default_char_pr: H2_CHAR_PR },
        3 => ParagraphStyle { para_pr: 4, style: 4, default_char_pr: H3_CHAR_PR },
        4 => ParagraphStyle { para_pr: 5, style: 5, default_char_pr: H4_CHAR_PR },
        5 => ParagraphStyle { para_pr: 6, style: 6, default_char_pr: BOLD_CHAR_PR },
        _ => ParagraphStyle { para_pr: 7, style: 7, default_char_pr: BOLD_CHAR_PR },
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
            Inline::Link { text: children, url } => {
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

fn visible_paragraphs(blocks: &[Block], strict_mode: bool) -> Result<Vec<String>, CoreRsError> {
    let paragraph_specs = collect_paragraph_specs(blocks, strict_mode)?;
    Ok(paragraph_specs
        .into_iter()
        .map(|(_, runs)| runs.into_iter().map(|run| run.text).collect::<String>())
        .collect())
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
                    table.headers
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
