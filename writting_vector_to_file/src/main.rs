use std::path::Path;
use std::fs::{self, File, OpenOptions};
use std::io::{self, prelude::*, BufRead, BufReader, BufWriter};

fn main() {
    let file_path = "test_document.txt";

    if Path::new(file_path).exists() {
        // open file
        let mut f = File::open(file_path).unwrap();
        let mut contents: Vec<String> = io::BufReader::new(f).lines().collect::<io::Result<Vec<String>>>().expect("could not load file");
      
        if contents.contains(&"newly created file".to_string()) {
            let index = contents.iter().position(|r| r == "newly created file").unwrap();
            contents.insert(index+1, "used to be a new file, but now has been edited".to_string());
        } else {
            contents.push("used to be a new file, but now has been edited".to_string());
        }

        if contents.contains(&"newly created file".to_string()) {
            contents.remove(contents.iter().position(|r| r =="newly created file").unwrap());
        }

        let f = File::create(file_path).unwrap();
        let mut fi = BufWriter::new(f);

        dbg!(&contents);

        for line in contents {
            writeln!(fi, "{}", line).unwrap();
        }

        fi.flush().unwrap();
    } else {
        // create file

        let f = File::create(file_path).unwrap();
        let mut f = BufWriter::new(f);

        writeln!(f, "this is a test").unwrap();
        writeln!(f, "newly created file").unwrap();

        f.flush().unwrap();
    }
}
