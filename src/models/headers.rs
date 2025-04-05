use strum::Display;

#[derive(Display)]
pub enum Headers {
    #[strum(to_string = "Host")]
    Host,
    #[strum(to_string = "User-Agent")]
    UserAgent,
    #[strum(to_string = "Content-Type")]
    ContentType,
    #[strum(to_string = "Accept")]
    Accept,
    #[strum(to_string = "Accept-Encoding")]
    AcceptEncoding,
    #[strum(to_string = "Content-Length")]
    ContentLength,
}
