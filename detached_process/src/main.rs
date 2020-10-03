use std::env;
use std::process::Command;

fn main() {
    // for (key, value) in env::vars() {
    //     println!("{}: {}", key, value);
    // }
    println!("main thread start");
    let mut command = Command::new("python3");
    let mut python_file = env::current_dir().unwrap();
    python_file.push("loop.py");
    command.arg(python_file);
    command.spawn().expect("Unknow Error");
    // loop.py will keep running even the main thread exits
}
