mod models;
mod routes;
mod server;

#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    collections::HashMap,
    default, env,
    error::Error,
    fs,
    io::{BufReader, Read, Write},
    net::TcpStream,
};

use codecrafters_http_server::ThreadPool;
use models::{
    method::Method,
    request::{self, ReqError, Request},
    response::{Response, Status},
};
use server::{App, MethodHandlerMap};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    if let Some(file_path) = read_dir_name_from_env() {
        let _ = fs::create_dir_all(file_path);
    };

    // let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    let app = App::new("127.0.0.1:4221").build();

    let pool = ThreadPool::new(5);

    for stream in app.listener.incoming() {
        match stream {
            Ok(mut _stream) => pool.execute(|| {
                handle_connection(_stream);
            }),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    println!("Shutting down.");
}

fn user_agent(request: &Request, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let response = Response {
        body: Some(request.user_agent.clone()),
        content_length: request.user_agent.len(),
        content_type: String::from("text/plain"),
        status: Status::Ok,
        ..Default::default()
    };

    let response = format!("{}", response);
    println!("User Agent response: {}", response);

    if let Err(e) = stream.write_all(response.into_bytes().as_slice()) {
        eprintln!("Failed to write response: {:?}", e); // Prevent shutdown on a failed write
    }

    Result::Ok(())
}

fn read_dir_name_from_env() -> Option<String> {
    env::args().last()
}

fn files(request: &Request, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    println!("Request: {:?}", request);

    let file_path = read_dir_name_from_env();
    let path = request
        .path
        .split("/")
        .filter(|p| !p.is_empty())
        .skip(1)
        .take(1)
        .next()
        .ok_or(ReqError {
            msg: "Bad request".to_string(),
        });

    println!("It werks? {:?}", path);

    if let Ok(path) = path {
        println!("Path: {}", path);

        match fs::File::open(format!("{}{}", file_path.unwrap(), path)) {
            Ok(f) => {
                let mut reader = BufReader::new(f);

                let mut res = String::new();

                reader.read_to_string(&mut res)?;

                let response = Response {
                    status: Status::Ok,
                    content_type: String::from("application/octet-stream"),
                    content_length: res.len(),
                    body: Some(res),
                    ..Default::default()
                };
                let res = format!("{}", response);
                println!("Response: {:?}", res);

                if let Err(e) = stream.write_all(res.into_bytes().as_slice()) {
                    eprintln!("Failed to write response: {:?}", e); // Prevent shutdown on a failed write
                }
            }
            Err(_) => {
                send_404(request, stream);
            }
        }
    }
    Ok(())
}

fn root(request: &Request, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    send_200(request, stream);

    Ok(())
}

fn files_body(request: &Request, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    println!("in files body: {:?}", request);

    if let Some(dir) = read_dir_name_from_env() {
        fs::create_dir_all(&dir)?;

        let path = request
            .path
            .split("/")
            .filter(|p| !p.is_empty())
            .skip(1)
            .take(1)
            .next()
            .ok_or(ReqError {
                msg: "Bad request".to_string(),
            });

        println!("It werks? {:?}", path);

        let file_path_name = format!("{}/{}", dir, path.unwrap());
        let mut file = fs::File::create_new(&file_path_name)?;

        file.write_all(request.body.as_bytes())?;

        println!("file path name: {:?}", &file_path_name);
        send_201(request, stream);
    }
    Ok(())
}

fn setup_routes() -> HashMap<String, MethodHandlerMap> {
    let mut routes: HashMap<String, MethodHandlerMap> = HashMap::new();

    let mut root_map: MethodHandlerMap = HashMap::new();
    root_map.insert(Method::Get, root);

    let mut echo_map: MethodHandlerMap = HashMap::new();
    echo_map.insert(Method::Get, echo);

    let mut user_agent_map: MethodHandlerMap = HashMap::new();
    user_agent_map.insert(Method::Get, user_agent);

    let mut files_map: MethodHandlerMap = HashMap::new();
    files_map.insert(Method::Get, files);
    files_map.insert(Method::Post, files_body);

    routes.insert(String::from("/"), root_map);
    routes.insert(String::from("echo"), echo_map);
    routes.insert(String::from("user-agent"), user_agent_map);
    routes.insert(String::from("files"), files_map);

    routes
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut routes = setup_routes();

    let req = Request::try_from(&mut stream).unwrap();

    println!("Req object: {:?}", req);
    //  TODO:  Extract content length and type, then read the body in
    let a: Vec<_> = req.path.split("/").filter(|x| !x.is_empty()).collect::<_>();

    let a = match a.first() {
        Some(s) => String::from(*s),
        None => String::from("/"),
    };

    println!("{:?}", a);

    match routes.get_key_value(&a) {
        Some((route, route_handler)) => {
            let entry = route_handler.get_key_value(&req.method);

            match entry {
                Some((m, handler)) => {
                    println!("Method: {:?}", m);

                    let _ = handler(&req, &mut stream);
                }
                None => send_404(&req, &mut stream),
            }

            // (route_handler.handler)(&req, &mut stream);
        }
        None => {
            send_404(&req, &mut stream);
        }
    }
    Ok(())
}

fn echo(request: &Request, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let req_path: Vec<&str> = request.path.split("/").collect();
    let response_body = match req_path.last() {
        Some(s) => String::from(*s),
        None => String::new(),
    };

    let encoding = if request.accept_encoding == "gzip" {
        String::from("gzip")
    } else {
        String::new()
    };

    let response = Response {
        status: models::response::Status::Ok,
        content_type: String::from("text/plain"),
        content_length: response_body.len(),
        body: Some(response_body),
        content_encoding: encoding,
    };

    let res = format!("{}", response);
    println!("Response: {:?}", res);

    if let Err(e) = stream.write_all(res.into_bytes().as_slice()) {
        eprintln!("Failed to write response: {:?}", e); // Prevent shutdown on a failed write
    }

    Ok(())
}

fn send_200(request: &Request, stream: &mut TcpStream) {
    let str = String::from("HTTP/1.1 200 OK\r\n\r\n");

    let buf = str.into_bytes();

    println!("sending 200");
    if let Err(e) = stream.write_all(buf.as_slice()) {
        eprintln!("Failed to write response: {:?}", e); // Prevent shutdown on a failed write
    }
}

fn send_201(request: &Request, stream: &mut TcpStream) {
    let response = Response {
        status: models::response::Status::Created,
        content_type: String::from("text/plain"),
        content_length: 0,
        body: None,
        ..Default::default()
    };

    let response = format!("{}", response);

    println!("sending 201");
    if let Err(e) = stream.write_all(response.into_bytes().as_slice()) {
        eprintln!("Failed to write response: {:?}", e); // Prevent shutdown on a failed write
    }
}

fn send_404(request: &Request, stream: &mut TcpStream) {
    let str = String::from("HTTP/1.1 404 Not Found\r\n\r\n");

    let buf = str.into_bytes();

    println!("sending 404");
    if let Err(e) = stream.write_all(buf.as_slice()) {
        eprintln!("Failed to write response: {:?}", e); // Prevent shutdown on a failed write
    }
}
