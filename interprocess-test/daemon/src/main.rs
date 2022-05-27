use interprocess::local_socket::{LocalSocketListener, LocalSocketStream};
use std::{
    error::Error,
    io::{self, prelude::*, BufReader},
};
use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;

pub fn main() -> Result<(), Box<dyn Error>> {
    fn handle_error(connection: io::Result<LocalSocketStream>) -> Option<LocalSocketStream> {
        connection
            .map_err(|error| eprintln!("Incoming connection failed: {}", error))
            .ok()
    }

    let sock = "/tmp/example.sock";

    reset_sock(&sock).unwrap();

    let listener = LocalSocketListener::bind(sock)?; // need to figure out how to set the file permissions on the socket
    
    change_socket();

    for mut conn in listener.incoming().filter_map(handle_error) {
        println!("Incoming connection!");

        conn.write_all(b"Hello from server!\n")?;

        // Add buffering to the connection to read a line.
        let mut conn = BufReader::new(conn);
        let mut buffer = String::new();
        conn.read_line(&mut buffer)?;

        println!("Client answered: {}", buffer);

        write_message(&buffer);
        run_command();
    }
    Ok(())
}

fn change_socket() {
    Command::new("chmod")
            .arg("777")
            .arg("/tmp/example.sock")
            .spawn()
            .expect("chmod command failed to start");
}

fn run_command() {
    Command::new("ls")
            .arg("-l") // these arguments can't be folded into the new argument
            .arg("-a")
            .spawn()
            .expect("ls command failed to start");
}

fn reset_sock(sock: &str) -> std::io::Result<()> {
    if std::path::Path::new(sock).exists() {
        std::fs::remove_file(sock)?;
        Ok(())
    }
    else {
        Ok(())
    }
}

fn write_message(message: &str) -> std::io::Result<()> {
    let filename = "server_output.txt";

    if std::path::Path::new(filename).exists() {
        let mut f = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(filename)
                        .unwrap();

        if let Err(e) = writeln!(f, "{}", &message) {
            eprintln!("Couldn't write to the file: {}", e);
        }

        Ok(())
    } else {
        let mut f = File::create(filename)?;
        f.write_all(message.as_bytes())?;
        Ok(()) // this is necessary for the ? to work at the end of the previous 2 lines
               // it also requires the std::io::Result as a return parameter
    }
}
