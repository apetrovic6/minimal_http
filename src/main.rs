mod models;
mod router;
mod server;

use std::{
    env,
    error::Error,
    fs::{self},
    io::{BufReader, Read, Write},
    net::TcpStream,
};

use flate2::{write::GzEncoder, Compression};
use models::{
    content_type::ContentType,
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
        ContentType::TextPlain,
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

                let _ = Response::from(
                    Some(res.into_bytes()),
                    ContentType::OctetStream,
                    "",
                    Status::Ok,
                )
                .send(stream);
            }
            Err(_) => {
                let _ =
                    Response::from(None, ContentType::TextPlain, "", Status::NotFound).send(stream);
            }
        }
    }
    Ok(())
}

fn root(request: &Request, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    Response::from(None, ContentType::TextPlain, "", Status::Ok).send(stream)
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

        let _ = Response::from(None, ContentType::OctetStream, "", Status::Created).send(stream);
    };
    Ok(())
}

fn echo(request: &Request, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let req_path: Vec<&str> = request.path.split("/").collect();
    let response_body = match req_path.last() {
        Some(s) => String::from(*s),
        None => String::new(),
    };

    let encoding = request.get_encoding();

    let body = Response::encode_payload(response_body, &encoding);

    Response::from(
        Some(body),
        ContentType::TextPlain,
        encoding.to_string(),
        Status::Ok,
    )
    .send(stream)
}
