use std::process::Command;

fn main() {
    println!("spawn");
    Command::new("ls")
        .spawn()
        .expect("ls command failed to start");

    println!("output");
    Command::new("ls")
        .output()
        .expect("ls command failed to start");

    println!("status");
    Command::new("ls")
        .status()
        .expect("ls command failed to start");
}
