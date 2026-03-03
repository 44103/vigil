use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use vigil::config::{load_config, resolve_data_dir};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Activity {
    date_time: String,
    process_name: String,
    window_title: String,
}

struct AppState {
    data_dir: PathBuf,
}

#[tokio::main]
async fn main() {
    let config = load_config();
    let data_dir = resolve_data_dir(&config);
    let port = config.server_port.unwrap_or(3000);

    let state = Arc::new(AppState { data_dir });

    // Start monitor in the background
    tokio::spawn(async move {
        vigil::monitor::run().await;
    });

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/activities/:date", get(get_activities))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind port {}", port));

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn get_activities(
    Path(date): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let file_path = state.data_dir.join(format!("{date}.csv"));

    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => return (StatusCode::NOT_FOUND, "File not found").into_response(),
    };

    let mut rdr = csv::Reader::from_reader(file);
    let activities = rdr
        .deserialize()
        .collect::<Result<Vec<Activity>, _>>()
        .map_err(|e| {
            eprintln!("CSV parse error: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse CSV records",
            )
                .into_response()
        });

    match activities {
        Ok(data) => Json(data).into_response(),
        Err(err_resp) => err_resp,
    }
}
