// src/admin/resources/image_resource.rs
use crate::db::mongo::get_collection;
use adminx::{AdmixResource, error::AdminxError};
use async_trait::async_trait;
use crate::models::picture::PictureStatus;
use mongodb::{Collection, bson::Document};
use serde_json::{json, Value};
use crate::enums::{
    common_enums::{
        StatusEnum,
    }
};
use futures::future::BoxFuture;
use std::collections::HashMap;
use convert_case::{Case, Casing};
use strum::IntoEnumIterator;
use crate::utilities::s3_utility::{upload_image_to_s3};

pub struct PictureOptions;

impl PictureOptions {
    pub fn statuses_options() -> Vec<Value> {
        let mut options = vec![];
        for variant in PictureStatus::iter() {
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

#[derive(Debug, Clone)]
pub struct PictureResource;

#[async_trait]
impl AdmixResource for PictureResource {
    // ===========================
    // REQUIRED IMPLEMENTATIONS
    // ===========================
    fn new() -> Self {
        PictureResource
    }

    fn resource_name(&self) -> &'static str {
        "Pictures"
    }

    fn base_path(&self) -> &'static str {
        "pictures"
    }

    fn collection_name(&self) -> &'static str {
        "pictures"
    }

    fn get_collection(&self) -> Collection<Document> {
        get_collection::<Document>("pictures")
    }

    fn clone_box(&self) -> Box<dyn AdmixResource> {
        Box::new(Self::new())
    }

    fn menu_group(&self) -> Option<&'static str> {
        Some("Master")
    }

    fn menu(&self) -> &'static str {
        "Pictures"
    }

    // ===========================
    // CONFIGURATION OVERRIDES
    // ===========================
    fn allowed_roles(&self) -> Vec<String> {
        vec!["admin".to_string(), "superadmin".to_string()]
    }

    fn supports_file_upload(&self) -> bool {
        true
    }
    
    fn max_file_size(&self) -> usize {
        5 * 1024 * 1024 // 5MB for images
    }
    
    fn allowed_file_extensions(&self) -> Vec<&'static str> {
        vec!["jpg", "jpeg", "png", "gif", "webp", "bmp", "pdf"]
    }
    
    fn permit_keys(&self) -> Vec<&'static str> {
        vec!["title", "image_file", "status"]
    }
    
    // FIXED: Remove 'async' keyword and correct method signature
    fn process_file_upload(&self, field_name: &str, file_data: &[u8], filename: &str) -> BoxFuture<'static, Result<HashMap<String, String>, AdminxError>> {
        let filename = filename.to_string();
        let field_name = field_name.to_string();
        let file_data = file_data.to_vec();
        let data_size = file_data.len();
        
        Box::pin(async move {
            tracing::info!("Processing file upload for field: {}, filename: {}, size: {} bytes", 
                          field_name, filename, data_size);
            
            // Generate unique filename to avoid conflicts
            let timestamp = chrono::Utc::now().timestamp();
            let file_extension = filename.split('.').last().unwrap_or("jpg");
            let unique_filename = format!("images/{}_{}.{}", timestamp, field_name, file_extension);
            
            // Use your actual S3 upload utility
            match upload_image_to_s3(unique_filename.clone(), file_data).await {
                Ok(public_url) => {
                    let mut urls = HashMap::new();
                    urls.insert("image_url".to_string(), public_url);
                    
                    tracing::info!("File uploaded successfully to S3: {}", unique_filename);
                    Ok(urls)
                }
                Err(e) => {
                    tracing::error!("S3 upload failed for {}: {}", unique_filename, e);
                    Err(AdminxError::InternalError)
                }
            }
        })
    }

    

    // ===========================
    // UI STRUCTURE OVERRIDES
    // ===========================
    fn form_structure(&self) -> Option<Value> {
        Some(json!({
            "groups": [
                {
                    "title": "Image Details",
                    "fields": [
                        {
                            "name": "title",
                            "field_type": "text",
                            "label": "Title",
                            "value": "",
                            "required": true,
                        },
                        {
                            "name": "image_file",
                            "field_type": "file",
                            "label": "Upload Image",
                            "accept": "image/*",
                            "required": true,
                            "help_text": "Upload an image file (JPG, PNG, GIF, WebP). Maximum size: 5MB."
                        },
                        {
                            "name": "deleted",
                            "field_type": "boolean", 
                            "label": "Mark as Deleted",
                            "value": "false",
                            "required": false,
                            "options": PictureOptions::boolean_options(),
                            "help_text": "Mark this image as deleted (soft delete)"
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
                    "field": "url", 
                    "label": "Image URL",
                    "sortable": false,
                    "type": "url"
                },
                {
                    "field": "status",
                    "label": "Status",
                    "sortable": true,
                    "type": "badge"
                },
                {
                    "field": "deleted",
                    "label": "Deleted",
                    "sortable": true,
                    "type": "boolean"
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
                    "title": "Image Information",
                    "fields": [
                        {
                            "field": "title",
                            "label": "Title"
                        },
                        {
                            "field": "url",
                            "label": "Image URL",
                            "type": "url"
                        },
                        {
                            "field": "status",
                            "label": "Status",
                            "type": "badge"
                        },
                        {
                            "field": "deleted",
                            "label": "Deleted",
                            "type": "boolean"
                        }
                    ]
                },
                {
                    "title": "System Information",
                    "fields": [
                        {
                            "field": "_id",
                            "label": "Image ID"
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
            "title": "Image Filters",
            "filters": [
                {
                    "field": "title",
                    "type": "text",
                    "label": "Title",
                    "placeholder": "Search by title..."
                },
                {
                    "field": "status",
                    "type": "select",
                    "label": "Status",
                    "options": PictureOptions::statuses_options(),
                },
                {
                    "field": "deleted",
                    "type": "boolean",
                    "label": "Show Deleted",
                    "options": PictureOptions::boolean_options(),
                },
                {
                    "field": "created_at",
                    "type": "date_range",
                    "label": "Created Date"
                }
            ]
        }))
    }
}