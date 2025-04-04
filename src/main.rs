mod models;
mod router;
mod server;

#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    env,
    error::Error,
    fs::{self},
    io::{BufReader, Read, Write},
    net::TcpStream,
};

use flate2::{write::GzEncoder, Compression};
use models::{
    encoding::EncodingType,
    request::{ReqError, Request},
    response::Response,
    status::Status,
};
use router::Router;
use server::App;

fn main() {
    println!("Logs from your program will appear here!");

    if let Some(file_path) = read_dir_name_from_env() {
        let _ = fs::create_dir_all(file_path);
    };

    let file_router = Router::new("files").get("", files).post("", files_body);
    // .route(Router::new("test").get("manjo", files).delete("", files));

    App::new("127.0.0.1:4221")
        .get("/", root)
        .get("echo", echo)
        .get("user-agent", user_agent)
        .get("files", files)
        .post("files", files_body)
        // .with_router(file_router)
        .build()
        .run();

    println!("Shutting down.");
}

fn user_agent(request: &Request, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    Response::from(
        Some(request.user_agent.clone().into_bytes()),
        "text/plain",
        "",
        Status::Ok,
    )
    .send(stream)
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
                    body: Some(res.into_bytes()),
                    ..Default::default()
                };
                println!("Response: {:?}", response);

                if let Err(e) = stream.write_all(&response.to_bytes()) {
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
    // send_200(request, stream);

    Response::from(None, "text/plain", "", Status::Ok).send(stream)
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

fn echo(request: &Request, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let req_path: Vec<&str> = request.path.split("/").collect();
    let response_body = match req_path.last() {
        Some(s) => String::from(*s),
        None => String::new(),
    };

    let encoding = if request.accept_encoding.contains(&EncodingType::Gzip) {
        EncodingType::Gzip
    } else {
        EncodingType::None
    };

    let ugala = if encoding == EncodingType::Gzip {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&response_body.into_bytes()).unwrap();
        encoder.finish().unwrap()
    } else {
        Vec::from(response_body.as_bytes())
    };

    let response = Response {
        status: Status::Ok,
        content_type: String::from("text/plain"),
        content_length: ugala.len(),
        body: Some(ugala),
        content_encoding: encoding.to_string(),
    };

    if let Err(e) = stream.write_all(&response.to_bytes()) {
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
        status: Status::Created,
        content_type: String::from("text/plain"),
        content_length: 0,
        body: None,
        ..Default::default()
    };

    println!("sending 201");
    if let Err(e) = stream.write_all(&response.to_bytes()) {
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
