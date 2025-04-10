pub mod app;
pub mod models;
pub mod router;
pub mod thread_pool;

const BASE_URL: &str = "127.0.0.1:4221";

#[cfg(test)]
mod tests {

    use std::{net::TcpStream, sync::Arc, thread, time::Duration};

    use crate::{
        app::{App, ServerResponse},
        models::{request::Request, response::Response, status::Status},
    };

    use super::*;

    // #[test]
    // fn root() {
    //     App::new(BASE_URL).get("/", root_handler).build().run();
    //
    //     let Ok(res) = reqwest::blocking::get(BASE_URL) else {
    //         panic!("Couldn't send request to the server");
    //     };
    //
    //     assert_eq!(res.status(), 200);
    // }

    #[test]
    fn root() {
        // Build the server
        let app = App::new(BASE_URL).get("/", root_handler).build();

        // Clone for the server thread
        let server = Arc::clone(&app);
        let handle = thread::spawn(move || {
            server.run();
        });

        // 🕓 Wait until the server has started and is ready to accept connections
        wait_until_server_ready(BASE_URL);

        // Send the request
        let res = reqwest::blocking::get(format!("http://{}", BASE_URL))
            .expect("Couldn't send request to the server");

        // Verify response
        assert_eq!(res.status(), 200);

        // Shut down server
        app.shutdown();
        handle.join().unwrap();
    }

    /// Try to connect to the server until it responds or timeout
    fn wait_until_server_ready(addr: &str) {
        for _ in 0..10 {
            if TcpStream::connect(addr).is_ok() {
                return;
            }
            thread::sleep(Duration::from_millis(50));
        }
        panic!("Server did not become ready in time");
    }

    /// Basic handler for `/`
    fn root_handler(_: &Request, res: Response) -> ServerResponse {
        res.status(Status::Ok).into()
    }
}
