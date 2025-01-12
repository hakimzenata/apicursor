use axum::Router;
use dotenvy::dotenv;
use tokio::net::TcpListener;
#[allow(dead_code)]
use workspace::{modules, utils};

mod router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    utils::logger::init_logger();
    utils::logger::log_info_app("Environment variables loaded 🌍".to_string());

    let server = format!(
        "{}:{}",
        utils::config::get_env_var("SERVER_HOST", "127.0.0.1"),
        utils::config::get_env_var("SERVER_PORT", "3000")
    );
    utils::logger::log_info_app(format!("Server address configured: {} 📡", server));

    let listener = TcpListener::bind(&server).await?;
    utils::logger::log_info_app("TCP listener bound successfully 🔗".to_string());

    let app = router::create_app().await?;
    utils::logger::log_info_app("Application router created successfully 🛠️".to_string());

    let rootrouter = Router::new().nest("/api", app);
    utils::logger::log_info_app("Root router configured with API routes 🌐".to_string());

    utils::logger::log_info_app(format!("🚀 Server starting on {}", server));
    axum::serve(listener, rootrouter).await?;
    Ok(())
}
