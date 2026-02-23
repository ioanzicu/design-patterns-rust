fn process_names(input: Vec<&str>) {
    let out: Vec<_> = input
        .into_iter()
        .filter(|x: &&str| x.len() >= 5)
        .map(|x| x.to_uppercase())
        .collect();

    println!("{:?}", out);
}

fn main() {
    let input = vec!["Alice", "Bob", "Charlie", "David", "Eve"];
    process_names(input);
}
