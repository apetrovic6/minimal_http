mod test_utils;

#[cfg(test)]
mod tests {

    use std::{sync::Arc, thread};

    use server::{
        app::{App, ServerResponse},
        models::{
            content_type::ContentType, encoding::EncodingType, request::Request, response::Response,
        },
    };

    use crate::test_utils::{BASE_URL, wait_until_server_ready};

    #[test]
    fn echo() {
        let app = App::new(BASE_URL).get("echo", echo_handler).build();

        let server = Arc::clone(&app);
        let handle = thread::spawn(move || server.run());

        wait_until_server_ready(BASE_URL);

        let res = reqwest::blocking::get(format!("http://{}/echo/testing", BASE_URL))
            .expect("Couldn't send request to the server");

        assert_eq!(res.status(), 200);
        assert!(res.text().unwrap().contains("testing"));

        app.shutdown();
        handle.join().unwrap();
    }

    fn echo_handler(req: &Request, res: Response) -> ServerResponse {
        let req_path: Vec<&str> = req.path.split("/").collect();

        let response_body = match req_path.last() {
            Some(s) => String::from(*s),
            None => String::new(),
        };

        let encoding = if req.accept_encoding.contains(&EncodingType::Gzip) {
            EncodingType::Gzip
        } else {
            EncodingType::None
        };

        let body = Response::encode_payload(response_body, &encoding);

        res.body(body)
            .content_type(ContentType::TextPlain)
            .encoding_type(encoding)
            .into()
    }
}
