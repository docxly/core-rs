#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Inline {
    Text(String),
    Emphasis(Vec<Inline>),
    Strong(Vec<Inline>),
    Code(String),
    Link {
        text: Vec<Inline>,
        url: String,
    },
    Image(ImageData),
    HardBreak,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageData {
    pub alt: String,
    pub title: Option<String>,
    pub mime_type: String,
    pub extension: String,
    pub data: Vec<u8>,
}
