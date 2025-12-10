// /test/src/admin/resources/notification_resource.rs
use actix_web::HttpResponse;
use crate::db::mongo::get_collection;
use adminx::AdmixResource;
use async_trait::async_trait;
use mongodb::{Collection, bson::Document};
use serde_json::{json, Value};
use crate::models::config::{ConfigStatus, ConfigDataType};
use convert_case::{Casing, Case};
use strum::IntoEnumIterator;

#[derive(Debug, Clone)]
pub struct ConfigResource;

pub struct ConfigOptions;

impl ConfigOptions {
    pub fn statuses_options() -> Vec<Value> {
        let mut options = vec![];
        for variant in ConfigStatus::iter() {
            let value = serde_json::to_string(&variant).unwrap().replace('"', "");
            let label = value.to_case(Case::Title);
            options.push(json!({ "value": value, "label": label }));
        }
        options
    }

    pub fn data_types_options() -> Vec<Value>{
        let mut options = vec![];
        for variant in ConfigDataType::iter() {
            let value = serde_json::to_string(&variant).unwrap().replace('"', "");
            let label = value.to_case(Case::Title);
            options.push(json!({ "value": value, "label": label }));
        }
        options
    }


    pub fn boolean_options() -> Vec<Value> {
        vec![
            json!({ "value": "true",  "label": "True"  }),
            json!({ "value": "false", "label": "False" }),
        ]
    }
}


#[async_trait]
impl AdmixResource for ConfigResource {
    // ===========================
    // REQUIRED IMPLEMENTATIONS
    // ===========================
    fn new() -> Self {
        ConfigResource
    }

