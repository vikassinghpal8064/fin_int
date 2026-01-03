use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer);
    let request_line = request.lines().next().unwrap();

    // Example: "GET /java_script/other.js HTTP/1.1"
    let path = request_line.split_whitespace().nth(1).unwrap();

    // Default to index.html if root is requested
    let file_path = if path == "/" {
        "src/static/index.html".to_string()
    } else {
        format!("src/static{}", path)
    };

    // Try to read the file
    let contents = fs::read_to_string(&file_path).unwrap_or_else(|_| "<h1>404 Not Found</h1>".to_string());

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}