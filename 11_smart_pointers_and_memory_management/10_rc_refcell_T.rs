use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // 1.   Create a shared, mutable list.
    //      RefCell allows mutation. Rc allows multiple owners.
    let shared_data = Rc::new(RefCell::new(Vec::new()));

    // 2.   Create multiple owners (clones of the pointer, not the data).
    let owner1 = Rc::clone(&shared_data);
    let owner2 = Rc::clone(&shared_data);

    // 3.   Mutate the data via the first owner.
    //      We use .borrow_mut() to get write access.
    owner1.borrow_mut().push("Data from Owner 1".to_string());

    // 4.   Mutate the SAME data via the second owner.
    owner2.borrow_mut().push("Data from Owner 2".to_string());

    // 5.   Read the data from the original owner.
    //      We use .borrow() to get read access.
    println!("Final data: {:?}", shared_data.borrow());
    // ["Data from Owner 1", "Data from Owner 2"]
}
