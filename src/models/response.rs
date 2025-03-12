use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Response {
    pub status: Status,
    pub content_type: String,
    pub content_length: usize,
    pub body: String,
}

impl Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status, self.content_type, self.content_length, self.body
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Status {
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
