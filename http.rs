use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received Request:\n{}", request);

    // Create a simple HTML page
    let html = r#"
        <html>
            <head><title>Rust HTTP Server</title></head>
            <body style="font-family: sans-serif; text-align:center;">
                <h1>Hello from Rust </h1>
                <p>This page is served by your Rust TCP server!</p>
            </body>
        </html>
    "#;

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        html.len(),
        html
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

