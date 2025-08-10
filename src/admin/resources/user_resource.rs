// src/admin/resources/user_resource.rs
use crate::dbs::mongo::get_collection;
use adminx::AdmixResource;
use async_trait::async_trait;
use mongodb::{Collection, bson::Document};
use serde_json::{json, Value};


#[derive(Debug, Clone)]
pub struct UserResource;

#[async_trait]
impl AdmixResource for UserResource {
    // ===========================
    // REQUIRED IMPLEMENTATIONS
    // ===========================
    fn new() -> Self {
        UserResource
    }

    fn resource_name(&self) -> &'static str {
        "Users"
    }

    fn base_path(&self) -> &'static str {
        "users"
    }

    fn collection_name(&self) -> &'static str {
        "users"
    }

    fn get_collection(&self) -> Collection<Document> {
        get_collection::<Document>("users")
    }

    fn clone_box(&self) -> Box<dyn AdmixResource> {
        Box::new(Self::new())
    }

    fn menu_group(&self) -> Option<&'static str> {
        Some("Management")
    }

    fn menu(&self) -> &'static str {
        "Users"
    }

    // ===========================
    // CONFIGURATION OVERRIDES
    // ===========================
    fn allowed_roles(&self) -> Vec<String> {
        vec!["admin".to_string(), "superadmin".to_string()]
    }

    fn permit_keys(&self) -> Vec<&'static str> {
        vec!["name", "email"]
    }

    fn permit_filter_keys(&self) -> Vec<&'static str> {
        vec!["name", "email"]
    }

    // ===========================
    // UI STRUCTURE OVERRIDES (Optional)
    // ===========================
    fn form_structure(&self) -> Option<Value> {
        // Using manual form structure for better control
        Some(json!({
            "groups": [
                {
                    "title": "User Details",
                    "fields": [
                        {
                            "name": "name",
                            "field_type": "text",
                            "label": "Full Name",
                            "value": "",
                            "options": null
                        },
                        {
                            "name": "email",
                            "field_type": "email", 
                            "label": "Email Address",
                            "value": "",
                            "options": null
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
                    "field": "name",
                    "label": "Name",
                    "sortable": true
                },
                {
                    "field": "email", 
                    "label": "Email",
                    "sortable": true
                },
                {
                    "field": "created_at",
                    "label": "Created At",
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
                    "title": "User Information",
                    "fields": [
                        {
                            "field": "name",
                            "label": "Name"
                        },
                        {
                            "field": "email",
                            "label": "Email"
                        }
                    ]
                },
                {
                    "title": "System Information",
                    "fields": [
                        {
                            "field": "_id",
                            "label": "User ID"
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
                    "field": "name",
                    "type": "text",
                    "label": "Name",
                    "placeholder": "Search by name..."
                },
                {
                    "field": "email",
                    "type": "text", 
                    "label": "Email",
                    "placeholder": "Search by email..."
                },
                
                // ===========================
                // SELECT/DROPDOWN FILTERS
                // ===========================
                // {
                //     "field": "status",
                //     "type": "select",
                //     "label": "Status",
                //     "options": [
                //         {"value": "", "label": "All Statuses"},
                //         {"value": "active", "label": "Active"},
                //         {"value": "inactive", "label": "Inactive"},
                //         {"value": "suspended", "label": "Suspended"}
                //     ]
                // },
                
                // ===========================
                // DATE RANGE FILTERS
                // ===========================
                {
                    "field": "created_at",
                    "type": "date_range",
                    "label": "Created Date",
                    "placeholder": "Select date range..."
                },
                {
                    "field": "updated_at",
                    "type": "date_range",
                    "label": "Updated Date",
                    "placeholder": "Select date range..."
                },
                
                // ===========================
                // BOOLEAN/CHECKBOX FILTERS
                // ===========================
                // {
                //     "field": "deleted",
                //     "type": "boolean",
                //     "label": "Email Verified",
                //     "options": [
                //         {"value": "", "label": "All"},
                //         {"value": "true", "label": "Verified"},
                //         {"value": "false", "label": "Not Verified"}
                //     ]
                // },
                
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