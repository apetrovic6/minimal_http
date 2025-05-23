use std::{
    error::Error,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use crate::models::headers::Header;

use super::{encoding::EncodingType, method::Method};

#[derive(Debug, Default)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub query: String,
    pub host: String,
    pub user_agent: String,
    pub accept: String,
    pub content_type: String,
    pub content_length: usize,
    pub accept_encoding: Vec<EncodingType>,
    pub body: String,
}

#[derive(Debug)]
pub struct ReqError {
    pub msg: String,
}

impl Request {
    fn parse_method_and_path(strings: Vec<&str>) -> Result<(String, Method, String), ReqError> {
        let [method, path, _]: [_; 3] = strings.try_into().ok().unwrap();

        let method = match method.parse::<Method>() {
            Ok(m) => m,
            Err(_) => {
                return Err(ReqError {
                    msg: String::from("Coudn't parse method type"),
                });
            }
        };

        let (path_part, query_part) = path.split_once('?').unwrap_or((path, ""));
        let path = path_part.trim_start_matches('/').to_string();

        let query_value = query_part.split('=').nth(1).unwrap_or("none");

        Ok((path, method, query_value.to_string()))
    }

    fn parse_string_from_header(query: Header, headers: &[String]) -> String {
        headers
            .iter()
            .find(|s| s.contains(&query.to_string()))
            .and_then(|s| s.split_whitespace().last())
            .map(ToString::to_string)
            .unwrap_or_default()
    }

    fn parse_encodings_from_header(query: Header, headers: &[String]) -> Vec<String> {
        headers
            .iter()
            .find(|s| s.contains(&query.to_string()))
            .map(|s| {
                s.split_whitespace()
                    .map(ToString::to_string)
                    .skip(1)
                    .map(|w| w.trim_end_matches(',').to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    fn parse_header_and_body(
        request: &mut Request,
        stream: &TcpStream,
    ) -> Result<(), Box<dyn Error>> {
        let mut buf_reader = BufReader::new(stream);

        // 1) Read lines until empty line -> headers
        let mut headers = Vec::new();
        loop {
            let mut line = String::new();
            let bytes_read = buf_reader.read_line(&mut line).unwrap();
            if bytes_read == 0 || line.trim().is_empty() {
                break; // end of headers
            }
            headers.push(line);
        }

        let content_length = Self::parse_string_from_header(Header::ContentLength, &headers)
            .parse::<usize>()
            .unwrap_or_default();

        let method_path: Vec<&str> = headers
            .first()
            .ok_or("No request line found in headers")?
            .split(' ')
            .collect();

        // TODO: Try figuring out the path with PathBuf::from()
        let (path, method, query) = Self::parse_method_and_path(method_path).unwrap();

        let host = Self::parse_string_from_header(Header::Host, &headers);
        let user_agent = Self::parse_string_from_header(Header::UserAgent, &headers);
        let content_type = Self::parse_string_from_header(Header::ContentType, &headers);
        let accept = Self::parse_string_from_header(Header::Accept, &headers);
        let accept_encoding = Self::parse_encodings_from_header(Header::AcceptEncoding, &headers);

        let mut body_bytes = vec![0u8; content_length];

        let _ = buf_reader.read_exact(&mut body_bytes);

        let encodings = accept_encoding
            .iter()
            .map(|e| e.parse::<EncodingType>().unwrap())
            .collect::<Vec<_>>();

        request.host = host;
        request.content_type = content_type;
        request.accept = accept;
        request.user_agent = user_agent;
        request.content_length = content_length;
        request.method = method;
        request.path = path;
        request.query = query;
        request.accept_encoding = encodings;
        request.body = String::from_utf8(body_bytes).unwrap();

        Ok(())
    }
}

impl TryFrom<&mut TcpStream> for Request {
    type Error = ReqError;

    fn try_from(stream: &mut TcpStream) -> Result<Self, Self::Error> {
        let mut request = Self::default();

        Self::parse_header_and_body(&mut request, stream).map_err(|_| ReqError {
            msg: "Something went wrong".to_string(),
        })?;

        Ok(request)
    }
}
