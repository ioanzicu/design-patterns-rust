fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let greater_than_two: Vec<_> = numbers.iter().filter(|&&n| n > 2).collect();
    println!("Numbers greater than 2: {:?}", greater_than_two); // 3, 4, 5

    let total: i32 = (1..=10).sum();
    println!("The sum of numbers from 1 to 10 is: {}", total); // 55

    let product = numbers
        .iter()
        .fold(1, |accumulator, &item| accumulator * item);
    println!("The product of the number is: {}", product);
    // 1 * 2 = 2
    // 2 * 3 = 6
    // 6 * 4 = 24
    // 24 * 5 = 120
}
