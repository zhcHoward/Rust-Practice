use nix::unistd::{fork, ForkResult};
use std::thread;

fn main() {
    match fork() {
        Ok(ForkResult::Parent { child }) => {
            println!(
                "Continuing execution in parent process, new child has pid: {}",
                child
            );
        }
        Ok(ForkResult::Child) => {
            for _ in 0..5 {
                println!("I'm a new child process");
                thread::sleep(std::time::Duration::from_secs(1));
            }
        }
        Err(_) => println!("Fork failed"),
    }
}
