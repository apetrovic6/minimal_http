use strum::Display;

#[derive(Display)]
pub enum Header {
    #[strum(to_string = "host")]
    Host,
    #[strum(to_string = "user-agent")]
    UserAgent,
    #[strum(to_string = "content-type")]
    ContentType,
    #[strum(to_string = "accept")]
    Accept,
    #[strum(to_string = "accept-encoding")]
    AcceptEncoding,
    #[strum(to_string = "content-length")]
    ContentLength,
}
