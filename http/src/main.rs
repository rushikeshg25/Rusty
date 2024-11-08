#[allow(dead_code)]
pub mod types;
pub mod utils;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};
use utils::http_lib::http_helper::parse_request;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").expect("Failed to bind Tcp to this address");
    println!("Listening on 6969");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let bytes_read = stream
        .read(&mut buffer)
        .expect("Failed to read from client!");
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("{}", request);
    let parsed_req = parse_request(&request);

    stream.write().expect("Failed to write response to client");
    stream.flush().expect("Failed to flush stream");
}
