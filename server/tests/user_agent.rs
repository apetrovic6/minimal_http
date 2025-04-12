mod test_utils;

#[cfg(test)]
mod tests {

    use std::{sync::Arc, thread};

    use reqwest::header::USER_AGENT;
    use server::{
        app::{App, ServerResponse},
        models::{request::Request, response::Response, status::Status},
    };

    use crate::test_utils::{BASE_URL, wait_until_server_ready};

    #[test]
    fn user_agent() {
        let app = App::new(BASE_URL)
            .get("usr-agent", user_agent_handler)
            .build();

        let server = Arc::clone(&app);
        let handle = thread::spawn(move || server.run());

        wait_until_server_ready(BASE_URL);

        let client = reqwest::blocking::Client::new();

        // Create a custom User-Agent string
        let custom_user_agent = "reqwest";

        let res = client
            .get(format!("http://{}/usr-agent", BASE_URL))
            .header(USER_AGENT, custom_user_agent)
            .build()
            .unwrap();

        println!("boga isusa {:?}", res);

        let res = client.execute(res).unwrap();

        assert_eq!(res.status(), 200);

        let body = res.text().expect("Failed to read response body");
        assert!(
            body.contains("reqwest"),
            "Expected 'reqwest' in body, got: '{}'",
            body
        );

        app.shutdown();
        handle.join().unwrap();
    }

    fn user_agent_handler(req: &Request, res: Response) -> ServerResponse {
        res.body(req.user_agent.clone().into_bytes())
            .status(Status::Ok)
            .into()
    }
}
