// test/src/main.rs
use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware::Logger};
use dotenv::dotenv;
use std::env;
use crate::dbs::mongo::init_mongo_client;
use crate::admin::initializer::AdminxInitializer;
use crate::qrushes::qrush_integrated::QrushIntegrated;
use crate::services::data_service::DataService;
use crate::srotas::initialize::SrotasVector;
use crate::srotas::service as srotas_service;

use debugx::{debugx, debugx_json};

mod dbs;
mod qrushes;
mod admin;
mod models;
mod structs;
mod services;
mod utils;
mod srotas;



async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Test App Backend is Running!")
}


async fn test_queue() -> impl Responder {
    DataService::create_record("127", "Snm").await;
    HttpResponse::Ok().body("test queue!")
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    println!("🔧 Initializing database connection...");
    let db = init_mongo_client().await;
    
    // Initialize AdminX components using the initializer
    let adminx_config = AdminxInitializer::initialize(db.clone()).await;
    
    // 🎯 GLOBAL Qrush initialization - happens ONCE for the entire application
    println!("🌍 Initializing Qrush globally...");
    QrushIntegrated::initialize(None).await;
    println!("✅ Global Qrush initialization complete!");
    
    let server_address = env::var("SERVER_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:8083".to_string());

    // 4) Srotas Vector global init (SDK + OVM)
    println!("Initializing Srotas Vector (SDK/OVM)...");
    let srotas_ovm = SrotasVector::initialize(None).await.expect("Srotas Vector initialization failed");
    println!("✅ Srotas Vector ready at {}", std::env::var("SROTAS_BASE_URL").unwrap_or_else(|_| "http://localhost:3030".into()));
    
    // Print startup information
    // AdminxInitializer::print_startup_info(&server_address);
    
    HttpServer::new(move || {
        println!("🔄 Creating new app instance...");
        // 👷 Worker-specific setup - only enqueues jobs for this worker
        // Uses the GLOBAL Qrush instance that was initialized above
        let qrush_worker_config = QrushIntegrated::setup_worker_sync();
        
        App::new()
            .app_data(web::Data::new(adminx_config.clone()))
            .app_data(web::Data::new(qrush_worker_config))
            // inject the Srotas Vector context
            .app_data(web::Data::new(srotas_ovm.clone()))
            // mount the OVM service under /srotas
            .service(web::scope("/srotas").configure(srotas_service::configure))
            .wrap(Logger::default())
            .wrap(AdminxInitializer::get_session_middleware(&adminx_config))
            .service(AdminxInitializer::get_routes_service())
            // Qrush metrics routes
            .service(
                web::scope("/qrush")
                    .configure(|cfg| QrushIntegrated::configure_routes(cfg))
            )
            .route("/", web::get().to(health_check))
            .route("/health", web::get().to(health_check))
            .route("/test_queue", web::get().to(test_queue))
    })
    .bind(server_address)?
    .run()
    .await
}