use axum::http::StatusCode;
use serde::Serialize;
#[derive(Serialize, Debug)]
pub struct ApiResponse {
    pub timestamp: String,
    pub message: String,
    pub return_code: String,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    #[serde(flatten)]
    pub base: ApiResponse,
    pub user: super::user::User,
}

impl ApiResponse {
    pub fn ok(message: &str) -> axum::Json<Self> {
        axum::Json(Self {
            timestamp: chrono::Utc::now().to_rfc3339(),
            message: message.to_string(),
            return_code: StatusCode::OK.to_string(),
        })
    }
}
