use axum::Router;

use crate::modules::api::auth::handlers;
pub fn adminrouter() -> Router {
    Router::new()
        .route("/admin", axum::routing::get(handlers::get_admin_users))
        .route(
            "/admin/{id}",
            axum::routing::get(handlers::get_admin_user_by_id),
        )
}
