use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;

const CHUNK_SIZE: usize = 8 * 1024; // 8KB chunks

fn generate_test_file(path: &Path, size_mb: usize) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    // Create a 1KB pattern to repeat
    let pattern = vec![65u8; 1024]; // A bunch of 'A's

    for _ in 0..(size_mb * 1024) {
        writer.write_all(&pattern)?;
    }

    writer.flush()?; // Ensure all data is written to disk
    println!("Generated a {} MB test file at {:?}", size_mb, path);
    Ok(())
}

fn process_large_file_in_chunks(file_path: &Path) -> io::Result<()> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file); // Buffering is still good!
    let mut chunk = vec![0u8; CHUNK_SIZE];

    loop {
        // Attempt to fill the chunk buffer
        // reader.read() might not fill the whole buffer if EOF is reached.
        let bytes_read = match reader.read(&mut chunk) {
            Ok(0) => break, // End of file
            Ok(n) => n,
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue, // Retry on interrupt
            Err(e) => return Err(e),                                          // Other error
        };
        // Process the data in `&chunk[..bytes_read]`
        // For example, count occurrences of a byte, hash the chunk, etc.
        println!("Processed a chunk of {} bytes", bytes_read);
        // In a real app, you'd do something more useful here.
        if bytes_read < CHUNK_SIZE {
            break; // Likely reached EOF or a partial last
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let test_file_path = Path::new("test_large_file.bin");

    // 1. Generate a 2MB file (plenty of 8KB chunks)
    generate_test_file(test_file_path, 2)?;

    // 2. Process it
    println!("Starting processing...");
    process_large_file_in_chunks(test_file_path)?;

    // 3. Optional: Clean up the file afterwards
    std::fs::remove_file(test_file_path)?;

    Ok(())
}
