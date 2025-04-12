use std::net::TcpStream;
use std::time::Duration;

pub const BASE_URL: &str = "127.0.0.1:4221";

pub fn wait_until_server_ready(addr: &str) {
    for _ in 0..10 {
        if TcpStream::connect(addr).is_ok() {
            return;
        }
        std::thread::sleep(Duration::from_millis(50));
    }

    panic!("Server did not become ready in time");
}
