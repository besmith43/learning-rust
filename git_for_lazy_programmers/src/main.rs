use std::process::Command;
use std::process::Output;
use std::io;
use rand::Rng;

fn main() {
    let run_result = run();
    match run_result {
        Ok(_) => println!("great success!"),
        Err(err) => println!("failed, reason: {err}"),
    };
}

// had to use Box<dyn std::error::Error> to handle multiple types of errors
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let messages = vec![
        "omg another update",
        "i don't know what is in this commit lol",
        "oops broke everything",
        "fixed the stuff",
        "why no work"
    ];

    let index = rand::thread_rng().gen_range(0..messages.len());
    let message = messages[index];

    // push local changes
    git_command(&["add", "."])?;
    git_command(&["commit", "-m", message])?;

    // pull remote
    let result = git_command(&["pull"])?;
    let output_string = std::str::from_utf8(&result.stdout)?;

    // find if we have a merge conflict and fix
    let conflicting_files = get_conflicting_files(output_string);
    for file_name in conflicting_files.iter() {
        println!("conflicting file: {file_name}");
        let fixed_string = clean_conflicted_string(std::fs::read_to_string(file_name)?);
        let _result = std::fs::write(file_name, fixed_string);
    }

    if conflicting_files.len() > 0 {
        git_command(&["add", "."])?;
        git_command(&["commit", "-m", "merging... screw you"])?;
    }

    git_command(&["push"])?;

    Ok(())
}

fn git_command(arr: &[&str]) -> Result<Output, io::Error> {
    Command::new("git").args(arr.to_vec()).output()
}

fn get_conflicting_files(text: &str) -> Vec<String> {
    let mut conflicting_files = Vec::new();

    let merge_conflict_str = "Merge conflict in ";
    let mut string_index = 0;

    while let Some(conflict_index) = text[string_index..].find(merge_conflict_str) {
        let conflict_slice = &text[string_index+conflict_index..];
        let end_line = conflict_slice.find('\n').unwrap();
        let file_name = &conflict_slice[merge_conflict_str.len()..end_line];
        conflicting_files.push(file_name.to_string());
        string_index += conflict_index + end_line;
    }
    conflicting_files
}

fn clean_conflicted_string(conflicted_file: String) -> String {
    StringChipper::chip_away(conflicted_file.as_str(), |string_chipper| {
        string_chipper.remove_line("<<<<<<< HEAD")?;
        string_chipper.remove_lines("=======", ">>>>>>>")
    }) 
}

struct StringChipper {
    index: u32,
    start_index: u32,
    find_index: u32,
}

impl StringChipper {
    // I don't know if this is the right way to recreate this
    // need a function for chip_away
    // needs a find_index integer variable for within self
    // needs an index variable for within self
    // needs a find function for self
    // needs an inner and length function for self
    // what even is an inner function???
    // is this an extension of String?
    pub fn remove_line(&mut self, str_match: &str) -> Result<(), ()> {
        self.find_index = 0;
        let start_index = self.find(str_match)?;
        let end_index = self.find("\n");
        let end_index = match end_index {
            Ok(i) => i,
            Err(_) => self.inner.len() - self.index - 1,
        };

        self.inner.replace_range(start_index..=self.index + end_index, ""); // this either needs to be the return statement or it needs a ?
        Ok(())
    }

    pub fn remove_lines(start_string: String, end_string: String) -> Result<(), ()> {
        // could this be done by just getting the start index and end index and then just iterate
        // over it with remove_line function calls?

        Ok(())
    }

    pub fn chip_away<F: Fn(&str) -> String>(file: &str, func: F) -> String {
        func(file)
    }
}

impl Iterator for StringChipper {
    type Item = u32;

    pub fn next(&mut self) -> Option<Self::Item> {
         
    }
}
