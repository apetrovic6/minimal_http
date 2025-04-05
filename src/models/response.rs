use super::{content_type::ContentType, encoding::EncodingType, status::Status};
use flate2::{write::GzEncoder, Compression};
use std::{fmt::Debug, io::Write};

pub trait IntoResponse<T> {
    fn into_response(self) -> Result<T, Box<dyn std::error::Error>>;
}

#[derive(Debug, Default)]
pub struct Response {
    status: Status,
    content_type: ContentType,
    content_length: usize,
    encoding_type: EncodingType,
    body: Option<Vec<u8>>,
}

impl From<Response> for Result<Response, Box<dyn std::error::Error>> {
    fn from(val: Response) -> Self {
        Ok(val)
    }
}

impl IntoResponse<Response> for Response {
    fn into_response(self) -> Result<Response, Box<dyn std::error::Error>> {
        Ok(self)
    }
}

impl Response {
    pub fn from(
        mut body: Option<Vec<u8>>,
        content_type: ContentType,
        status: Status,
        encoding_type: EncodingType,
    ) -> Self {
        Self {
            status,
            content_type,
            content_length: body.get_or_insert(Vec::new()).len(),
            encoding_type,
            body,
        }
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = status;

        self
    }

    pub fn content_type(mut self, content_type: ContentType) -> Self {
        self.content_type = content_type;
        self
    }

    pub fn encoding_type(mut self, encoding_type: EncodingType) -> Self {
        self.encoding_type = encoding_type;
        self
    }

    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);

        self
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let body = self.body.as_deref().unwrap_or(&[]);

        let headers =
            format!(
            "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nContent-Encoding: {}\r\n\r\n",
            self.status, self.content_type, body.len(), self.encoding_type
        );

        let mut response = Vec::with_capacity(headers.len() + body.len());

        response.extend_from_slice(headers.as_bytes());
        response.extend_from_slice(body);

        response
    }

    pub fn encode_payload<T>(payload: T, encoding_type: &EncodingType) -> Vec<u8>
    where
        T: Debug + Into<Vec<u8>>,
    {
        if *encoding_type == EncodingType::Gzip {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&payload.into()).unwrap();
            encoder.finish().unwrap()
        } else {
            payload.into()
        }
    }
}
