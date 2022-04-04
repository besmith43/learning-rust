use argh::FromArgs;

// https://crates.io/crates/argh
// welcome to cargo run.. you have to use the -- convention because cargo can't tell the difference
// between cmd args for it and for your program without it.  
// so for example: cargo run -- -j --height 5

#[derive(FromArgs)]
/// even the struct declaration needs a description
struct Args {
    /// variable discription
    #[argh(switch, short = 'j')]
    jump: bool,

    /// how high did you jump?
    #[argh(option)]
    height: usize,

    /// what is your name?
    #[argh(option, default = "String::from(\"Bob\")")]
    nickname: String,
}

fn main() {
    let args: Args = argh::from_env();

    if args.jump {
        println!("{} jumped {} ft high!!!", args.nickname, args.height);
    } else {
        println!("{} did not jump {} ft high", args.nickname, args.height);
    }
}
