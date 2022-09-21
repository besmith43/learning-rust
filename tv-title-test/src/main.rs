extern crate fs_extra;
use fs_extra::file::*;
use fs_extra::dir::*;
use fs_extra::error::*;

use std::path::{Path, PathBuf};
use std::str::FromStr;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

fn main() {
    println!("starting application");

    let mut options = fs_extra::dir::DirOptions::new();
    options.depth = 1;
    let path = PathBuf::from(r"C:\Users\besmi\OneDrive\Development");
    //let path = "/mnt/c/Users/besmi/OneDrive/Development";
    //let path = PathBuf::from(r"../../");

    let contents = fs_extra::dir::get_dir_content2(&path, &options).unwrap();

    let selected_contents = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&contents.directories[..])
        .with_prompt("Please select a directory")
        .interact()
        .unwrap();

    let selected_path = &contents.directories[selected_contents];

    let selected_path_vec: Vec<&str> = selected_path.split('\\').collect();

    println!("{}", selected_path_vec[selected_path_vec.len()-1]);
}
