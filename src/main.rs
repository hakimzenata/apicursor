use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;
mod models;
mod modules;
mod router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let server = format!(
        "{}:{}",
        dotenvy::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        dotenvy::var("PORT").unwrap_or_else(|_| "3000".to_string())
    );
    let listener = TcpListener::bind(server).await?;
    let rootrouter = Router::new()
        .merge(router::create_app())
        .route("/", get(method_router))
        .await?;

    axum::serve::bind_rustls_with_tokio(listener, rootrouter).await?;
    Ok(())
}
async fn method_router() -> &'static str {
    "👋 Hello, this is the API response!"
}

async fn api_handler() -> &'static str {
    "👋 Hello, this is the API response!"
}
