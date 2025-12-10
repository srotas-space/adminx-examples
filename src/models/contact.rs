// models/contact.rs
use mongodb::bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::{Deserialize, Serialize};
use validator::{Validate};
use crate::enums::{
    common_enums::{
        GenderEnum,
        StatusEnum,
        deserialize_gender
    }
};
use crate::libs::custom_validators::{
    validate_alpha_only,
    validate_email,
    validate_phone,
    validate_object_required,
    validate_username,
    validate_website,
    validate_message,
};


#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct Contact {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(required)]
    #[validate(custom = "validate_object_required")]
    pub user_id: Option<ObjectId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(required)]
    #[validate(custom = "validate_object_required")]
    pub parent_user_id: Option<ObjectId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(required)]
    #[validate(custom = "validate_alpha_only")]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(required)]
    #[validate(custom = "validate_alpha_only")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(required)]
    #[validate(custom = "validate_email")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_username")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_email")]
    pub official_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(required)]
    #[validate(custom = "validate_phone")]
    pub phone_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, deserialize_with = "deserialize_gender")]
    pub gender: Option<GenderEnum>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_message")]
    pub designation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_message")]
    pub company_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_message")]
    pub company_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_website")]
    pub website: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_message")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_message")]
    pub notes: Option<String>,

    pub status: StatusEnum,

    #[serde(default)]
    pub favorite: bool,

    #[serde(default)]
    pub deleted: bool,

    #[serde(default)]
    pub guest: bool,

    pub created_at: BsonDateTime,
    pub updated_at: BsonDateTime,
}



impl Default for Contact {
    fn default() -> Self {
        Self {
            id: None,
            user_id: None,
            parent_user_id: None,
            first_name: None,
            last_name: None,
            username: None,
            email: None,
            official_email: None,
            phone_number: None,
            age: Some(0),
            gender: Some(GenderEnum::default()),
            designation: None,
            company_name: None,
            company_address: None,
            website: None,
            description: None,
            notes: None,
            status: StatusEnum::default(),
            deleted: false,
            favorite: false, // Default: Not favorite
            guest: false,
            created_at: BsonDateTime::now(),
            updated_at: BsonDateTime::now(),
        }
    }
}

