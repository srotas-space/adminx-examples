// src/admin/resources/event_attendee_resource.rs
use crate::db::mongo::get_collection;
use adminx::AdmixResource;
use async_trait::async_trait;
use mongodb::{
    Collection,
    bson::{doc, Document, oid::ObjectId},
};
use serde_json::{json, Value};
use convert_case::{Casing, Case};

use crate::enums::common_enums::StatusEnum;
use crate::requests::enums::event_enums::RegistrationTypeEnum;

#[derive(Debug, Clone)]
pub struct EventAttendeeResource;

/* ----------------------------- Options Builder ----------------------------- */
pub struct AttendeeOptions;

impl AttendeeOptions {
    // StatusEnum में #[serde(rename_all="lowercase")] है → value को lowercase देना चाहिए
    pub fn registration_status_options() -> Vec<Value> {
        let vals = [
            "initial","active","inactive","pending","finished","accepted",
            "declined","expired","blocked","archived",
        ];
        vals.into_iter()
            .map(|v| json!({"value": v, "label": v.to_case(Case::Title)}))
            .collect()
    }

    // RegistrationTypeEnum में rename नहीं है → variants as-is
    pub fn registration_type_options() -> Vec<Value> {
        let vals = [
            "Initial","Required","Optional","NotRequired","Open","Invited","InviteOnly","Paid","Waitlist",
        ];
        vals.into_iter()
            .map(|v| json!({"value": v, "label": v}))
            .collect()
    }

    pub fn boolean_options() -> Vec<Value> {
        vec![
            json!({ "value": true,  "label": "True"  }),
            json!({ "value": false, "label": "False" }),
        ]
    }
}

/* --------------------------------- Helpers -------------------------------- */
fn parse_oid_opt(s: &str) -> Option<ObjectId> {
    ObjectId::parse_str(s).ok()
}

async fn update_set_by_id(set_doc: Document, id: &ObjectId) -> Result<u64, actix_web::HttpResponse> {
    let coll = get_collection::<Document>("event_attendees");
    coll.update_one(
        doc!{"_id": id, "deleted": { "$ne": true }},
        doc!{
            "$set": set_doc,
            "$currentDate": { "updated_at": true }
        },
        None
    )
    .await
    .map(|res| res.modified_count)
    .map_err(|e| {
        actix_web::HttpResponse::InternalServerError().json(json!({
            "error": "db_update_failed",
            "message": e.to_string()
        }))
    })
}

/* ------------------------------ Resource Impl ------------------------------ */
#[async_trait]
impl AdmixResource for EventAttendeeResource {
    fn new() -> Self { EventAttendeeResource }

    fn resource_name(&self) -> &'static str { "Event Attendees" }
    fn base_path(&self) -> &'static str { "event_attendees" }
    fn collection_name(&self) -> &'static str { "event_attendees" }
    fn get_collection(&self) -> Collection<Document> { get_collection::<Document>("event_attendees") }
    fn clone_box(&self) -> Box<dyn AdmixResource> { Box::new(Self::new()) }
    fn menu_group(&self) -> Option<&'static str> { Some("Events") }
    fn menu(&self) -> &'static str { "Attendees" }

    fn allowed_roles(&self) -> Vec<String> {
        vec!["admin".into(), "superadmin".into()]
    }

