use crate::AppState;
use crate::models::{NewTodo, Todo, UpdateTodo};
use axum::{
    Json,
    extract::{Json as AxJson, Path, State},
    http::StatusCode,
    response::IntoResponse,
};

// ---  Create  ---
/// Handler for POST /todos
/// Creates a new todo item.
pub async fn create_todo_db(
    State(state): State<AppState>,
    AxJson(payload): AxJson<NewTodo>, // Json extractor
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Todo,
        "INSERT INTO todos (title) VALUES ($1) RETURNING *",
        payload.title
    )
    .fetch_one(&state.db_pool)
    .await;

    match result {
        Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
        Err(e) => internal_db_error(e).into_response(),
    }
}

// ---  Read All  ---
/// Handler for GET /todos
/// Returns a list of all todo items.
pub async fn get_all_todos_db(State(state): State<AppState>) -> impl IntoResponse {
    // query_as! macros check SQL validity at compile time
    let result = sqlx::query_as!(Todo, "SELECT id, title, completed FROM todos ORDER BY id")
        .fetch_all(&state.db_pool) // Vec<Todo>
        .await;

    match result {
        Ok(todos) => (StatusCode::OK, Json(todos)).into_response(),
        Err(e) => internal_db_error(e).into_response(),
    }
}

// ---  Read Single  ---
/// Handler for GET /todos/{id}
/// REturns a single todo by its ID.
pub async fn get_todo(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Todo,
        "SELECT id, title, completed FROM todos WHERE id = $1",
        id
    )
    .fetch_optional(&state.db_pool)
    .await;

    match result {
        Ok(Some(todo)) => (StatusCode::OK, Json(todo)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => internal_db_error(e).into_response(),
    }
}

// ---  Update (Partial)  ---
/// Handler for PATCH /todos/{id}
/// Updates a todo item (partial updates).
pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    AxJson(payload): AxJson<UpdateTodo>,
) -> impl IntoResponse {
    // COALESCE handles partial updates: if the payload field is NULL,
    // it keeps the existing database value.
    let result = sqlx::query_as!(
        Todo,
        r#"
        UPDATE todos 
        SET
            title = COALESCE($1, title),
            completed = COALESCE($2, completed)
        WHERE id = $3
        RETURNING * 
        "#,
        payload.title,
        payload.completed,
        id
    )
    .fetch_optional(&state.db_pool)
    .await;

    match result {
        Ok(Some(todo)) => (StatusCode::OK, Json(todo)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => internal_db_error(e).into_response(),
    }
}

// ---  Delete  ---
/// Handler for DELETE /todos/{id}
/// Deletes a todo item by its ID.
pub async fn delete_todo(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    let result = sqlx::query!("DELETE FROM todos WHERE id = $1", id)
        .execute(&state.db_pool)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::NO_CONTENT.into_response(),
        Ok(_) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => internal_db_error(e).into_response(),
    }
}

// ---  Assignment - Get Server Status  ---
pub async fn get_server_status(State(state): State<AppState>) -> impl IntoResponse {
    (StatusCode::OK, "Server Status: OK. Database Pool Ready")
}

// ---  Assignment - Echo Handler  ---
pub async fn echo_path(Path(message): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, format!("You sent: [{}]", message))
}

fn internal_db_error(e: sqlx::Error) -> StatusCode {
    eprintln!("Database error: {}", e);
    StatusCode::INTERNAL_SERVER_ERROR
}
