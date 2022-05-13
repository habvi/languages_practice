use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

fn read_file_contents(path: PathBuf) -> Result<String, Error> {
    // open -> Result<File, Error>
    let mut file: File = match File::open(path) {
        Ok(file_handle) => file_handle,
        Err(io_error) => return Err(io_error),
    };

    let mut string = String::new();
    // read_to_string : add file to string
    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(io_error) => return Err(io_error),
    };

    Ok(string)
}

fn main() {
    if read_file_contents(PathBuf::from("src/main.rs")).is_ok() {
        println!("The file found.");
    }

    if read_file_contents(PathBuf::from("non-existant-file.txt")).is_err() {
        println!("The program reported an error the file doesn't exist");
    }
}