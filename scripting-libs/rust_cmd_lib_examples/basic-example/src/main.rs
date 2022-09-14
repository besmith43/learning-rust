use cmd_lib::*;
//use std::process::Command;

fn main() {
    let msg = "I love rust";

    run_cmd!(echo $msg).unwrap();
    run_cmd!(echo "This is the message: $msg").unwrap();

    // control flow tests
    let file = "~/.bashrc";
    let result = run_fun!(cat ${file} | wc -l).unwrap();

    println!("{:?}", result);

    if result.parse::<i32>().unwrap() >= 5 {
        println!("bashrc is huge!");
    }

    /*
    // this fails to work because gum throws an error when it
    // can't get control over the terminal
    // also run_cmd! doesn't handle &&
    if run_cmd!(gum confirm "commit changes?").is_ok() {
        run_cmd!(echo "you said yes").unwrap();
    }
    */

    /*
    // even with the old style, gum isn't going to work from a rust program
    let output = Command::new("gum")
                                    .arg("confirm")
                                    .arg("\"commit changes?\"")
                                    .output()
                                    .expect("failed to execute command");

    println!("{:?}", output);
    */

    /*
    // this also fails because this library is designed for handling single line commands and using rust for logic and control flow
    run_cmd!(
        if [ -d ~/Bin ]; then
            echo "bin directory exists"
        fi
    ).unwrap();
    */
}
