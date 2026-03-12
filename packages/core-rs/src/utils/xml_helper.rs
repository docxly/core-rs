use roxmltree::Node;

use crate::error::CoreRsError;

pub fn escape_text(input: &str) -> String {
    sanitize_xml_chars(input)
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub fn escape_attr(input: &str) -> String {
    escape_text(input)
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

pub fn normalized_node_text(node: Node<'_, '_>) -> String {
    node.text().unwrap_or_default().trim().to_string()
}

pub fn required_u32_attr(node: Node<'_, '_>, attr: &str, label: &str) -> Result<u32, CoreRsError> {
    let value = node
        .attribute(attr)
        .ok_or_else(|| CoreRsError::InvalidHwpx(format!("missing {attr} attribute on {label}")))?;
    value.parse::<u32>().map_err(|error| {
        CoreRsError::InvalidHwpx(format!("invalid {attr} attribute on {label}: {error}"))
    })
}

fn sanitize_xml_chars(input: &str) -> String {
    input
        .chars()
        .map(|ch| {
            if is_valid_xml_char(ch) {
                ch
            } else {
                '\u{FFFD}'
            }
        })
        .collect()
}

const fn is_valid_xml_char(ch: char) -> bool {
    matches!(
        ch as u32,
        0x9 | 0xA | 0xD | 0x20..=0xD7FF | 0xE000..=0xFFFD | 0x10000..=0x10FFFF
    )
}

#[cfg(test)]
mod tests {
    use roxmltree::Document;

    use super::{escape_attr, escape_text, normalized_node_text, required_u32_attr};

    #[test]
    fn replaces_invalid_xml_characters() {
        assert_eq!(escape_text("a\u{0000}b\u{000B}c"), "a\u{FFFD}b\u{FFFD}c");
        assert_eq!(escape_attr("x\u{001F}\"'"), "x\u{FFFD}&quot;&apos;");
    }

    #[test]
    fn parses_normalized_text_and_required_u32_attr() {
        let xml = Document::parse(r#"<root value="42">  hello  </root>"#).unwrap();
        let node = xml.root_element();

        assert_eq!(normalized_node_text(node), "hello");
        assert_eq!(required_u32_attr(node, "value", "root").unwrap(), 42);
    }
}
