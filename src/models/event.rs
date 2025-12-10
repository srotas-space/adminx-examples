// /Users/xsm/Documents/workspace/XARD/xard-be/src/models/event.rs


use actix_web::Error;
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
        StatusEnum,
        OnboardEnum,
    },
};

use crate::requests::enums::{
    event_enums::{
        EventStatusEnum,
        ApprovalStatusEnum,
        EventCategoryEnum,
        EventTypeEnum,
        RegistrationTypeEnum,
        PaymentTypeEnum,
        AgeRestrictionEnum,
        DressCodeEnum,
    }
};
use crate::libs::custom_validators::{
    validate_alpha_only,
    validate_email,
    validate_alphanumeric,
    validate_phone,
    validate_website,
    validate_message,
};

use crate::requests::{
    validators::{
        close_validator::{
            validate_textual
        }
    },
};

use crate::utilities::bason_utility::convert_to_bson;
use crate::{
    handle_global_error,
    custom_error_expression,
    handle_custom_error,
};
use anyhow::{Error as AnyhowError};

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct Event {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub user_id: ObjectId,

    #[validate(custom = "validate_textual")]
    pub title: String,

    #[validate(custom = "validate_textual")]
    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    #[validate(custom = "validate_textual")]
    pub address: String,

    // Event date and time fields
    pub event_date: BsonDateTime,

    pub start_time: BsonDateTime,

    pub end_time: BsonDateTime,

    pub start_date: BsonDateTime,

    pub end_date: BsonDateTime,

    // Media and images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    // Event status and approval
    pub status: EventStatusEnum,
    pub approval_status: ApprovalStatusEnum,

    // Event properties
    #[serde(default)]
    pub is_public: bool,

    #[serde(default)]
    pub is_virtual: bool,

    #[serde(default)]
    pub is_paid: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    // Attendee management
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attendees: Option<u32>,

    #[serde(default)]
    pub current_attendees: u32,

    #[serde(default)]
    pub attendees_count: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<ObjectId>>,

    // Creator information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,

    // Timestamps
    pub created_at: BsonDateTime,
    pub updated_at: BsonDateTime,

    // Categorization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    pub category: EventCategoryEnum,

    // Organizer information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_alpha_only")]
    pub organizer_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_email")]
    pub organizer_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_phone")]
    pub organizer_phone: Option<String>,

    // Registration and access
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_website")]
    pub registration_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_website")]
    pub meeting_link: Option<String>,

    #[serde(default)]
    pub requires_registration: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_deadline: Option<BsonDateTime>,

    // Event requirements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_restriction: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dress_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_textual")]
    pub special_instructions: Option<String>,

    // Custom fields for extensibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Value>,

    // Additional enums for better type safety
    pub event_type: EventTypeEnum,
    pub registration_type: RegistrationTypeEnum,
    pub payment_type: PaymentTypeEnum,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_code: Option<String>,

    // System fields
    #[serde(default)]
    pub deleted: bool,

    #[serde(default)]
    pub locked: bool,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            id: None,
            user_id: ObjectId::new(),
            title: String::new(),
            description: String::new(),
            location: None,
            address: String::new(),
            event_date: BsonDateTime::now(),
            start_time: BsonDateTime::now(),
            end_time: BsonDateTime::now(),
            start_date: BsonDateTime::now(),
            end_date: BsonDateTime::now(),
            image: None,
            status: EventStatusEnum::Initial,
            approval_status: ApprovalStatusEnum::Initial,
            is_public: true,
            is_virtual: false,
            is_paid: false,
            price: None,
            max_attendees: None,
            current_attendees: 0,
            attendees_count: 0,
            attendees: None,
            created_by: None,
            created_at: BsonDateTime::now(),
            updated_at: BsonDateTime::now(),
            tags: None,
            category: EventCategoryEnum::Initial,
            organizer_name: None,
            organizer_email: None,
            organizer_phone: None,
            registration_url: None,
            meeting_link: None,
            requires_registration: false,
            registration_deadline: None,
            age_restriction: None,
            dress_code: None,
            special_instructions: None,
            custom_fields: None,
            event_type: EventTypeEnum::Initial,
            registration_type: RegistrationTypeEnum::Initial,
            payment_type: PaymentTypeEnum::Initial,
            qr_code: None,
            deleted: false,
            locked: false,
        }
    }
}

