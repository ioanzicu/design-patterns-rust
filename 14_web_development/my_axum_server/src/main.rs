use axum::{routing::get, Router};

// asyn function that returns a type that implements `IntoResponse`
// that can be converted to an HTTP response.
async fn hello_world_handler() -> &'static str {
    "Hello, Web!"
}

async fn root_handler() -> &'static str {
    "This is the root."
}

async fn about_handler() -> &'static str {
    "This is the about page."
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/about", get(about_handler))
        .route("/hello", get(hello_world_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind to address 127.0.0.1:8080");
    
    println!("Server listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.expect("Failed to start server");
}
