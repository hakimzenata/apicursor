use crate::router::create_app;
use dotenvy::dotenv;
mod router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match dotenv() {
        Ok(_) => println!("Loaded .env file"),
        Err(e) => println!("Failed to load .env file: {}", e),
    }

    let server = format!(
        "{}:{}",
        dotenvy::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        dotenvy::var("PORT").unwrap_or_else(|_| "3000".to_string())
    );
    println!("Listening on {}", server);

    if let Err(e) = create_app(server).await {
        println!("Failed to start server: {}", e);
        return Err(e);
    }

    println!("Server started successfully");
    Ok(())
}
