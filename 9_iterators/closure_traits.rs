// Accepts closure that only need immutable access (`Fn`)
fn call_reporter<F>(reporter: F)
where
    F: Fn() -> String, // Trait bound: must implement Fn
{
    println!("Report: {}", reporter());
}

// Accepts closure that might mutate their environment (`FnMut`)
fn call_mutator<F>(mut mutator: F)
// `mut`` is required
where
    F: FnMut(),
{
    // Can be called multiple times
    mutator();
    mutator();
}

// Accepts any closure but consumes it (`FnOnce`)
fn call_once<F>(consumer: F)
where
    F: FnOnce(), // Trait bound: must implement FnOnce
{
    consumer();
    // Calling `consumer()` again here would cause a compile error
}

fn main() {
    let message = String::from("System status OK");

    // This closure captures `message` by reference, it implements `Fn`
    let report_closure = || message.clone();
    call_reporter(report_closure); // System status OK

    let mut counter = 0;
    // This closure captures `counter` by mutable reference, so it implements `FnMut`
    let mut increment_closure = || {
        counter += 1;
        println!("Counter is now: {}", counter);
    };
    // We pass ownership of the closure to `call_mutator`
    call_mutator(increment_closure);
    // Counter is now: 1
    // Counter is now: 2

    let data = String::from("Consume me");
    // This closure movec `data`, it implements `FnOnce`
    let consume_closure = || {
        println!("Consumed: {}", data);
    };
    call_once(consume_closure); // Consumed: Consume me
}
