use std::env;
use std::process;
use std::process::Command;

const USAGE: &str = "vide [filepath]";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_usage();
        process::exit(1);
    }

    let mut editor = Command::new("vim").arg(&args[1]).spawn().unwrap();
    editor.wait().unwrap();
}

fn print_usage() {
    println!("Usage: {}", USAGE);
}
