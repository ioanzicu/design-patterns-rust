use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct GlobalCounter {
    count: Arc<Mutex<u32>>, // Shared, thread safe mutable counter
}

impl GlobalCounter {
    fn new() -> Self {
        GlobalCounter {
            count: Arc::new(Mutex::new(0)),
        }
    }

    fn increment(&self) {
        // Acquire the lock. This blocks if another thread has the lock.
        // unwrap() here will panic if the mutex is poisoned - the thread that acquired the lock panicked.
        let mut num_guard = self.count.lock().unwrap();
        *num_guard += 1; // Mutate the data through the MutexGuard
                         // Lock is released when num_guard goes out of scope
    }

    fn get_value(&self) -> u32 {
        *self.count.lock().unwrap() // Lock, get value, unlock
    }
}

fn main() {
    let global_counter = GlobalCounter::new();
    let mut handles = vec![];
    println!("Initial count: {}", global_counter.get_value()); // 0

    // Spawn multiple threads that all increment the same counter
    for i in 0..5 {
        // Clone the Arc to give ownership to the new thread
        let counter_clone = Arc::clone(&global_counter.count);

        let handle = thread::spawn(move || {
            // This is a different way to use the Arc<Mutex<T>> directly
            // without the GlobalCounter struct, for illustration.
            for _ in 0..100 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
                // Slight delay to make interleaving more likely
                thread::sleep(Duration::from_micros(1));
            }

            println!("Thread {} finished incrementing.", i)
        });

        handles.push(handle);
    }

    // Using the methods on GlobalCounter struct from the main thread
    // This is a bit contrived here as the struct methods would also
    // contend for the same lock if called concurrently with the threads above,
    // but demonstrates method usage.
    global_counter.increment(); // Main thread also increments
                                // Wait for all spawned threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Final count: {}", global_counter.get_value()); // Expected: 5 * 100 + 1 = 501
}
