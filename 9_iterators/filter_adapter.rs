// &T  - .iter() on a collection gives an itertor of references
// &T that act like one layer of wrapping aroung each item.

// &&T - .filter() often pass a reference to that item to your closure,
// which can add a second layer of wrapping, resulting in &&T

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let evens: Vec<i32> = numbers
        .iter() // yields &32
        .filter(|&&x| x % 2 == 0) // double reference
        .copied()
        .collect();

    println!("Original: {:?}", numbers); // [1, 2, 3, ..., 10]
    println!("Evens: {:?}", evens); // [2, 4, 6, 8, 10]

    let scores = vec![85, 42, 95, 60, 77];
    let adjusted_high_scores: Vec<i32> = scores
        .iter() // yields &32
        .filter(|&&score| score > 70) // double reference
        .map(|&score| score + 5) // return new value + 5
        .collect();

    println!("Adjusted high scores: {:?}", adjusted_high_scores);
    // [90, 100, 92]
}
