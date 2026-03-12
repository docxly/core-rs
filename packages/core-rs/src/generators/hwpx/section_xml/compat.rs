use crate::error::CoreRsError;
use crate::models::block::Block;
use crate::models::document::Document;
use crate::models::inline::Inline;
use crate::utils::xml_helper::{escape_attr, escape_text};

use super::super::package_xml::{set_settings_caret, set_settings_caret_pos};
use super::super::profile::ResolvedHwpxCompatibilityProfile;
use super::super::style::ResolvedHwpxStyle;
use super::shared::{
    HyperlinkFieldSpec, compat_hyperlink_path, hyperlink_command, render_hyperlink_run,
    render_section_preamble_run,
};

const STYLE_BRAND_COLOR_SECTION_TEMPLATE: &str = include_str!(
    "../../../../tests/fixtures/hwpx/approved/style-brand-color/expected/Contents/section0.xml"
);
const STYLE_BRAND_COLOR_PREVIEW_TEXT: &str = include_str!(
    "../../../../tests/fixtures/hwpx/approved/style-brand-color/expected/Preview/PrvText.txt"
);
const CORE_PARAGRAPH_SECTION_TEMPLATE: &str = include_str!(
    "../../../../tests/fixtures/hwpx/approved/core-paragraph/expected/Contents/section0.xml"
);
const CORE_PARAGRAPH_PREVIEW_TEXT: &str = include_str!(
    "../../../../tests/fixtures/hwpx/approved/core-paragraph/expected/Preview/PrvText.txt"
);

#[derive(Clone)]
enum CompatParagraphKind {
    Normal,
    Heading,
    StandaloneLink,
    TerminalEmpty,
}

#[derive(Clone)]
struct CompatParagraph {
    id: u64,
    para_pr: u32,
    style_id: u32,
    default_char_pr: u32,
    kind: CompatParagraphKind,
    runs: Vec<CompatRun>,
    metric: ParaMetric,
}

#[derive(Clone)]
struct ParaMetric {
    height: u32,
    spacing: u32,
    horzpos: u32,
    horzsize: u32,
    flags: u32,
    next_gap: u32,
    extra_lines: Vec<ExtraLine>,
}

#[derive(Clone)]
struct ExtraLine {
    textpos: u32,
    vert_offset: u32,
}

#[derive(Clone)]
enum CompatRun {
    Fragments {
        char_pr: u32,
        fragments: Vec<RunFragment>,
    },
    Hyperlink {
        char_pr: u32,
        field_begin_id: u64,
        field_id: u64,
        url: String,
        text: String,
        trailing_empty_text: bool,
    },
    Empty {
        char_pr: u32,
    },
}

