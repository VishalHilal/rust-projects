use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received Request:\n{}", request);

    // Only serve index.html for now
    let get_root = request.starts_with("GET / ");

    let (status_line, filename) = if get_root {
        ("HTTP/1.1 200 OK", "public/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "public/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap_or_else(|_| "<h1>404 Not Found</h1>".to_string());

    let response = format!(
        "{status_line}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("🚀 Server running at http://127.0.0.1:7878");

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}

