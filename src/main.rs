// main.rs (debug version)
use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware::Logger};
use dotenv::dotenv;
use std::env;
use crate::dbs::mongo::init_mongo_client;
use adminx::{
    adminx_initialize, 
    get_adminx_config, 
    setup_adminx_logging, 
    get_adminx_session_middleware,
    register_all_admix_routes,
    registry::register_resource,
    AdmixResource,
};

mod dbs;
mod jobs;
mod admin;
mod models;
mod structs;

// Import your UserResource
use crate::admin::resources::user_resource::UserResource;
use crate::admin::resources::notification_resource::NotificationResource;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Test App Backend is Running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // Initialize AdminX components
    let adminx_config = get_adminx_config();
    setup_adminx_logging(&adminx_config);
    
    println!("🔧 Initializing database connection...");
    let db = init_mongo_client().await;
    let _ = adminx_initialize(db.clone()).await;
    
    println!("📝 Registering UserResource...");
    // Register your resources with AdminX
    register_resource(Box::new(UserResource::new()));
    register_resource(Box::new(NotificationResource::new()));
    println!("✅ UserResource registered successfully!");
    
    // Debug: Check if resource was registered
    let resources = adminx::registry::all_resources();
    println!("📋 Total resources registered: {}", resources.len());
    for resource in &resources {
        println!("   - Resource: '{}' at path: '{}'", 
                 resource.resource_name(), 
                 resource.base_path());
    }
    
    let server_address = env::var("SERVER_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:8083".to_string());
    
    println!("🚀 Starting server at: {}", server_address);
    println!("📋 AdminX Panel available at: http://{}/adminx", server_address);
    println!("👤 Users management at: http://{}/adminx/users/list", server_address);
    println!("🔧 Debug info:");
    println!("   - Main dashboard: http://{}/adminx", server_address);
    println!("   - Login page: http://{}/adminx/login", server_address);
    println!("   - Users list: http://{}/adminx/users/list", server_address);
    println!("   - New user: http://{}/adminx/users/new", server_address);
    
    HttpServer::new(move || {
        println!("🔄 Creating new app instance...");
        App::new()
            .app_data(web::Data::new(adminx_config.clone()))
            .wrap(Logger::default())
            .wrap(get_adminx_session_middleware(&adminx_config))
            .service(register_all_admix_routes()) // This will include your UserResource
            .route("/", web::get().to(health_check))
            .route("/health", web::get().to(health_check))
    })
    .bind(server_address)?
    .run()
    .await
}