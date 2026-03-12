use crate::models::block::Block;
use crate::models::document::Document;
use crate::models::inline::Inline;

use super::style::ResolvedHwpxStyle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResolvedHwpxCompatibilityProfile {
    LegacyDefault,
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

fn is_core_paragraph(document: &Document) -> bool {
    !document.blocks.is_empty()
        && document
            .blocks
            .iter()
            .all(|block| matches!(block, Block::Paragraph(inlines) if is_plain_inline_slice(inlines)))
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
