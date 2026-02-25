use axum::{
    extract::Path,
    routing::get,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Activity {
    date_time: String,
    process_name: String,
    window_title: String,
}

#[tokio::main]
async fn main() {
    // Define routing and application state
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        // Endpoint to get activities from CSV
        .route("/activities/:date", get(get_activities));

    // Bind to the address and port
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    // Start the server
    axum::serve(listener, app).await.unwrap();
}

// Handler to read CSV file and return as JSON using method chaining
async fn get_activities(Path(date): Path<String>) -> Json<Vec<Activity>> {
    let file_path = format!("data/{}.csv", date);
    let file = File::open(file_path).expect("Failed to open file");
    
    csv::Reader::from_reader(file)
        .deserialize()
        .collect::<Result<Vec<Activity>, _>>()
        // Map the inner Vec<Activity> to Json<Vec<Activity>>
        .map(Json)
        // Extract the final Json value
        .expect("Failed to parse CSV records")
}
