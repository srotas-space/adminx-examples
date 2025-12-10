// src/admin/resources/event_resource.rs
use crate::db::mongo::get_collection;
use adminx::AdmixResource;
use async_trait::async_trait;
use mongodb::{Collection, bson::{doc, Document, oid::ObjectId}};
use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct EventResource;

/* ----------------------------- Options Builder ----------------------------- */
pub struct EventOptions;

impl EventOptions {
    pub fn status_options() -> Vec<Value> {
        // event_enums.rs → no serde rename; variants as-is
        [
            "Initial","Draft","Published","Cancelled","Completed",
        ].into_iter().map(|v| json!({"value": v, "label": v})).collect()
    }

    pub fn approval_options() -> Vec<Value> {
        ["Initial","Pending","Approved","Rejected"]
            .into_iter().map(|v| json!({"value": v, "label": v})).collect()
    }

    pub fn category_options() -> Vec<Value> {
        [
            "Initial","Business","Technology","Education","Entertainment","Sports","Health",
            "FoodAndDrink","ArtAndCulture","Networking","Conference","Workshop","Meetup",
            "Webinar","Seminar","Other",
        ].into_iter().map(|v| json!({"value": v, "label": v})).collect()
    }

    pub fn event_type_options() -> Vec<Value> {
        ["Initial","Physical","Virtual","Hybrid"]
            .into_iter().map(|v| json!({"value": v, "label": v})).collect()
    }

    pub fn registration_type_options() -> Vec<Value> {
        ["Initial","Required","Optional","NotRequired","Open","Invited","InviteOnly","Paid","Waitlist"]
            .into_iter().map(|v| json!({"value": v, "label": v})).collect()
    }

    pub fn payment_type_options() -> Vec<Value> {
        ["Initial","Free","Paid","Donation"]
            .into_iter().map(|v| json!({"value": v, "label": v})).collect()
    }

    // Custom(String) variants को list में नहीं दिखाते
    pub fn age_options() -> Vec<Value> {
        ["Initial","AllAges","AdultsOnly","ChildrenOnly","Teenagers","Seniors"]
            .into_iter().map(|v| json!({"value": v, "label": v})).collect()
    }

    pub fn dress_code_options() -> Vec<Value> {
        ["Initial","Casual","BusinessCasual","Business","Formal","BlackTie","Costume","Sportswear"]
            .into_iter().map(|v| json!({"value": v, "label": v})).collect()
    }

    pub fn boolean_options() -> Vec<Value> {
        vec![ json!({"value": true, "label":"True"}), json!({"value": false, "label":"False"}) ]
    }
}

/* --------------------------------- Helpers -------------------------------- */

fn parse_oid_opt(s: &str) -> Option<ObjectId> {
    ObjectId::parse_str(s).ok()
}

async fn update_one_by_id(set_doc: Document, id: &ObjectId) -> Result<u64, actix_web::HttpResponse> {
    let coll = get_collection::<Document>("events");
    coll.update_one(doc!{"_id": id}, doc!{"$set": set_doc}, None)
        .await
        .map(|res| res.modified_count)
        .map_err(|e| {
            actix_web::HttpResponse::InternalServerError().json(json!({
                "error": "db_update_failed",
                "message": e.to_string()
            }))
        })
}

