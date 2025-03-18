mod models;
mod routes;

#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    collections::HashMap,
    env,
    error::Error,
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
};

use codecrafters_http_server::ThreadPool;
use models::{
    request::{ReqError, Request},
    response::{Response, Status},
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    if let Some(file_path) = read_dir_name_from_env() {
        let _ = fs::create_dir_all(file_path);
    };

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    let pool = ThreadPool::new(5);

    for stream in listener.incoming() {
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
        body: request.user_agent.clone(),
        content_length: request.user_agent.len(),
        content_type: String::from("text/plain"),
        status: Status::Ok,
    };

    let response = format!("{}", response);
    println!("User Agent response: {}", response);

    if let Err(e) = stream.write_all(response.into_bytes().as_slice()) {
        eprintln!("Failed to write response: {:?}", e); // Prevent shutdown on a failed write
    }

    Result::Ok(())
}

fn read_dir_name_from_env() -> Option<String> {
    let file_path = env::args().last();

    // let mut path_vec = file_path
    //     .split("/")
    //     .filter(|f| !f.is_empty())
    //     .collect::<Vec<_>>();

    println!("Path {:?}", file_path);
    // println!("Path vec {:?}", path_vec);
    //
    file_path
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
                    body: res,
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

fn handle_connection(mut stream: TcpStream) {
    let mut routes: HashMap<String, fn(&Request, &mut TcpStream) -> Result<(), Box<dyn Error>>> =
        HashMap::new();

    routes.insert(String::from("/"), root);
    routes.insert(String::from("echo"), echo);
    routes.insert(String::from("user-agent"), user_agent);
    routes.insert(String::from("files"), files);

    let buf_reader = BufReader::new(&stream);

    let req: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Req: {:?}", req);

    let req = Request::try_from(req).unwrap();

    let a: Vec<_> = req.path.split("/").filter(|x| !x.is_empty()).collect::<_>();

    let a = match a.first() {
        Some(s) => String::from(*s),
        None => String::from("/"),
    };

    println!("{:?}", a);

    match routes.get_key_value(&a) {
        Some((route, handler)) => {
            handler(&req, &mut stream);
        }
        None => {
            send_404(&req, &mut stream);
        }
    }
}

fn echo(reguest: &Request, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let req_path: Vec<&str> = reguest.path.split("/").collect();
    let response_body = match req_path.last() {
        Some(s) => String::from(*s),
        None => String::new(),
    };

    let response = Response {
        status: models::response::Status::Ok,
        content_type: String::from("text/plain"),
        content_length: response_body.len(),
        body: response_body,
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

fn send_404(request: &Request, stream: &mut TcpStream) {
    let str = String::from("HTTP/1.1 404 Not Found\r\n\r\n");

    let buf = str.into_bytes();

    println!("sending 404");
    if let Err(e) = stream.write_all(buf.as_slice()) {
        eprintln!("Failed to write response: {:?}", e); // Prevent shutdown on a failed write
    }
}
