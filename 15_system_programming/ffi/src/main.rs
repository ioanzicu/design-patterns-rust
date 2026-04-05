unsafe extern "C" {
    fn multiply_by_two(x: i32) -> i32;
}

fn main() {
    // Calling an external function is an unsafe operation
    // because the Rust compiler cannot guarantee its safety.
    unsafe {
        let number = 21;
        let result = multiply_by_two(number);
        println!("{} * 2 from C (via build.rs) = {}", number, result); // 42
    }
}
