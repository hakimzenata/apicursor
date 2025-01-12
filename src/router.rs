use crate::api::{auth, status};
use axum::{routing::get, Router};

pub async fn create_app() -> Result<Router, Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/api", get(api_handler))
        .merge(status::get_status_route())
        .nest("/api/admin", auth::adminrouter());

    Ok(app)
}

async fn api_handler() -> &'static str {
    "👋 Hello, this is the API response!"
}
