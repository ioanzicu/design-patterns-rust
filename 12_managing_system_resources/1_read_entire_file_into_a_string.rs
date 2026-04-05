use std::fs;
use std::path::Path;
use std::process;

fn safe_write(path: &Path, content: &str) {
    if path.exists() {
        // The file already exists. Decide what to do:
        // 1. Return an error.
        // 2. Ask the user for confirmation.
        // 3. Do nothing.
        eprintln!("Error: File '{}' already exists. Aborting to prevent overwrite.", path.display());
    } else {
        // The file doesn't exist, so it's safe to write.
        if let Err(e) = fs::write(path, content) {
            eprintln!("Error writing to new file: {}", e);
        } else {
            println!("Successfully wrote to new file '{}'", path.display());
        }
    }
}

fn main() {
    let sample_path = Path::new("a_file.txt");
    // --- Success Case: Read an existing file ---
    if let Err(e) = fs::write(sample_path, "Hello from a Rust test file!") {
        eprintln!("Setup failed: Could not write to sample file: {}", e);
        process::exit(1);
    }

    println!("---  Attempting to read '{}'  ---", sample_path.display());
    match fs::read_to_string(sample_path) {
        Ok(contents) => {
            println!("Success! File contents: '{}'", contents);
        }
        Err(e) => {
            eprintln!("Unexpected error reading the file: {}", e);
        }
    }

    // Clean up the created file.
    let _ = fs::remove_file(sample_path);
    println!("\n---------------------------\n");
    
    // --- Failure Case: Try to read a non-existent file ---
    let non_existent_path = Path::new("no_such_file.txt");
    println!("---  Attempting to read '{}'  ---", non_existent_path.display());
    match fs::read_to_string(non_existent_path) {
        Ok(_) => {
            // This should not happen.
            println!("Unexpectedly found a file that should not exist!");
        }
        Err(e) => {
            // This is the expected outcome.
            eprintln!("Correctly failed to read non-existent file. Error: {}", e);
        }
    }
}
