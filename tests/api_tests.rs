use axum::http::StatusCode;
use axum::Json;
use serde_json::json;
use tokio::test;

use workspace::api_prelude::*;
use workspace::modules::api::auth::handlers;
use workspace::modules::api::status;

#[tokio::test]
async fn test_get_admin_users() {
    let response = handlers::get_admin_users().await;
    assert_eq!(response.0.len(), 2);
    assert_eq!(response.0[0].user.name, "Admin User 1");
    assert_eq!(response.0[1].user.name, "Admin User 2");
}

#[tokio::test]
async fn test_get_admin_user_by_id() {
    let response = handlers::get_admin_user_by_id(axum::extract::Path(1)).await;
    assert_eq!(response.0.user.name, "Admin User 1");
    assert_eq!(response.0.user.email, "admin1@example.com");
}

#[tokio::test]
async fn test_status_handler() {
    let response = status::status_handler().await;
    assert_eq!(response.0.message, "✅ Status: OK");
    assert_eq!(response.0.return_code, StatusCode::OK.to_string());
}
