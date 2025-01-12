use std::{fmt::format, sync::Arc};

use axum::{
    extract::Path,
    routing::{delete, get, post, put},
    Router,
};
use lazy_static::lazy_static;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct ApiResponse {
    timestamp: String,
    message: String,
}

#[derive(Serialize, Debug)]
struct StatusResponse {
    #[serde(flatten)]
    base: ApiResponse,
    status: String,
}

#[derive(Serialize, Debug)]
struct UserResponse {
    #[serde(flatten)]
    base: ApiResponse,
    user: User,
}
#[derive(Clone, Serialize, Debug)]
struct User {
    id: u64,
    name: String,
    email: String,
}

lazy_static::lazy_static! {
    static ref USERS_DATA: Arc<Vec<User>> = Arc::new(vec![
        User {
            id: 1,
            name: "Admin User 1".to_string(),
            email: "admin1@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Admin User 2".to_string(),
            email: "admin2@example.com".to_string(),
        },
    ]);
}

async fn api_handler() -> axum::Json<ApiResponse> {
    let response = ApiResponse {
        timestamp: chrono::Utc::now().to_rfc3339(),
        message: "👋 Hello, this is the API response!".to_string(),
    };
    println!("🔍 API response: {:?}", response);
    axum::Json(response)
}

async fn status_handler() -> axum::Json<StatusResponse> {
    let response_time = chrono::Utc::now().to_rfc3339();
    let response = StatusResponse {
        base: ApiResponse {
            timestamp: response_time,
            message: "✅ Status is OK!".to_string(),
        },
        status: "✨ Running smoothly".to_string(),
    };
    axum::Json(response)
}

pub async fn create_app(server: String) -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind(&server).await?;

    // Create router 🛣️
    let app = Router::new()
        .route("/api", get(api_handler))
        .route("/api/status", get(status_handler))
        .nest("/api/admin", adminrouter())
        .into_make_service();

    println!("🚀 Server starting on {}", server);
    axum::serve(listener, app).await?;
    Ok(())
}

fn adminrouter() -> Router {
    Router::new()
        .route("/", get(get_main_admin))
        .route("/all", get(get_admin_users))
        .route("/{id}", get(get_admin_user_by_id))
}

#[axum::debug_handler]
async fn get_main_admin() -> axum::Json<UserResponse> {
    let admin_user = USERS_DATA.first().unwrap();
    axum::Json(UserResponse {
        base: ApiResponse {
            timestamp: chrono::Utc::now().to_rfc3339(),
            message: format!("👤 Hello, this is the admin user: {}", admin_user.name),
        },
        user: admin_user.clone(),
    })
}

#[axum::debug_handler]
async fn get_admin_users() -> axum::Json<Vec<UserResponse>> {
    println!("👥 Fetching all admin users");
    let users_data = USERS_DATA.clone();
    axum::Json(
        users_data
            .iter()
            .map(|user| UserResponse {
                base: ApiResponse {
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    message: format!("👤 Admin user: {}", user.name),
                },
                user: user.clone(),
            })
            .collect(),
    )
}

#[axum::debug_handler]
async fn get_admin_user_by_id(Path(id): Path<u64>) -> axum::Json<UserResponse> {
    println!("🔍 Looking up user with id: {}", id);
    let users_data = USERS_DATA.clone();
    if let Some(user) = users_data.iter().find(|&user| user.id == id) {
        axum::Json(UserResponse {
            base: ApiResponse {
                timestamp: chrono::Utc::now().to_rfc3339(),
                message: format!("✨ Found user with id: {}", id),
            },
            user: user.clone(),
        })
    } else {
        // Handle user not found case
        axum::Json(UserResponse {
            base: ApiResponse {
                timestamp: chrono::Utc::now().to_rfc3339(),
                message: format!("❌ User with id: {} not found", id),
            },
            user: User {
                id: 0,
                name: "Not Found".to_string(),
                email: "notfound@example.com".to_string(),
            },
        })
    }
}
