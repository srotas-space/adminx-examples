// models/notification.rs
use mongodb::bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Notification {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: Option<ObjectId>, // ✅ Foreign key reference to User Notification
    pub title: Option<String>,
    pub notify_type: Option<String>,
    pub message: Option<String>, // ✅ "success" or "failed"
    pub destination: Option<String>,
    pub extras: Option<Value>,
    pub deleted: bool,
    pub x_platform: Option<String>, // ✅ Stores `X-PLATFORM` header info
    pub created_at: BsonDateTime,
    pub updated_at: BsonDateTime,
    pub sent_at: BsonDateTime,
}

impl Default for Notification {
    fn default() -> Self {
        Self {
            id: None,
            user_id: None,
            title: None,
            notify_type: None,
            message: None,
            destination: None,
            extras: Some(serde_json::json!({})),
            x_platform: None,
            deleted: false,
            sent_at: BsonDateTime::now(),
            created_at: BsonDateTime::now(),
            updated_at: BsonDateTime::now(),
        }
    }
}
