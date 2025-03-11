use std::io::Write;
#[allow(unused_imports)]
use std::net::TcpListener;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let str = String::from("HTTP/1.1 200 OK\r\n\r\n");

                let buf = str.into_bytes();

                let res = _stream.write_all(buf.as_slice());
                println!("200 returned");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
