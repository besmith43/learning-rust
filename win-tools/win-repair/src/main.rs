use std::process::Command;

fn main() {
    println!("Running SFC");

    Command::new("sfc")
            .arg("/scannow")
            .status()
            .unwrap();

    println!("Running DISM");

    Command::new("dism")
            .arg("/online")
            .arg("/cleanup-image")
            .arg("/restorehealth")
            .status()
            .unwrap();
}
