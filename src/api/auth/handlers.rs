use crate::models::{response::UserResponse, user::User};
use axum::{extract::Path, Json};

pub async fn get_admin_users() -> Json<Vec<UserResponse>> {
    Json(vec![
        UserResponse {
            base: ApiResponse {
                timestamp: chrono::Utc::now().to_rfc3339(),
                message: "Admin user fetched".to_string(),
            },
            user: User {
                id: 1,
                name: "Admin User 1".to_string(),
                email: "admin1@example.com".to_string(),
            },
        },
        UserResponse {
            base: ApiResponse {
                timestamp: chrono::Utc::now().to_rfc3339(),
                message: "Admin user fetched".to_string(),
            },
            user: User {
                id: 2,
                name: "Admin User 2".to_string(),
                email: "admin2@example.com".to_string(),
            },
        },
    ])
}

pub async fn get_admin_user_by_id(Path(id): Path<u64>) -> Json<UserResponse> {
    Json(UserResponse {
        base: ApiResponse {
            timestamp: chrono::Utc::now().to_rfc3339(),
            message: format!("Admin user {} fetched", id),
        },
        user: User {
            id,
            name: format!("Admin User {}", id),
            email: format!("admin{}@example.com", id),
        },
    })
}
