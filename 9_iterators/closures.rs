fn main() {
    let add_one = |x| x + 1;
    println!("5 + 1 = {}", add_one(5)); // 6

    let multiply = |a: i32, b: i32| -> i32 { a * b };
    println!("3 * 4 = {}", multiply(3, 4)); // 12

    let complex_closure = |x: i32| {
        println!("Calculating for input: {}", x);

        // x^2 + 2x + 1
        let result = x * x + 2 * x + 1;
        result
    };

    println!("Complex result for 3: {}", complex_closure(3)); // 9 + 6 + 1 = 16
}
