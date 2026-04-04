use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;

const DEFAULT_PORT: u16 = 4221;

fn main() {
    let server_address = format!("127.0.0.1:{DEFAULT_PORT}");
    let listener = TcpListener::bind(server_address).unwrap();
    println!("Starting server: listening at port {DEFAULT_PORT}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    match stream.peer_addr() {
        Ok(address) => println!("Handling connection from {}", address),
        Err(e) => println!("Handling connection (peer address unavailable: {})", e),
    }

    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(buffer_end) => {
            // Parse request, handle invalid UTF-8 bytes
            let request = String::from_utf8_lossy(&buffer[..buffer_end]);
            println!("Received HTTP request: {}", request);

            // Evaluate and send response
            if let Some(request_line) = request.lines().next() {
                let url = request_line.split_whitespace().nth(1);
                let response = match url {
                    Some("/") => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
                    Some(text) => {
                        if let Some(body) = text.strip_prefix("/echo/") {
                            format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", body.len(), body)
                        } else {
                            "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
                        }
                    }
                    _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
                };
                stream.write_all(response.as_bytes()).unwrap();
                println!("Sent HTTP response: {}", response);
            }
        }
        Err(e) => {
            println!("Failed to read from stream: {}", e);
        }
    }
}
