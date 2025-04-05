mod models;
mod router;
mod server;

use std::{
    env,
    error::Error,
    fs::{self},
    io::{BufReader, Read, Write},
    net::TcpStream,
    os,
};

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

    // let file_router = Router::new("files").get("", files).post("", files_body);
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

fn user_agent(request: &Request, mut res: Response) -> Result<Response, Box<dyn Error>> {
    res.body = Some(request.user_agent.clone().into_bytes());
    res.status = Status::Ok;

    Ok(res)
}

fn read_dir_name_from_env() -> Option<String> {
    env::args().last()
}

fn files(req: &Request, mut res: Response) -> Result<Response, Box<dyn Error>> {
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
        res.status = Status::NotFound;

        return Ok(res);
    };

    println!("Path: {}", path);

    let Ok(f) = fs::File::open(format!("{}{}", file_path.unwrap(), path)) else {
        res.status = Status::NotFound;
        println!("ugala");
        return Ok(res);
    };

    // match fs::File::open(format!("{}{}", file_path.unwrap(), path)) {
    //     Ok(f) => {
    let mut reader = BufReader::new(f);

    let mut result = String::new();

    reader.read_to_string(&mut result)?;

    res.body = Some(result.into_bytes());
    res.content_type = ContentType::OctetStream;
    res.status = Status::Ok;

    Ok(res)

    // }
    // Err(_) => {
    //     let _ = Response::from(None, ContentType::TextPlain, "", Status::NotFound).send(res);
    // }
    // }
}

fn root(req: &Request, mut res: Response) -> Result<Response, Box<dyn Error>> {
    res.content_type = ContentType::TextPlain;
    res.status = Status::Ok;
    res.encoding_type = EncodingType::None;

    Ok(res)
}

fn files_body(request: &Request, mut res: Response) -> Result<Response, Box<dyn Error>> {
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

        res.content_type = ContentType::OctetStream;
        res.status = Status::Created;
    };

    Ok(res)
}

fn echo(request: &Request, mut res: Response) -> Result<Response, Box<dyn Error>> {
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
    res.body = Some(body);
    res.content_type = ContentType::TextPlain;
    res.encoding_type = encoding;

    Ok(res)
}
