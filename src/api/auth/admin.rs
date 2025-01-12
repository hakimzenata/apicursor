use axum::Router;

pub fn adminrouter() -> Router {
    Router::new()
        .route("/", axum::routing::get(handlers::get_admin_users))
        .route("/:id", axum::routing::get(handlers::get_admin_user_by_id))
}
