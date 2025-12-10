// src/admin/resources/notification_resource.rs - Fixed Version
use crate::db::mongo::get_collection;
use adminx::AdmixResource;
use async_trait::async_trait;
use mongodb::{Collection, bson::Document};
use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct NotificationResource;

#[async_trait]
impl AdmixResource for NotificationResource {
    // ===========================
    // REQUIRED IMPLEMENTATIONS
    // ===========================
    fn new() -> Self {
        NotificationResource
    }

    fn resource_name(&self) -> &'static str {
        "notifications"
    }

    fn base_path(&self) -> &'static str {
        "notifications"
    }

    fn collection_name(&self) -> &'static str {
        "notifications"
    }

    fn get_collection(&self) -> Collection<Document> {
        get_collection::<Document>("notifications")
    }

    fn clone_box(&self) -> Box<dyn AdmixResource> {
        Box::new(Self::new())
    }

    fn menu_group(&self) -> Option<&'static str> {
        Some("Master")
    }

    fn menu(&self) -> &'static str {
        "Notifications"
    }

    fn allowed_roles(&self) -> Vec<String> {
        vec!["admin".to_string(), "superadmin".to_string()]
    }

    // FIXED: Make permit_keys match the actual fields you want to use
    fn permit_keys(&self) -> Vec<&'static str> {
        vec!["title", "message"]
    }

    // ===========================
    // UI STRUCTURE OVERRIDES (Optional)
    // ===========================
    fn form_structure(&self) -> Option<Value> {
        Some(json!({
            "groups": [
                {
                    "title": "Notification Details",
                    "fields": [
                        {
                            "name": "title",
                            "field_type": "text",
                            "label": "Title",
                            "value": "",
                            "required": true,  // FIXED: Added required field
                            "options": null
                        },
                        {
                            "name": "message",
                            "field_type": "textarea", 
                            "label": "Description",
                            "value": "",
                            "required": true,  // FIXED: Added required field
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
                    "field": "title",
                    "label": "Title",
                    "sortable": true
                },
                {
                    "field": "message", 
                    "label": "Message",
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
                    "title": "Notification Information",
                    "fields": [
                        {
                            "field": "title",
                            "label": "Title"
                        },
                        {
                            "field": "message",
                            "label": "Description"
                        }
                    ]
                },
                {
                    "title": "System Information",
                    "fields": [
                        {
                            "field": "id",  // FIXED: Use "id" instead of "_id" for templates
                            "label": "Notification ID"
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
                // TEXT FILTERS
                // ===========================
                {
                    "field": "title",
                    "type": "text",
                    "label": "Title",
                    "placeholder": "Search by title..."
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
                }
            ]
        }))
    }
}