use crate::models::block::Block;

use super::super::{
    BOLD_CHAR_PR, H1_CHAR_PR, H2_CHAR_PR, H3_CHAR_PR, H4_CHAR_PR, LEGACY_QUOTE_CHAR_PR,
    LEGACY_QUOTE_PARA_PR, LIST_LEVEL1_PARA_PR, LIST_LEVEL2_PARA_PR, LineSegProfile, ListSemantic,
    NORMAL_CHAR_PR, OrderedListShape, ParagraphStyle, QUOTE_LEVEL1_PARA_PR, QUOTE_LEVEL2_PARA_PR,
    QuoteRenderMode, RunSpec, SectionItem,
};

pub(super) fn normal_style(quote_depth: u8, quote_render_mode: QuoteRenderMode) -> ParagraphStyle {
    if quote_depth == 0 {
        return ParagraphStyle {
            para_pr: 0,
            style: 0,
            default_char_pr: NORMAL_CHAR_PR,
            line_seg: LineSegProfile::Standard,
            list_semantic: ListSemantic::None,
        };
    }

    quoted_style(quote_depth, 0, NORMAL_CHAR_PR, quote_render_mode)
}

pub(super) fn unordered_list_style(
    depth: u8,
    quote_depth: u8,
    quote_render_mode: QuoteRenderMode,
) -> ParagraphStyle {
    if quote_depth > 0 {
        return quoted_style(quote_depth, 0, NORMAL_CHAR_PR, quote_render_mode);
    }

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

pub(super) fn ordered_list_style(
    shape: OrderedListShape,
    depth: u8,
    quote_depth: u8,
    quote_render_mode: QuoteRenderMode,
) -> ParagraphStyle {
    if quote_depth > 0 {
        return quoted_style(quote_depth, 0, NORMAL_CHAR_PR, quote_render_mode);
    }

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

pub(super) fn heading_style(
    level: u8,
    quote_depth: u8,
    quote_render_mode: QuoteRenderMode,
) -> ParagraphStyle {
    match level {
        1 => paragraph_style(2, 2, H1_CHAR_PR, quote_depth, quote_render_mode),
        2 => paragraph_style(3, 3, H2_CHAR_PR, quote_depth, quote_render_mode),
        3 => paragraph_style(4, 4, H3_CHAR_PR, quote_depth, quote_render_mode),
        4 => paragraph_style(5, 5, H4_CHAR_PR, quote_depth, quote_render_mode),
        5 => paragraph_style(6, 6, BOLD_CHAR_PR, quote_depth, quote_render_mode),
        _ => paragraph_style(7, 7, BOLD_CHAR_PR, quote_depth, quote_render_mode),
    }
}

fn paragraph_style(
    para_pr: u32,
    style: u32,
    default_char_pr: u32,
    quote_depth: u8,
    quote_render_mode: QuoteRenderMode,
) -> ParagraphStyle {
    if quote_depth == 0 {
        return ParagraphStyle {
            para_pr,
            style,
            default_char_pr,
            line_seg: LineSegProfile::Standard,
            list_semantic: ListSemantic::None,
        };
    }

    quoted_style(quote_depth, style, default_char_pr, quote_render_mode)
}

fn quoted_style(
    quote_depth: u8,
    style: u32,
    default_char_pr: u32,
    quote_render_mode: QuoteRenderMode,
) -> ParagraphStyle {
    if quote_render_mode == QuoteRenderMode::LegacyQuoteOnly {
        return ParagraphStyle {
            para_pr: LEGACY_QUOTE_PARA_PR,
            style,
            default_char_pr: LEGACY_QUOTE_CHAR_PR,
            line_seg: LineSegProfile::QuoteLegacy,
            list_semantic: ListSemantic::None,
        };
    }

    let capped_depth = quote_depth.min(2);
    ParagraphStyle {
        para_pr: if capped_depth == 1 {
            QUOTE_LEVEL1_PARA_PR
        } else {
            QUOTE_LEVEL2_PARA_PR
        },
        style,
        default_char_pr,
        line_seg: if capped_depth == 1 {
            LineSegProfile::QuoteLevel1
        } else {
            LineSegProfile::QuoteLevel2
        },
        list_semantic: ListSemantic::None,
    }
}

pub(super) fn plain_paragraph_item(
    text: String,
    quote_depth: u8,
    quote_render_mode: QuoteRenderMode,
) -> SectionItem {
    let style = normal_style(quote_depth, quote_render_mode);
    SectionItem::Paragraph(
        style,
        vec![RunSpec::Text {
            char_pr: style.default_char_pr,
            text,
        }],
    )
}

pub(super) fn unsupported_name(block: &Block) -> &'static str {
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
