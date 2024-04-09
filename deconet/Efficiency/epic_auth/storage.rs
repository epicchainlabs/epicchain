use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

/// Reads the contents of a file into a string.
fn read_file(file_path: &str) -> io::Result<String> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut contents = String::new();
    for line in reader.lines() {
        contents.push_str(&line?);
        contents.push('\n');
    }
    Ok(contents)
}

/// Writes a string to a file, creating the file if it does not exist.
fn write_file(file_path: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Appends a string to the end of a file.
fn append_to_file(file_path: &str, content: &str) -> io::Result<()> {
    let mut file = fs::OpenOptions::new().append(true).open(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Deletes a file from the file system.
fn delete_file(file_path: &str) -> io::Result<()> {
    fs::remove_file(file_path)?;
    Ok(())
}

/// Copies a file from one location to another.
fn copy_file(source_path: &str, destination_path: &str) -> io::Result<()> {
    fs::copy(source_path, destination_path)?;
    Ok(())
}

/// Moves a file from one location to another.
fn move_file(source_path: &str, destination_path: &str) -> io::Result<()> {
    fs::rename(source_path, destination_path)?;
    Ok(())
}

/// Creates a new directory.
fn create_directory(dir_path: &str) -> io::Result<()> {
    fs::create_dir(dir_path)?;
    Ok(())
}

/// Removes an existing directory and all its contents.
fn remove_directory(dir_path: &str) -> io::Result<()> {
    fs::remove_dir_all(dir_path)?;
    Ok(())
}

fn main() {
    let file_path = "example.txt";
    let content_to_write = "Hello, world!\n";
    write_file(file_path, content_to_write).unwrap();

    let content_to_append = "More content\n";
    append_to_file(file_path, content_to_append).unwrap();

    let content_read = read_file(file_path).unwrap();
    println!("Content read from file:\n{}", content_read);

    delete_file(file_path).unwrap();
}
