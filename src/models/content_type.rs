use strum::Display;

#[derive(Debug, Default, Display)]
pub enum ContentType {
    #[default]
    #[strum(to_string = "text/plain")]
    TextPlain,
    #[strum(to_string = "application/octet-stream")]
    OctetStream,
    #[strum(to_string = "application/json")]
    Json,
}
