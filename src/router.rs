use crate::modules::api::router::get_api_router;
use crate::utils::logger::log_info_app;
use axum::http::StatusCode;
use axum::Router;

pub async fn create_app() -> Result<Router, Box<dyn std::error::Error>> {
    log_info_app("Creating app 🏗️".to_string());

    let app = Router::new()
        .merge(get_api_router())
        .fallback(not_found_handler);
    Ok(app)
}

async fn not_found_handler() -> axum::Json<ApiResponse> {
    let response = ApiResponse {
        timestamp: chrono::Utc::now().to_rfc3339(),
        message: "404 - Not Found".to_string(),
        return_code: StatusCode::NOT_FOUND.to_string(),
    };
    axum::Json(response)
}
