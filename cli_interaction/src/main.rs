use fs_extra::file::{read_to_string, write_all};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::path::Path;
use chrono::prelude::*;

fn main() {
    let tmp_log = String::from("C:/tmp/cli_interaction.txt");
    let mut tmp_content;

    if Path::new(&tmp_log).is_file() {
        tmp_content = read_to_string(&tmp_log).unwrap();
    }
    else {
        tmp_content = String::new();
    }

    let items = vec!["banana", "orange", "apple"];

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                                .items(&items)
                                .default(0)
                                .interact()
                                .unwrap();

    let temp = format!("{} - {} was selected\n", Utc::now(), items[selection]);

    println!("{}", &temp);

    tmp_content.push_str(&temp);
    write_all(tmp_log, &tmp_content);
}
