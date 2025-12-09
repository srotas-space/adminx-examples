// test/src/models/image_model.rs
use actix_web::web;
use mongodb::{
    bson::{doc, oid::ObjectId, to_bson, DateTime as BsonDateTime},
    Collection, Database,
};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use crate::services::redis_service::get_redis_connection;
use strum_macros::EnumIter; 


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum ImageStatus {
    Active,
    Inactive,
}

impl ToString for ImageStatus {
    fn to_string(&self) -> String {
        match self {
            ImageStatus::Active => "active".to_string(),
            ImageStatus::Inactive => "inactive".to_string(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub title: String,
    pub image_url: String,
    pub status: ImageStatus,
    pub deleted: bool,
    pub created_at: BsonDateTime,
    pub updated_at: BsonDateTime,
}


impl Default for Image {
    fn default() -> Self {
        Self {
            id: None,
            title: String::from(""),
            image_url: String::from(""),
            status: ImageStatus::Active,
            deleted: false,
            created_at: BsonDateTime::now(),
            updated_at: BsonDateTime::now(),
        }
    }
}

