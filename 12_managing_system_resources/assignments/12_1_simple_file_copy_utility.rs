use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::process;

// Usage
//
// ./12_1_simple_file_copy_utility ../a_sample.txt ../c_sample
//
fn main() -> io::Result<()> {
    if env::args().len() < 3 {
        eprintln!("Usage: mycopy <source_file_path> <destination_file_path>");
        process::exit(1);
    }

    // 1. Get cli arguments: source_file_path and destination_file_path
    let source_file_arg = match env::args().nth(1) {
        Some(path) => path,
        None => {
            // not expected to happen, but in any cases
            eprintln!("Usage: mycopy <source_file_path> <destination_file_path>");
            process::exit(1);
        }
    };

    let destination_file_arg = match env::args().nth(2) {
        Some(path) => path,
        None => {
            // not expected to happen, but in any cases
            eprintln!("Usage: mycopy <source_file_path> <destination_file_path>");
            process::exit(1);
        }
    };

    println!(
        "src: {:?},\ndst: {:?}",
        source_file_arg, destination_file_arg
    );

    let source_file_path = Path::new(&source_file_arg);

    // 2. Open and read the entire file content of source_file
    let source_file = File::open(source_file_path)?;
    let reader = BufReader::new(source_file);

    // 3. Write the content to the destination file, create it if not exists
    let new_filename = match source_file_path.extension().and_then(|e| e.to_str()) {
        Some(ext) => format!("{}_copy.{}", destination_file_arg, ext),
        None => format!("{}_copy", destination_file_arg), // Add _copy if no extension found
    };

    println!("Copying {} to {}", source_file_arg, new_filename);

    let mut destination_file = match File::create(&new_filename) {
        Ok(file) => file,
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    eprintln!(
                        "Error: Permission denied. You don't have right to create files here."
                    );
                }
                std::io::ErrorKind::AlreadyExists => {
                    eprintln!("Error: The file '{}' already exists.", new_filename);
                }
                _ => eprintln!("Error creating file {}: {}", new_filename, e),
            }
            std::process::exit(1);
        }
    };

    for line in reader.lines() {
        let line = line?; // Handle potential I/O errors

        writeln!(destination_file, "{}", line)?;
    }

    destination_file.flush()?;

    Ok(())
}
