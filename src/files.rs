use std::env;
use std::env::VarError;

use std::fs;
use std::io::{Error, ErrorKind};

pub fn get_directory_path() -> Result<String, VarError> {
    let mut args_iter = env::args();
    loop {
        match args_iter.next() {
            Some(arg) => {
                if arg == "--directory" {
                    let path = args_iter.next().unwrap_or_default();
                    // println!("{path}");
                    return Ok(path);
                }
            }
            None => return Err(VarError::NotPresent),
        }
    }
}

pub fn read_file(path: &str) -> Result<String, Error> {
    let mut file_path = get_directory_path().expect("Directory path not existent");
    file_path.push_str("/");
    file_path.push_str(path);

    // println!("{file_path}");

    let file_result: Result<String, Error> = fs::read_to_string(file_path);
    match file_result {
        Ok(content) => Ok(content),
        Err(_) => Err(Error::from(ErrorKind::NotFound)),
    }
}
