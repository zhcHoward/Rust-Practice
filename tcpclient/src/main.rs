use std::io::Write;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8192").expect("Failed to connect 8192");
    let message = "Hello World!";
    stream.write(message.as_bytes()).unwrap();
}
