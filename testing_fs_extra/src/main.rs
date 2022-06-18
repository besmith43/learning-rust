use std::path::Path;
use std::{thread, time};
use std::sync::mpsc::{self, TryRecvError};

extern crate fs_extra;
use fs_extra::file::*;
use fs_extra::error::*;

fn example_copy() -> Result<()> {
    let path_from = Path::new("./temp");
    let path_to = path_from.join("out");
    let test_file = (path_from.join("test_file.txt"), path_to.join("test_file.txt"));


    fs_extra::dir::create_all(&path_from, true)?;
    fs_extra::dir::create_all(&path_to, true)?;

    write_all(&test_file.0, "test_data")?;
    assert!(test_file.0.exists());
    assert!(!test_file.1.exists());


    let options = CopyOptions {
        buffer_size: 1,
        ..Default::default()
    };

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let handler = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            thread::sleep(time::Duration::from_millis(500));
        };
        copy_with_progress(&test_file.0, &test_file.1, &options, handler).unwrap();
        assert!(test_file.0.exists());
        assert!(test_file.1.exists());

    });
    loop {
        match rx.try_recv() {
            Ok(process_info) => {
                println!("{} of {} bytes",
                         process_info.copied_bytes,
                         process_info.total_bytes);
            }
            Err(TryRecvError::Disconnected) => {
                println!("finished");
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
    }
    Ok(())

}


fn main() -> Result<()>{
    example_copy()?;
    Ok(())
}
