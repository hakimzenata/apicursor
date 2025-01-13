#![allow(unused)]
use crate::utils::logger::{log_error_app, log_info_app};
use bb8::{Pool, PooledConnection};
use bb8_redis::RedisConnectionManager;
use redis::{Client, RedisError};

pub struct RedisClient {
    client: Client,
    pool: Pool<RedisConnectionManager>,
}

impl RedisClient {
    pub async fn open(full_url: &str) -> Result<Self, RedisError> {
        let client = Client::open(full_url)?;
        log_info_app("Connecting to Redis 🔥".to_string());

        let manager = RedisConnectionManager::new(full_url)?;
        let pool = Pool::builder().build(manager).await?;

        match pool.get().await {
            Ok(_) => {
                log_info_app("Redis client connected successfully with pool 🔥".to_string());
            }
            Err(e) => {
                log_error_app(format!("Error connecting to Redis: {:?}", e));
            }
        }

        Ok(Self { client, pool })
    }

    pub async fn get_connection(
        &self,
    ) -> Result<PooledConnection<'_, RedisConnectionManager>, RedisError> {
        self.pool.get().await.map_err(|e| {
            RedisError::from((
                redis::ErrorKind::IoError,
                "Failed to get connection from pool",
                e.to_string(),
            ))
        })
    }
}
