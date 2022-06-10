use std::process::Command;

const TARGET_FILE: &str = "/tmp/test";

fn main() {
    let mut editor = Command::new("vim").arg(TARGET_FILE).spawn().unwrap();
    editor.wait().unwrap();
}
