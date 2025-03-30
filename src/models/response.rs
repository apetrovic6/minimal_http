use std::fmt::{self, Display};

#[derive(Debug, Default)]
pub struct Response {
    pub status: Status,
    pub content_type: String,
    pub content_length: usize,
    pub content_encoding: String,
    pub body: Option<String>,
}

impl Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let body_str = self.body.as_deref().unwrap_or("");

        write!(
            f,
            "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nContent-Encoding: {}\r\n\r\n{}",
            self.status, self.content_type, self.content_length, self.content_encoding, body_str
        )
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Status {
    #[default]
    Ok = 200,
    Created = 201,
    Accepted = 202,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let status_code = *self as i32;
        match self {
            Status::Ok => write!(f, "{} OK", status_code),
            Status::Created => write!(f, "{} Created", status_code),
            Status::Accepted => write!(f, "{} Accepted", status_code),
        }
    }
}
