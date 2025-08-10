use mongodb::bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::{Deserialize, Serialize};
use crate::structs::enums::{StatusEnum};
use serde_json::{Value, json};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub title: Option<String>,
    pub description: Option<String>,

    pub extras: Option<Value>,

    pub status: StatusEnum,

    #[serde(default)]
    pub deleted: bool,

    pub created_at: BsonDateTime,
    pub updated_at: BsonDateTime
}

impl Default for Notification {
    fn default() -> Self {
        Self {
            id: None,
            title: None,
            description: None,
            extras: None,
            status: StatusEnum::Initial,
            deleted: false,
            created_at: BsonDateTime::now(),
            updated_at: BsonDateTime::now(),
        }
    }
}