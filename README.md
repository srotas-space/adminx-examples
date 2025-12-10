# AdminX Actix + MongoDB Starter (Xard – Smart Contact Manager)

This repo is a **practical starter template** showing how to use  
**[AdminX](https://crates.io/crates/adminx)** with:

- 🦀 **Rust**
- 🌐 **Actix-Web**
- 🍃 **MongoDB**
- 🧰 **AdminX** (admin panel framework)
- 🗂️ **S3 (file uploads)**
- 💾 **Redis (optional)**

It powers a small “Smart Contact Manager” called **Xard**, with resources like:

- Users
- Contacts
- Events
- Event Attendees
- Pictures (with uploads)
- Notifications
- Config

---

## ✨ What this example gives you

- Ready-to-run **Actix-Web server** with AdminX mounted
- Opinionated project layout (`db`, `models`, `admin`, etc.)
- Central **AdminX initializer** that:
  - connects to MongoDB
  - registers resources
  - wires session middleware
  - exposes `/adminx` routes
- Concrete **AdminX resources**:
  - `UserResource` (simple CRUD)
  - `PictureResource` (CRUD + file uploads to S3)
- Working **form/list structures** using AdminX JSON config
- How to plug AdminX into an existing Actix app

---

## 📦 Dependencies

The important bits from `Cargo.toml` (only the relevant section):

```toml
[dependencies]
actix-web = "4"
actix = "0.13"
actix-cors = "0.6"
actix-service = "2.0"
mongodb = { version = "2.4", features = ["tokio-runtime"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
schemars = { version = "0.8", features = ["derive"] }
async-trait = "0.1.89"
actix-session = "0.10.1"

adminx = "0.2.4"
convert_case = "0.8.0"
strum = "0.26"
strum_macros = "0.26"
tracing = "0.1"
tracing-subscriber = "0.3.19"
dotenv = "0.15"
```

You can of course add Redis, S3, image, etc. as in this starter.

---

## 🗂 Project Structure

High-level layout used in this example:

```txt
src/
  admin/
    initializer.rs        # AdminX bootstrap & resource registration
    resources/
      user_resource.rs
      picture_resource.rs
      event_resource.rs
      event_attendee_resource.rs
      notification_resource.rs
      config_resource.rs
  db/
    mongo.rs              # Mongo client + get_collection helper
    redis_service.rs      # (optional)
  models/
    user.rs
    picture.rs
    event.rs
    contact.rs
    notification.rs
    config.rs
    ...
  config/
    env_vars.rs
    constants.rs
  libs/
    s3_utility.rs
    bason_utility.rs
    general_library.rs
  utilities/
    validators/
    custom_regex.rs
    ...
  enums/
    common_enums.rs
    request_enums.rs
  main.rs
```

---

## 🔧 MongoDB Setup

A minimal Mongo helper (your actual file has more):

```rust
// src/db/mongo.rs
use mongodb::{Client, Database};
use once_cell::sync::OnceCell;
use std::env;

static DB_INSTANCE: OnceCell<Database> = OnceCell::new();

pub async fn init_mongo_client() -> Database {
    let uri = env::var("MONGO_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let db_name = env::var("MONGO_DB")
        .unwrap_or_else(|_| "xard".to_string());

    let client = Client::with_uri_str(&uri)
        .await
        .expect("Failed to connect to MongoDB");
    let db = client.database(&db_name);

    DB_INSTANCE.set(db.clone()).ok();
    db
}

pub fn get_collection<T>(name: &str) -> mongodb::Collection<T> {
    let db = DB_INSTANCE
        .get()
        .expect("MongoDB not initialized. Call init_mongo_client() first.");
    db.collection::<T>(name)
}
```

---

## 🚀 AdminX Initializer

This is the central glue that:

- loads AdminX config
- sets up logging
- initializes AdminX with Mongo
- registers all resources
- exposes:
  - `SessionMiddleware`
  - `Scope` with all `/adminx` routes

```rust
// src/admin/initializer.rs
use mongodb::Database;
use adminx::{
    adminx_initialize,
    get_adminx_config,
    setup_adminx_logging,
    get_adminx_session_middleware,
    register_all_admix_routes,
    registry::register_resource,
    AdmixResource,
    AdminxConfig,
};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::Scope;

// Import your resources
use crate::admin::resources::user_resource::UserResource;
use crate::admin::resources::picture_resource::PictureResource;
use crate::admin::resources::event_resource::EventResource;
use crate::admin::resources::event_attendee_resource::EventAttendeeResource;
use crate::admin::resources::notification_resource::NotificationResource;
use crate::admin::resources::config_resource::ConfigResource;

pub struct AdminxInitializer;

impl AdminxInitializer {
    pub async fn initialize(db: Database) -> AdminxConfig {
        // 1. Load configuration (from env)
        let adminx_config = get_adminx_config();

        // 2. Setup logging/tracing
        setup_adminx_logging(&adminx_config);

        // 3. Initialize AdminX (connects to Mongo, builds registry storage, etc.)
        let _adminx_instance = adminx_initialize(db.clone()).await;

        // 4. Register all resources
        Self::register_resources();

        adminx_config
    }

    fn register_resources() {
        // Order defines menu order
        let resources: Vec<Box<dyn AdmixResource>> = vec![
            Box::new(UserResource::new()),
            Box::new(PictureResource::new()),
            Box::new(EventResource::new()),
            Box::new(EventAttendeeResource::new()),
            Box::new(NotificationResource::new()),
            Box::new(ConfigResource::new()),
        ];

        for resource in resources {
            register_resource(resource);
        }
    }

    /// Get the AdminX session middleware
    pub fn get_session_middleware(
        config: &AdminxConfig,
    ) -> SessionMiddleware<CookieSessionStore> {
        get_adminx_session_middleware(config)
    }

    /// Get the AdminX routes scope (mounted at `/admin`)
    pub fn get_routes_service() -> Scope {
        register_all_admix_routes()
    }
}
```

---

## 🌐 main.rs – Wiring Actix + AdminX

A simplified version of your `main.rs`:

```rust
// src/main.rs
mod db;
mod admin;
mod models;
mod config;
mod libs;
mod errors;
mod utilities;
mod enums;
mod services;
mod requests;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use crate::db::mongo::init_mongo_client;
use crate::admin::initializer::AdminxInitializer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // 1. Init Mongo
    let db = init_mongo_client().await;

    // 2. Init AdminX
    let adminx_config = AdminxInitializer::initialize(db.clone()).await;

    // 3. Start Actix
    let server_address = "0.0.0.0:8080";

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(adminx_config.clone()))
            .wrap(Logger::default())
            .wrap(AdminxInitializer::get_session_middleware(&adminx_config))
            // Mount AdminX at /admin
            .service(
                web::scope("/admin")
                    .service(AdminxInitializer::get_routes_service())
            )
    })
    .bind(server_address)?
    .run()
    .await
}
```

Now visiting `http://localhost:8080/admin` will show the AdminX UI.

---

## 👤 Example 1 – `UserResource`

A simple AdminX resource backed by the `users` collection.

```rust
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

    /// Fields that can be set via forms/API
    fn permit_keys(&self) -> Vec<&'static str> {
        vec![
            "first_name",
            "last_name",
            "email",
            "phone_number",
            "status",
            "deleted",
        ]
    }

    // ===========================
    // UI STRUCTURE – FORM
    // ===========================
    fn form_structure(&self) -> Option<Value> {
        Some(json!({
            "groups": [
                {
                    "title": "User Details",
                    "fields": [
                        {
                            "name": "first_name",
                            "field_type": "text",
                            "label": "First Name",
                            "required": true
                        },
                        {
                            "name": "last_name",
                            "field_type": "text",
                            "label": "Last Name"
                        },
                        {
                            "name": "email",
                            "field_type": "email",
                            "label": "Email",
                            "required": true
                        },
                        {
                            "name": "phone_number",
                            "field_type": "text",
                            "label": "Phone number"
                        },
                        {
                            "name": "status",
                            "field_type": "select",
                            "label": "Status",
                            "options": [
                                { "label": "Active", "value": "active" },
                                { "label": "Inactive", "value": "inactive" }
                            ]
                        }
                    ]
                }
            ]
        }))
    }

    // ===========================
    // UI STRUCTURE – LIST TABLE
    // ===========================
    fn list_structure(&self) -> Option<Value> {
        Some(json!({
            "columns": [
                { "field": "first_name",   "label": "First Name",  "sortable": true },
                { "field": "last_name",    "label": "Last Name",   "sortable": true },
                { "field": "email",        "label": "Email",       "sortable": true },
                { "field": "phone_number", "label": "Phone",       "sortable": false },
                { "field": "status",       "label": "Status",      "type": "badge" },
                { "field": "created_at",   "label": "Created At",  "type": "datetime" }
            ],
            "filters": [
                {
                    "field": "status",
                    "type": "select",
                    "label": "Status",
                    "options": [
                        { "label": "All",     "value": "" },
                        { "label": "Active",  "value": "active" },
                        { "label": "Inactive","value": "inactive" }
                    ]
                }
            ]
        }))
    }
}
```

Once this resource is registered in `AdminxInitializer`, AdminX will automatically provide:

- List view (`/admin/users`)
- Create / edit forms
- View page
- JSON API for the same resource

---

## 🖼️ Example 2 – `PictureResource` (File Uploads to S3)

This resource shows:

- how to **enable file uploads** in a resource
- how to **upload to S3** using your `s3_utility`

```rust
// src/admin/resources/picture_resource.rs
use crate::db::mongo::get_collection;
use adminx::{AdmixResource, error::AdminxError};
use async_trait::async_trait;
use mongodb::{Collection, bson::Document};
use serde_json::{json, Value};
use crate::libs::s3_utility::upload_image_to_s3;
use futures::future::BoxFuture;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PictureResource;

#[async_trait]
impl AdmixResource for PictureResource {
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

    fn allowed_roles(&self) -> Vec<String> {
        vec!["admin".to_string(), "superadmin".to_string()]
    }

    fn permit_keys(&self) -> Vec<&'static str> {
        vec!["title", "image_file", "status", "deleted"]
    }

    // ===========================
    // FILE UPLOAD SUPPORT
    // ===========================
    fn supports_file_upload(&self) -> bool {
        true
    }

    fn max_file_size(&self) -> usize {
        5 * 1024 * 1024 // 5MB
    }

    fn allowed_file_extensions(&self) -> Vec<&'static str> {
        vec!["jpg", "jpeg", "png", "gif", "webp", "bmp"]
    }

    fn process_file_upload(
        &self,
        field_name: &str,
        file_data: &[u8],
        filename: &str,
    ) -> BoxFuture<'static, Result<HashMap<String, String>, AdminxError>> {
        let filename = filename.to_string();
        let field_name = field_name.to_string();
        let file_data = file_data.to_vec();
        let data_size = file_data.len();

        Box::pin(async move {
            tracing::info!(
                "Processing file upload for field: {}, filename: {}, size: {} bytes",
                field_name,
                filename,
                data_size
            );

            let timestamp = chrono::Utc::now().timestamp();
            let file_extension = filename.split('.').last().unwrap_or("jpg");
            let unique_filename =
                format!("images/{}_{}.{}", timestamp, field_name, file_extension);

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
    // UI STRUCTURE – FORM
    // ===========================
    fn form_structure(&self) -> Option<Value> {
        Some(json!({
            "groups": [{
                "title": "Picture Details",
                "fields": [
                    {
                        "name": "title",
                        "field_type": "text",
                        "label": "Title",
                        "required": true
                    },
                    {
                        "name": "image_file",
                        "field_type": "file",
                        "label": "Upload Image",
                        "accept": "image/*",
                        "required": true
                    },
                    {
                        "name": "status",
                        "field_type": "select",
                        "label": "Status",
                        "options": [
                            { "label": "Active", "value": "active" },
                            { "label": "Inactive", "value": "inactive" }
                        ]
                    }
                ]
            }]
        }))
    }

    // ===========================
    // UI STRUCTURE – LIST TABLE
    // ===========================
    fn list_structure(&self) -> Option<Value> {
        Some(json!({
            "columns": [
                { "field": "title",      "label": "Title",      "sortable": true },
                { "field": "image_url",  "label": "Image URL" },
                { "field": "status",     "label": "Status",     "type": "badge" },
                { "field": "deleted",    "label": "Deleted",    "type": "boolean" },
                { "field": "created_at", "label": "Created At", "type": "datetime" }
            ]
        }))
    }
}
```

Now AdminX will provide:

- an upload-enabled create/edit form for pictures  
- list view with badges & flags  
- S3 image uploads handled via your `upload_image_to_s3` utility  

---

## 🔑 Environment Variables

Typical `.env` for this starter:

```env
# Mongo
MONGO_URI=mongodb://localhost:27017
MONGO_DB=xard

# AdminX auth/session
ADMINX_BASIC_AUTH="adminx:password"
JWT_SECRET=your-super-secret-jwt-key-min-32-chars
SESSION_SECRET=your-session-secret-key-must-be-long
ENVIRONMENT=development

# S3 (for PictureResource)
AWS_REGION=ap-south-1
AWS_ACCESS_KEY_ID=xxxx
AWS_SECRET_ACCESS_KEY=xxxx
AWS_S3_BUCKET=xard-bucket

# Logging
RUST_LOG=info,actix_web=info,adminx=debug
```

---

## ▶️ Run it

```bash
# 1) Install Rust & MongoDB
# 2) Set up .env
cargo run

# Open:
# http://localhost:8080/adminx
```

You now have a working **AdminX Admin Panel** with real resources (`User`, `Picture`, `Event`, etc.) running on Actix + MongoDB.

---

## ✅ Using this as a template

To use this in your own project:

1. Copy the `admin/`, `db/`, and `models/` patterns.
2. Implement your own resources by following:
   - `UserResource` pattern for simple CRUD
   - `PictureResource` pattern for file uploads
3. Register them in `AdminxInitializer`.
4. Mount AdminX scope in your `main.rs`.
5. Adjust menu groups, allowed roles, form/list structures as needed.

---

Made with ❤️ using **AdminX**, **Actix-Web**, and **MongoDB**.
