use memmap::MmapOptions;
use std::fs::File;

const DATA_VAR: i32 = 10;
static STATIC: u32 = 20;

fn text_var() {
    println!("Test Func!");
}

fn main() {
    let bss_var: &'static str = "aaa";
    let stack_var = 0;
    let heap_var: Vec<i64> = Vec::new();
    let fpath = "/home/howard/Workspaces/Rust/Rust-Practice/pointer/src/main.rs";
    let file = File::open(fpath).unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    println!("stack var: {:p}", &stack_var);
    println!("mmap var: {:p}", &mmap);
    println!("heap var: {:p}", &heap_var);
    println!("fpath: {:p}", &fpath);
    println!("STATIC: {:p}", &STATIC);
    println!("bss var: {:p}", bss_var);
    println!("data var: {:p}", &DATA_VAR);
    println!("text var: {:p}", &text_var);
}
