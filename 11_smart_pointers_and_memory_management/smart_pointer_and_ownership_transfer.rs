use std::rc::Rc;

fn main() {
    // Box<T> - unique ownership

    let b1 = Box::new(5);
    let b2 = b1;
    // Ownership of the Box (and the heap data 5) MOVES from b1 to b2.
    // println!("{}", b1); // Error! b1 was moved.
    println!("{}", b2); // Ok - 5

    // Rc<T> and Arc<T> - shared ownership
    let rc1 = Rc::new("hello".to_string());
    let rc2 = Rc::clone(&rc1); // rc1 and rc2 now both point to "hello"
                               // Ownership of "hello" is shared.

    println!("rc1: {}\nrc2: {}", rc1, rc2); // Both are valid.
    println!("Strong count: {}", Rc::strong_count(&rc1)); // Will be 2
}