async fn add_to_set_and_inc(
    id: &ObjectId,
    user_oid: &ObjectId,
) -> Result<u64, actix_web::HttpResponse> {
    let coll = get_collection::<Document>("events");
    coll.update_one(
        doc!{ "_id": id, "deleted": false, "locked": { "$ne": true } },
        doc!{
            "$addToSet": { "attendees": user_oid },
            "$inc": { "current_attendees": 1i32, "attendees_count": 1i32 },
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

async fn pull_attendee_and_dec(
    id: &ObjectId,
    user_oid: &ObjectId,
) -> Result<u64, actix_web::HttpResponse> {
    let coll = get_collection::<Document>("events");
    coll.update_one(
        doc!{ "_id": id, "deleted": false },
        doc!{
            "$pull": { "attendees": user_oid },
            "$inc": { "current_attendees": -1i32, "attendees_count": -1i32 },
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
impl AdmixResource for EventResource {
    fn new() -> Self { EventResource }

    fn resource_name(&self) -> &'static str { "Events" }
    fn base_path(&self) -> &'static str { "events" }
    fn collection_name(&self) -> &'static str { "events" }
    fn get_collection(&self) -> Collection<Document> { get_collection::<Document>("events") }
    fn clone_box(&self) -> Box<dyn AdmixResource> { Box::new(Self::new()) }
    fn menu_group(&self) -> Option<&'static str> { Some("Events") }
    fn menu(&self) -> &'static str { "Events" }

    fn allowed_roles(&self) -> Vec<String> {
        vec!["admin".to_string(), "superadmin".to_string()]
    }

    fn permit_keys(&self) -> Vec<&'static str> {
        vec![
            "user_id","title","description","location","address",
            "event_date","start_time","end_time","start_date","end_date",
            "image","status","approval_status","is_public","is_virtual","is_paid",
            "price","max_attendees","current_attendees","attendees_count","created_by",
            "tags","category","organizer_name","organizer_email","organizer_phone",
            "registration_url","meeting_link","requires_registration","registration_deadline",
            "age_restriction","dress_code","special_instructions","custom_fields",
            "event_type","registration_type","payment_type","qr_code","deleted","locked"
        ]
    }

    fn form_structure(&self) -> Option<Value> {
        Some(json!({
            "groups": [
                {
                    "title": "Core",
                    "fields": [
                        { "name":"title","field_type":"text","label":"Title","required":true },
                        { "name":"description","field_type":"editor","label":"Description","required":true },
                        { "name":"image","field_type":"text","label":"Image URL" },
                        { "name":"category","field_type":"select","label":"Category","required":true,
                          "options": EventOptions::category_options() },
                        { "name":"tags","field_type":"text","label":"Tags (comma-separated)" }
                    ]
                },
                {
                    "title": "Schedule",
                    "fields": [
                        { "name":"start_date","field_type":"datetime","label":"Start Date","required":true },
                        { "name":"end_date","field_type":"datetime","label":"End Date","required":true },
                        { "name":"start_time","field_type":"datetime","label":"Start Time","required":true },
                        { "name":"end_time","field_type":"datetime","label":"End Time","required":true },
                        { "name":"event_date","field_type":"datetime","label":"Event Date","required":true },
                        { "name":"registration_deadline","field_type":"datetime","label":"Registration Deadline" }
                    ]
                },
                {
                    "title": "Location & Mode",
                    "fields": [
                        { "name":"is_virtual","field_type":"boolean","label":"Virtual?","options": EventOptions::boolean_options() },
                        { "name":"is_public","field_type":"boolean","label":"Public?","options": EventOptions::boolean_options() },
                        { "name":"location","field_type":"text","label":"Location" },
                        { "name":"address","field_type":"text","label":"Address","required":true },
                        { "name":"meeting_link","field_type":"text","label":"Meeting Link" }
                    ]
                },
                {
                    "title": "Access & Registration",
                    "fields": [
                        { "name":"requires_registration","field_type":"boolean","label":"Requires Registration?","options": EventOptions::boolean_options() },
                        { "name":"registration_url","field_type":"text","label":"Registration URL" },
                        { "name":"registration_type","field_type":"select","label":"Registration Type","options": EventOptions::registration_type_options() },
                        { "name":"event_type","field_type":"select","label":"Event Type","options": EventOptions::event_type_options() }
                    ]
                },
                {
                    "title": "Pricing & Capacity",
                    "fields": [
                        { "name":"is_paid","field_type":"boolean","label":"Paid Event?","options": EventOptions::boolean_options() },
                        { "name":"price","field_type":"number","label":"Price" },
                        { "name":"payment_type","field_type":"select","label":"Payment Type","options": EventOptions::payment_type_options() },
                        { "name":"max_attendees","field_type":"number","label":"Max Attendees" }
                    ]
                },
                {
                    "title": "Organizer",
                    "fields": [
                        { "name":"organizer_name","field_type":"text","label":"Organizer Name" },
                        { "name":"organizer_email","field_type":"email","label":"Organizer Email" },
                        { "name":"organizer_phone","field_type":"text","label":"Organizer Phone" }
                    ]
                },
                {
                    "title": "Policies & Notes",
                    "fields": [
                        { "name":"age_restriction","field_type":"select","label":"Age Restriction","options": EventOptions::age_options() },
                        { "name":"dress_code","field_type":"select","label":"Dress Code","options": EventOptions::dress_code_options() },
                        { "name":"special_instructions","field_type":"editor_text","label":"Special Instructions" }
                    ]
                },
                {
                    "title": "Status & System",
                    "fields": [
                        { "name":"status","field_type":"select","label":"Event Status","options": EventOptions::status_options(),"required":true },
                        { "name":"approval_status","field_type":"select","label":"Approval Status","options": EventOptions::approval_options(),"required":true },
                        { "name":"locked","field_type":"boolean","label":"Locked?","options": EventOptions::boolean_options() },
                        { "name":"deleted","field_type":"boolean","label":"Deleted?","options": EventOptions::boolean_options() },
                        { "name":"qr_code","field_type":"text","label":"QR Code" },
                        { "name":"custom_fields","field_type":"editor_json","label":"Custom Fields (JSON)" },
                        { "name":"created_by","field_type":"text","label":"Created By" }
                    ]
                }
            ]
        }))
    }

    fn list_structure(&self) -> Option<Value> {
        Some(json!({
            "columns": [
                { "field":"title","label":"Title","sortable":true },
                { "field":"category","label":"Category","sortable":true },
                { "field":"status","label":"Status","sortable":true },
                { "field":"approval_status","label":"Approval","sortable":true },
                { "field":"is_public","label":"Public","sortable":true },
                { "field":"is_virtual","label":"Virtual","sortable":true },
                { "field":"is_paid","label":"Paid","sortable":true },
                { "field":"price","label":"Price","type":"number","sortable":true },
                { "field":"start_date","label":"Start","type":"datetime","sortable":true },
                { "field":"end_date","label":"End","type":"datetime","sortable":true },
                { "field":"attendees_count","label":"Attendees","type":"number","sortable":true },
                { "field":"locked","label":"Locked","sortable":true },
                { "field":"deleted","label":"Deleted","sortable":true },
                { "field":"created_at","label":"Created","type":"datetime","sortable":true },
                { "field":"updated_at","label":"Updated","type":"datetime","sortable":true }
            ],
            "actions": ["view","edit","delete"]
        }))
    }

    fn view_structure(&self) -> Option<Value> {
        Some(json!({
            "sections": [
                {
                    "title":"Core",
                    "fields":[
                        {"field":"title","label":"Title"},
                        {"field":"description","label":"Description"},
                        {"field":"category","label":"Category"},
                        {"field":"tags","label":"Tags"},
                        {"field":"image","label":"Image URL"}
                    ]
                },
                {
                    "title":"Schedule",
                    "fields":[
                        {"field":"start_date","label":"Start Date","type":"datetime"},
                        {"field":"end_date","label":"End Date","type":"datetime"},
                        {"field":"start_time","label":"Start Time","type":"datetime"},
                        {"field":"end_time","label":"End Time","type":"datetime"},
                        {"field":"event_date","label":"Event Date","type":"datetime"},
                        {"field":"registration_deadline","label":"Registration Deadline","type":"datetime"}
                    ]
                },
                {
                    "title":"Location & Mode",
                    "fields":[
                        {"field":"is_virtual","label":"Virtual","type":"boolean"},
                        {"field":"is_public","label":"Public","type":"boolean"},
                        {"field":"location","label":"Location"},
                        {"field":"address","label":"Address"},
                        {"field":"meeting_link","label":"Meeting Link"}
                    ]
                },
                {
                    "title":"Access & Registration",
                    "fields":[
                        {"field":"requires_registration","label":"Requires Registration","type":"boolean"},
                        {"field":"registration_url","label":"Registration URL"},
                        {"field":"registration_type","label":"Registration Type"},
                        {"field":"event_type","label":"Event Type"}
                    ]
                },
                {
                    "title":"Pricing & Capacity",
                    "fields":[
                        {"field":"is_paid","label":"Paid","type":"boolean"},
                        {"field":"price","label":"Price"},
                        {"field":"payment_type","label":"Payment Type"},
                        {"field":"max_attendees","label":"Max Attendees"},
                        {"field":"current_attendees","label":"Current Attendees"},
                        {"field":"attendees_count","label":"Total Attendees"}
                    ]
                },
                {
                    "title":"Organizer",
                    "fields":[
                        {"field":"organizer_name","label":"Organizer Name"},
                        {"field":"organizer_email","label":"Organizer Email"},
                        {"field":"organizer_phone","label":"Organizer Phone"}
                    ]
                },
                {
                    "title":"Policies & Notes",
                    "fields":[
                        {"field":"age_restriction","label":"Age Restriction"},
                        {"field":"dress_code","label":"Dress Code"},
                        {"field":"special_instructions","label":"Special Instructions"}
                    ]
                },
                {
                    "title":"System",
                    "fields":[
                        {"field":"_id","label":"Event ID"},
                        {"field":"status","label":"Status"},
                        {"field":"approval_status","label":"Approval Status"},
                        {"field":"qr_code","label":"QR Code"},
                        {"field":"locked","label":"Locked","type":"boolean"},
                        {"field":"deleted","label":"Deleted","type":"boolean"},
                        {"field":"created_by","label":"Created By"},
                        {"field":"created_at","label":"Created At","type":"datetime"},
                        {"field":"updated_at","label":"Updated At","type":"datetime"}
                    ]
                }
            ]
        }))
    }

    fn filters(&self) -> Option<Value> {
        Some(json!({
            "filters": [
                { "field":"title","type":"text","label":"Title" },
                { "field":"category","type":"select","label":"Category","options": EventOptions::category_options() },
                { "field":"status","type":"select","label":"Status","options": EventOptions::status_options() },
                { "field":"approval_status","type":"select","label":"Approval","options": EventOptions::approval_options() },
                { "field":"is_public","type":"boolean","label":"Public","options": EventOptions::boolean_options() },
                { "field":"is_virtual","type":"boolean","label":"Virtual","options": EventOptions::boolean_options() },
                { "field":"is_paid","type":"boolean","label":"Paid","options": EventOptions::boolean_options() },
                { "field":"requires_registration","type":"boolean","label":"Requires Registration","options": EventOptions::boolean_options() },
                { "field":"event_type","type":"select","label":"Event Type","options": EventOptions::event_type_options() },
                { "field":"registration_type","type":"select","label":"Registration Type","options": EventOptions::registration_type_options() },
                { "field":"payment_type","type":"select","label":"Payment Type","options": EventOptions::payment_type_options() },
                { "field":"price","type":"number_range","label":"Price Range" },
                { "field":"max_attendees","type":"number_range","label":"Max Attendees Range" },
                { "field":"current_attendees","type":"number_range","label":"Current Attendees Range" },
                { "field":"start_date","type":"date_range","label":"Start Date" },
                { "field":"end_date","type":"date_range","label":"End Date" },
                { "field":"created_at","type":"date_range","label":"Created Date" },
                { "field":"updated_at","type":"date_range","label":"Updated Date" },
                { "field":"deleted","type":"boolean","label":"Deleted","options": EventOptions::boolean_options() },
                { "field":"locked","type":"boolean","label":"Locked","options": EventOptions::boolean_options() }
            ]
        }))
    }

    fn custom_actions(&self) -> Vec<adminx::actions::CustomAction> {
        vec![
            adminx::actions::CustomAction {
                name: "set_status",
                method: "POST",
                handler: |req, _path, body| {
                    let id_str = req.match_info().get("id").unwrap_or("");
                    let Some(id) = parse_oid_opt(id_str) else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_object_id"})) });
                    };
                    Box::pin(async move {
                        let status = body.get("status").and_then(|v| v.as_str()).unwrap_or("Initial").to_string();
                        match update_one_by_id(doc!{ "status": status }, &id).await {
                            Ok(modified) => actix_web::HttpResponse::Ok().json(json!({ "success": true, "modified": modified })),
                            Err(resp) => resp
                        }
                    })
                },
                ui: None,
            },
            adminx::actions::CustomAction {
                name: "set_approval",
                method: "POST",
                handler: |req, _path, body| {
                    let id_str = req.match_info().get("id").unwrap_or("");
                    let Some(id) = parse_oid_opt(id_str) else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_object_id"})) });
                    };
                    Box::pin(async move {
                        let approval = body.get("approval_status").and_then(|v| v.as_str()).unwrap_or("Initial").to_string();
                        match update_one_by_id(doc!{ "approval_status": approval }, &id).await {
                            Ok(modified) => actix_web::HttpResponse::Ok().json(json!({ "success": true, "modified": modified })),
                            Err(resp) => resp
                        }
                    })
                },
                ui: None,
            },
            adminx::actions::CustomAction {
                name: "toggle_lock",
                method: "POST",
                handler: |req, _path, body| {
                    let id_str = req.match_info().get("id").unwrap_or("");
                    let Some(id) = parse_oid_opt(id_str) else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_object_id"})) });
                    };
                    let on = body.get("locked").and_then(|v| v.as_bool()).unwrap_or(true);
                    Box::pin(async move {
                        match update_one_by_id(doc!{ "locked": on }, &id).await {
                            Ok(modified) => actix_web::HttpResponse::Ok().json(json!({ "success": true, "locked": on, "modified": modified })),
                            Err(resp) => resp
                        }
                    })
                },
                ui: None,
            },
            adminx::actions::CustomAction {
                name: "toggle_public",
                method: "POST",
                handler: |req, _path, body| {
                    let id_str = req.match_info().get("id").unwrap_or("");
                    let Some(id) = parse_oid_opt(id_str) else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_object_id"})) });
                    };
                    let on = body.get("is_public").and_then(|v| v.as_bool()).unwrap_or(true);
                    Box::pin(async move {
                        match update_one_by_id(doc!{ "is_public": on }, &id).await {
                            Ok(modified) => actix_web::HttpResponse::Ok().json(json!({ "success": true, "is_public": on, "modified": modified })),
                            Err(resp) => resp
                        }
                    })
                },
                ui: None,
            },
            adminx::actions::CustomAction {
                name: "toggle_virtual",
                method: "POST",
                handler: |req, _path, body| {
                    let id_str = req.match_info().get("id").unwrap_or("");
                    let Some(id) = parse_oid_opt(id_str) else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_object_id"})) });
                    };
                    let on = body.get("is_virtual").and_then(|v| v.as_bool()).unwrap_or(true);
                    Box::pin(async move {
                        match update_one_by_id(doc!{ "is_virtual": on }, &id).await {
                            Ok(modified) => actix_web::HttpResponse::Ok().json(json!({ "success": true, "is_virtual": on, "modified": modified })),
                            Err(resp) => resp
                        }
                    })
                },
                ui: None,
            },
            adminx::actions::CustomAction {
                name: "toggle_paid",
                method: "POST",
                handler: |req, _path, body| {
                    let id_str = req.match_info().get("id").unwrap_or("");
                    let Some(id) = parse_oid_opt(id_str) else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_object_id"})) });
                    };
                    let on = body.get("is_paid").and_then(|v| v.as_bool()).unwrap_or(true);
                    Box::pin(async move {
                        match update_one_by_id(doc!{ "is_paid": on }, &id).await {
                            Ok(modified) => actix_web::HttpResponse::Ok().json(json!({ "success": true, "is_paid": on, "modified": modified })),
                            Err(resp) => resp
                        }
                    })
                },
                ui: None,
            },
            adminx::actions::CustomAction {
                name: "generate_qr",
                method: "POST",
                handler: |req, _path, _body| {
                    let id_str = req.match_info().get("id").unwrap_or("");
                    let Some(id) = parse_oid_opt(id_str) else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_object_id"})) });
                    };
                    let qr = format!("xard:event:{}", id.to_hex());
                    Box::pin(async move {
                        match update_one_by_id(doc!{ "qr_code": qr.clone() }, &id).await {
                            Ok(modified) => actix_web::HttpResponse::Ok().json(json!({ "success": true, "qr_code": qr, "modified": modified })),
                            Err(resp) => resp
                        }
                    })
                },
                ui: None,
            },
            adminx::actions::CustomAction {
                name: "add_attendee",
                method: "POST",
                handler: |req, _path, body| {
                    let id_str = req.match_info().get("id").unwrap_or("");
                    let Some(event_id) = parse_oid_opt(id_str) else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_object_id"})) });
                    };
                    let user_id = body.get("user_id").and_then(|v| v.as_str()).unwrap_or("");
                    let Some(user_oid) = parse_oid_opt(user_id) else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_user_id"})) });
                    };
                    Box::pin(async move {
                        match add_to_set_and_inc(&event_id, &user_oid).await {
                            Ok(modified) => actix_web::HttpResponse::Ok().json(json!({ "success": true, "modified": modified })),
                            Err(resp) => resp
                        }
                    })
                },
                ui: None,
            },
            adminx::actions::CustomAction {
                name: "remove_attendee",
                method: "POST",
                handler: |req, _path, body| {
                    let id_str = req.match_info().get("id").unwrap_or("");
                    let Some(event_id) = parse_oid_opt(id_str) else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_object_id"})) });
                    };
                    let user_id = body.get("user_id").and_then(|v| v.as_str()).unwrap_or("");
                    let Some(user_oid) = parse_oid_opt(user_id) else {
                        return Box::pin(async { actix_web::HttpResponse::BadRequest().json(json!({"error":"invalid_user_id"})) });
                    };
                    Box::pin(async move {
                        match pull_attendee_and_dec(&event_id, &user_oid).await {
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
