mod test_utils;

#[cfg(test)]
mod tests {

    use std::{sync::Arc, thread};

    use server::{
        app::{App, ServerResponse},
        models::{request::Request, response::Response, status::Status},
    };

    use crate::test_utils::{BASE_URL, wait_until_server_ready};

    #[test]
    fn root() {
        // Build the server
        let app = App::new(BASE_URL).get("/", root_handler).build();

        // Clone for the server thread
        let server = Arc::clone(&app);
        let handle = thread::spawn(move || {
            server.run();
        });

        wait_until_server_ready(BASE_URL);

        // Send the request
        let res = reqwest::blocking::get(format!("http://{}", BASE_URL))
            .expect("Couldn't send request to the server");

        // Verify response
        assert_eq!(res.status(), 200);

        // Shut down srver
        app.shutdown();
        handle.join().unwrap();
    }

    fn root_handler(_: &Request, res: Response) -> ServerResponse {
        res.status(Status::Ok).into()
    }
}
