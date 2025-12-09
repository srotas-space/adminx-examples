// src/services/redis_service.rs
use redis::{AsyncCommands, Client, RedisError, ErrorKind};
use std::env;
use tokio::sync::OnceCell;
use log::{info, error};
use std::collections::VecDeque;
use serde::{Serialize, Deserialize};
use serde_json;

pub const REDIS_60_EXPIRY_SECONDS: usize = 60;
pub const REDIS_300_EXPIRY_SECONDS: usize = 300;
pub const REDIS_600_EXPIRY_SECONDS: usize = 600;

static REDIS_CONN: OnceCell<Client> = OnceCell::const_new();

/// ✅ Initialize Redis client once (works with redis:// or rediss://)
pub async fn init_redis() {
    let redis_url = env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

    match Client::open(redis_url.clone()) {
        Ok(client) => {
            if REDIS_CONN.set(client).is_ok() {
                info!("✅ Redis client initialized: {}", redis_url);
            } else {
                error!("⚠️ Redis client was already initialized.");
            }
        }
        Err(err) => {
            error!("❌ Failed to create Redis client: {}", err);
        }
    }
}

pub async fn get_redis_connection() -> Result<redis::aio::Connection, RedisError> {
    match REDIS_CONN.get() {
        Some(client) => {
            info!("🔌 Attempting async connection to Redis...");
            match client.get_async_connection().await {
                Ok(conn) => {
                    info!("✅ Redis async connection established");
                    Ok(conn)
                }
                Err(e) => {
                    error!("❌ Redis async connection failed: {:?}", e);
                    Err(e)
                }
            }
        }
        None => {
            error!("❌ Redis client not initialized");
            Err(RedisError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Redis client not initialized",
            )))
        }
    }
}



/*------------------------------------------------------------*/
/// START  Set key with expiry (in seconds)
/*------------------------------------------------------------*/
pub async fn redis_set_key_with_expiry(
    key: String,
    value: String,
    expiry_seconds: usize,
) -> Result<(), RedisError> {
    let mut conn = get_redis_connection().await?;
    let _: () = conn.set_ex(&key, &value, expiry_seconds as u64).await?;
    info!("🔹 Redis SET: {} -> {} ({}s)", &key, &value, expiry_seconds);
    Ok(())
}

/// ✅ Get a key
pub async fn redis_get_key(key: String) -> Result<Option<String>, RedisError> {
    let mut conn = get_redis_connection().await?;
    conn.get(&key).await
}

pub async fn redis_delete_key(key: String) -> Result<(), RedisError> {
    let mut conn = get_redis_connection().await?;
    conn.del(&key).await
}
/*------------------------------------------------------------*/
/// END Set key with expiry (in seconds)
/*------------------------------------------------------------*/



/*------------------------------------------------------------
/// START  Set a list of string values with expiry
------------------------------------------------------------*/
pub async fn redis_set_list_with_expiry(
    key: &str,
    values: Vec<String>,
    expiry_seconds: usize,
) -> Result<(), RedisError> {
    let mut conn = get_redis_connection().await?;
    let _: () = conn.del(key).await?; // Clear existing
    let _: () = conn.rpush(key, values).await?;

    let expiry_i64: i64 = expiry_seconds
    .try_into()
    .map_err(|_| RedisError::from((ErrorKind::TypeError, "TTL value too large for Redis")))?;

    let _: () = conn.expire(key, expiry_i64).await?;
    Ok(())
}

/// Get list of string values from Redis
pub async fn redis_get_list(key: &str) -> Result<Vec<String>, RedisError> {
    let mut conn = get_redis_connection().await?;
    let values: Vec<String> = conn.lrange(key, 0, -1).await?;
    Ok(values)
}
/*------------------------------------------------------------
/// START  Set a list of string values with expiry
------------------------------------------------------------*/


/*-------------------------------------------------------------
START  Set queue (No Expiry)
--------------------------------------------------------------*/
pub async fn redis_set_queue<T: Serialize>(
    key: String,
    queue: VecDeque<T>,
) -> Result<(), RedisError> {
    let mut conn = get_redis_connection().await?;

    let serialized = serde_json::to_string(&Vec::from(queue))
        .map_err(|e| RedisError::from((ErrorKind::TypeError, "Serialization error", e.to_string())))?;

    let _: () = conn.set(&key, serialized).await?;
    info!("🔹 Redis QUEUE SET (no expiry): {}", &key);
    Ok(())
}

/*-------------------------------------------------------------
END  Set queue (No Expiry)
--------------------------------------------------------------*/



/*-------------------------------------------------------------
START  Get queue
--------------------------------------------------------------*/
pub async fn redis_get_queue<T: for<'de> Deserialize<'de>>(
    key: String,
) -> Result<Option<VecDeque<T>>, RedisError> {
    let mut conn = get_redis_connection().await?;
    let data: Option<String> = conn.get(&key).await?;

    if let Some(json_str) = data {
        let vec: Vec<T> = serde_json::from_str(&json_str)
            .map_err(|e| RedisError::from((ErrorKind::TypeError, "Deserialization error", e.to_string())))?;
        Ok(Some(VecDeque::from(vec)))
    } else {
        Ok(None)
    }
}
/*-------------------------------------------------------------
START  Get queue
--------------------------------------------------------------*/
