use structopt::StructOpt;
use std::path::Path;
use std::fs::{self, File, OpenOptions};
use std::io::{self, prelude::*, BufRead, BufReader, BufWriter};

#[derive(StructOpt, Debug)]
enum Opt {
    New {
        #[structopt(short = "m")]
        multiline: bool
    },
    Append,
    Edit,
    Delete,
    Replace
}

static FILENAME: &str = "test.txt";

fn main() {
    match Opt::from_args() {
        Opt::New { multiline } => {
            if multiline {
                new_multiline_file();
            } else {
                new_file();
            }
        },
        Opt::Append => {
            append_file();
        },
        Opt::Edit => {
            edit_file();
        },
        Opt::Delete => {
            delete_file();
        },
        Opt::Replace => {
            replace_file();
        },
    }
}

fn new_file() {
    let mut f = File::create(FILENAME).unwrap();
    f.write_all(b"this is a test of the emergency broadcast system\n").unwrap();
}

fn new_multiline_file() {
    let mut f = File::create(FILENAME).unwrap();
    f.write_all(b"this is a test of the emergency broadcast system\n").unwrap();
    f.write_all(b"this is a second line of text\n").unwrap();
}

fn append_file() {
    if Path::new(FILENAME).exists() {
        let mut f = OpenOptions::new().append(true).open(FILENAME).unwrap();
        f.write_all(b"this is a new line of text\n").unwrap();
    } else {
        println!("the file must exist first before it can be appended");
    }
}

fn edit_file() {
    let data = fs::read_to_string(FILENAME).unwrap();
    
    let updated_data: String;

    if data.contains(&"emergency".to_string()) {
        updated_data = data.replace("emergency", "normal");
    } else if data.contains(&"replacing".to_string()) {
        updated_data = data.replace("replacing", "no idea");
    } else {
        updated_data = data.replace("new", "old");
    }

    dbg!(&updated_data);

    // removing the original file before writing the new data as a test
    fs::remove_file(FILENAME).unwrap();
    // had to add create method as part of the remove file test
    let mut f = OpenOptions::new().write(true).create(true).open(FILENAME).unwrap();
    f.write_all(updated_data.as_bytes()).unwrap();

    // it seems that the only way to keep the extra characters from showing up at the end of the
    // file is to delete the original file before writting the new contents to disk
}

fn delete_file() {
    fs::remove_file(FILENAME).unwrap();
}

// this is designed to replace the file's contents
fn replace_file() {
    let mut f = File::create(FILENAME).unwrap();
    f.write_all(b"replacing the contents of the file with this\n").unwrap();
}

