/// This function takes a name (a string slice) and an age (an unsigned 32-bit integer)
/// and returns a nicely formatted greeting.
///
/// # Examples
///
/// ```
/// // This code block will be run as a test!
/// use test_doc::format_greeting; // Assuming this is how users would import
///
/// let name = "Alice";
/// let age = 30;
/// let greeting = format_greeting(name, age);
/// assert_eq!(greeting, "Hello, Alice! You are 30 years old.");
/// ```
///
///
/// You can have multiple examples:
/// ```
/// use test_doc::format_greeting;
///
/// let greeting_bob = format_greeting("Barry", 32);
/// assert!(greeting_bob.contains("Barry"));
/// assert!(greeting_bob.contains("32"));
/// ```
pub fn format_greeting(name: &str, age: u32) -> String {
    format!("Hello, {}! You are {} years old.", name, age)
}

// To make the `use test_doc::format_greeting;` line in the doc test work easily
// when `cargo test --doc` is run from the crate root `format_greeting` needs to be
// part of the public API accessible via `test_doc::format_greeting`.
