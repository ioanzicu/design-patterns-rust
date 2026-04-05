use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct SharedResource {
    id: u32,
    data: String,
}

fn main() {
    // Create shared data wrapped in an Arc
    let shared_resource_main = Arc::new(SharedResource {
        id: 1001,
        data: "This is some super important data shared across threads".to_string(),
    });
    println!(
        "Main thread: Initial strong count = {}",
        Arc::strong_count(&shared_resource_main)
    ); // 1

    let mut thread_handles = vec![];
    for i in 0..3 {
        // Spawn 3 threads
        // Clone the Arc for each thread. This is crucial.
        // The cloned Arc is moved into the thread's closure.
        let shared_resource_for_thread = Arc::clone(&shared_resource_main);
        println!(
            "Main thread: Count before spawning thread {} = {}",
            i,
            Arc::strong_count(&shared_resource_main)
        ); // 2, 3, 4

        let handle = thread::spawn(move || {
            // This thread now has its own Arc pointing to the shared data
            println!(
                "Thread {}: Started. Accessing resource ID: {}, Data: '{}'. Strong count here: {}",
                i,
                shared_resource_for_thread.id,
                shared_resource_for_thread.data,
                Arc::strong_count(&shared_resource_for_thread)
            ); // Count might be higher due to other clones

            thread::sleep(Duration::from_millis(50)); // Simulate some work
            println!("Thread {}: Finished.", i);

            // When shared_resource_for_thread goes out of scope here, the count is decremented.
        });

        thread_handles.push(handle);
    }

    // The main thread still has its reference
    println!(
        "Main thread: After spawning threads, strong count = {}",
        Arc::strong_count(&shared_resource_main)
    ); // 4

    println!("Main thread: Resource data: {}", shared_resource_main.data);

    // Wait for all threads to complete
    for handle in thread_handles {
        handle.join().unwrap();
    }

    println!("Main thread: All threads finished. Final strong count (before shared_resource_main drops): {}", Arc::strong_count(&shared_resource_main));
    // Should be 1

    // When shared_resource_main drops, the count goes to 0, and SharedResource is deallocated.
}
