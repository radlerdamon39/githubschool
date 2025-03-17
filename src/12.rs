use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut cmd = Command::new("rustc");
    let output = cmd.arg("-o").arg("main").arg("main.rs").output().unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
}
