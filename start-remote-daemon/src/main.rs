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
const BUF_SIZE: usize = 1024;
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

fn start_daemon() {
    debug!("start local daemon.");
    Command::new(COMMAND).spawn().unwrap();
}

fn start_tcp_server() {
    info!("Start TCP server...");
    let address = format!("0.0.0.0:{}", TCP_PORT);
    let listener = TcpListener::bind(address).unwrap();
    let (mut stream, address) = listener.accept().unwrap();
    info!("Recieved connection from {}", address);
    let mut buf = [0; BUF_SIZE];
    let mut len: usize;
    let mut msg: String;
    loop {
        len = stream.read(&mut buf).unwrap();
        msg = format!("Recieved: {}", std::str::from_utf8(&buf[..len]).unwrap());
        debug!("{}", msg);
        if &buf[..len] == b"exit" {
            break;
        }
        stream.write(msg.as_bytes()).unwrap();
    }
}

fn setup_ssh_connection(username: &str, host: &str) -> Session {
    debug!("setup ssh connection");
    let ssh_address = format!("{}:22", host);
    let tcp = TcpStream::connect(ssh_address).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    match sess.userauth_agent(&username) {
        Ok(_) => sess,
        Err(_) => {
            let msg = format!("Please input password for {}: ", &username);
            let pass = rpassword::read_password_from_tty(Some(&msg)).unwrap();
            match sess.userauth_password(&username, &pass) {
                Ok(_) => sess,
                Err(e) => {
                    println!("Password Wrong! {}", e.message());
                    exit(1);
                }
            }
        }
    }
}

fn main() {
    logger::init_logger();

    let opt = Opt::from_args();
    debug!("{:?}", opt);
    if opt.daemon {
        start_daemon();
        exit(0);
    }
    if opt.destination.as_str() == "" {
        start_tcp_server();
        exit(0);
    }

    let pattern = Regex::new(r"(?:(?P<user>[^@]+)@)?(?P<host>[^@]+)").unwrap();
    match parse(&opt.destination, &pattern) {
        None => {
            println!("Failed to parse destination: {:?}", opt.destination);
            exit(1);
        }
        Some((username, host)) => {
            let ssh = setup_ssh_connection(&username, &host);

            // start remote daemon
            let mut channel = ssh.channel_session().unwrap();
            debug!("run command: 'remote-demo -d' on remote");
            channel.exec("remote-demo -d").unwrap();
            debug!("close ssh connection");
            channel.close().unwrap();

            // connect remote tcp server
            debug!("setup tcp connection on {} port", TCP_PORT);
            let tcp_address = format!("{}:{}", host, TCP_PORT);
            let mut stream = TcpStream::connect(tcp_address).unwrap();
            let mut buf = [0; BUF_SIZE];
            let mut len: usize;
            loop {
                print!("Send: ");
                stdout().flush().unwrap();
                len = stdin().read(&mut buf).unwrap();
                len -= 1; // ignores trailing b'\n'
                debug!("user input: {:?}", &buf[..len]);
                stream.write(&buf[..len]).unwrap();
                if &buf[..len] == b"exit" {
                    break;
                }
                len = stream.read(&mut buf).unwrap();
                let msg = std::str::from_utf8(&buf[..len]).unwrap();
                debug!("received message: {}", msg);
                println!("{}", msg);
            }
        }
    }
}
