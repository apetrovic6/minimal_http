use super::{content_type::ContentType, encoding::EncodingType, status::Status};
use flate2::{write::GzEncoder, Compression};
use std::{error::Error, io::Write, net::TcpStream};

#[derive(Debug, Default)]
pub struct Response {
    pub status: Status,
    pub content_type: ContentType,
    pub content_length: usize,
    pub content_encoding: String,
    pub encoding_type: EncodingType,
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
            content_type,
            content_length: body.get_or_insert(Vec::new()).len(),
            content_encoding: content_encoding.into(),
            body,
            ..Default::default()
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

    pub fn encode_payload<T: Into<Vec<u8>>>(payload: T, encoding_type: &EncodingType) -> Vec<u8> {
        if *encoding_type == EncodingType::Gzip {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&payload.into()).unwrap();
            encoder.finish().unwrap()
        } else {
            Vec::from(payload.into())
        }
    }
}
