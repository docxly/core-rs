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
    use super::{escape_attr, escape_text};

    #[test]
    fn replaces_invalid_xml_characters() {
        assert_eq!(escape_text("a\u{0000}b\u{000B}c"), "a\u{FFFD}b\u{FFFD}c");
        assert_eq!(escape_attr("x\u{001F}\"'"), "x\u{FFFD}&quot;&apos;");
    }
}
