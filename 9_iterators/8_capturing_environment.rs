fn main() {
    let factor = 3;
    let threshold = 50;

    // Captures 'factor' by immutable reference (&)
    // It implements the Fn trait
    let multiply_by_factor = |n| n * factor;
    println!("5 times factor: {}", multiply_by_factor(5)); // 15

    // Captures 'items_processed' by mutable reference (&mut) because it modifies it.
    // It implements FnMut trait
    let mut items_processed = 0;

    let mut process_item = |item_id| {
        println!("Processing item {}", item_id);
        items_processed += 1;
    };

    process_item(101); // 102
    process_item(102); // 103
    println!("Items processed: {}", items_processed); // 2

    // Takes ownership of 'data_to_own' because of the 'move' keyword
    // It implements the FnOnce trait - can be called only once, only once it is moved
    let data_to_own = vec![1, 2, 3];
    let consume_data = move || println!("Consuming data: {:?}", data_to_own);
    consume_data(); // [1, 2, 3]
                    // println!("{:?}", data_to_own); // Error! data_to_own was moved

    // Closures + iterators often involves capturing
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let greater_than_threshold: Vec<i32> = numbers
        .into_iter() // take ownership
        .filter(|&num| num > threshold) // capture the 'threshold' by reference
        .collect();

    println!("Numbers > {}: {:?}", threshold, greater_than_threshold); // 50, []
}
