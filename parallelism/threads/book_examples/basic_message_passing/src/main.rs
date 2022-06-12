use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // it seems that the only variables that are passed to a spawned threads are the ones that are
    // used within that new thread

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
                        String::from("hi"),
                        String::from("from"),
                        String::from("the"),
                        String::from("thread"),
                    ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
                        String::from("more"),
                        String::from("messages"),
                        String::from("for"),
                        String::from("you"),
                    ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // didn't need the handle join because the rx for loop is keeping it from exiting prematurely
    for received in rx {
        println!("Got: {}", received);
    }
}
