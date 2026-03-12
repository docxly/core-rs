use crate::error::CoreRsError;
use crate::models::block::TableBlock;
use crate::models::inline::Inline;
use crate::utils::xml_helper::required_u32_attr;

use super::super::registry::{CharStyle, ParagraphContract, Registry};
use super::table::parse_table;

#[derive(Debug, Clone)]
pub(super) struct ActiveHyperlink {
    pub(super) begin_id: u64,
    pub(super) path: String,
    pub(super) text: Vec<Inline>,
}

#[derive(Debug, Default)]
pub(super) struct RunState {
    pub(super) inlines: Vec<Inline>,
    pub(super) table: Option<TableBlock>,
    pub(super) active_hyperlink: Option<ActiveHyperlink>,
}

pub(super) fn parse_run(
    node: roxmltree::Node<'_, '_>,
    registry: &Registry,
    contract: ParagraphContract,
    is_heading: bool,
    state: &mut RunState,
) -> Result<(), CoreRsError> {
    let char_pr_id = required_u32_attr(node, "charPrIDRef", "hp:run")?;
    let char_style = if contract.legacy_quote || is_heading {
        CharStyle::Plain
    } else {
        registry.char_style(char_pr_id)
    };

    if let Some(tbl_node) = node
        .children()
        .find(|child| child.is_element() && child.tag_name().name() == "tbl")
    {
        if state.table.is_some() || !state.inlines.is_empty() {
            return Err(CoreRsError::InvalidHwpx(
                "table paragraph mixed with inline content".to_string(),
            ));
        }
        state.table = Some(parse_table(tbl_node)?);
        return Ok(());
    }

    for child in node.children().filter(|child| child.is_element()) {
        match child.tag_name().name() {
            "secPr" => {}
            "ctrl" => handle_control(child, state)?,
            "t" => {
                let target_inlines = match state.active_hyperlink.as_mut() {
                    Some(link) => &mut link.text,
                    None => &mut state.inlines,
                };
                parse_text_node(child, char_style, target_inlines)?;
            }
            other => {
                return Err(CoreRsError::UnsupportedFeature(format!(
                    "HWPX parser does not support hp:run child {other}"
                )));
            }
        }
    }

    Ok(())
}

fn handle_control(node: roxmltree::Node<'_, '_>, state: &mut RunState) -> Result<(), CoreRsError> {
    if has_col_pr(node) {
        return Ok(());
    }
    if let Some((path, begin_id)) = field_begin(node) {
        if state.active_hyperlink.is_some() {
            return Err(CoreRsError::InvalidHwpx(
                "nested hyperlink fields are not supported".to_string(),
            ));
        }
        state.active_hyperlink = Some(ActiveHyperlink {
            begin_id,
            path,
            text: Vec::new(),
        });
        return Ok(());
    }
    if let Some(end_id) = field_end(node) {
        let link = state.active_hyperlink.take().ok_or_else(|| {
            CoreRsError::InvalidHwpx("hyperlink field is missing hp:fieldBegin".to_string())
        })?;
        if link.begin_id != end_id {
            return Err(CoreRsError::InvalidHwpx(
                "hyperlink field begin/end ids do not match".to_string(),
            ));
        }
        let text = if link.text.is_empty() {
            vec![Inline::Text(link.path.clone())]
        } else {
            link.text
        };
        push_inline(
            &mut state.inlines,
            Inline::Link {
                text,
                url: link.path,
            },
        );
        return Ok(());
    }
    if has_field_like_marker(node) {
        return Err(CoreRsError::InvalidHwpx(
            "HWPX parser encountered an unsupported field control".to_string(),
        ));
    }
    if state.active_hyperlink.is_some() {
        return Err(CoreRsError::UnsupportedFeature(
            "HWPX parser does not support controls inside hyperlinks".to_string(),
        ));
    }
    Err(CoreRsError::UnsupportedFeature(
        "HWPX parser does not support this control shape".to_string(),
    ))
}

fn field_begin(node: roxmltree::Node<'_, '_>) -> Option<(String, u64)> {
    if !node.is_element() || node.tag_name().name() != "ctrl" {
        return None;
    }
    let field_begin = node
        .children()
        .find(|child| child.is_element() && child.tag_name().name() == "fieldBegin")?;
    if field_begin.attribute("type") != Some("HYPERLINK") {
        return None;
    }
    let begin_id = field_begin.attribute("id")?.parse::<u64>().ok()?;
    let path = field_begin
        .descendants()
        .find(|child| {
            child.is_element()
                && child.tag_name().name() == "stringParam"
                && child.attribute("name") == Some("Path")
        })
        .and_then(|child| child.text())
        .unwrap_or_default()
        .to_string();
    Some((path, begin_id))
}

