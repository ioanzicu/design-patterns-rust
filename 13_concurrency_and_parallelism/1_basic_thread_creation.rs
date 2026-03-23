use std::thread;
use std::time::Duration;

fn main() {
    println!("Main thread: Starting up!");

    // Spawn a new thread
    let handle = thread::spawn(|| {
        // This code runs in the new thread
        for i in 1..=5 {
            println!("New thread: count {}", i);
            thread::sleep(Duration::from_millis(500)); // Pause for 0.5 seconds
        }
        println!("New thread: I'm done!");
    });

    // The main thread continues its work immediately
    for i in 1..=3 {
        println!("Main thread: working... {}", i);
        thread::sleep(Duration::from_millis(300)); // Pause for 0.3 seconds
    }

    println!("Main thread: Waiting for the new thread to finish...");

    // We'll see how to properly wait for the handle next.
    // For now, if main exits, the spawned thread might be killed.
    // To ensure the spawned thread finishes in this example we can
    // add a longer sleep here, but using join() is the correct way.

    thread::sleep(Duration::from_secs(3)); // Temporary, to see spawned thread output
                                           // The correct way to wait for the spawned thread:

    handle.join().unwrap(); // We'll explain join() shortly
    println!("Main thread: All done!");
}
