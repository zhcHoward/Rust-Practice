use ssh2::Session;
use std::env;
use std::io::Read;
use std::net::TcpStream;
use std::process::exit;
extern crate rpassword;

fn main() {
    let username = env::var("USER").unwrap();
    println!();
    let pass = rpassword::read_password_from_tty(Some("Please input your password: ")).unwrap();
    let mut session = Session::new().unwrap();
    let stream = TcpStream::connect("192.168.1.47:22").unwrap();
    session.set_tcp_stream(stream);
    session.handshake().unwrap();
    match session.userauth_password(&username, &pass) {
        Ok(_) => println!("Password Correct!"),
        Err(e) => {
            println!("Password Wrong! {}", e.message());
            exit(1);
        }
    }
    let mut channel = session.channel_session().unwrap();
    channel.exec("ip a").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    print!("ip a:\n{}", &s);
    channel.wait_close().unwrap();
    println!("Channel exited {}", channel.exit_status().unwrap());
}
