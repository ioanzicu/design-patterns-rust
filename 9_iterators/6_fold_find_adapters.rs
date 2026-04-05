fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let product = numbers.iter().fold(1, |acc, &x| acc * x);
    println!("Product: {}", product); // 120

    let first_even = numbers.iter().find(|&&x| x % 2 == 0); // 2
    match first_even {
        Some(n) => println!("First even number: {}", n), // 2
        None => println!("No even numbers found."),
    }

    let first_gt_3_value: Option<i32> = numbers.iter().find(|&&x| x > 3).copied();
    println!("First > 3 value: {:?}", first_gt_3_value); // Some(4)
}
