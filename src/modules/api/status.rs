use crate::models::response::ApiResponse;
use crate::utils::logger::log_info_app;
use axum::http::StatusCode;
use axum::{routing::get, Router};

pub async fn status_handler() -> axum::Json<ApiResponse> {
    log_info_app("Status handler called ✅".to_string());
    let response = ApiResponse {
        timestamp: chrono::Utc::now().to_rfc3339(),
        message: "✅ Status: OK".to_string(),
        return_code: StatusCode::OK.to_string(),
    };
    axum::Json(response)
}

pub fn get_status_route() -> Router {
    Router::new().route("/status", get(status_handler))
}
