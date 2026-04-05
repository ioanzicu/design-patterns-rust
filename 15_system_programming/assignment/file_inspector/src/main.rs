use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1); // skip program name

    let mut count_lines = false;
    let mut count_words = false;
    let mut count_chars = false;
    let mut file_path: Option<String> = None;

    // ---  Argument parsing  ---
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-l" | "--lines" => count_lines = true,
            "-w" | "--words" => count_words = true,
            "-c" | "--chars" => count_chars = true,
            _ => {
                if file_path.is_none() {
                    file_path = Some(arg);
                } else {
                    return Err("Multiple file paths provided".into());
                }
            }
        }
    }

    let file_path = file_path.ok_or("No file path provided")?;

    // If no flags -> enable all
    if !count_lines && !count_words && !count_chars {
        count_lines = true;
        count_words = true;
        count_chars = true;
    }

    // ---  File processing  ---
    let file = File::open(&file_path).map_err(|_| format!("Failed to open file: {}", file_path))?;

    let reader = BufReader::new(file);

    let mut lines = 0usize;
    let mut words = 0usize;
    let mut chars = 0usize;

    for line_result in reader.lines() {
        let line = line_result.map_err(|_| "Failed to read line")?;

        lines += 1;
        words += line.split_whitespace().count();
        chars += line.chars().count() + 1;
    }

    // ---  Output  ---
    println!("----------");
    if count_lines {
        println!("Lines: {}", lines);
    }
    if count_words {
        println!("Words: {}", words);
    }
    if count_chars {
        println!("Chars: {}", chars);
    }
    println!("----------");

    Ok(())
}
