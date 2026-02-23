fn main() {
    // iter() - Immutable Borrow
    let names = vec!["Ioan", "Decebal", "Burebista"];
    for name in names.iter() {
        println!("Hello, {}!", name);
    }

    println!("THe names vector is still available: {:?}\n", names);

    // iter_mut() - Mutable Borrow, inplace value update
    let mut numbers = vec![10, 20, 30];
    for num in numbers.iter_mut() {
        *num *= 2;
    }

    println!("The numbers vector has been modified: {:?}\n", numbers);

    // into_iter() - Take Full Ownership and consume the vectors
    let messages = vec![String::from("1"), String::from("2")];
    for msg in messages.into_iter() {
        println!("Processing message: {}", msg);
    }

    // messages vector has been moved and is no longer valid
    // println!("Messages: {:?}", messages); // error
}
