use mongodb::{bson::{doc, oid::ObjectId, DateTime as BsonDateTime}, Database, Collection};
use actix_web::{web, Error};
use serde::{Deserialize, Serialize};

/// Enum for specifying which model the image belongs to
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ImageModelType {
    Contact,
    User,
    Event,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    /// The model type (Contact, User, etc.)
    pub model_type: ImageModelType,

    /// The associated model's ID (Contact ID or User ID)
    pub model_id: ObjectId,

    /// Image URL or file path
    pub url: String,

    /// Optional image metadata
    pub alt_text: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub file_size: Option<u64>, // File size in bytes

    /// Timestamps
    pub created_at: BsonDateTime,
    pub updated_at: BsonDateTime,
}

// ✅ Helper function to create a new Image
impl Image {
    pub fn new(model_type: ImageModelType, model_id: ObjectId, url: String, alt_text: Option<String>) -> Self {
        Self {
            id: None,
            model_type,
            model_id,
            url,
            alt_text,
            width: None,
            height: None,
            file_size: None,
            created_at: BsonDateTime::now(),
            updated_at: BsonDateTime::now(),
        }
    }
}

/// **✅ Function to Save Image to MongoDB**
pub async fn save_image(db: web::Data<Database>, image: Image) -> Result<ObjectId, Error> {
    let collection: Collection<Image> = db.collection("images");

    let insert_result = collection
        .insert_one(image, None)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to insert image"))?;

    // ✅ Extract and return the newly inserted `_id`
    match insert_result.inserted_id.as_object_id() {
        Some(object_id) => Ok(object_id),
        None => Err(actix_web::error::ErrorInternalServerError("Failed to get inserted ID")),
    }
}
