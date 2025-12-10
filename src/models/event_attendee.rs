// /Users/xsm/Documents/workspace/XARD/xard-be/src/models/event_attendee.rs

use actix_web::Error;
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId, DateTime as BsonDateTime},
    options::{FindOptions}
};
use futures::{TryStreamExt, StreamExt};
use serde::{Deserialize, Serialize};
use validator::{Validate};

use crate::enums::{
    common_enums::StatusEnum,
};
use crate::libs::custom_validators::{
    validate_email,
    validate_phone,
};
use crate::utilities::bason_utility::convert_to_bson;
use crate::{
    handle_global_error,
    custom_error_expression,
    handle_custom_error,
};
use anyhow::{Error as AnyhowError};

use crate::requests::enums::{
    event_enums::{
        EventStatusEnum,
        RegistrationTypeEnum,
    }
};
use strum_macros::EnumIter;


#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct EventAttendee {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<ObjectId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<ObjectId>,

    // Attendee information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_email")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_phone")]
    pub phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub designation: Option<String>,

    // Registration details
    pub registration_status: StatusEnum,
    pub registration_type: RegistrationTypeEnum,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<BsonDateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_in_date: Option<BsonDateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_out_date: Option<BsonDateTime>,

    // Additional information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dietary_requirements: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessibility_needs: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency_contact: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    // Payment information
    #[serde(default)]
    pub payment_status: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_amount: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<BsonDateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_reference: Option<String>,

    // Timestamps
    pub created_at: BsonDateTime,
    pub updated_at: BsonDateTime,

    // System fields
    #[serde(default)]
    pub deleted: bool,
}

impl Default for EventAttendee {
    fn default() -> Self {
        Self {
            id: None,
            event_id: None,
            user_id: None,
            email: None,
            phone: None,
            first_name: None,
            last_name: None,
            company: None,
            designation: None,
            registration_status: StatusEnum::Initial,
            registration_type: RegistrationTypeEnum::Initial,
            registration_date: None,
            check_in_date: None,
            check_out_date: None,
            dietary_requirements: None,
            accessibility_needs: None,
            emergency_contact: None,
            notes: None,
            payment_status: false,
            payment_amount: None,
            payment_date: None,
            payment_reference: None,
            created_at: BsonDateTime::now(),
            updated_at: BsonDateTime::now(),
            deleted: false,
        }
    }
}

impl EventAttendee {
    /// Register for an event
    pub async fn register_for_event(&self, db: &Database) -> Result<Self, Error> {
        let collection = db.collection::<EventAttendee>("event_attendees");
        
        let mut attendee = self.clone();
        attendee.registration_date = Some(BsonDateTime::now());
        attendee.created_at = BsonDateTime::now();
        attendee.updated_at = BsonDateTime::now();
        
        let insert_result = collection.insert_one(&attendee, None).await
            .map_err(|e| custom_error_expression!(internal_error, 500, e.to_string()))?;
        
        attendee.id = insert_result.inserted_id.as_object_id();
        Ok(attendee)
    }

    /// Check in to an event
    pub async fn check_in(&self, db: &Database) -> Result<(), Error> {
        let attendee_id = match self.id {
            Some(id) => id,
            None => return Err(handle_custom_error!(bad_request, 400, "Missing attendee ID".to_string())),
        };

        let collection = db.collection::<EventAttendee>("event_attendees");
        let filter = doc! { "_id": attendee_id, "deleted": false };
        let update = doc! {
            "$set": {
                "check_in_date": BsonDateTime::now()
            },
            "$currentDate": {
                "updated_at": true
            }
        };

        let update_result = collection.update_one(filter, update, None).await
            .map_err(|e| custom_error_expression!(internal_error, 500, e.to_string()))?;

        if update_result.matched_count == 0 {
            return Err(handle_custom_error!(not_found, 404, "Attendee not found".to_string()));
        }

        Ok(())
    }

