use axum::Router;
use dotenvy::dotenv;
use tokio::net::TcpListener;
use workspace::utils::{
    config::get_env_var,
    db::RedisClient,
    logger::{init_logger, log_error_app, log_info_app},
};
mod router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = dotenv() {
        log_error_app(format!("Error loading environment variables: {}", e));
    } else {
        log_info_app("Environment variables loaded 🌍".to_string());
    }
    init_logger();
    log_info_app("Environment variables loaded 🌍".to_string());

    let server = format!(
        "{}:{}",
        get_env_var("SERVER_HOST", "127.0.0.1"),
        get_env_var("SERVER_PORT", "3000")
    );
    log_info_app(format!("Server address configured: {} 📡", server));

    let listener = TcpListener::bind(&server).await?;
    log_info_app("TCP listener bound successfully 🔗".to_string());

    let app = router::create_app().await?;
    log_info_app("Application router created successfully 🛠️".to_string());

    let rootrouter = Router::new().nest("/api", app);
    log_info_app("Root router configured with API routes 🌐".to_string());

    // Connect to Redis
    log_info_app("Connecting to Redis 🔥".to_string());
    let redis_full_url = get_env_var("REDIS_URL", "redis://default:@localhost:6379/0");
    log_info_app(format!("Redis full URL: {}", redis_full_url));

    let redis_client = match RedisClient::open(&redis_full_url).await {
        Ok(client) => {
            log_info_app("Redis client connected successfully 🔥".to_string());
            Some(client)
        }
        Err(e) => {
            log_error_app(format!("Error connecting to Redis: {:?}", e));
            None
        }
    };

    log_info_app(format!("🚀 HTTP Server starting on {}", server));
    axum::serve(listener, rootrouter).await?;
    log_info_app("HTTP server started successfully 🚀".to_string());

    Ok(())
}
