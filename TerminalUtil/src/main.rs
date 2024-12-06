use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Define the file path
    let file_path = "/Users/josef/.zsh_history";

    // Check if the file exists
    if !Path::new(file_path).exists() {
        eprintln!("File not found: {}", file_path);
        return Ok(());
    }

    // Open the file
    let file = File::open(file_path)?;

    // Use a BufReader for efficient reading
    let reader = io::BufReader::new(file);

    // Read and print lines
    for line in reader.lines() {
        let line = line?; // Handle potential IO errors per line
        println!("{}", line);
    }

    Ok(())
}
