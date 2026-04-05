use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    // A shared configuration that many threads will read, but one will update.
    let config = Arc::new(RwLock::new("Initial Config".to_string()));
    let mut handles = vec![];

    // --- Spawn multiple READER threads ---
    for i in 0..5 {
        let config_clone = Arc::clone(&config);
        handles.push(thread::spawn(move || {
            // Acquire a read lock.
            // Multiple threads can hold this at the same time.
            let config_guard = config_clone.read().unwrap();
            println!("Reader {}: sees config: '{}'", i, *config_guard);
        }));
    }

    // --- Spawn one WRITER thread ---
    let config_clone = Arc::clone(&config);
    handles.push(thread::spawn(move || {
        // Simulate some work before writing
        thread::sleep(std::time::Duration::from_millis(10));

        // Acquire a write lock.
        // This will wait until all readers are done.
        // Once held, no new readers or writers can get a lock.
        let mut config_guard = config_clone.write().unwrap();
        *config_guard = "Updated Config".to_string();
        println!("--- Writer: Updated config! ---");
    }));

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("\nFinal config value: '{}'", *config.read().unwrap());
}
