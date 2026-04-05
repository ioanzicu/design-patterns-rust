#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    // Box<i32> store an integer on the heap
    let heap_int = Box::new(5);
    println!("Value in a Box: {}", heap_int); // 5

    // Create a Box<Point> to store a struct on the heap
    let heap_point = Box::new(Point { x: 33.3, y: 33.3 });
    println!("Struct in a Box: {:?}", heap_point); // Point { x: 33.3, y: 33.3 }

    let value_from_box = *heap_int;
    println!("Explicitly dereferenced value: {}", value_from_box); // 5

    // Access a field directly thanks to automatic dereferencing (Deref coercion)
    println!(
        "Accessing field via deref coercion: heap_point.x = {}",
        heap_point.x
    ); // 33.3

    // When `heap_int` and `heap_point` go out of scope here, the memory they manage
    // on the heap is automatically freed via the Drop trait.
}
