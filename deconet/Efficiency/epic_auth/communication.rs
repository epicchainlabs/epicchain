use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

// Server function
fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 512];
                match stream.read(&mut buffer) {
                    Ok(_) => {
                        let received = String::from_utf8_lossy(&buffer[..]);
                        println!("Received message: {}", received);
                        let response = "Message received";
                        stream.write(response.as_bytes()).expect("Failed to write");
                    }
                    Err(e) => {
                        println!("Failed to read from stream: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Failed to establish a connection: {}", e);
            }
        }
    }
}

// Client function
fn start_client() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect");

    let message = "Hello, server!";
    stream.write(message.as_bytes()).expect("Failed to write");

    let mut response = String::new();
    stream.read_to_string(&mut response).expect("Failed to read");
    println!("Server response: {}", response);
}

fn main() {
    // Start the server in a separate thread
    std::thread::spawn(|| {
        start_server();
    });

    // Sleep for a short while to allow the server to start
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Start the client
    start_client();
}
