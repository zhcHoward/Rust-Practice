use std::io::{Read, Write};
use std::net::UdpSocket;
use std::str;

fn main() {
    let mut socket = UdpSocket::bind("127.0.0.1:8193").expect("Failed to connect socket");
    socket.connect("127.0.0.1:8192").unwrap();
    socket
        .send("How are you?".as_bytes())
        .expect("Send message failed");
}
