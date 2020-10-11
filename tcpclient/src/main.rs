use std::io::{stdin, stdout, Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8192").expect("Failed to connect 8192");
    let mut buf = [0; 1024];
    let mut len: usize;
    loop {
        stdout().write(b"Send: ").unwrap();
        stdout().flush().unwrap();
        len = stdin().read(&mut buf).unwrap();
        len -= 1;
        stream.write(&buf[..len]).unwrap();
        if &buf[..len] == b"exit" {
            break;
        }
        len = stream.read(&mut buf).unwrap();
        stdout().write(b"Received: ").unwrap();
        stdout().write(&buf[..len]).unwrap();
        stdout().write(b"\n").unwrap();
    }
}
