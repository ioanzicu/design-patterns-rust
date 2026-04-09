use axum::http::Method;
use axum::{Router, http::StatusCode, routing::get};
use dotenvy::dotenv;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

pub mod handlers;
pub mod models;

use handlers::{
    create_todo_db, delete_todo, echo_path, get_all_todos_db, get_server_status, get_todo,
    update_todo,
};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Service is healthy")
}

#[tokio::main]
async fn main() {
    println!("---  APP STARTING  ---");

    dotenv().ok(); 

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let static_dir = env::var("STATIC_DIR").unwrap_or_else(|_| "wasm_client/www".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database connection pool");

    println!("🚀 Database connection pool initialized.");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("🚀 Database migrations ran successfully.");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([axum::http::header::CONTENT_TYPE])
        .allow_origin(Any);

    let app_state = AppState { db_pool: pool };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/todos", get(get_all_todos_db).post(create_todo_db))
        .route(
            "/todos/{id}",
            get(get_todo).patch(update_todo).delete(delete_todo),
        )
        .route("/status", get(get_server_status))
        .route("/echo/{message}", get(echo_path))
        .fallback_service(ServeDir::new(static_dir))
        .with_state(app_state)
        .layer(cors);

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let listener = TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to address {}", addr));

    println!("🚀 Server listening on http://{}", addr);

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Failed to start server");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl + C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    };
    println!("Graceful shutdown initiated.");
}