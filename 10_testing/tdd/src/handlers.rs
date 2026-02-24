// TDD for API handlers

#[derive(serde::Serialize)] // for serde_json::to_string
struct User {
    id: u32,
    username: String,
}

// A stub for the handler to allow the code to compile
fn get_user_handler(id: u32) -> Result<String, String> {
    match id {
        1 => {
            let user = User {
                id: 1,
                username: "Alice".to_string(),
            };

            serde_json::to_string(&user).map_err(|e| e.to_string())
        }
        _ => Err("User not found".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_success() {
        let expected_user = User {
            id: 1,
            username: "Alice".to_string(),
        };
        let expected_json = serde_json::to_string(&expected_user).unwrap();

        let result = get_user_handler(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_json);
    }

    #[test]
    fn test_get_user_not_found() {
        let result = get_user_handler(999); // some non-existing ID
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "User not found");
    }
}
