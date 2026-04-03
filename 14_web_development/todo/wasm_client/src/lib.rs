use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// ---  Redefine the models for the fronted  ---
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Serialize)]
pub struct NewTodo {
    pub title: String,
}

// A simple macro to log to the browser console
macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    }
}

// Backend API URL
const API_URL: &str = "http://localhost:8080/todos";

/// Fetches the current list of todos from the backend API.
/// Returns the JSON as a `JsValue` (which will be a string).
#[wasm_bindgen]
pub async fn fetch_todos() -> Result<JsValue, JsValue> {
    log!("Wasm: Fetching from {}", API_URL);

    let client = reqwest::Client::new();
    let resp = client
        .get(API_URL)
        .send()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let json_text = resp
        .text() // get the raw JSON string
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Convert the Rust String into a JavaScript String (JsValue)
    Ok(JsValue::from_str(&json_text))
}

/// Adds a new todo item bby POSTing to the backend API.
#[wasm_bindgen]
pub async fn add_todo(title: String) -> Result<JsValue, JsValue> {
    log!("Wasm: Adding todo: {}", &title);
    let new_todo = NewTodo { title };

    let client = reqwest::Client::new();
    let resp = client
        .post(API_URL)
        .json(&new_todo) // Send NewTodo struct as a JSON body
        .send()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let json_text = resp
        .text()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Convert the Rust String into a JavaScript String (JsValue)
    Ok(JsValue::from_str(&json_text))
}
