use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    // Listening for connections
    let listener: TcpListener = TcpListener::bind("127.0.0.1:1420").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        hande_connection(stream);
    }
}

fn hande_connection(mut stream: TcpStream) {
    // A buffer to hold read data
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) =
        if buffer.starts_with(get) {
            ("HTTP/1.1 200", "Pages/App/index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "Pages/Errors/404.html")
        };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}