    fn resource_name(&self) -> &'static str {
        "configs"
    }

    fn base_path(&self) -> &'static str {
        "configs"
    }

    fn collection_name(&self) -> &'static str {
        "configs"
    }

    fn get_collection(&self) -> Collection<Document> {
        get_collection::<Document>("configs")
    }

    fn clone_box(&self) -> Box<dyn AdmixResource> {
        Box::new(Self::new())
    }

    fn menu_group(&self) -> Option<&'static str> {
        Some("Settings")
    }

    fn menu(&self) -> &'static str {
        "Configs"
    }

    // ===========================
    // CONFIGURATION OVERRIDES
    // ===========================
    fn allowed_roles(&self) -> Vec<String> {
        vec!["admin".to_string(), "superadmin".to_string()]
    }


    fn permit_keys(&self) -> Vec<&'static str> {
        vec!["key", "data", "data_type", "status", "deleted"]
    }

    // ===========================
    // UI STRUCTURE OVERRIDES (Optional)
    // ===========================

    // {
    //   "field_type": "boolean",      // Yes/No radio buttons
    //   "field_type": "editor_text",  // Simple text editor
    //   "field_type": "editor_html",  // HTML-only editor  
    //   "field_type": "editor_json",  // JSON-only editor
    //   "field_type": "editor"        // Combined Text/HTML/JSON editor
    // }


    fn form_structure(&self) -> Option<Value> {
        // Using manual form structure for better control
        Some(json!({
            "groups": [
                {
                    "title": "Details",
                    "fields": [
                        {
                            "name": "key",
                            "field_type": "text",
                            "label": "key",
                            "value": "",
                            "required": true,
                            "options": null
                        },
                        {
                            "name": "data",
                            "field_type": "editor", 
                            "label": "data",
                            "value": "",
                            "required": true,
                            "options": null
                        },
                        {
                            "name": "data_type",
                            "field_type": "select", 
                            "label": "data_type",
                            "value": "",
                            "required": true,
                            "options": ConfigOptions::data_types_options()
                        },
                        {
                            "name": "status",
                            "field_type": "select", 
                            "label": "status",
                            "value": "",
                            "required": true,
                            "options": ConfigOptions::statuses_options()
                        },
                        {
                            "name": "deleted",
                            "field_type": "boolean", 
                            "label": "deleted",
                            "value": "",
                            "required": true,
                            "options": ConfigOptions::boolean_options()
                        }
                    ]
                }
            ]
        }))
    }

    fn list_structure(&self) -> Option<Value> {
        Some(json!({
            "columns": [
                {
                    "field": "key",
                    "label": "key",
                    "sortable": true
                },
                {
                    "field": "data_type", 
                    "label": "data_type",
                    "sortable": true
                },
                {
                    "field": "status", 
                    "label": "status",
                    "sortable": true
                },
                {
                    "field": "deleted", 
                    "label": "deleted",
                    "sortable": true
                },
                {
                    "field": "created_at",
                    "label": "Created At",
                    "type": "datetime",
                    "sortable": true
                },
                {
                    "field": "updated_at",
                    "label": "Updated At",
                    "type": "datetime",
                    "sortable": true
                }
            ],
            "actions": ["view", "edit", "delete"]
        }))
    }

    fn view_structure(&self) -> Option<Value> {
        Some(json!({
            "sections": [
                {
                    "title": "Details",
                    "fields": [
                        {
                            "field": "key",
                            "label": "key"
                        },
                        {
                            "field": "data_type",
                            "label": "data_type"
                        }
                    ]
                },
                {
                    "title": "More Details",
                    "fields": [
                        {
                            "field": "_id",
                            "label": "id"
                        },
                        {
                            "field": "status",
                            "label": "status",
                            "type": "boolean"
                        },
                        {
                            "field": "data",
                            "label": "data",
                            "type": "datetime"
                        },
                        
                        {
                            "field": "deleted",
                            "label": "deleted",
                            "type": "boolean"
                        },
                        {
                            "field": "created_at",
                            "label": "Created At",
                            "type": "datetime"
                        },
                        {
                            "field": "updated_at", 
                            "label": "Updated At",
                            "type": "datetime"
                        }
                    ]
                }
            ]
        }))
    }

    fn filters(&self) -> Option<Value> {
        Some(json!({
            "filters": [
                // ===========================
                // TEXT FILTERS (like your example)
                // ===========================
                {
                    "field": "key",
                    "type": "text",
                    "label": "key",
                    "placeholder": ""
                },
                {
                    "field": "status",
                    "type": "select", 
                    "label": "status",
                    "placeholder": "",
                    "options": ConfigOptions::statuses_options()
                },
                
                // ===========================
                // DATE RANGE FILTERS
                // ===========================
                {
                    "field": "created_at",
                    "type": "date_range",
                    "label": "Created Date",
                    "placeholder": ""
                },
                {
                    "field": "updated_at",
                    "type": "date_range",
                    "label": "Updated Date",
                    "placeholder": ""
                },
                
                // ===========================
                // BOOLEAN/CHECKBOX FILTERS
                // ===========================
                {
                    "field": "deleted",
                    "type": "boolean",
                    "label": "Deleted",
                    "options": ConfigOptions::boolean_options()
                },
            ]
        }))
    }

    // ===========================
    // CUSTOM ACTIONS (Optional)
    // ===========================
    fn custom_actions(&self) -> Vec<adminx::actions::CustomAction> {
        vec![
            adminx::actions::CustomAction {
                name: "ban",
                method: "POST",
                handler: |req, _path, body| {
                    let user_id = req
                        .match_info()
                        .get("id")
                        .unwrap_or("unknown")
                        .to_string(); // own it

                    // Extract values from `body` but convert them to owned types
                    let reason: String = body
                        .get("reason")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();

                    let duration: i64 = body
                        .get("duration")
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0);

                    let ban_type: String = body
                        .get("ban_type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("temporary")
                        .to_string();

                    // Now we move only owned data into the future
                    Box::pin(async move {
                        tracing::info!(
                            "Banning user {}: reason={}, duration={}, type={}",
                            user_id,
                            reason,
                            duration,
                            ban_type
                        );

                        // TODO: add real ban logic (DB update, etc.)

                        HttpResponse::Ok().json(json!({
                            "success": true,
                            "message": format!("User {} has been banned", user_id),
                            "user_id": user_id,
                            "reason": reason,
                            "duration": duration,
                            "ban_type": ban_type,
                        }))
                    })
                },
                ui: Some(adminx::actions::ActionUi {
                    label: Some("Ban User".into()),
                    confirm: Some("Are you sure you want to ban this user?".into()),
                    fields: Some(vec![
                        adminx::actions::ActionField {
                            name: "reason".into(),
                            label: Some("Reason".into()),
                            field_type: "text".into(),
                            required: Some(true),
                            options: None,
                        },
                        adminx::actions::ActionField {
                            name: "duration".into(),
                            label: Some("Duration (days)".into()),
                            field_type: "number".into(),
                            required: Some(false),
                            options: None,
                        },
                        adminx::actions::ActionField {
                            name: "ban_type".into(),
                            label: Some("Ban Type".into()),
                            field_type: "select".into(),
                            required: Some(true),
                            options: Some(vec![
                                json!("temporary"),
                                json!("permanent"),
                            ]),
                        },
                    ]),
                }),
            },
        ]
    }

}