    fn permit_keys(&self) -> Vec<&'static str> {
        vec![
            "event_id","user_id","email","phone","first_name","last_name","company","designation",
            "registration_status","registration_type","registration_date",
            "check_in_date","check_out_date",
            "dietary_requirements","accessibility_needs","emergency_contact","notes",
            "payment_status","payment_amount","payment_date","payment_reference",
            "deleted"
        ]
    }

    fn form_structure(&self) -> Option<Value> {
        Some(json!({
            "groups": [
                {
                    "title": "Links",
                    "fields": [
                        { "name": "event_id", "field_type": "text", "label": "Event ID (ObjectId)" },
                        { "name": "user_id",  "field_type": "text", "label": "User ID (ObjectId)" }
                    ]
                },
                {
                    "title": "Contact",
                    "fields": [
                        { "name": "first_name", "field_type": "text",  "label": "First Name" },
                        { "name": "last_name",  "field_type": "text",  "label": "Last Name" },
                        { "name": "email",      "field_type": "email", "label": "Email" },
                        { "name": "phone",      "field_type": "text",  "label": "Phone" },
                        { "name": "company",    "field_type": "text",  "label": "Company" },
                        { "name": "designation","field_type": "text",  "label": "Designation" }
                    ]
                },
                {
                    "title": "Registration",
                    "fields": [
                        { "name": "registration_status", "field_type": "select",   "label": "Registration Status", "options": AttendeeOptions::registration_status_options(), "required": true },
                        { "name": "registration_type",   "field_type": "select",   "label": "Registration Type",   "options": AttendeeOptions::registration_type_options(),   "required": true },
                        { "name": "registration_date",   "field_type": "datetime", "label": "Registration Date" }
                    ]
                },
                {
                    "title": "Attendance",
                    "fields": [
                        { "name": "check_in_date",  "field_type": "datetime", "label": "Check-in Date" },
                        { "name": "check_out_date", "field_type": "datetime", "label": "Check-out Date" }
                    ]
                },
                {
                    "title": "Payment",
                    "fields": [
                        { "name": "payment_status",    "field_type": "boolean",  "label": "Paid?", "options": AttendeeOptions::boolean_options() },
                        { "name": "payment_amount",    "field_type": "number",   "label": "Amount" },
                        { "name": "payment_date",      "field_type": "datetime", "label": "Payment Date" },
                        { "name": "payment_reference", "field_type": "text",     "label": "Payment Reference" }
                    ]
                },
                {
                    "title": "Other",
                    "fields": [
                        { "name": "dietary_requirements",  "field_type": "text", "label": "Dietary Requirements" },
                        { "name": "accessibility_needs",   "field_type": "text", "label": "Accessibility Needs" },
                        { "name": "emergency_contact",     "field_type": "text", "label": "Emergency Contact" },
                        { "name": "notes",                 "field_type": "editor_text", "label": "Notes" }
                    ]
                },
                {
                    "title": "System",
                    "fields": [
                        { "name": "deleted", "field_type": "boolean", "label": "Deleted?", "options": AttendeeOptions::boolean_options() }
                    ]
                }
            ]
        }))
    }

    fn list_structure(&self) -> Option<Value> {
        Some(json!({
            "columns": [
                { "field": "first_name", "label": "First", "sortable": true },
                { "field": "last_name",  "label": "Last",  "sortable": true },
                { "field": "email",      "label": "Email", "sortable": true },
                { "field": "phone",      "label": "Phone", "sortable": true },
                { "field": "registration_status", "label": "Reg. Status", "sortable": true },
                { "field": "registration_type",   "label": "Reg. Type",   "sortable": true },
                { "field": "payment_status", "label": "Paid", "sortable": true },
                { "field": "payment_amount", "label": "Amount", "type": "number", "sortable": true },
                { "field": "check_in_date",  "label": "Check-in",  "type": "datetime", "sortable": true },
                { "field": "check_out_date", "label": "Check-out", "type": "datetime", "sortable": true },
                { "field": "created_at", "label": "Created", "type": "datetime", "sortable": true },
                { "field": "updated_at", "label": "Updated", "type": "datetime", "sortable": true }
            ],
            "actions": ["view", "edit", "delete"]
        }))
    }

    fn view_structure(&self) -> Option<Value> {
        Some(json!({
            "sections": [
                {
                    "title": "Links",
                    "fields": [
                        { "field": "_id",      "label": "Attendee ID" },
                        { "field": "event_id", "label": "Event ID" },
                        { "field": "user_id",  "label": "User ID" }
                    ]
                },
                {
                    "title": "Contact",
                    "fields": [
                        { "field": "first_name", "label": "First Name" },
                        { "field": "last_name",  "label": "Last Name" },
                        { "field": "email",      "label": "Email" },
                        { "field": "phone",      "label": "Phone" },
                        { "field": "company",    "label": "Company" },
                        { "field": "designation","label": "Designation" }
                    ]
                },
                {
                    "title": "Registration",
                    "fields": [
                        { "field": "registration_status", "label": "Status" },
                        { "field": "registration_type",   "label": "Type" },
                        { "field": "registration_date",   "label": "Date", "type": "datetime" }
                    ]
                },
                {
                    "title": "Attendance",
                    "fields": [
                        { "field": "check_in_date",  "label": "Check-in",  "type": "datetime" },
                        { "field": "check_out_date", "label": "Check-out", "type": "datetime" }
                    ]
                },
                {
                    "title": "Payment",
                    "fields": [
                        { "field": "payment_status",    "label": "Paid", "type": "boolean" },
                        { "field": "payment_amount",    "label": "Amount" },
                        { "field": "payment_date",      "label": "Date", "type": "datetime" },
                        { "field": "payment_reference", "label": "Reference" }
                    ]
                },
                {
                    "title": "Other",
                    "fields": [
                        { "field": "dietary_requirements", "label": "Dietary Requirements" },
                        { "field": "accessibility_needs",  "label": "Accessibility Needs" },
                        { "field": "emergency_contact",    "label": "Emergency Contact" },
                        { "field": "notes",                "label": "Notes" }
                    ]
                },
                {
                    "title": "System",
                    "fields": [
                        { "field": "deleted",     "label": "Deleted", "type": "boolean" },
                        { "field": "created_at",  "label": "Created At", "type": "datetime" },
                        { "field": "updated_at",  "label": "Updated At", "type": "datetime" }
                    ]
                }
            ]
        }))
    }

    fn filters(&self) -> Option<Value> {
        Some(json!({
            "filters": [
                { "field": "event_id", "type": "text", "label": "Event ID" },
                { "field": "user_id",  "type": "text", "label": "User ID" },
                { "field": "email",    "type": "text", "label": "Email" },
                { "field": "phone",    "type": "text", "label": "Phone" },
                { "field": "first_name", "type": "text", "label": "First Name" },
                { "field": "last_name",  "type": "text", "label": "Last Name" },
                { "field": "company",    "type": "text", "label": "Company" },
                { "field": "designation","type": "text", "label": "Designation" },
                { "field": "payment_reference","type": "text", "label": "Payment Ref" },

                { "field": "registration_status", "type": "select", "label": "Reg. Status",
                  "options": AttendeeOptions::registration_status_options() },
                { "field": "registration_type", "type": "select", "label": "Reg. Type",
                  "options": AttendeeOptions::registration_type_options() },

                { "field": "payment_status", "type": "boolean", "label": "Paid?",
                  "options": AttendeeOptions::boolean_options() },
                { "field": "deleted", "type": "boolean", "label": "Deleted?",
                  "options": AttendeeOptions::boolean_options() },

                { "field": "registration_date", "type": "date_range", "label": "Registration Date" },
                { "field": "check_in_date",     "type": "date_range", "label": "Check-in Date" },
                { "field": "check_out_date",    "type": "date_range", "label": "Check-out Date" },
                { "field": "payment_date",      "type": "date_range", "label": "Payment Date" },
                { "field": "created_at",        "type": "date_range", "label": "Created Date" },
                { "field": "updated_at",        "type": "date_range", "label": "Updated Date" }
            ]
        }))
    }

    fn custom_actions(&self) -> Vec<adminx::actions::CustomAction> {
        vec![
            adminx::actions::CustomAction {
                name: "check_in",
                method: "POST",
                handler: |req, _path, _body| {
                    let id = req.match_info().get("id").and_then(parse_oid_opt);
                    let Some(id) = id else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_object_id"})) });
                    };
                    Box::pin(async move {
                        let coll = get_collection::<Document>("event_attendees");
                        let res = coll.update_one(
                            doc!{"_id": &id, "deleted": { "$ne": true }},
                            doc!{ "$currentDate": { "check_in_date": true, "updated_at": true } },
                            None
                        ).await;
                        match res {
                            Ok(r) => actix_web::HttpResponse::Ok().json(json!({ "success": true, "modified": r.modified_count })),
                            Err(e) => actix_web::HttpResponse::InternalServerError().json(json!({"error":"db_update_failed","message":e.to_string()}))
                        }
                    })
                },
                ui: None,
            },
            adminx::actions::CustomAction {
                name: "check_out",
                method: "POST",
                handler: |req, _path, _body| {
                    let id = req.match_info().get("id").and_then(parse_oid_opt);
                    let Some(id) = id else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_object_id"})) });
                    };
                    Box::pin(async move {
                        let coll = get_collection::<Document>("event_attendees");
                        let res = coll.update_one(
                            doc!{"_id": &id, "deleted": { "$ne": true }},
                            doc!{ "$currentDate": { "check_out_date": true, "updated_at": true } },
                            None
                        ).await;
                        match res {
                            Ok(r) => actix_web::HttpResponse::Ok().json(json!({ "success": true, "modified": r.modified_count })),
                            Err(e) => actix_web::HttpResponse::InternalServerError().json(json!({"error":"db_update_failed","message":e.to_string()}))
                        }
                    })
                },
                ui: None,
            },
            adminx::actions::CustomAction {
                name: "cancel",
                method: "POST",
                handler: |req, _path, _body| {
                    let id = req.match_info().get("id").and_then(parse_oid_opt);
                    let Some(id) = id else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_object_id"})) });
                    };
                    Box::pin(async move {
                        match update_set_by_id(doc!{ "deleted": true, "registration_status": "inactive" }, &id).await {
                            Ok(modified) => actix_web::HttpResponse::Ok().json(json!({ "success": true, "modified": modified })),
                            Err(resp) => resp
                        }
                    })
                },
                ui: None,
            },
            adminx::actions::CustomAction {
                name: "reinstate",
                method: "POST",
                handler: |req, _path, _body| {
                    let id = req.match_info().get("id").and_then(parse_oid_opt);
                    let Some(id) = id else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_object_id"})) });
                    };
                    Box::pin(async move {
                        match update_set_by_id(doc!{ "deleted": false, "registration_status": "active" }, &id).await {
                            Ok(modified) => actix_web::HttpResponse::Ok().json(json!({ "success": true, "modified": modified })),
                            Err(resp) => resp
                        }
                    })
                },
                ui: None,
            },
            adminx::actions::CustomAction {
                name: "set_registration_status",
                method: "POST",
                handler: |req, _path, body| {
                    let id = req.match_info().get("id").and_then(parse_oid_opt);
                    let Some(id) = id else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_object_id"})) });
                    };
                    let st = body.get("registration_status").and_then(|v| v.as_str()).unwrap_or("initial").to_string();
                    Box::pin(async move {
                        match update_set_by_id(doc!{ "registration_status": st }, &id).await {
                            Ok(modified) => actix_web::HttpResponse::Ok().json(json!({ "success": true, "modified": modified })),
                            Err(resp) => resp
                        }
                    })
                },
                ui: None,
            },
            adminx::actions::CustomAction {
                name: "toggle_payment",
                method: "POST",
                handler: |req, _path, body| {
                    let id = req.match_info().get("id").and_then(parse_oid_opt);
                    let Some(id) = id else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_object_id"})) });
                    };
                    let on = body.get("payment_status").and_then(|v| v.as_bool()).unwrap_or(true);
                    Box::pin(async move {
                        let coll = get_collection::<Document>("event_attendees");
                        let update = if on {
                            doc!{
                                "$set": { "payment_status": true },
                                "$currentDate": { "payment_date": true, "updated_at": true }
                            }
                        } else {
                            doc!{
                                "$set": { "payment_status": false },
                                "$currentDate": { "updated_at": true }
                            }
                        };
                        match coll.update_one(doc!{"_id": &id, "deleted": { "$ne": true }}, update, None).await {
                            Ok(r) => actix_web::HttpResponse::Ok().json(json!({ "success": true, "payment_status": on, "modified": r.modified_count })),
                            Err(e) => actix_web::HttpResponse::InternalServerError().json(json!({"error":"db_update_failed","message":e.to_string()}))
                        }
                    })
                },
                ui: None,
            },
            adminx::actions::CustomAction {
                name: "set_payment_info",
                method: "POST",
                handler: |req, _path, body| {
                    let id = req.match_info().get("id").and_then(parse_oid_opt);
                    let Some(id) = id else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_object_id"})) });
                    };
                    let amount = body.get("payment_amount").and_then(|v| v.as_f64());
                    let reference = body.get("payment_reference").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let stamp_now = body.get("stamp_now").and_then(|v| v.as_bool()).unwrap_or(false);

                    Box::pin(async move {
                        let mut set_doc = Document::new();
                        if let Some(a) = amount { set_doc.insert("payment_amount", a); }
                        if let Some(r) = reference { set_doc.insert("payment_reference", r); }

                        if set_doc.is_empty() && !stamp_now {
                            return actix_web::HttpResponse::BadRequest().json(json!({"error":"no_fields_to_update"}));
                        }

                        let coll = get_collection::<Document>("event_attendees");
                        let mut update = doc!{ "$set": set_doc };
                        if stamp_now {
                            update.insert("$currentDate", doc!{ "payment_date": true, "updated_at": true });
                        } else {
                            update.insert("$currentDate", doc!{ "updated_at": true });
                        }

                        match coll.update_one(doc!{"_id": &id, "deleted": { "$ne": true }}, update, None).await {
                            Ok(r) => actix_web::HttpResponse::Ok().json(json!({ "success": true, "modified": r.modified_count })),
                            Err(e) => actix_web::HttpResponse::InternalServerError().json(json!({"error":"db_update_failed","message":e.to_string()}))
                        }
                    })
                },
                ui: None,
            },
            adminx::actions::CustomAction {
                name: "link_user",
                method: "POST",
                handler: |req, _path, body| {
                    let id = req.match_info().get("id").and_then(parse_oid_opt);
                    let Some(id) = id else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_object_id"})) });
                    };
                    let user_id = body.get("user_id").and_then(|v| v.as_str()).unwrap_or("");
                    let Some(user_oid) = parse_oid_opt(user_id) else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_user_id"})) });
                    };
                    Box::pin(async move {
                        match update_set_by_id(doc!{ "user_id": user_oid }, &id).await {
                            Ok(modified) => actix_web::HttpResponse::Ok().json(json!({ "success": true, "modified": modified })),
                            Err(resp) => resp
                        }
                    })
                },
                ui: None,
            },
            adminx::actions::CustomAction {
                name: "link_event",
                method: "POST",
                handler: |req, _path, body| {
                    let id = req.match_info().get("id").and_then(parse_oid_opt);
                    let Some(id) = id else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_object_id"})) });
                    };
                    let event_id = body.get("event_id").and_then(|v| v.as_str()).unwrap_or("");
                    let Some(event_oid) = parse_oid_opt(event_id) else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_event_id"})) });
                    };
                    Box::pin(async move {
                        match update_set_by_id(doc!{ "event_id": event_oid }, &id).await {
                            Ok(modified) => actix_web::HttpResponse::Ok().json(json!({ "success": true, "modified": modified })),
                            Err(resp) => resp
                        }
                    })
                },
                ui: None,
            },
        ]
    }
}
