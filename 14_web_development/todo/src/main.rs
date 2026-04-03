use axum::http::Method;
use axum::{Router, http::StatusCode, routing::get};
use dotenvy::dotenv;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::signal; // for gracefull shutdown
use tower_http::cors::{Any, CorsLayer};

// ---  Modules. ---
pub mod handlers;
pub mod models;

// --- Import from our modules  ---
use handlers::{
    create_todo_db, delete_todo, echo_path, get_all_todos_db, get_server_status, get_todo,
    update_todo,
};

// ---  Application State  ---
// Clone allows the state to be shared with all threads
#[derive(Clone)]
pub struct AppState {
    db_pool: PgPool,
}

async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Service is healthy")
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file (DATABASE_URL)
    dotenv().ok(); // .ok() ignores errors if .env if not found

    // Get the database URL form the environment
    let database_url = env::var("DATABASE_URL").expect("DATBASE_URL must be set in .env file");

    // ---  Create the Database Connection Pool  ---
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

    // Define CORS Policy
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([axum::http::header::CONTENT_TYPE])
        .allow_origin(Any);

    // Initialize AppState with shared state
    let app_state = AppState { db_pool: pool };

    // Define the Router with CRUD handlers and add the state
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/todos", get(get_all_todos_db).post(create_todo_db))
        .route(
            "/todos/{id}",
            get(get_todo).patch(update_todo).delete(delete_todo),
        )
        .route("/status", get(get_server_status))
        .route("/echo/{message}", get(echo_path))
        .with_state(app_state) // makes the `app_state` available to all routes
        .layer(cors);

    // Bind and Serve
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr)
        .await
        .expect(&format!("Failed to bind to address http://{}", addr));

    println!("🚀 Server listening on http://{}", addr);

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Failed to start server");
}

// ---  Graceful Shutdown Handler  ---
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

    // for non unix like Windows
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    };
    println!("Graceful shutdown initiated.");
}
