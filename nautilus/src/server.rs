use std::io::{Read, Write};
use std::net::TcpListener;

pub fn run() {
    println!("Starting server on 0.0.0.0:8080");
    let listener = TcpListener::bind("0.0.0.0:8080").expect("bind 0.0.0.0:8080");

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let mut buffer = [0_u8; 512];
            let _ = stream.read(&mut buffer);
            let request_line = std::str::from_utf8(&buffer)
                .ok()
                .and_then(|text| text.lines().next())
                .unwrap_or("");
            let path = request_line
                .split_whitespace()
                .nth(1)
                .unwrap_or("/");
            let body = if path == "/healthz" { "ok" } else { "Hello world" };
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );
            let _ = stream.write_all(response.as_bytes());
        }
    }
}
