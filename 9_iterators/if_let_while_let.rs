// `if let` and `while let` allows to write shorter code that in `match` handles only one case

// `while let` - loop as long as the pattern keeps matching
// paired with methods that return an option like `.next()`, `.pop()`
// until it returns a `None`.
// In async programming this pattern processes stream items or polls future until ready.

fn main() {
    let maybe_value: Option<i32> = Some(10);

    // Insted of `match`
    if let Some(value) = maybe_value {
        println!("Got a value using if let: {}", value);
    } else {
        println!("No value found.");
    }

    let mut data_stack = vec![Some(1), Some(2), None, Some(3)];
    // loop {
    //     match data_stack.pop() {
    //         Some(Some(x)) => println!("Some({})", x),
    //         _ => break,
    //     }
    // }

    // Process items from the stack as logn as they are Some(Some(value))
    while let Some(Some(value)) = data_stack.pop() {
        // Outer `Some` matches the `Option` from `pop()`.
        // Inner `Sime` matches the `Option<i32>` that was inside the `Vec`
        // The loop contiunes as long as we successfully pop a `Some(value)`
        println!("Processing value from stack: {}", value);
    }

    // Loop stops when pop() returns None or Some(None)
    println!("Stack processing finished. Remaining: {:?}", data_stack);
    // [Some(1), Some(2)] or reversed depending on pop order
}
