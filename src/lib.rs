pub mod models;
pub mod modules;
pub mod utils;

pub mod api_prelude {
    pub use crate::models::response::ApiResponse;
    pub use
    pub use axum::{
        extract::Path,
        http::StatusCode,
        routing::{get, post},
        Json, Router,
    };
}
