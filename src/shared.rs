use std::fs::File;
use std::io::{self, Read};

use failure::Error;

pub type AppResult<T> = Result<T, Error>;


pub fn read_stdin() -> AppResult<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.into())
}


pub fn read_file(filename: &str) -> AppResult<String> {
    let mut buffer = String::new();
    let mut handle = File::open(filename)?;

    handle.read_to_string(&mut buffer)?;
    Ok(buffer.into())
}
