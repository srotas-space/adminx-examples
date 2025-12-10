// src/models/invitation.rs
use mongodb::bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Invitation {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: Option<ObjectId>, // ✅ Foreign key reference to User model
    pub email: Option<String>,
    pub message: Option<String>,
    pub created_at: BsonDateTime,
    pub updated_at: BsonDateTime,
}

impl Invitation {
    /// Creates a new invitation with the given email and message.
    pub fn new(user_id: Option<ObjectId>, email: Option<String>, message: Option<String>) -> Self {
        Invitation {
            id: None,
            user_id,
            email,
            message: Some(message.unwrap_or_else(|| "Join Xard!".to_string())),
            created_at: BsonDateTime::now(),
            updated_at: BsonDateTime::now(),
        }
    }
}
