// this is a test of the various error handling methods in rust
//
// the following methods are tested:
//
// match
// options
// some/none
// question mark
// unwrap
// expect
// 

use std::process::Command;

fn main() {
    Command::new("nonexistent_executable")
            .status()
            .expect("ls failed to run");  // having the expect will halt execution just like unwrap but has a string that you give it and the error of the result

    run_result(); //not handling the returned result is acceptable, but will cause a warning and not stop the flow of execution

    run_option().unwrap(); //unwrap automatically panics on an error result and stops execution

    }

fn run_result() ->Result<(), String> {
    match failing() {
        Ok(_) => println!("match function was successful"),
        Err(error) => println!("{}", error),
    }

    failing()?;

    Ok(())
}

fn failing() -> Result<(), String> {
    Err("this will always fail".to_string())
}

fn run_option() -> Option<usize> {
    search()?;

    /*let result = match search() {
        Some(index) => index,
        None() => println!("Nothing found"),
    };*/

    search().unwrap();

    let index_result: Option<usize> = Some(1);

    index_result
}

fn search() -> Option<usize> {
    None
}
