use std::thread;
use std::time::Duration;

fn main() {
    // to pass a value into the spawned thread, you need to use the "move" keyword before the
    // double pipe
    let v = vec![1, 2, 3];


    // must set the variable and join it to keep the main thread from exiting execution
    let handle = thread::spawn(move || {
        println!("here's a vector {:?}", v);
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);

            // if you remove the sleeps from each for loop, it seems to complete the main thread
            // before giving the control of the console to the spawned thread
            thread::sleep(Duration::from_millis(1));
        }
    });

    // if you place the handle join here, then it won't run at the same time as the main thread's
    // for loop

    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // if you don't add the following handle join line and setting thread spawn to a variable, 
    // the execution of the spawned thread will need before it completes when the main thread exits

    handle.join().unwrap();
}
