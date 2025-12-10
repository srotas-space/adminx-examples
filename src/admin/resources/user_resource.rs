// src/admin/resources/user_resource.rs
use crate::db::mongo::get_collection;
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
        Some("Master")
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
        vec!["first_name", "email", "status", "deleted"]
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
                    "field": "first_name",
                    "label": "First Name",
                    "sortable": true
                },
                {
                    "field": "email", 
                    "label": "Email",
                    "sortable": true
                },
                {
                    "field": "status", 
                    "label": "Status",
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
                            "field": "first_name",
                            "label": "First Name"
                        },
                        {
                            "field": "last_name",
                            "label": "Last Name"
                        },
                        {
                            "field": "email",
                            "label": "Email"
                        },
                        {
                            "field": "phone_number",
                            "label": "Phone number"
                        },
                        {
                            "field": "status",
                            "label": "Status"
                        },
                        {
                            "field": "created_at",
                            "label": "Created AT"
                        },
                        {
                            "field": "updated_at",
                            "label": "Update AT"
                        },
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
                    "field": "phone_number",
                    "type": "text",
                    "label": "Phone number",
                    "placeholder": "Search by Phone..."
                },
                {
                    "field": "email",
                    "type": "text", 
                    "label": "Email",
                    "placeholder": "Search by email..."
                },
                
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
            ]
        }))
    }
}