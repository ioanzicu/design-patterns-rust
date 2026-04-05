// fail when input is empty
fn create_greeting(name: &str) -> Result<String, String> {
    if name.trim().is_empty() {
        Err("Name cannot be empty".to_string())
    } else {
        Ok(format!("Hello, {}!", name))
    }
}

fn find_even_number(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_greeting_success() {
        let result = create_greeting("Rustacean");
        assert!(result.is_ok(), "Greeting should be Ok for valid name");
        assert_eq!(result.unwrap(), "Hello, Rustacean!");
    }

    #[test]
    fn test_create_greeting_failure_empty_name() {
        let result = create_greeting("");
        assert!(result.is_err(), "Greeting should be Err for empty name");
        // check if error message is equal
        assert_eq!(result.unwrap_err(), "Name cannot be empty");
    }

    #[test]
    fn test_create_greeting_failure_whitespace_name() {
        let result = create_greeting("   ");
        assert!(
            result.is_err(),
            "Greeting should be Err for whitespace-only name"
        );
    }

    #[test]
    fn test_find_even_number_some_found() {
        let numbers = [1, 3, 4, 5, 7];
        let result = find_even_number(&numbers);
        assert!(result.is_some(), "Should find an even number");
        assert_eq!(result.unwrap(), 4);
    }

    #[test]
    fn test_find_even_number_none_found() {
        let numbers = [1, 3, 5, 7, 9];
        let result = find_even_number(&numbers);
        assert!(
            result.is_none(),
            "Should return None if no even number is present"
        );
    }

    #[test]
    fn test_find_even_number_empty_slice() {
        let numbers: [i32; 0] = [];
        let result = find_even_number(&numbers);
        assert!(result.is_none(), "Should return None for an empty slice");
    }

    #[test]
    fn test_greeting_with_matches_macro() {
        let result_ok = create_greeting("Pat");
        assert!(
            matches!(result_ok, Ok(ref s) if s == "Hello, Pat!"),
            "Expected Ok(\"Hello, Pat!\"), got {:?}",
            result_ok
        );

        let result_err = create_greeting("  ");
        assert!(
            matches!(result_err, Err(ref s) if s == "Name cannot be empty"),
            "Expeted specific error, got {:?}",
            result_err
        )
    }
}
