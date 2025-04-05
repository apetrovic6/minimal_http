use std::fmt::Display;

#[derive(Debug, Default)]
pub enum ContentType {
    #[default]
    TextPlain,
    OctetStream,
    Json,
}

impl ContentType {
    fn description(&self) -> &'static str {
        match self {
            ContentType::TextPlain => "text/plain",
            ContentType::OctetStream => "application/octet-stream",
            ContentType::Json => "application/json",
        }
    }
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}
