use std::{
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use crate::models::headers::Headers;

use super::{encoding::EncodingType, method::Method};

#[derive(Debug, Default)]
pub struct Request {
    pub method: Method,
    pub path: String,
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
    fn parse_method_and_path(strings: Vec<&str>) -> Result<(String, Method), ReqError> {
        let [method, path, _]: [_; 3] = strings.try_into().ok().unwrap();

        let method = match method.parse::<Method>() {
            Ok(m) => m,
            Err(_) => {
                return Err(ReqError {
                    msg: String::from("Wrong Error"),
                })
            }
        };

        let path = path.split('/').collect::<Vec<&str>>().join("/");

        Ok((path, method))
    }

    fn parse_string_from_header(query: &str, headers: &[String]) -> String {
        headers
            .iter()
            .find(|s| s.contains(query))
            .and_then(|s| s.split_whitespace().last())
            .map(ToString::to_string)
            .unwrap_or_default()
    }

    fn parse_encodings_from_header(query: &str, headers: &[String]) -> Vec<String> {
        headers
            .iter()
            .find(|s| s.contains(query))
            .map(|s| {
                s.split_whitespace()
                    .map(ToString::to_string)
                    .skip(1)
                    .map(|w| w.trim_end_matches(',').to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    fn parse_header_and_body(request: &mut Request, stream: &TcpStream) {
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

        let content_length =
            Self::parse_string_from_header(Headers::ContentLength.to_string().as_str(), &headers)
                .parse::<usize>()
                .unwrap_or_default();

        let method_path: Vec<&str> = headers.first().unwrap().split(' ').collect();

        // TODO: Try figuring out the path with PathBuf::from()
        let (path, method) = Self::parse_method_and_path(method_path).unwrap();

        println!("Headers: {:?}", headers);

        let host = Self::parse_string_from_header("Host", &headers);
        let user_agent = Self::parse_string_from_header("User-Agent", &headers);
        let content_type = Self::parse_string_from_header("Content-Type", &headers);
        let accept = Self::parse_string_from_header("Accept", &headers);
        let accept_encoding = Self::parse_encodings_from_header("Accept-Encoding", &headers);

        let mut body_bytes = vec![0u8; content_length];

        let _ = buf_reader.read_exact(&mut body_bytes);

        println!("Body: {:?}", body_bytes);
        println!("Accept Encoding header: {:?}", accept_encoding);

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
        request.accept_encoding = encodings;
        request.body = String::from_utf8(body_bytes).unwrap();
    }

    pub fn get_encoding(&self) -> EncodingType {
        if self.accept_encoding.contains(&EncodingType::Gzip) {
            EncodingType::Gzip
        } else {
            EncodingType::None
        }
    }
}

impl TryFrom<&mut TcpStream> for Request {
    type Error = ReqError;

    fn try_from(stream: &mut TcpStream) -> Result<Self, Self::Error> {
        let mut request = Self::default();

        Self::parse_header_and_body(&mut request, stream);
        println!("Request: {:?}", request);
        Ok(request)
    }
}
