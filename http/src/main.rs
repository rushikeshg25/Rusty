pub mod types;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};
use types::http_types::http_types::{http_request, http_response};
#[allow(unused)]
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
    let parsed_req = (&request);
}

fn parse_http_request(request: &str) {}
