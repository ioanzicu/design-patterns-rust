fn main() {
    // iterators are lazy
    let fruits = vec!["apple", "banana", "cherry"];
    let mut fruit_iterator = fruits.iter();

    // next() - yelds immutable references &str
    println!("First call: {:?}", fruit_iterator.next()); // "apple"
    println!("Second call: {:?}", fruit_iterator.next()); // "banana"
    println!("Third call: {:?}", fruit_iterator.next()); // "cherry"

    println!("Fourth call: {:?}", fruit_iterator.next()); // None
    println!("Fifth call: {:?}", fruit_iterator.next()); // None
}
