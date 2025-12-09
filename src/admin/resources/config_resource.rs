// /test/src/admin/resources/notification_resource.rs
use crate::dbs::mongo::get_collection;
use adminx::AdmixResource;
use async_trait::async_trait;
use mongodb::{Collection, bson::Document};
use serde_json::{json, Value};
use crate::models::config_model::{ConfigStatus, ConfigDataType};
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
                
                // ===========================
                // NUMBER RANGE FILTERS
                // ===========================
                // {
                //     "field": "age",
                //     "type": "number_range",
                //     "label": "Age Range",
                //     "min_placeholder": "Min age",
                //     "max_placeholder": "Max age"
                // },
                
                // ===========================
                // MULTI-SELECT FILTERS
                // ===========================
                // {
                //     "field": "tags",
                //     "type": "multi_select",
                //     "label": "Tags",
                //     "options": [
                //         {"value": "premium", "label": "Premium User"},
                //         {"value": "beta", "label": "Beta Tester"},
                //         {"value": "vip", "label": "VIP Member"}
                //     ]
                // }
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
                handler: |req, _path, _body| {
                    let user_id = req.match_info().get("id").unwrap_or("unknown").to_string();

                    Box::pin(async move {
                        tracing::info!("Banning user: {}", user_id);
                        
                        // TODO: Add your actual ban logic here
                        // For example, update a status field in the database
                        
                        actix_web::HttpResponse::Ok().json(serde_json::json!({
                            "success": true,
                            "message": format!("User {} has been banned", user_id)
                        }))
                    })
                },
            },
        ]
    }

    // ===========================
    // CRUD OPERATIONS - Using defaults with optional overrides
    // ===========================
    
    // All CRUD operations (list, get, create, update, delete) will use 
    // the default implementations from the trait automatically!
    
    // Only override if you need custom behavior, for example:
    
    /*
    fn create(&self, req: &HttpRequest, payload: Value) -> futures::future::BoxFuture<'static, HttpResponse> {
        use futures::FutureExt;
        
        // Example: Custom validation
        if let Value::Object(ref map) = payload {
            if let Some(email) = map.get("email").and_then(|v| v.as_str()) {
                if !email.contains('@') {
                    return Box::pin(async move {
                        HttpResponse::BadRequest().json(json!({
                            "error": "Invalid email format"
                        }))
                    });
                }
            }
        }
        
        // If validation passes, call the default implementation
        // You would need to manually call the default trait logic here
        // or restructure this to work with the trait's default implementation
        
        Box::pin(async move {
            HttpResponse::Created().json(json!({
                "success": true,
                "message": "User created with custom validation"
            }))
        })
    }
    */
}