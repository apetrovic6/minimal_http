use std::{error::Error, io::Write, net::TcpStream};

use super::{content_type::ContentType, status::Status};

#[derive(Debug, Default)]
pub struct Response {
    pub status: Status,
    pub content_type: ContentType,
    pub content_length: usize,
    pub content_encoding: String,
    pub body: Option<Vec<u8>>,
}

impl Response {
    pub fn from(
        mut body: Option<Vec<u8>>,
        content_type: ContentType,
        content_encoding: impl Into<String>,
        status: Status,
    ) -> Self {
        Self {
            status,
            content_type: content_type.into(),
            content_length: body.get_or_insert(Vec::new()).len(),
            content_encoding: content_encoding.into(),
            body,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let body = self.body.as_deref().unwrap_or(&[]);

        let headers = format!(
            "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nContent-Encoding: {}\r\n\r\n",
            self.status, self.content_type, self.content_length, self.content_encoding
        );
        let mut response = Vec::with_capacity(headers.len() + body.len());

        response.extend_from_slice(headers.as_bytes());
        response.extend_from_slice(body);

        response
    }

    pub fn send(&self, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
        if let Err(e) = stream.write_all(&self.to_bytes()) {
            eprintln!("Failed to write response: {:?}", e); // Prevent shutdown on a failed write
        }

        Result::Ok(())
    }
}
