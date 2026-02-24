// TDD for library functions
//
// stub `fn is_palindrome(_s: &str) -> bool { todo!() }`

// 1. Red - empty function to compile the code
// run the tests and fail them
// pub fn is_palindrome(_s: &str) -> bool {
//     false
// }

// 2. Green - minimal code to pass the tests
// pub fn is_palindrome(s: &str) -> bool {
//     if s.is_empty() {
//         return true;
//     }
//     let forward_chars: Vec<char> = s.chars().collect();
//     let reversed_chars: Vec<char> = s.chars().rev().collect();
//     forward_chars == reversed_chars
// }

// 3. Refactor - improve the code
// pub fn is_palindrome(s: &str) -> bool {
//     let mut fwd_iter = s.chars();
//     let mut rev_iter = s.chars().rev();

//     while let (Some(f_char), Some(r_char)) = (fwd_iter.next(), rev_iter.next()) {
//         // compare only up to the middle of the string
//         // if fwd_iter > next_back() -> false

//         if f_char != r_char {
//             return false;
//         }

//         // let's rely on the loop eventually consuming one iterator
//         // faster if length is odd, leading to one `next()` being None.
//         // This simple loop only works if we consume from
//         // both ends until they meet or cross.
//         // A more correct iterative comparison for non-allocating.
//     }

//     true
// }

// Shorter refactored version
pub fn is_palindrome(s: &str) -> bool {
    // 5. To lowercase

    let s_cleaned: String = s.to_lowercase().chars().collect();
    s_cleaned.chars().eq(s_cleaned.chars().rev())
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;

    #[test]
    fn test_empty_string_is_palindrome() {
        assert!(is_palindrome(""));
    }

    #[test]
    fn test_single_char_is_palindrome() {
        assert!(is_palindrome("a"));
    }

    #[test]
    fn test_simple_palindrome() {
        assert!(is_palindrome("madam"));
    }

    #[test]
    fn test_non_palindrome() {
        assert!(!is_palindrome("hello"));
    }

    // 4. Next cycle - RED for case-insensitivity
    // will fail, since no lowercase conversion yet.
    #[test]
    fn test_case_insensitive_palindrome() {
        assert!(is_palindrome("Madam"));
    }
}