    /// Check out of an event
    pub async fn check_out(&self, db: &Database) -> Result<(), Error> {
        let attendee_id = match self.id {
            Some(id) => id,
            None => return Err(handle_custom_error!(bad_request, 400, "Missing attendee ID".to_string())),
        };

        let collection = db.collection::<EventAttendee>("event_attendees");
        let filter = doc! { "_id": attendee_id, "deleted": false };
        let update = doc! {
            "$set": {
                "check_out_date": BsonDateTime::now()
            },
            "$currentDate": {
                "updated_at": true
            }
        };

        let update_result = collection.update_one(filter, update, None).await
            .map_err(|e| custom_error_expression!(internal_error, 500, e.to_string()))?;

        if update_result.matched_count == 0 {
            return Err(handle_custom_error!(not_found, 404, "Attendee not found".to_string()));
        }

        Ok(())
    }

    /// Get attendees for a specific event
    pub async fn get_event_attendees(
        db: &Database,
        event_id: ObjectId,
        page: u64,
        limit: i64,
    ) -> Result<Vec<Self>, Error> {
        let collection = db.collection::<EventAttendee>("event_attendees");
        let skip = (page - 1) * limit as u64;

        let filter = doc! {
            "event_id": event_id,
            "deleted": false
        };

        let find_options = FindOptions::builder()
            .sort(doc! { "created_at": -1 })
            .skip(skip)
            .limit(limit)
            .build();

        let mut cursor = collection.find(filter, find_options).await
            .map_err(|e| custom_error_expression!(internal_error, 500, e.to_string()))?;

        let mut attendees = Vec::new();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(attendee) => attendees.push(attendee),
                Err(e) => return Err(handle_custom_error!(internal_error, 500, e.to_string())),
            }
        }

        Ok(attendees)
    }

    /// Cancel attendance
    pub async fn cancel_attendance(&self, db: &Database) -> Result<(), Error> {
        let attendee_id = match self.id {
            Some(id) => id,
            None => return Err(handle_custom_error!(bad_request, 400, "Missing attendee ID".to_string())),
        };

        let collection = db.collection::<EventAttendee>("event_attendees");
        let filter = doc! { "_id": attendee_id };
        let update = doc! {
            "$set": {
                "deleted": true,
                "registration_status": convert_to_bson(&StatusEnum::Inactive)?
            },
            "$currentDate": {
                "updated_at": true
            }
        };

        let update_result = collection.update_one(filter, update, None).await
            .map_err(|e| custom_error_expression!(internal_error, 500, e.to_string()))?;

        if update_result.matched_count == 0 {
            return Err(handle_custom_error!(not_found, 404, "Attendee not found".to_string()));
        }

        Ok(())
    }

    /// Update attendee information
    pub async fn update_attendee(&self, db: &Database) -> Result<Self, Error> {
        let attendee_id = match self.id {
            Some(id) => id,
            None => return Err(handle_custom_error!(bad_request, 400, "Missing attendee ID".to_string())),
        };

        let collection = db.collection::<EventAttendee>("event_attendees");
        let filter = doc! { "_id": attendee_id, "deleted": false };
        
        let update_doc = doc! {
            "$set": {
                "email": &self.email,
                "phone": &self.phone,
                "first_name": &self.first_name,
                "last_name": &self.last_name,
                "company": &self.company,
                "designation": &self.designation,
                "dietary_requirements": &self.dietary_requirements,
                "accessibility_needs": &self.accessibility_needs,
                "emergency_contact": &self.emergency_contact,
                "notes": &self.notes,
                "payment_status": self.payment_status,
                "payment_amount": &self.payment_amount,
                "payment_reference": &self.payment_reference,
            },
            "$currentDate": {
                "updated_at": true
            }
        };

        let update_result = collection.update_one(filter, update_doc, None).await
            .map_err(|e| custom_error_expression!(internal_error, 500, e.to_string()))?;

        if update_result.matched_count == 0 {
            return Err(handle_custom_error!(not_found, 404, "Attendee not found".to_string()));
        }

        Ok(self.clone())
    }
}
