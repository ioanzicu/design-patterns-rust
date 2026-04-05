struct MyValue {
    data: i32,
}

impl MyValue {
    fn display(&self) {
        println!("MyValue data: {}", self.data);
    }
}

fn main() {
    let b = Box::new(MyValue { data: 33 });
    let rc_val = std::rc::Rc::new(MyValue { data: 100 });

    // Calling display() on Box<MyValue>
    // 1. `b` is `Box<MyValue>`. `&b` is `&Box<MyValue>`.
    // 2. `Box<MyValue>` implements `Deref<Target = MyValue>`.
    // 3. So, `&Box<MyValue>` coerces to `&MyValue`.
    // 4. `display()` is called on `&MyValue`.

    b.display(); // Works due to deref coercion

    // Similarly for Rc<MyValue>
    rc_val.display(); // Works

    // Explicit dereference is also possible, but often not needed for method calls.
    (*b).display();
    (*rc_val).display();

    // With a String (which Box<String> and Rc<String> dereference to &str)
    let name_box = Box::new("Rustacean".to_string());

    // String -> &str, so we can call &str methods
    println!("Is '{}' empty? {}", name_box, name_box.is_empty()); // .is_empty() is a &str method

    // Deref coersion also works with function arguments
    fn print_str_slice(s: &str) {
        println!("Slice: {}", s);
    }

    print_str_slice(&name_box); // &Box<String> -> &String -> &str
}
