use axum::{
    extract::Path,
    routing::get,
    Json,
    Router,
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::fs::File;
mod monitor;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Activity {
    date_time: String,
    process_name: String,
    window_title: String,
}

#[tokio::main]
async fn main() {
    tokio::spawn(monitor::run());

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/activities/:date", get(get_activities));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind port 3000");
    
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn get_activities(Path(date): Path<String>) -> impl IntoResponse {
    let file_path = format!("data/{date}.csv");
    
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => return (StatusCode::NOT_FOUND, "File not found").into_response(),
    };

    let result = csv::Reader::from_reader(file)
        .deserialize::<Activity>()
        .collect::<Result<Vec<Activity>, _>>();

    match result {
        Ok(data) => Json(data).into_response(),
        Err(e) => {
            eprintln!("CSV parse error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse CSV records").into_response()
        }
    }
}
