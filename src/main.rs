use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    println!("Starting server!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

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
    println!("Handling connection from {}", stream.peer_addr().unwrap());

    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(n) => {
            let request = String::from_utf8_lossy(&buffer[..n]);
            println!("Received request: {}", request);

            let response = "HTTP/1.1 200 OK\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(e) => {
            println!("Failed to read from stream: {}", e);
        }
    }
}
