use std::io::{Read, Write};
use std::net::UdpSocket;
use std::str;

fn main() {
    let mut buf = [0; 1024];
    let socket = UdpSocket::bind("127.0.0.1:8192").unwrap();
    match socket.recv_from(&mut buf) {
        Ok((len, address)) => println!(
            "Received {:?} from {:?}",
            str::from_utf8(&buf[..len]).unwrap(),
            address
        ),
        Err(err) => println!("Received failed, error: {:?}", err),
    }
    // let contents = "哈哈哈哈哈哈啊哈哈哈哈哈哈哈哈哈哈哈哈呵呵哈哈哈哈哈哈哈哈";
    // socket
    //     .send_to(contents.as_bytes(), "127.0.0.1:8191")
    //     .expect("Send message failed");
}
