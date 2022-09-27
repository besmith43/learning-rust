use xshell::{Shell, cmd};

fn main() {
    let sh = Shell::new();
    let branch = "main";
    let commit_hash = cmd!(sh, "git rev-parse {branch}").run();
}
