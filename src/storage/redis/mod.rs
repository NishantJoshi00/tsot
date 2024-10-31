// redis_storage/mod.rs
#[cfg(feature = "async")]
mod async_impl;

use fred::{clients::RedisClient, interfaces::ClientLike, types::RedisConfig};
use std::sync::Arc;

/// Configuration for the Redis storage module
#[derive(Clone)]
pub struct RedisStorageConfig {
    /// Redis connection host
    pub host: String,
    /// Redis connection port
    pub port: u16,
    /// Optional username for authentication
    pub username: Option<String>,
    /// Optional password for authentication
    pub password: Option<String>,
    /// Default expiration time for entries
    pub default_expiry: Option<u64>,
}

/// Redis storage module implementation
#[derive(Clone)]
pub struct RedisStorageModule {
    /// Underlying Redis client
    client: Arc<RedisClient>,
}

impl RedisStorageModule {
    /// Create a new Redis storage module
    ///
    /// # Arguments
    /// * `config` - Configuration for Redis connection and default settings
    ///
    /// # Returns
    /// A new RedisStorageModule instance
    pub async fn new(config: RedisStorageConfig) -> Result<Self, fred::error::RedisError> {
        // Create Redis configuration
        let redis_config = RedisConfig {
            username: config.username.clone(),
            password: config.password.clone(),
            server: fred::types::ServerConfig::Centralized {
                server: fred::types::Server {
                    host: config.host.clone().into(),
                    port: config.port,
                },
            },
            ..Default::default()
        };

        // Create the client
        let client = Arc::new(RedisClient::new(redis_config, None, None, None));

        // Connect to Redis
        client.connect();
        client.wait_for_connect().await?;

        Ok(Self { client })
    }
}
