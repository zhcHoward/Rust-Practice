use std::io::Read;
use std::net::TcpListener;
use std::str;

fn main() {
    let mut buf = [0; 1024];
    let listener = TcpListener::bind("127.0.0.1:8192").expect("Failed to bind port 8192");
    println!("Waiting for connection...");
    for stream in listener.incoming() {
        match stream {
            Ok(s) => println!("new connection: {:?}", s),
            Err(e) => println!("Error: {:?}", e),
        }
    }
    // let connection = listener.accept().unwrap();
    // println!("Received connection from {:?}", connection.1);
    // let mut stream = connection.0;
    // let size = stream.read(&mut buf).unwrap();
    // println!(
    //     "Received message: {}",
    //     str::from_utf8(&buf[..size]).unwrap()
    // )
}
