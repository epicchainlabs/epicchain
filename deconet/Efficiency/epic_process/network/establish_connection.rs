use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

/// Function to establish a TCP connection to a given host and port.
pub fn establish_tcp_connection(host: &str, port: u16) -> std::io::Result<TcpStream> {
    let address = format!("{}:{}", host, port);
    TcpStream::connect(address)
}

/// Function to listen for incoming TCP connections on a given port.
pub fn listen_for_tcp_connections(port: u16) -> std::io::Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;
        println!("Received data: {:?}", String::from_utf8_lossy(&buffer));
        stream.write_all(b"Hello from server")?;
    }
    Ok(())
}

/// Function to send data over a TCP connection.
pub fn send_data_over_tcp(stream: &mut TcpStream, data: &[u8]) -> std::io::Result<usize> {
    stream.write(data)
}

/// Function to receive data over a TCP connection.
pub fn receive_data_over_tcp(stream: &mut TcpStream, buffer: &mut [u8]) -> std::io::Result<usize> {
    stream.read(buffer)
}

/// Function to close a TCP connection.
pub fn close_tcp_connection(stream: &mut TcpStream) -> std::io::Result<()> {
    stream.shutdown(std::net::Shutdown::Both)
}

/// Function to get the remote address of a TCP stream.
pub fn get_remote_address(stream: &TcpStream) -> std::io::Result<std::net::SocketAddr> {
    stream.peer_addr()
}

/// Function to check if a TCP stream is still open.
pub fn is_tcp_stream_open(stream: &TcpStream) -> bool {
    stream.peer_addr().is_ok()
}