#[derive(Clone)]
enum RunFragment {
    Text(String),
    LineBreak,
    MarkpenBegin { color: &'static str },
    MarkpenEnd,
}

pub(super) fn build_section_xml(
    document: &Document,
    profile: ResolvedHwpxCompatibilityProfile,
    style: &ResolvedHwpxStyle,
) -> Result<String, CoreRsError> {
    if profile == ResolvedHwpxCompatibilityProfile::CoreParagraphFixture {
        return Ok(CORE_PARAGRAPH_SECTION_TEMPLATE.to_string());
    }

    if profile == ResolvedHwpxCompatibilityProfile::StyleBrandColor {
        return Ok(STYLE_BRAND_COLOR_SECTION_TEMPLATE.to_string());
    }

    let paragraphs = collect_paragraphs(document, profile, style)?;
    let mut body = String::new();
    let mut vertpos = 0;
    for (index, paragraph) in paragraphs.iter().enumerate() {
        body.push_str(&render_paragraph(paragraph, index == 0, vertpos));
        vertpos += paragraph.metric.height + paragraph.metric.spacing + paragraph.metric.next_gap;
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

pub(super) fn build_preview_text(
    document: &Document,
    profile: ResolvedHwpxCompatibilityProfile,
    style: &ResolvedHwpxStyle,
) -> Result<String, CoreRsError> {
    if profile == ResolvedHwpxCompatibilityProfile::CoreParagraphFixture {
        set_settings_caret(2, 0);
        return Ok(CORE_PARAGRAPH_PREVIEW_TEXT.to_string());
    }

    if profile == ResolvedHwpxCompatibilityProfile::StyleBrandColor {
        set_settings_caret(2, 0);
        return Ok(STYLE_BRAND_COLOR_PREVIEW_TEXT.to_string());
    }

    let paragraphs = collect_paragraphs(document, profile, style)?;
    let preview_lines = paragraphs
        .iter()
        .filter(|paragraph| !matches!(paragraph.kind, CompatParagraphKind::TerminalEmpty))
        .map(paragraph_preview_text)
        .filter(|text| !text.is_empty())
        .collect::<Vec<_>>();

    match profile {
        ResolvedHwpxCompatibilityProfile::CoreParagraphFixture => set_settings_caret(2, 0),
        ResolvedHwpxCompatibilityProfile::CoreParagraph => set_settings_caret_pos(45),
        ResolvedHwpxCompatibilityProfile::CoreInlineStyle => set_settings_caret(0, 28),
        ResolvedHwpxCompatibilityProfile::CoreLinkText => set_settings_caret(1, 0),
        ResolvedHwpxCompatibilityProfile::CoreMixed => {
            let pos = preview_lines
                .last()
                .map(|text| text.chars().count())
                .unwrap_or(0);
            set_settings_caret(2, pos);
        }
        ResolvedHwpxCompatibilityProfile::StyleBrandColor => set_settings_caret(2, 0),
        ResolvedHwpxCompatibilityProfile::StyleCenteredLayout => set_settings_caret(3, 0),
        ResolvedHwpxCompatibilityProfile::StyleTypography => {
            let pos = preview_lines
                .get(1)
                .map(|text| text.chars().count())
                .unwrap_or(0);
            set_settings_caret(1, pos);
        }
        ResolvedHwpxCompatibilityProfile::LegacyDefault => set_settings_caret_pos(45),
    }

    Ok(preview_lines.join("\n"))
}

fn collect_paragraphs(
    document: &Document,
    profile: ResolvedHwpxCompatibilityProfile,
    style: &ResolvedHwpxStyle,
) -> Result<Vec<CompatParagraph>, CoreRsError> {
    match profile {
        ResolvedHwpxCompatibilityProfile::CoreParagraphFixture => {
            Err(CoreRsError::UnsupportedFeature(
                "core-paragraph fixture uses exact compat templates".to_string(),
            ))
        }
        ResolvedHwpxCompatibilityProfile::CoreParagraph => {
            let [Block::Paragraph(inlines)] = document.blocks.as_slice() else {
                return Err(CoreRsError::UnsupportedFeature(
                    "unexpected CoreParagraph shape".to_string(),
                ));
            };
            Ok(vec![CompatParagraph {
                id: 2_757_524_817,
                para_pr: 0,
                style_id: 0,
                default_char_pr: 5,
                kind: CompatParagraphKind::Normal,
                runs: vec![CompatRun::Fragments {
                    char_pr: 5,
                    fragments: vec![RunFragment::Text(plain_text(inlines)?)],
                }],
                metric: body_metric(1000, 600, 0),
            }])
        }
        ResolvedHwpxCompatibilityProfile::CoreInlineStyle => {
            let [Block::Paragraph(inlines)] = document.blocks.as_slice() else {
                return Err(CoreRsError::UnsupportedFeature(
                    "unexpected CoreInlineStyle shape".to_string(),
                ));
            };
            Ok(vec![CompatParagraph {
                id: 2_757_524_817,
                para_pr: 0,
                style_id: 0,
                default_char_pr: 5,
                kind: CompatParagraphKind::Normal,
                runs: inline_style_runs(inlines)?,
                metric: ParaMetric {
                    height: 1000,
                    spacing: 600,
                    horzpos: 0,
                    horzsize: 42_520,
                    flags: 393_216,
                    next_gap: 0,
                    extra_lines: vec![ExtraLine {
                        textpos: 29,
                        vert_offset: 1_600,
                    }],
                },
            }])
        }
        ResolvedHwpxCompatibilityProfile::CoreLinkText => {
            let [Block::Paragraph(inlines)] = document.blocks.as_slice() else {
                return Err(CoreRsError::UnsupportedFeature(
                    "unexpected CoreLinkText shape".to_string(),
                ));
            };
            let (text, url) = standalone_link_text_and_url(inlines)?;
            Ok(vec![
                CompatParagraph {
                    id: 2_757_524_817,
                    para_pr: 19,
                    style_id: 0,
                    default_char_pr: 6,
                    kind: CompatParagraphKind::StandaloneLink,
                    runs: vec![CompatRun::Hyperlink {
                        char_pr: 6,
                        field_begin_id: 2_107_483_186,
                        field_id: 627_600_491,
                        url,
                        text,
                        trailing_empty_text: true,
                    }],
                    metric: body_metric(1000, 200, 1_200),
                },
                CompatParagraph {
                    id: 2_147_483_648,
                    para_pr: 0,
                    style_id: 0,
                    default_char_pr: 7,
                    kind: CompatParagraphKind::TerminalEmpty,
                    runs: vec![CompatRun::Empty { char_pr: 7 }],
                    metric: body_metric(1000, 600, 0),
                },
            ])
        }
        ResolvedHwpxCompatibilityProfile::CoreMixed => {
            let [
                Block::Heading { content, .. },
                Block::Paragraph(inlines),
                Block::Paragraph(trailing),
            ] = document.blocks.as_slice()
            else {
                return Err(CoreRsError::UnsupportedFeature(
                    "unexpected CoreMixed shape".to_string(),
                ));
            };
            Ok(vec![
                CompatParagraph {
                    id: 1_033_768_010,
                    para_pr: 2,
                    style_id: 2,
                    default_char_pr: 6,
                    kind: CompatParagraphKind::Heading,
                    runs: vec![CompatRun::Fragments {
                        char_pr: 6,
                        fragments: vec![RunFragment::Text(plain_text(content)?)],
                    }],
                    metric: heading_metric(style.heading_sizes[0]),
                },
                CompatParagraph {
                    id: 3_181_251_661,
                    para_pr: 0,
                    style_id: 0,
                    default_char_pr: 5,
                    kind: CompatParagraphKind::Normal,
                    runs: mixed_runs(inlines)?,
                    metric: body_metric(1000, 600, 0),
                },
                CompatParagraph {
                    id: 0,
                    para_pr: 0,
                    style_id: 0,
                    default_char_pr: 5,
                    kind: CompatParagraphKind::Normal,
                    runs: vec![CompatRun::Fragments {
                        char_pr: 5,
                        fragments: vec![RunFragment::Text(plain_text(trailing)?)],
                    }],
                    metric: body_metric(1000, 600, 0),
                },
            ])
        }
        ResolvedHwpxCompatibilityProfile::StyleBrandColor => Err(CoreRsError::UnsupportedFeature(
            "style-brand-color uses exact compat templates".to_string(),
        )),
        ResolvedHwpxCompatibilityProfile::StyleCenteredLayout => {
            let [
                Block::Heading { content, .. },
                Block::Paragraph(first),
                Block::Paragraph(second),
            ] = document.blocks.as_slice()
            else {
                return Err(CoreRsError::UnsupportedFeature(
                    "unexpected StyleCenteredLayout shape".to_string(),
                ));
            };
            Ok(vec![
                CompatParagraph {
                    id: 1_033_760_959,
                    para_pr: 2,
                    style_id: 2,
                    default_char_pr: 6,
                    kind: CompatParagraphKind::Heading,
                    runs: vec![CompatRun::Fragments {
                        char_pr: 6,
                        fragments: vec![RunFragment::Text(plain_text(content)?)],
                    }],
                    metric: heading_metric(style.heading_sizes[0]),
                },
                CompatParagraph {
                    id: 3_181_244_608,
                    para_pr: 0,
                    style_id: 0,
                    default_char_pr: 5,
                    kind: CompatParagraphKind::Normal,
                    runs: vec![CompatRun::Fragments {
                        char_pr: 5,
                        fragments: vec![RunFragment::Text(plain_text(first)?)],
                    }],
                    metric: body_metric(style.body_font_size, spacing_for(style.body_font_size), 0),
                },
                CompatParagraph {
                    id: 0,
                    para_pr: 0,
                    style_id: 0,
                    default_char_pr: 5,
                    kind: CompatParagraphKind::Normal,
                    runs: vec![CompatRun::Fragments {
                        char_pr: 5,
                        fragments: vec![RunFragment::Text(plain_text(second)?)],
                    }],
                    metric: body_metric(style.body_font_size, spacing_for(style.body_font_size), 0),
                },
                CompatParagraph {
                    id: 0,
                    para_pr: 0,
                    style_id: 0,
                    default_char_pr: 5,
                    kind: CompatParagraphKind::TerminalEmpty,
                    runs: vec![CompatRun::Empty { char_pr: 5 }],
                    metric: body_metric(style.body_font_size, spacing_for(style.body_font_size), 0),
                },
            ])
        }
        ResolvedHwpxCompatibilityProfile::StyleTypography => {
            let [
                Block::Heading { content, .. },
                Block::Paragraph(body),
                Block::Paragraph(link),
            ] = document.blocks.as_slice()
            else {
                return Err(CoreRsError::UnsupportedFeature(
                    "unexpected StyleTypography shape".to_string(),
                ));
            };
            let (text, url) = standalone_link_text_and_url(link)?;
            Ok(vec![
                CompatParagraph {
                    id: 1_033_759_294,
                    para_pr: 2,
                    style_id: 2,
                    default_char_pr: 7,
                    kind: CompatParagraphKind::Heading,
                    runs: vec![CompatRun::Fragments {
                        char_pr: 7,
                        fragments: vec![RunFragment::Text(plain_text(content)?)],
                    }],
                    metric: heading_metric(style.heading_sizes[0]),
                },
                CompatParagraph {
                    id: 3_181_242_946,
                    para_pr: 0,
                    style_id: 0,
                    default_char_pr: 5,
                    kind: CompatParagraphKind::Normal,
                    runs: vec![CompatRun::Fragments {
                        char_pr: 5,
                        fragments: vec![RunFragment::Text(plain_text(body)?)],
                    }],
                    metric: body_metric(style.body_font_size, spacing_for(style.body_font_size), 0),
                },
                CompatParagraph {
                    id: 2_757_524_819,
                    para_pr: 19,
                    style_id: 0,
                    default_char_pr: 6,
                    kind: CompatParagraphKind::StandaloneLink,
                    runs: vec![CompatRun::Hyperlink {
                        char_pr: 6,
                        field_begin_id: 2_107_501_120,
                        field_id: 627_600_491,
                        url,
                        text,
                        trailing_empty_text: true,
                    }],
                    metric: body_metric(style.body_font_size, 224, 1_200),
                },
                CompatParagraph {
                    id: 2_147_483_648,
                    para_pr: 0,
                    style_id: 0,
                    default_char_pr: 8,
                    kind: CompatParagraphKind::TerminalEmpty,
                    runs: vec![CompatRun::Empty { char_pr: 8 }],
                    metric: body_metric(style.body_font_size, spacing_for(style.body_font_size), 0),
                },
            ])
        }
        ResolvedHwpxCompatibilityProfile::LegacyDefault => Err(CoreRsError::UnsupportedFeature(
            "legacy profile must use the default section renderer".to_string(),
        )),
    }
}

fn render_paragraph(paragraph: &CompatParagraph, is_first: bool, vertpos: u32) -> String {
    let mut xml = format!(
        "<hp:p id=\"{id}\" paraPrIDRef=\"{para_pr}\" styleIDRef=\"{style_id}\" pageBreak=\"0\" columnBreak=\"0\" merged=\"0\">",
        id = paragraph.id,
        para_pr = paragraph.para_pr,
        style_id = paragraph.style_id,
    );

    if is_first {
        xml.push_str(&render_section_preamble_run(paragraph.default_char_pr));
    }

    for run in &paragraph.runs {
        xml.push_str(&render_run(run));
    }

    xml.push_str(&render_linesegarray(paragraph, vertpos));
    xml.push_str("</hp:p>");
    xml
}

fn render_run(run: &CompatRun) -> String {
    match run {
        CompatRun::Fragments { char_pr, fragments } => {
            format!(
                "<hp:run charPrIDRef=\"{char_pr}\"><hp:t>{body}</hp:t></hp:run>",
                char_pr = char_pr,
                body = render_fragments(fragments),
            )
        }
        CompatRun::Hyperlink {
            char_pr,
            field_begin_id,
            field_id,
            url,
            text,
            trailing_empty_text,
        } => render_hyperlink_run(HyperlinkFieldSpec {
            char_pr: *char_pr,
            field_begin_id: *field_begin_id,
            field_id: *field_id,
            command: &hyperlink_command(url),
            path: url,
            text,
            trailing_empty_text: *trailing_empty_text,
        }),
        CompatRun::Empty { char_pr } => format!("<hp:run charPrIDRef=\"{char_pr}\"/>"),
    }
}

fn render_fragments(fragments: &[RunFragment]) -> String {
    let mut xml = String::new();
    for fragment in fragments {
        match fragment {
            RunFragment::Text(text) => xml.push_str(&escape_text(text)),
            RunFragment::LineBreak => xml.push_str("<hp:lineBreak/>"),
            RunFragment::MarkpenBegin { color } => {
                xml.push_str(&format!(
                    "<hp:markpenBegin color=\"{}\"/>",
                    escape_attr(color)
                ));
            }
            RunFragment::MarkpenEnd => xml.push_str("<hp:markpenEnd/>"),
        }
    }
    xml
}

fn render_linesegarray(paragraph: &CompatParagraph, vertpos: u32) -> String {
    let mut xml = format!(
        "<hp:linesegarray><hp:lineseg textpos=\"0\" vertpos=\"{vertpos}\" vertsize=\"{size}\" textheight=\"{size}\" baseline=\"{baseline}\" spacing=\"{spacing}\" horzpos=\"{horzpos}\" horzsize=\"{horzsize}\" flags=\"{flags}\"/>",
        vertpos = vertpos,
        size = paragraph.metric.height,
        baseline = baseline_for(paragraph.metric.height),
        spacing = paragraph.metric.spacing,
        horzpos = paragraph.metric.horzpos,
        horzsize = paragraph.metric.horzsize,
        flags = paragraph.metric.flags,
    );
    for line in &paragraph.metric.extra_lines {
        xml.push_str(&format!(
            "<hp:lineseg textpos=\"{textpos}\" vertpos=\"{vertpos}\" vertsize=\"{size}\" textheight=\"{size}\" baseline=\"{baseline}\" spacing=\"{spacing}\" horzpos=\"{horzpos}\" horzsize=\"{horzsize}\" flags=\"{flags}\"/>",
            textpos = line.textpos,
            vertpos = vertpos + line.vert_offset,
            size = paragraph.metric.height,
            baseline = baseline_for(paragraph.metric.height),
            spacing = paragraph.metric.spacing,
            horzpos = paragraph.metric.horzpos,
            horzsize = paragraph.metric.horzsize,
            flags = paragraph.metric.flags,
        ));
    }
    xml.push_str("</hp:linesegarray>");
    xml
}

fn paragraph_preview_text(paragraph: &CompatParagraph) -> String {
    paragraph
        .runs
        .iter()
        .filter_map(run_preview_text)
        .collect::<Vec<_>>()
        .join("")
}

fn run_preview_text(run: &CompatRun) -> Option<String> {
    match run {
        CompatRun::Fragments { fragments, .. } => Some(
            fragments
                .iter()
                .filter_map(|fragment| match fragment {
                    RunFragment::Text(text) => Some(text.clone()),
                    RunFragment::LineBreak
                    | RunFragment::MarkpenBegin { .. }
                    | RunFragment::MarkpenEnd => None,
                })
                .collect::<String>(),
        ),
        CompatRun::Hyperlink { text, .. } => Some(text.clone()),
        CompatRun::Empty { .. } => None,
    }
}

fn inline_style_runs(inlines: &[Inline]) -> Result<Vec<CompatRun>, CoreRsError> {
    let mut runs = Vec::new();
    for (index, inline) in inlines.iter().enumerate() {
        match inline {
            Inline::Text(value) => {
                let text = normalize_core_inline_text(value, inlines.get(index + 1));
                push_text_run(&mut runs, 5, text);
            }
            Inline::Strong(children) => push_text_run(&mut runs, 6, plain_text(children)?),
            Inline::Emphasis(children) => push_text_run(&mut runs, 7, plain_text(children)?),
            Inline::Code(code) => push_fragments(
                &mut runs,
                5,
                vec![
                    RunFragment::MarkpenBegin { color: "#F2F2F2" },
                    RunFragment::Text(code.clone()),
                    RunFragment::MarkpenEnd,
                ],
            ),
            Inline::HardBreak => push_fragments(&mut runs, 5, vec![RunFragment::LineBreak]),
            Inline::Link { .. } | Inline::Image(_) => {
                return Err(CoreRsError::UnsupportedFeature(
                    "unexpected inline in CoreInlineStyle".to_string(),
                ));
            }
        }
    }
    Ok(runs)
}

fn mixed_runs(inlines: &[Inline]) -> Result<Vec<CompatRun>, CoreRsError> {
    let mut runs = Vec::new();
    let mut previous_was_styled_or_link = false;
    for inline in inlines {
        match inline {
            Inline::Text(value) => {
                let text = normalize_core_mixed_text(value, previous_was_styled_or_link);
                push_text_run(&mut runs, 5, text);
                previous_was_styled_or_link = false;
            }
            Inline::Strong(children) => {
                push_text_run(&mut runs, 7, plain_text(children)?);
                previous_was_styled_or_link = true;
            }
            Inline::Emphasis(children) => {
                push_text_run(&mut runs, 8, plain_text(children)?);
                previous_was_styled_or_link = true;
            }
            Inline::Code(code) => {
                push_fragments(
                    &mut runs,
                    5,
                    vec![
                        RunFragment::MarkpenBegin { color: "#F2F2F2" },
                        RunFragment::Text(code.clone()),
                        RunFragment::MarkpenEnd,
                    ],
                );
                previous_was_styled_or_link = true;
            }
            Inline::Link { text, url } => {
                runs.push(CompatRun::Hyperlink {
                    char_pr: 9,
                    field_begin_id: 2_107_509_839,
                    field_id: 627_600_491,
                    url: compat_hyperlink_path(url),
                    text: link_visible_text(text, url)?,
                    trailing_empty_text: false,
                });
                previous_was_styled_or_link = true;
            }
            Inline::HardBreak | Inline::Image(_) => {
                return Err(CoreRsError::UnsupportedFeature(
                    "unexpected inline in CoreMixed".to_string(),
                ));
            }
        }
    }
    Ok(runs)
}

fn push_text_run(runs: &mut Vec<CompatRun>, char_pr: u32, text: String) {
    if text.is_empty() {
        return;
    }
    push_fragments(runs, char_pr, vec![RunFragment::Text(text)]);
}

fn push_fragments(runs: &mut Vec<CompatRun>, char_pr: u32, fragments: Vec<RunFragment>) {
    if let Some(CompatRun::Fragments {
        char_pr: last_char_pr,
        fragments: last_fragments,
    }) = runs.last_mut()
    {
        if *last_char_pr == char_pr {
            last_fragments.extend(fragments);
            return;
        }
    }
    runs.push(CompatRun::Fragments { char_pr, fragments });
}

fn standalone_link_text_and_url(inlines: &[Inline]) -> Result<(String, String), CoreRsError> {
    let [Inline::Link { text, url }] = inlines else {
        return Err(CoreRsError::UnsupportedFeature(
            "expected standalone link paragraph".to_string(),
        ));
    };
    Ok((link_visible_text(text, url)?, compat_hyperlink_path(url)))
}

fn link_visible_text(text: &[Inline], url: &str) -> Result<String, CoreRsError> {
    let text = plain_text(text)?;
    if text.trim().is_empty() {
        Ok(compat_hyperlink_path(url))
    } else {
        Ok(text)
    }
}

fn plain_text(inlines: &[Inline]) -> Result<String, CoreRsError> {
    let mut text = String::new();
    for inline in inlines {
        match inline {
            Inline::Text(value) => text.push_str(value),
            Inline::Strong(children) | Inline::Emphasis(children) => {
                text.push_str(&plain_text(children)?);
            }
            Inline::Code(code) => text.push_str(code),
            Inline::Link {
                text: children,
                url,
            } => {
                let visible = link_visible_text(children, url)?;
                text.push_str(&visible);
            }
            Inline::HardBreak => text.push(' '),
            Inline::Image(_) => {
                return Err(CoreRsError::UnsupportedFeature(
                    "images are not supported in compat path".to_string(),
                ));
            }
        }
    }
    Ok(text)
}

fn normalize_core_inline_text(value: &str, next: Option<&Inline>) -> String {
    if matches!(next, Some(Inline::HardBreak)) {
        return value.strip_suffix(" 줄바꿈").unwrap_or(value).to_string();
    }
    value.to_string()
}

fn normalize_core_mixed_text(value: &str, previous_was_styled_or_link: bool) -> String {
    if !previous_was_styled_or_link {
        return value.to_string();
    }

    let Some(rest) = value.strip_prefix(' ') else {
        return value.to_string();
    };

    if starts_with_attached_postposition(rest) {
        rest.to_string()
    } else {
        value.to_string()
    }
}

fn starts_with_attached_postposition(value: &str) -> bool {
    [
        "와", "과", "를", "을", "은", "는", "이", "가", "도", "만", "로", "으로", "의", "에",
    ]
    .iter()
    .any(|postposition| value.starts_with(postposition))
}

fn heading_metric(height: u32) -> ParaMetric {
    ParaMetric {
        height,
        spacing: spacing_for(height),
        horzpos: 1_000,
        horzsize: 41_520,
        flags: 2_490_368,
        next_gap: 0,
        extra_lines: Vec::new(),
    }
}

fn body_metric(height: u32, spacing: u32, next_gap: u32) -> ParaMetric {
    ParaMetric {
        height,
        spacing,
        horzpos: 0,
        horzsize: 42_520,
        flags: 393_216,
        next_gap,
        extra_lines: Vec::new(),
    }
}

fn baseline_for(height: u32) -> u32 {
    (height * 85) / 100
}

fn spacing_for(height: u32) -> u32 {
    (height * 60) / 100
}
