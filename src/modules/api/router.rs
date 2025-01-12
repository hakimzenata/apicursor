use axum::{routing::get, Router};

use super::{auth::admin, status};
use crate::models::response::ApiResponse;
use crate::utils::logger::log_info_app;
use axum::http::StatusCode;

pub fn get_api_router() -> Router {
    Router::new()
        .route("/", get(api_handler))
        .merge(status::get_status_route())
        .merge(admin::adminrouter())
}

async fn api_handler() -> axum::Json<ApiResponse> {
    log_info_app("API handler called 🚀".to_string());
    let response = ApiResponse {
        timestamp: chrono::Utc::now().to_rfc3339(),
        message: "Hello, if you see this, the API is working! 🚀".to_string(),
        return_code: StatusCode::OK.to_string(),
    };
    axum::Json(response)
}
