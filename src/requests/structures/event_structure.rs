// /Users/xsm/Documents/workspace/XARD/xard-be/src/requests/structures/event_structure.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::{Validate, ValidationError};

use crate::requests::enums::event_enums::{
    ApprovalStatusEnum,
    EventCategoryEnum,
    EventStatusEnum,
    EventTypeEnum,
    PaymentTypeEnum,
    RegistrationTypeEnum,
};

#[derive(Default, Serialize, Deserialize, Debug, Validate)]
pub struct EventRequestBody {
    // Required/basic fields
    pub title: String,
    pub description: String,
    pub address: String,
    pub location: Option<String>,
    // Date & time
    pub event_date: DateTime<Utc>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,

    // Optional / properties
    pub image: Option<String>,
    pub status: Option<EventStatusEnum>,
    pub approval_status: Option<ApprovalStatusEnum>,
    pub is_public: Option<bool>,
    pub is_virtual: Option<bool>,
    pub is_paid: Option<bool>,
    pub price: Option<f64>,
    pub max_attendees: Option<u32>,

    // Categorization
    pub tags: Option<Vec<String>>,
    pub category: Option<EventCategoryEnum>,

    // Organizer info
    pub organizer_name: Option<String>,
    pub organizer_email: Option<String>,
    pub organizer_phone: Option<String>,

    // Registration / access
    pub registration_url: Option<String>,
    pub meeting_link: Option<String>,
    pub requires_registration: Option<bool>,
    pub registration_deadline: Option<DateTime<Utc>>,

    // Requirements / misc
    pub age_restriction: Option<String>,
    pub dress_code: Option<String>,
    pub special_instructions: Option<String>,

    // Extensibility
    pub custom_fields: Option<Value>,

    // Additional enums
    pub event_type: Option<EventTypeEnum>,
    pub registration_type: Option<RegistrationTypeEnum>,
    pub payment_type: Option<PaymentTypeEnum>,

    // Platform info
    pub x_platform: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug, Validate)]
pub struct EventRequestFilters {
    // Search / basic
    pub query: Option<String>,

    // Category and type
    pub category: Option<EventCategoryEnum>,
    pub event_type: Option<EventTypeEnum>,
    pub status: Option<EventStatusEnum>,
    pub approval_status: Option<ApprovalStatusEnum>,

    // Properties
    pub is_public: Option<bool>,
    pub is_virtual: Option<bool>,
    pub is_paid: Option<bool>,

    // Location / organizer
    pub location: Option<String>,
    pub organizer: Option<String>,

    // Date range
    pub date_from: Option<String>,
    pub date_to: Option<String>,

    // Attendees
    pub max_attendees_min: Option<u32>,
    pub max_attendees_max: Option<u32>,

    // Price range
    pub price_min: Option<f64>,
    pub price_max: Option<f64>,

    // Tags / custom
    pub tags: Option<Vec<String>>,
    pub has_tags: Option<bool>,

    // Sorting
    pub sort_by: Option<String>,   // "created_at", "event_date", "title", "price", "max_attendees"
    pub sort_order: Option<String>,// "asc", "desc"

    // User-specific
    pub user_id: Option<String>,
    pub created_by: Option<String>,

    // Additional
    pub requires_registration: Option<bool>,
    pub age_restriction: Option<String>,
    pub dress_code: Option<String>,

    // Platform
    pub x_platform: Option<String>,
}

/// Join Event Request Body
#[derive(Default, Serialize, Deserialize, Debug, Validate)]
pub struct JoinEventRequest {
    pub user_id: Option<String>,
    pub event_id: Option<String>,
    pub notes: Option<String>,
    pub emergency_contact: Option<String>,
    pub dietary_requirements: Option<String>,
    pub accessibility_needs: Option<String>,
}


/// Leave Event Request Body
#[derive(Default, Serialize, Deserialize, Debug, Validate)]
pub struct LeaveEventRequest {
    pub reason: Option<String>,
}

/// Event Search Request (advanced)
#[derive(Default, Serialize, Deserialize, Debug, Validate)]
pub struct EventSearchRequest {
    // Text
    pub search_term: Option<String>,

    // Geo
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub radius_km: Option<f64>,

    // Time
    pub date_range: Option<String>, // "today", "tomorrow", "this_week", "this_month", "custom"
    pub start_date: Option<String>,
    pub end_date: Option<String>,

    // Advanced filters
    pub filters: Option<EventRequestFilters>,

    // Pagination
    pub page: Option<u32>,
    pub limit: Option<u32>,

    // Sorting
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

/// Event Analytics Request
#[derive(Default, Serialize, Deserialize, Debug, Validate)]
pub struct EventAnalyticsRequest {
    pub period: Option<String>, // "day", "week", "month", "quarter", "year"
    pub start_date: Option<String>,
    pub end_date: Option<String>,

    pub event_ids: Option<Vec<String>>,
    pub organizer_id: Option<String>,
    pub category: Option<EventCategoryEnum>,

    pub include_attendee_stats: Option<bool>,
    pub include_revenue_stats: Option<bool>,
    pub include_engagement_stats: Option<bool>,
    pub include_geographic_stats: Option<bool>,
}

/// Event Invitation Request
#[derive(Default, Serialize, Deserialize, Debug, Validate)]
pub struct EventInvitationRequest {
    pub event_id: String,

    // Recipients
    pub user_ids: Option<Vec<String>>,
    pub emails: Option<Vec<String>>,

    // Details
    pub message: Option<String>,
    pub invite_type: Option<String>, // "direct", "bulk", "public"

    // Settings
    pub expires_at: Option<String>,
    pub requires_response: Option<bool>,
    pub allow_plus_one: Option<bool>,

    // Platform
    pub x_platform: Option<String>,
}

#[derive(Deserialize)]
pub struct EventPathInfo {
    pub id: String,
}

#[derive(Deserialize)]
pub struct InvitationPathInfo {
    pub id: String,
}


/// Join Event Request Body
#[derive(Default, Serialize, Deserialize, Debug, Validate)]
pub struct ShareEventRequest {
    pub user_ids: Option<String>,
    pub emails: Option<String>,
    pub notes: Option<String>,
}

