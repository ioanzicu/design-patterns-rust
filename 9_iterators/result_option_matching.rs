fn process_input(input: &str) {
    let result: Result<i32, _> = input.parse();
    match result {
        Ok(number) => {
            // Destructured `Ok`, binding the i32 to `number`.
            println!("Successfully parsed number: {}", number);
        }
        Err(error) => {
            // Destructured `Err`, binding the ParseIntError to `error`.
            println!("Failed to parse. Error: {}", error);
        }
    }
}

fn main() {
    let maybe_name: Option<String> = Some(String::from("Alesia"));
    match maybe_name {
        // Found a name
        Some(name) => {
            // Destructured `Some`, binding the String to `name`.
            println!("Found a name: {}.", name);
        }
        None => {
            println!("No name was provided.");
        }
    }

    process_input("123"); // Successfully parsed ...
    process_input("abc"); // Failed to parse
}
