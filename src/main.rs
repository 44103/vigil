use axum::{
    extract::Path,
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // Define routing and application state
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(|| async { "OK" }))
        // Add endpoint with path parameter
        // The parameter named ":name" will be extracted
        .route("/hello/:name", get(hello_handler));

    // Bind to the address and port
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    // Start the server
    axum::serve(listener, app).await.unwrap();
}

// The variable name "name" in Path(name) maps to ":name" in the route definition
async fn hello_handler(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}
