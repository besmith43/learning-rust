/* 
 * articles that I got this from
 * https://stackoverflow.com/questions/43292357/how-can-one-detect-the-os-type-using-rust
 * https://doc.rust-lang.org/reference/conditional-compilation.html
 * https://stackoverflow.com/questions/39204908/how-to-check-release-debug-builds-using-cfg-in-rust
 */

fn main() {
    #[cfg(debug_assertions)]
    println!("Hello from Debug");

    #[cfg(not(debug_assertions))]
    println!("Hello from Release");

    if cfg!(debug_assertions) {
        println!("debug enabled and operated through macro");
    } else {
        println!("release mode from the else statement");
    }

    test_print();

    check_os();

    example();
}

#[cfg(debug_assertions)]
fn test_print() {
    println!("debug print called");
}

#[cfg(not(debug_assertions))]
fn test_print() {
    println!("release print called");
}

fn check_os() {
    if cfg!(target_os = "windows") {
        println!("built on Windows");
    } else if cfg!(target_os = "macos") {
        println!("built on MacOS");
    } else if cfg!(target_os = "linux") {
        println!("build on Linux");
    } else {
        println!("No idea");
    }
}

#[cfg(target_os = "macos")]
fn example() {
    println!("mac os");
}

#[cfg(target_os = "windows")]
fn example() {
    println!("windows");
}

#[cfg(target_os = "linux")]
fn example() {
    println!("linux");
}

