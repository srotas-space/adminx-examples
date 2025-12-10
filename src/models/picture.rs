use mongodb::{bson::{doc, oid::ObjectId, DateTime as BsonDateTime}};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use crate::enums::{
    common_enums::{
        StatusEnum,
    }
};


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum PictureStatus {
    Active,
    Inactive,
}

impl ToString for PictureStatus {
    fn to_string(&self) -> String {
        match self {
            PictureStatus::Active => "active".to_string(),
            PictureStatus::Inactive => "inactive".to_string(),
        }
    }
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Picture {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    /// Picture URL or file path
    pub url: String,

    /// Optional Picture metadata
    pub title: Option<String>,

    #[serde(default)]
    pub deleted: bool,

    pub status: StatusEnum,

    /// Timestamps
    pub created_at: BsonDateTime,
    pub updated_at: BsonDateTime,
}


impl Default for Picture {
    fn default() -> Self {
        Self {
            id: None,
            url: String::new(),
            title: None,
            status: StatusEnum::Active,
            deleted: false,
            created_at: BsonDateTime::now(),
            updated_at: BsonDateTime::now(),
        }
    }
}
