use actix_web::web;
use mongodb::{
    bson::{doc, oid::ObjectId, to_bson, DateTime as BsonDateTime},
    Collection, Database,
};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use crate::services::redis_service::{get_redis_connection};
use strum_macros::EnumIter;

/// **Enum for Config Status**
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum ConfigStatus {
    Active,
    Inactive,
}

impl ToString for ConfigStatus {
    fn to_string(&self) -> String {
        match self {
            ConfigStatus::Active => "active".to_string(),
            ConfigStatus::Inactive => "inactive".to_string(),
        }
    }
}

/// **Enum for Config Data Type**
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum ConfigDataType {
    Json,
    String,
}

impl ToString for ConfigDataType {
    fn to_string(&self) -> String {
        match self {
            ConfigDataType::Json => "json".to_string(),
            ConfigDataType::String => "string".to_string(),
        }
    }
}

/// **Config Model**
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub key: String,
    pub data: Option<Value>, // Can be string or JSON
    pub data_type: ConfigDataType, // ✅ New

    pub status: ConfigStatus,

    pub created_at: BsonDateTime,
    pub updated_at: BsonDateTime,
}

impl Config {
    /// Create a new Config
    pub fn new(
        key: String,
        data: Option<Value>,
        data_type: ConfigDataType,
        status: ConfigStatus,
    ) -> Self {
        Self {
            id: None,
            key,
            data,
            data_type,
            status,
            created_at: BsonDateTime::now(),
            updated_at: BsonDateTime::now(),
        }
    }
}

/// Get config by key with Redis cache fallback
pub async fn get_config_by_key(
    db: web::Data<Database>,
    key: &str,
) -> Result<Option<Config>, Box<dyn std::error::Error>> {
    let mut redis_conn = get_redis_connection().await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Redis connection failed"))?;
    let cache_key = format!("config:{}", key);

    // ✅ Try from Redis cache first
    if let Ok(cached_data) = redis_conn.get::<_, String>(&cache_key).await {
        if let Ok(mut config) = serde_json::from_str::<Config>(&cached_data) {
            // If it's JSON type but data is stringified, parse it
            if config.data_type == ConfigDataType::Json {
                if let Some(Value::String(ref s)) = config.data {
                    if let Ok(parsed_json) = serde_json::from_str::<Value>(s) {
                        config.data = Some(parsed_json);
                    }
                }
            }

            return Ok(Some(config));
        } else {
            eprintln!("⚠️ Failed to deserialize cached config for '{}'", key);
        }
    }

    // ✅ Fallback to MongoDB
    let collection: Collection<Config> = db.collection("configs");
    if let Some(mut config) = collection.find_one(doc! { "key": key }, None).await? {
        // If it's JSON type but stringified, parse it
        if config.data_type == ConfigDataType::Json {
            if let Some(Value::String(ref s)) = config.data {
                if let Ok(parsed_json) = serde_json::from_str::<Value>(s) {
                    config.data = Some(parsed_json);
                }
            }
        }

        let serialized = serde_json::to_string(&config)?;
        let _: () = redis_conn.set_ex(&cache_key, serialized, 86400).await?;
        return Ok(Some(config));
    }

    Ok(None)
}

/// Update config and invalidate Redis cache
pub async fn update_config(
    db: web::Data<Database>,
    key: &str,
    new_data: Option<Value>,
    new_status: ConfigStatus,
) -> Result<(), Box<dyn std::error::Error>> {
    let collection: Collection<Config> = db.collection("configs");
    let filter = doc! { "key": key };

    let mut set_doc = doc! {
        "status": to_bson(&new_status)?,
        "updated_at": BsonDateTime::now(),
    };

    if let Some(data_value) = new_data {
        let (normalized_data, data_type) = match &data_value {
            Value::String(s) => {
                match serde_json::from_str::<Value>(s) {
                    Ok(parsed_json) => (parsed_json, ConfigDataType::Json),
                    Err(_) => (data_value.clone(), ConfigDataType::String),
                }
            }
            _ => (data_value.clone(), ConfigDataType::Json),
        };

        set_doc.insert("data", to_bson(&normalized_data)?);
        set_doc.insert("data_type", to_bson(&data_type.to_string())?);
    }

    let update_doc = doc! { "$set": set_doc };
    collection.update_one(filter, update_doc, None).await?;

    // Invalidate Redis cache
    let mut redis_conn = get_redis_connection().await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Redis connection failed"))?;
    let cache_key = format!("config:{}", key);
    let _: () = redis_conn.del(cache_key).await?;

    Ok(())
}
