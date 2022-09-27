use rscripter::*;

fn main() {
    log!("Hello World");
    log!(echo("Hello World"));
    cmd!("ping", "-n", "3", "1.1.1.1");
}
