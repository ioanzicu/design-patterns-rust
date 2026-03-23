use std::thread;

fn main() {
    let message = String::from("Hello from the main thread!");
    let important_number = 33;

    // The `move` keyword forces the closure to take
    // ownership of `message` and `important_number`.
    let handle = thread::spawn(move || {
        println!("Spawned thread received message: '{}'", message);
        println!("Spawned thread received number: {}", important_number);

        // `message` and `important_number` are now owned by this closure's environment.
        // The original variables in main are no longer accessible if they were move (like String).
        // For Copy types like i32, a copy is moved.
    });

    // Attempting to use `message` here would cause a compile error:
    // println!("Main thread still has message: {}", message);
    // ERROR! value borrowed here after move
    // `important_number` was an i32, which is Copy, so a copy was moved.
    // The original `important_number` in main is still valid.
    println!(
        "Main thread still has important_number: {}",
        important_number
    );

    // Wait for the thread to finish
    handle.join().unwrap();
    println!("Main thread: Spawned thread finished.");
}
