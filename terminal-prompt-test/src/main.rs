extern crate term;
use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let mut t = term::stdout().unwrap();

    t.fg(term::color::GREEN).unwrap();
    write!(t, "hello, ").unwrap();

    t.fg(term::color::RED).unwrap();
    writeln!(t, "world!").unwrap();

    t.fg(term::color::GREEN).unwrap();
    writeln!(t, "please write something").unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).expect("didn't type in anything");

    writeln!(t, "This is what you typed in: {}", input).unwrap();

    writeln!(t, "press Enter to continue").unwrap();
    let mut i = String::new();
    stdin().read_line(&mut i).expect("");

    t.reset().unwrap();
}
