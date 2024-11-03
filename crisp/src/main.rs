use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    // Read the request from the client
    stream.read(&mut buffer).unwrap();
    // Parse the request (simple handling for demonstration)
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", request);

    // Prepare a simple HTTP response
    let response = "HTTP/1.1 200 OK\r\n\r\nHello from Crisp!";
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    // Bind the server to localhost on port 7878
    let listener = TcpListener::bind("127.0.0.1:6969").expect("Failed to bind to address");
    println!("Listening on port 6969...");

    // Accept incoming connections in a loop
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);  // Handle each client connection
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
