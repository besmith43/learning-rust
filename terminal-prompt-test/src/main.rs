use std::io;

fn main() {
    let mut user_input = String::new();

    println!("Testing that a window is opened when this executable is launched");
    io::stdin().read_line(&mut user_input).expect("failed to read line");
}
