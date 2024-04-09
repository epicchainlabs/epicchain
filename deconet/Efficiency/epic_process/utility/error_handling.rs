use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

/// Enum representing different types of errors.
#[derive(Debug)]
pub enum CustomError {
    Io(io::Error),
    Custom(String),
    Parse(ParseIntError),
    Other(Box<dyn std::error::Error>),
}

impl From<io::Error> for CustomError {
    fn from(err: io::Error) -> Self {
        CustomError::Io(err)
    }
}

impl From<ParseIntError> for CustomError {
    fn from(err: ParseIntError) -> Self {
        CustomError::Parse(err)
    }
}

/// Function to read a file and return its content as a string.
pub fn read_file(file_path: &str) -> Result<String, CustomError> {
    let mut file = File::open(file_path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

/// Function to handle a custom error.
pub fn handle_custom_error() -> Result<(), CustomError> {
    Err(CustomError::Custom("Custom error message".to_string()))
}

/// Function to parse a string into an integer and chain errors.
pub fn parse_int_and_chain_error(s: &str) -> Result<i32, CustomError> {
    let num_str = "not a number";
    let num = num_str.parse::<i32>()?;
    Ok(num)
}

/// Function to demonstrate custom error propagation.
pub fn propagate_custom_error() -> Result<(), CustomError> {
    let result = handle_custom_error()?;
    Ok(result)
}

fn main() {
    let file_path = "example.txt";
    match read_file(file_path) {
        Ok(content) => println!("File content: {}", content),
        Err(err) => eprintln!("Error reading file: {:?}", err),
    }

    match handle_custom_error() {
        Ok(_) => println!("No custom error occurred"),
        Err(err) => eprintln!("Custom error occurred: {:?}", err),
    }

    match parse_int_and_chain_error("not a number") {
        Ok(num) => println!("Parsed number: {}", num),
        Err(err) => eprintln!("Error parsing number: {:?}", err),
    }

    match propagate_custom_error() {
        Ok(_) => println!("Custom error propagation successful"),
        Err(err) => eprintln!("Error propagating custom error: {:?}", err),
    }
}
