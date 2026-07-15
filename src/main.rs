use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::PathBuf;
use std::thread;

const DEFAULT_PORT: u16 = 4221;

fn main() {
    let server_address = format!("127.0.0.1:{DEFAULT_PORT}");
    let listener = TcpListener::bind(server_address).unwrap();
    println!("Starting server: listening at port {DEFAULT_PORT}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_connection(stream);
                });
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
        Err(e) => {
            println!("Handling connection (peer address unavailable: {})", e);
            return;
        }
    }

    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(buffer_end) => {
            // Parse request, handle invalid UTF-8 bytes
            let request = String::from_utf8_lossy(&buffer[..buffer_end]);
            println!("Received HTTP request: {}", request);

            // Evaluate and send response
            let mut request_lines = request.lines();
            if let Some(url_line) = request_lines.next() {
                let url = url_line.split_whitespace().nth(1);

                let response = match url {
                    Some("/user-agent") => {
                        let body = request_lines
                            .find(|line| line.starts_with("User-Agent: "))
                            .and_then(|line| line.strip_prefix("User-Agent: "))
                            .unwrap_or("");
                        format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", body.len(), body)
                    }
                    Some(text) => {
                        if text == "/" {
                            "HTTP/1.1 200 OK\r\n\r\n".to_string()
                        } else if let Some(body) = text.strip_prefix("/echo/") {
                            format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", body.len(), body)
                        } else if let Some(file_name) = text.strip_prefix("/files/") {
                            // Find path to file directory
                            let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                            let file_path = project_root.join("static").join(file_name);

                            // Check and read file content
                            match file_path.try_exists() {
                                Ok(true) => {
                                    let body: String =
                                        fs::read_to_string(&file_path).unwrap_or("".to_string());
                                    format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}", body.len(), body)
                                }
                                _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
                            }
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
