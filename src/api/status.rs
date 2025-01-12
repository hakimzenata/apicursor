use crate::models::response::ApiResponse;
use axum::{routing::get, Router};

async fn status_handler() -> axum::Json<ApiResponse> {
    let response = ApiResponse {
        timestamp: chrono::Utc::now().to_rfc3339(),
        message: "✅ Status: OK".to_string(),
    };
    axum::Json(response)
}

pub fn get_status_route() -> Router {
    Router::new().route("/status", get(status_handler))
}
