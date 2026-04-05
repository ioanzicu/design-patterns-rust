// mini_grep "search_term" path/to/my/file.txt
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;
use std::{env, fs};

fn list_directory_contents(dir_path_str: &str) -> io::Result<()> {
    let path = Path::new(dir_path_str);
    if !path.is_dir() {
        eprintln!(
            "Error: '{}' is not a directory or does not exist.",
            path.display()
        );
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Path is not a directory",
        ));
    }

    println!("Contents of directory '{}':", path.display());
    // fs::read_dir returns a Result containing an iterator over DirEntry results
    for entry_result in fs::read_dir(path)? {
        // '?' propagates I/O errors from read_dir itself
        let entry = match entry_result {
            Ok(e) => e,
            Err(e) => {
                // Error accessing a specific entry, log it and continue
                eprintln!(
                    "Warning: Could not access an entry in '{}', {}",
                    path.display(),
                    e
                );
                continue;
            }
        };

        let entry_path = entry.path();
        let entry_name = entry_path.file_name().unwrap_or_default().to_string_lossy();

        // Get metadata to determine if it's a file or directory
        // This can fail ex: permissions
        match fs::metadata(&entry_path) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    println!("   [DIR]   {}", entry_name);
                } else if metadata.is_file() {
                    println!("   [FILE]  {} ({} bytes)", entry_name, metadata.len());
                } else {
                    println!("   [OTHER] {}", entry_name); // Symlinks, etc.
                }
            }
            Err(e) => {
                eprintln!(
                    "Warning: Could not get metadata for '{}': {}",
                    entry_path.display(),
                    e
                );
            }
        }
    }
    Ok(())
}

fn read_and_print_file_lines(file_path_str: &str) {
    let path = Path::new(file_path_str);

    // Attempt to open the file
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: Could not open file '{}': {}", path.display(), e);
            process::exit(1);
        }
    };

    // Use BufReader for efficient line-by-line reading
    let reader = BufReader::new(file);
    println!("--- Contents of '{}' ---", path.display());

    for (index, line_result) in reader.lines().enumerate() {
        match line_result {
            Ok(line_content) => {
                println!("Line {}: {}", index + 1, line_content);
            }
            Err(e) => {
                // Log error for a specific line but continue if possible,
                // or decide to exit if line read errors are critical.
                eprintln!(
                    "Error reading line {} from '{}': {}",
                    index + 1,
                    path.display(),
                    e
                );
            }
        }
    }

    println!("--- End of '{}' ---", path.display());
}

// TEST
// cargo run -- sample_cli_read.txt

fn main() {
    // env::args() returns an iterator over the command-line arguments.
    // We can collect these into a Vec<String>.
    let arguments: Vec<String> = env::args().collect();
    println!("Total arguments passed: {}", arguments.len());

    // Print each argument along with its index.
    // args[0] is typically the path used to execute the program.
    for (index, argument) in arguments.iter().enumerate() {
        println!("Argument [{}]: {}", index, argument);
    }

    if arguments.len() < 3 {
        if arguments.len() > 0 {
            eprintln!("\nUsage: {} <query> <file_path>", arguments[0]);
        } else {
            eprintln!("\nUsage: <program_name> <query> <file_path>");
        }

        eprintln!("Error: Not enough arguments provided.");
        std::process::exit(1);
    }

    let query_arg = &arguments[1];
    let file_path_arg = &arguments[2];
    println!("\nIntended query: {}", query_arg);
    println!("Intended file path: {}", file_path_arg);

    if !Path::new(file_path_arg).exists() {
        if file_path_arg == "sample_cli_read.txt" {
            // if not exists, create for testing
            std::fs::write(
                file_path_arg,
                "First line for CLI test.\nSecond line, with a keyword.\nThird and final line.",
            )
            .expect("Faile to create sample file.");
            println!("Created sample file: {}", file_path_arg);
        } else {
            eprintln!(
                "Specified file '{}' does not exist and won't be auto-created for this generic example.",
                file_path_arg
            );
            process::exit(1);
        }

        read_and_print_file_lines(file_path_arg);

        // Clean up the dummy file if created for test
        if file_path_arg == "sample_cli_read.txt" {
            std::fs::remove_file(file_path_arg).ok();
        }
    }

    // List files
    // println!("Attempting to list contents of '{}'...", dir_to_list);
    // if let Err(e) = list_directory_contents(&dir_path_list) {
    //     if e.kind() != io::ErrorKind::NotFound {
    //         eprintln!("An error occurred: {}", e);
    //     }
    //     process::exit(1);
    // }
}
