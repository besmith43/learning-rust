use duct::cmd;

fn main() {
    cmd!("echo", "hi").run();
}

