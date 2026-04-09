use crate::AppState;
use crate::models::{NewTodo, Todo, UpdateTodo};
use axum::{
    Json,
    extract::{Json as AxJson, Path, State},
    http::StatusCode,
    response::IntoResponse,
};

// --- Create ---
pub async fn create_todo_db(
    State(state): State<AppState>,
    AxJson(payload): AxJson<NewTodo>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, Todo>("INSERT INTO todos (title) VALUES ($1) RETURNING *")
        .bind(&payload.title)
        .fetch_one(&state.db_pool)
        .await;

    match result {
        Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
        Err(e) => internal_db_error(e).into_response(),
    }
}

// --- Read All ---
pub async fn get_all_todos_db(State(state): State<AppState>) -> impl IntoResponse {
    let result = sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos ORDER BY id")
        .fetch_all(&state.db_pool)
        .await;

    match result {
        Ok(todos) => (StatusCode::OK, Json(todos)).into_response(),
        Err(e) => internal_db_error(e).into_response(),
    }
}

// --- Read Single ---
pub async fn get_todo(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    let result = sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db_pool)
        .await;

    match result {
        Ok(Some(todo)) => (StatusCode::OK, Json(todo)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => internal_db_error(e).into_response(),
    }
}

// --- Update (Partial) ---
pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    AxJson(payload): AxJson<UpdateTodo>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, Todo>(
        r#"
        UPDATE todos 
        SET
            title = COALESCE($1, title),
            completed = COALESCE($2, completed)
        WHERE id = $3
        RETURNING * 
        "#
    )
    .bind(&payload.title)
    .bind(payload.completed)
    .bind(id)
    .fetch_optional(&state.db_pool)
    .await;

    match result {
        Ok(Some(todo)) => (StatusCode::OK, Json(todo)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => internal_db_error(e).into_response(),
    }
}

// --- Delete ---
pub async fn delete_todo(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    // Fixed: specify the database type (sqlx::Postgres) for the query result
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&state.db_pool)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::NO_CONTENT.into_response(),
        Ok(_) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => internal_db_error(e).into_response(),
    }
}

// --- Get Server Status ---
pub async fn get_server_status(State(_state): State<AppState>) -> impl IntoResponse {
    (StatusCode::OK, "Server Status: OK. Database Pool Ready")
}

// --- Echo Handler ---
pub async fn echo_path(Path(message): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, format!("You sent: [{}]", message))
}

fn internal_db_error(e: sqlx::Error) -> StatusCode {
    eprintln!("Database error: {}", e);
    StatusCode::INTERNAL_SERVER_ERROR
}