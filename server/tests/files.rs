mod test_utils;

#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::{BufReader, Read, Seek, Write},
        sync::Arc,
        thread,
    };

    use server::{
        app::{App, ServerResponse},
        models::{
            content_type::ContentType,
            request::Request,
            response::{IntoResponse, Response},
            status::Status,
        },
        router::Router,
    };

    use crate::test_utils::{BASE_URL, wait_until_server_ready};

    const TEST_FILE_PATH: &str = "/tmp";
    const TEST_FILE_NAME: &str = "test-file";
    const TEST_FILE_CONTENT: &str = "this is a test file";

    fn setup() -> Arc<App> {
        let router = Router::new("files").get("", files).post("", files_body);
        App::new(BASE_URL).with_router(router).build()
    }

    #[test]
    fn return_file_content() {
        let _ = fs::remove_file(format!("{}/{}", TEST_FILE_PATH, TEST_FILE_NAME));
        let Ok(mut file) = fs::File::create_new(format!("{}/{}", TEST_FILE_PATH, TEST_FILE_NAME))
        else {
            panic!("Couldn't create file");
        };

        file.write_all(TEST_FILE_CONTENT.as_bytes())
            .expect("Couldn't write to file");

        let app = setup();

        // Clone for the server thread
        let server = Arc::clone(&app);
        let handle = thread::spawn(move || {
            server.run();
        });

        wait_until_server_ready(BASE_URL);

        let client = reqwest::blocking::Client::new();

        // Send the request
        let res = client
            .get(format!("http://{}/files", BASE_URL))
            .query(&[("q", "test-file")])
            .build()
            .unwrap();

        let res = client.execute(res).unwrap();

        // Verify response
        assert_eq!(res.status(), 200);
        assert!(res.text().unwrap().contains(TEST_FILE_CONTENT));

        // Shut down srver
        app.shutdown();
        handle.join().unwrap();
    }

    #[test]
    fn save_file() {
        let _ = fs::remove_file(format!("{}/{}", TEST_FILE_PATH, TEST_FILE_NAME));

        let app = setup();

        // Clone for the server thread
        let server = Arc::clone(&app);
        let handle = thread::spawn(move || {
            server.run();
        });

        wait_until_server_ready(BASE_URL);

        let client = reqwest::blocking::Client::new();

        // Send the request
        let res = client
            .post(format!("http://{}/files", BASE_URL))
            .body(TEST_FILE_CONTENT)
            .build()
            .unwrap();

        let res = client.execute(res).unwrap();

        // Verify response
        assert_eq!(res.status(), 201);
        assert!(res.text().unwrap().contains(TEST_FILE_CONTENT));

        app.shutdown();
        handle.join().unwrap();
    }

    fn files(req: &Request, res: Response) -> ServerResponse {
        let file_path = TEST_FILE_PATH;

        let Ok(f) = fs::File::open(format!("{}/{}", file_path, req.query)) else {
            return res.status(Status::NotFound).into_response();
        };

        let mut reader = BufReader::new(f);

        let mut result = String::new();

        reader.read_to_string(&mut result)?;

        res.body(result.into_bytes())
            .content_type(ContentType::OctetStream)
            .status(Status::Ok)
            .into_response()
    }

    fn files_body(request: &Request, res: Response) -> ServerResponse {
        let dir = TEST_FILE_PATH;

        fs::create_dir_all(dir)?;

        let file_path_name = format!("{}/{}", dir, TEST_FILE_NAME);
        let mut file = fs::File::create_new(&file_path_name)?;

        file.write_all(request.body.as_bytes())?;
        file.seek(std::io::SeekFrom::Start(0))?;

        let mut body = String::new();
        file.read_to_string(&mut body)?;

        res.status(Status::Created)
            .content_type(ContentType::OctetStream)
            .body(body.into_bytes())
            .into()
    }
}
