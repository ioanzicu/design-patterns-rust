use argh::FromArgs;
use std::sync::mpsc;
use std::thread;
use std::{collections::HashMap, io};
use std::{fs, path::PathBuf};

/// CLI argument structure
#[derive(FromArgs)]
#[argh(description = "Count words in files concurrently")]
struct Args {
    /// a list of file paths
    #[argh(positional)]
    files: Vec<PathBuf>,
}

/// Process a single file and return a word frequency map
fn process_file(path: &PathBuf) -> Result<HashMap<String, u32>, io::Error> {
    let content: String = fs::read_to_string(path)?; // early return on error

    let mut freq: HashMap<String, u32> = HashMap::new();

    for word in content.split_whitespace() {
        let word = word
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_lowercase();

        if !word.is_empty() {
            *freq.entry(word).or_insert(0) += 1;
        }
    }

    Ok(freq)
}

/// Merge a local word frequency map into a global map
fn merge_maps(total: &mut HashMap<String, u32>, local: HashMap<String, u32>) {
    for (word, count) in local {
        *total.entry(word).or_insert(0) += count;
    }
}

/// Convert HashMap into sorted Vec<(word, count)> descending by count
fn sort_frequencies(freq_map: HashMap<String, u32>) -> Vec<(String, u32)> {
    let mut freq_vec: Vec<(String, u32)> = freq_map.into_iter().collect();

    // sort by frequency descending
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
    freq_vec
}

fn main() {
    let args: Args = argh::from_env();
    println!("Files to process: {:?}", args.files);

    let mut handles: Vec<thread::JoinHandle<()>> = vec![];

    let (tx, rx) = mpsc::channel::<Result<HashMap<String, u32>, String>>();

    // spawn a thread for each file
    for path in args.files {
        let tx_producer = tx.clone();

        let handle = thread::spawn(move || {
            let result: Result<HashMap<String, u32>, String> = match process_file(&path) {
                Ok(freq_map) => Ok(freq_map),
                Err(e) => Err(format!("Failed to read '{}': {}", path.display(), e)),
            };

            if let Err(e) = tx_producer.send(result) {
                eprintln!("Failed to send result {}", e);
            }
        });

        handles.push(handle);
    }

    // ensure the tx is closed so receiver loop can terminate
    drop(tx);

    let mut final_freq_map: HashMap<String, u32> = HashMap::new();

    // Aggregate maps as they arrive
    for result in rx {
        // loop will block until the tx is not closed
        match result {
            Ok(map) => merge_maps(&mut final_freq_map, map),
            Err(msg) => eprintln!("{}", msg),
        }
    }

    // Join all threads AFTER receiver finishes
    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("Thread panicked: {:?}", e);
        }
    }

    let sorted_vec = sort_frequencies(final_freq_map);

    let n = 10.min(sorted_vec.len()); // avoid out-of-bounds

    println!("\n\nTop {} words by frequency:\n", n);
    for i in 0..n {
        let (word, count) = &sorted_vec[i];
        println!("{}. Word: '{}', Count: '{}'", i + 1, word, count);
    }
}
