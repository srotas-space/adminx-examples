// models/user.rs
use actix_web::{Error};
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId, DateTime as BsonDateTime},
    options::{FindOptions}
};
use futures::{TryStreamExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::{Validate};
use crate::enums::{
    common_enums::{
        GenderEnum,
        StatusEnum,
        OnboardEnum,
    }
};
use crate::libs::custom_validators::{
    validate_alpha_only,
    validate_email,
    validate_alphanumeric,
    validate_phone,
    validate_username,
    validate_website,
    validate_message,
};
use crate::utilities::bason_utility::convert_to_bson;
use crate::{
    handle_global_error,
    custom_error_expression,
    handle_custom_error,
};
use anyhow::{Error as AnyhowError};

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[validate(required)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_alpha_only")]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_alpha_only")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(required)]
    #[validate(custom = "validate_email")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_email")]
    pub official_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(required)]
    #[validate(custom = "validate_phone")]
    pub phone_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_username")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<GenderEnum>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_alphanumeric")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkedin: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkedin_datum: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_alphanumeric")]
    pub designation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub firebase_token: Option<String>,

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
    pub protection_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_message")]
    pub x_platform: Option<String>, // ✅ Store X-PLATFORM header info
    pub status: StatusEnum,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_message")]
    pub notes: Option<String>,
    pub social_user_id: Option<String>,

    #[serde(default)]
    pub deleted: bool,
    #[serde(default)]
    pub locked: bool,

    pub onboard: OnboardEnum,

    pub created_at: BsonDateTime,
    pub updated_at: BsonDateTime,
}


impl Default for User {
    fn default() -> Self {
        Self {
            id: None,
            first_name: None,
            last_name: None,
            email: None,
            username: None,
            official_email: None,
            phone_number: None,
            ccode: None,
            age: Some(0),
            gender: Some(GenderEnum::Initial),
            description: None,
            linkedin: None,
            linkedin_datum: None,
            designation: None,
            firebase_token: None,
            company_name: None,
            company_address: None,
            website: None,
            x_platform: None,
            status: StatusEnum::Initial,
            onboard: OnboardEnum::Initial,
            protection_secret: None,
            social_user_id: None,
            notes: None,
            deleted: false,
            locked: false,
            created_at: BsonDateTime::now(),
            updated_at: BsonDateTime::now(),
        }
    }
}


impl User {
    pub fn full_name(&self) -> String {
        return match (&self.first_name, &self.last_name) {
            (Some(f), Some(l)) => format!("{f} {l}"),
            (Some(f), None)    => f.clone(),
            (None, Some(l))    => l.clone(),
            (None, None)       => String::new(),
        };
    }
}