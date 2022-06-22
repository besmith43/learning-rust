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

fn main() {
    run_result().unwrap();

    run_option().unwrap();
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
