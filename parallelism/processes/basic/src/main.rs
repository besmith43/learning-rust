use procspawn;

// got this example from the following
// https://docs.rs/procspawn/latest/procspawn/

fn main() {
    procspawn::init();

    let data = vec![1, 2, 3, 4];
    let handle = procspawn::spawn(data, |data| {
        println!("Received data {:?}", &data);
        data.into_iter().sum::<i64>()
    });
    let _result = handle.join().unwrap();
}
