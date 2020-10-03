use log::{debug, info};
use regex::Regex;
use rpassword;
use ssh2::Session;
use std::io::{stdin, stdout, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::process::Command;
use structopt::StructOpt;
mod logger;

const TCP_PORT: usize = 8192;
const COMMAND: &str = "remote-demo";

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(help = "destination", default_value = "")]
    destination: String,
    #[structopt(short, long, help = "wether to start as a daemon in the background")]
    daemon: bool,
}

fn parse(raw: &str, pat: &Regex) -> Option<(String, String)> {
    match pat.captures(raw) {
        None => None,
        Some(result) => {
            let username = match result.name("user") {
                None => std::env::var("USER").unwrap(),
                Some(r) => r.as_str().to_string(),
            };
            let host = result.name("host").unwrap().as_str().to_string();
            Some((username, host))
        }
    }
}

fn main() {
    logger::init_logger();

    let opt = Opt::from_args();
    debug!("{:?}", opt);
    if opt.daemon {
        debug!("start local daemon.");
        Command::new(COMMAND).spawn().unwrap();
        exit(0);
    }
    if opt.destination.as_str() == "" {
        info!("Start TCP server...");
        let address = format!("0.0.0.0:{}", TCP_PORT);
        let listener = TcpListener::bind(address).unwrap();
        let (mut stream, address) = listener.accept().unwrap();
        info!("Recieved connection from {}", address);
        let mut buf = [0; 1024];
        let mut len: usize;
        loop {
            len = stream.read(&mut buf).unwrap();
            let content = std::str::from_utf8(&buf[..len]).unwrap();
            debug!("Received: {}", content);
            if &buf[..len] == b"exit" {
                break;
            }
            let content = format!("Recieved: {}", content);
            stream.write(content.as_bytes()).unwrap();
        }
        exit(0);
    }

    let pattern = Regex::new(r"(?:(?P<user>[^@]+)@)?(?P<host>[^@]+)").unwrap();
    match parse(&opt.destination, &pattern) {
        None => {
            println!("Failed to parse destination: {:?}", opt.destination);
            exit(1);
        }
        Some((username, host)) => {
            // setup ssh connection
            debug!("setup ssh connection");
            let msg = format!("Please input password for {}: ", &username);
            let pass = rpassword::read_password_from_tty(Some(&msg)).unwrap();
            let ssh_address = format!("{}:{}", host, 22);
            let tcp = TcpStream::connect(ssh_address).unwrap();
            let mut sess = Session::new().unwrap();
            sess.set_tcp_stream(tcp);
            sess.handshake().unwrap();
            match sess.userauth_password(&username, &pass) {
                Ok(_) => (),
                Err(e) => {
                    println!("Password Wrong! {}", e.message());
                    exit(1);
                }
            }
            let mut channel = sess.channel_session().unwrap();
            debug!("run command: 'remote-demo -d' on remote");
            channel.exec("remote-demo -d").unwrap();
            let mut s = String::new();
            channel.read_to_string(&mut s).unwrap();
            debug!("close ssh connection");
            channel.wait_close().unwrap();
            debug!("setup tcp connection on {} port", TCP_PORT);
            let tcp_address = format!("{}:{}", host, TCP_PORT);
            let mut stream = TcpStream::connect(tcp_address).unwrap();
            let prompt = "Send: ".as_bytes();
            let mut buf = String::new();
            let mut content = [0; 1024];
            let mut len: usize;
            loop {
                stdout().write(prompt).unwrap();
                stdout().flush().unwrap();
                len = stdin().read_line(&mut buf).unwrap();
                debug!("user input: {}", &buf[..len - 1]);
                stream.write(&buf[..len - 1].as_bytes()).unwrap();
                debug!("send message: {}", &buf[..len - 1]);
                if &buf[..len - 1] == "exit" {
                    break;
                }
                buf.clear();
                len = stream.read(&mut content).unwrap();
                let msg = std::str::from_utf8(&content[..len]).unwrap();
                debug!("received message: {}", msg);
                println!("{}", msg);
            }
        }
    }
}