fn field_end(node: roxmltree::Node<'_, '_>) -> Option<u64> {
    if !node.is_element() || node.tag_name().name() != "ctrl" {
        return None;
    }
    let field_end = node
        .children()
        .find(|child| child.is_element() && child.tag_name().name() == "fieldEnd")?;
    field_end.attribute("beginIDRef")?.parse::<u64>().ok()
}

fn has_col_pr(node: roxmltree::Node<'_, '_>) -> bool {
    node.children()
        .any(|child| child.is_element() && child.tag_name().name() == "colPr")
}

fn has_field_like_marker(node: roxmltree::Node<'_, '_>) -> bool {
    node.children()
        .any(|child| child.is_element() && child.tag_name().name().starts_with("field"))
}

fn parse_text_node(
    node: roxmltree::Node<'_, '_>,
    char_style: CharStyle,
    inlines: &mut Vec<Inline>,
) -> Result<(), CoreRsError> {
    let mut code_buffer = String::new();
    let mut in_code = false;

    for child in node.children() {
        if child.is_text() {
            let value = child.text().unwrap_or_default();
            if value.is_empty() {
                continue;
            }
            if in_code {
                code_buffer.push_str(value);
            } else {
                push_text_with_style(inlines, value.to_string(), char_style);
            }
            continue;
        }
        if !child.is_element() {
            continue;
        }
        match child.tag_name().name() {
            "markpenBegin" => {
                if in_code {
                    return Err(CoreRsError::InvalidHwpx(
                        "nested hp:markpenBegin is not supported".to_string(),
                    ));
                }
                in_code = true;
            }
            "markpenEnd" => {
                if !in_code {
                    return Err(CoreRsError::InvalidHwpx(
                        "hp:markpenEnd without hp:markpenBegin".to_string(),
                    ));
                }
                push_inline(inlines, Inline::Code(std::mem::take(&mut code_buffer)));
                in_code = false;
            }
            "lineBreak" => {
                if in_code {
                    return Err(CoreRsError::UnsupportedFeature(
                        "HWPX parser does not support code spans containing line breaks"
                            .to_string(),
                    ));
                }
                push_inline(inlines, Inline::HardBreak);
            }
            other => {
                return Err(CoreRsError::UnsupportedFeature(format!(
                    "HWPX parser does not support hp:t child {other}"
                )));
            }
        }
    }

    if in_code {
        return Err(CoreRsError::InvalidHwpx(
            "unterminated markpen code span".to_string(),
        ));
    }

    Ok(())
}

pub(super) fn flatten_text_node(node: roxmltree::Node<'_, '_>) -> Result<String, CoreRsError> {
    let mut text = String::new();
    for child in node.children() {
        if child.is_text() {
            text.push_str(child.text().unwrap_or_default());
            continue;
        }
        if !child.is_element() {
            continue;
        }
        match child.tag_name().name() {
            "lineBreak" => text.push('\n'),
            "markpenBegin" | "markpenEnd" => {}
            other => {
                return Err(CoreRsError::UnsupportedFeature(format!(
                    "HWPX parser does not support hyperlink text child {other}"
                )));
            }
        }
    }
    Ok(text)
}

fn push_text_with_style(inlines: &mut Vec<Inline>, text: String, char_style: CharStyle) {
    if text.is_empty() {
        return;
    }
    let inline = match char_style {
        CharStyle::Plain => Inline::Text(text),
        CharStyle::Bold => Inline::Strong(vec![Inline::Text(text)]),
        CharStyle::Italic => Inline::Emphasis(vec![Inline::Text(text)]),
    };
    push_inline(inlines, inline);
}

fn push_inline(inlines: &mut Vec<Inline>, inline: Inline) {
    match inline {
        Inline::Text(text) => {
            if text.is_empty() {
                return;
            }
            if let Some(Inline::Text(existing)) = inlines.last_mut() {
                existing.push_str(&text);
            } else {
                inlines.push(Inline::Text(text));
            }
        }
        other => inlines.push(other),
    }
}
