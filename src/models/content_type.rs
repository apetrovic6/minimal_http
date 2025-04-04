use std::fmt::Display;

#[derive(Debug, Default)]
pub enum ContentType {
    #[default]
    TextPlain,
    OctetStream,
    Json,
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::TextPlain => write!(f, "text/plain"),
            ContentType::OctetStream => write!(f, "application/octet-stream"),
            ContentType::Json => write!(f, "application/json"),
        }
    }
}
