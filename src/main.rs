mod models;

#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use models::{
    request::Request,
    response::{Response, Status},
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let s = Status::Ok;
                let a = format!("{}", s);

                println!("{}", a);
                println!("in stream");
                handle_connection(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let req: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Req: {:?}", req);

    let req = Request::try_from(req).unwrap();

    println!("{:?}", &req);

    match req.path.as_str() {
        s if s.contains("echo") => echo(&req, &mut stream),
        "/" => send_200(&mut stream),
        _ => send_404(&mut stream),
    }
}

fn echo(reguest: &Request, stream: &mut TcpStream) {
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
}

fn send_200(stream: &mut TcpStream) {
    let str = String::from("HTTP/1.1 200 OK\r\n\r\n");

    let buf = str.into_bytes();

    println!("sending 200");
    if let Err(e) = stream.write_all(buf.as_slice()) {
        eprintln!("Failed to write response: {:?}", e); // Prevent shutdown on a failed write
    }
}

fn send_404(stream: &mut TcpStream) {
    let str = String::from("HTTP/1.1 404 Not Found\r\n\r\n");

    let buf = str.into_bytes();

    println!("sending 404");
    if let Err(e) = stream.write_all(buf.as_slice()) {
        eprintln!("Failed to write response: {:?}", e); // Prevent shutdown on a failed write
    }
}
