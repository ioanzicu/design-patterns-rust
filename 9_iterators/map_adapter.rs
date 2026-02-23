fn main() {
    let numbers = vec![1, 2, 3, 4];
    let squares: Vec<i32> = numbers
        .iter() // yields &i32
        .map(|x| x * x) // &32 * &i32 -> i32 | take be reference and return new value
        .collect(); // new vector with transformed values

    println!("Original: {:?}", numbers);

    println!("Squares: {:?}", squares);

    let names = vec!["alice", "bob", "charlie"];
    let upper_names: Vec<String> = names
        .iter() // yields &i32
        .map(|name| name.to_uppercase()) // &name -> name
        .collect(); // new vector with tranformed values

    println!("Upper names: {:?}", upper_names);
}
