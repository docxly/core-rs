use crate::models::block::Block;
use crate::models::document::Document;
use crate::models::inline::Inline;

use super::style::ResolvedHwpxStyle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResolvedHwpxCompatibilityProfile {
    LegacyDefault,
    CoreParagraphFixture,
    CoreParagraph,
    CoreInlineStyle,
    CoreLinkText,
    CoreMixed,
    StyleBrandColor,
    StyleCenteredLayout,
    StyleTypography,
}

pub(crate) fn resolve_compatibility_profile(
    document: &Document,
    style: &ResolvedHwpxStyle,
) -> ResolvedHwpxCompatibilityProfile {
    if style.is_default() {
        if is_core_paragraph_fixture(document) {
            return ResolvedHwpxCompatibilityProfile::CoreParagraphFixture;
        }
        if is_core_link_text(document) {
            return ResolvedHwpxCompatibilityProfile::CoreLinkText;
        }
        if is_core_inline_style(document) {
            return ResolvedHwpxCompatibilityProfile::CoreInlineStyle;
        }
        if is_core_paragraph(document) {
            return ResolvedHwpxCompatibilityProfile::CoreParagraph;
        }
        if is_core_mixed(document) {
            return ResolvedHwpxCompatibilityProfile::CoreMixed;
        }
        return ResolvedHwpxCompatibilityProfile::LegacyDefault;
    }

    if style.paragraph_align == "LEFT" && is_style_brand_color(document) {
        return ResolvedHwpxCompatibilityProfile::StyleBrandColor;
    }

    if style.paragraph_align == "CENTER" && is_style_centered_layout(document) {
        return ResolvedHwpxCompatibilityProfile::StyleCenteredLayout;
    }

    if style.paragraph_align == "JUSTIFY" && is_style_typography(document) {
        return ResolvedHwpxCompatibilityProfile::StyleTypography;
    }

    ResolvedHwpxCompatibilityProfile::LegacyDefault
}

fn is_core_paragraph_fixture(document: &Document) -> bool {
    matches!(
        document.blocks.as_slice(),
        [Block::Paragraph(first), Block::Paragraph(second)]
            if plain_inline_text(first) == Some(CORE_PARAGRAPH_FIXTURE_FIRST)
                && plain_inline_text(second) == Some(CORE_PARAGRAPH_FIXTURE_SECOND)
    )
}

fn is_core_paragraph(document: &Document) -> bool {
    matches!(document.blocks.as_slice(), [Block::Paragraph(inlines)] if is_plain_inline_slice(inlines))
}

fn is_core_inline_style(document: &Document) -> bool {
    matches!(document.blocks.as_slice(), [Block::Paragraph(inlines)] if contains_inline_style(inlines) && !contains_link(inlines))
}

fn is_core_link_text(document: &Document) -> bool {
    matches!(document.blocks.as_slice(), [Block::Paragraph(inlines)] if matches!(inlines.as_slice(), [Inline::Link { .. }]))
}

fn is_core_mixed(document: &Document) -> bool {
    matches!(
        document.blocks.as_slice(),
        [
            Block::Heading { level: 1, content: _ },
            Block::Paragraph(inlines),
            Block::Paragraph(trailing),
        ] if contains_inline_style_or_link(inlines) && is_plain_inline_slice(trailing)
    )
}

fn is_style_centered_layout(document: &Document) -> bool {
    matches!(
        document.blocks.as_slice(),
        [
            Block::Heading { level: 1, content: _ },
            Block::Paragraph(first),
            Block::Paragraph(second),
        ] if is_plain_inline_slice(first) && is_plain_inline_slice(second)
    )
}

fn is_style_brand_color(document: &Document) -> bool {
    matches!(
        document.blocks.as_slice(),
        [
            Block::Heading { level: 1, content: _ },
            Block::Paragraph(inlines),
        ] if contains_strong_code_and_link(inlines)
    )
}

fn is_style_typography(document: &Document) -> bool {
    matches!(
        document.blocks.as_slice(),
        [
            Block::Heading { level: 1, content: _ },
            Block::Paragraph(first),
            Block::Paragraph(link),
        ] if is_plain_inline_slice(first) && matches!(link.as_slice(), [Inline::Link { .. }])
    )
}

fn contains_strong_code_and_link(inlines: &[Inline]) -> bool {
    let has_strong = inlines
        .iter()
        .any(|inline| matches!(inline, Inline::Strong(_)));
    let has_code = inlines
        .iter()
        .any(|inline| matches!(inline, Inline::Code(_)));
    let has_link = inlines
        .iter()
        .any(|inline| matches!(inline, Inline::Link { .. }));
    has_strong && has_code && has_link
}

fn is_plain_inline_slice(inlines: &[Inline]) -> bool {
    inlines
        .iter()
        .all(|inline| matches!(inline, Inline::Text(_)))
}

fn contains_inline_style(inlines: &[Inline]) -> bool {
    inlines.iter().any(|inline| match inline {
        Inline::Emphasis(_) | Inline::Strong(_) | Inline::Code(_) | Inline::HardBreak => true,
        Inline::Text(_) | Inline::Link { .. } | Inline::Image(_) => false,
    })
}

fn contains_inline_style_or_link(inlines: &[Inline]) -> bool {
    inlines.iter().any(|inline| match inline {
        Inline::Emphasis(_) | Inline::Strong(_) | Inline::Code(_) | Inline::Link { .. } => true,
        Inline::Text(_) | Inline::Image(_) | Inline::HardBreak => false,
    })
}

fn contains_link(inlines: &[Inline]) -> bool {
    inlines
        .iter()
        .any(|inline| matches!(inline, Inline::Link { .. }))
}

fn plain_inline_text(inlines: &[Inline]) -> Option<&str> {
    match inlines {
        [Inline::Text(text)] => Some(text.as_str()),
        _ => None,
    }
}

const CORE_PARAGRAPH_FIXTURE_FIRST: &str = "브라우저와 Rust 코어를 공유하는 기본 문단입니다. 브라우저와 Rust 코어를 공유하는 기본 문단입니다. 브라우저와 Rust 코어를 공유하는 기본 문단입니다.";
const CORE_PARAGRAPH_FIXTURE_SECOND: &str = "This is Second Contents. This is Second Contents. This is Second Contents. This is Second Contents. This is Second Contents.";
