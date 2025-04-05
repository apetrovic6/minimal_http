mod models;
mod router;
mod server;

use std::{
    env,
    error::Error,
    fs::{self},
    io::{BufReader, Read, Write},
};

use models::{
    content_type::ContentType,
    encoding::EncodingType,
    request::{ReqError, Request},
    response::{IntoResponse, Response},
    status::Status,
};
use router::Router;
use server::App;

fn main() {
    println!("Logs from your program will appear here!");

    if let Some(file_path) = read_dir_name_from_env() {
        let _ = fs::create_dir_all(file_path);
    };

    // let file_router = Router::new("files")
    //     .get("", files)
    //     .post("", files_body)
    //     .route(Router::new("test").get("manjo", files).delete("", files));

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

fn user_agent(request: &Request, res: Response) -> ServerResponse {
    res.body(request.user_agent.clone().into_bytes())
        .status(Status::Ok)
        .into_response()
}

fn read_dir_name_from_env() -> Option<String> {
    env::args().last()
}

fn files(req: &Request, res: Response) -> ServerResponse {
    println!("Request: {:?}", req);

    let file_path = read_dir_name_from_env();
    let path = req
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

    let Ok(path) = path else {
        return Ok(res.status(Status::NotFound));
    };

    println!("Path: {}", path);

    let Ok(f) = fs::File::open(format!("{}{}", file_path.unwrap(), path)) else {
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

fn root(_: &Request, res: Response) -> ServerResponse {
    res.content_type(ContentType::TextPlain)
        .status(Status::Ok)
        .encoding_type(EncodingType::None)
        .into_response()
}

fn files_body(request: &Request, res: Response) -> ServerResponse {
    println!("in files body: {:?}", request);

    let Some(dir) = read_dir_name_from_env() else {
        return res.status(Status::NotFound).into_response();
    };

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

    res.status(Status::Ok)
        .content_type(ContentType::OctetStream)
        .into()
}

fn echo(request: &Request, res: Response) -> ServerResponse {
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

    println!("Encoding {:?}", encoding);

    let body = Response::encode_payload(response_body, &encoding);
    println!("Body {:?}", body);

    res.body(body)
        .content_type(ContentType::TextPlain)
        .encoding_type(encoding)
        .into()
}

type ServerResponse = Result<Response, Box<dyn Error>>;
