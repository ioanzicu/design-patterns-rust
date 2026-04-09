use serde::{Deserialize, Serialize};
use sqlx::FromRow; // derive macro - allow sqlx to map database rows to a struct

/// Todo item in the system
#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow, Clone)]
pub struct Todo {
    pub id: i32, // Postgres SERIAL maps to i32
    pub title: String,
    pub completed: bool,
}

/// NewTodo the payload for creating a new Todo
#[derive(Debug, Deserialize)]
pub struct NewTodo {
    pub title: String,
}

//. UpdateTodo the payload for updating a new Todo
#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
}
