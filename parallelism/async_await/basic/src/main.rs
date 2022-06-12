use std::time::Duration;
use tokio::time::sleep;

// got this example from the following youtube video by let's get rusty
// https://www.youtube.com/watch?v=K8LNPYNvT-U
//
// the main flavor comes from one of 2 options
// current thread: for single threaded operation
// multi thread: for multiple workers
//
// as seen here: https://docs.rs/tokio/0.3.1/tokio/attr.main.html

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let f = my_function();  // returning a future to be executed when the await keyword is called
    //f.await;  // the order of the await determines when the execution of the future would happen
    println!("let's get rusty");
    f.await;

    let mut handles = vec![];

    for i in 0..2 {
        let handle = tokio::spawn(async move {
            my_function2(i).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn my_function() {
    println!("I'm an async function");
    let s1 = read_from_database().await;
    println!("first result: {}", s1);
    let s2 = read_from_database().await;
    println!("second result: {}", s2);
}

async fn my_function2(i: u32) {
    println!("[{i}] I'm an async function");
    let s1 = read_from_database().await;
    println!("[{i}] first result: {}", s1);
    let s2 = read_from_database().await;
    println!("[{i}] second result: {}", s2);
}

async fn read_from_database() -> String {
    sleep(Duration::from_millis(50)).await;
    "DB Result".to_owned()
}
