// src/main.rs
mod db;
mod models;
mod services;
mod config;
mod libs;
mod errors;
mod utilities;
mod enums;
mod admin;
mod macros;
mod requests;


use dotenv::dotenv;
use dotenv::from_filename;
use std::env;
use actix_web::{web, guard, App, HttpServer, Responder, HttpResponse, middleware::Logger};
use tracing_actix_web::TracingLogger;
use db::mongo::init_mongo_client;
use mongodb::Database;
use actix_web::dev::HttpServiceFactory;
use log::{info, error};
use env_logger::Env;
use actix_web_prom::PrometheusMetricsBuilder;
use actix_files::Files;
use crate::services::redis_service::init_redis;
use crate::admin::initializer::AdminxInitializer;



async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Xard Backend is Running")
}


async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("404 - Route Not Found")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment configuration
    crate::config::env_vars::load_environment();
    
    // Your application setup here...
    println!("Starting application in {} mode", crate::config::env_vars::get_env());
    

    // 🔹 Initialize `env_logger` for debugging
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    info!("Logging Initialized");

    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics") // Expose Prometheus metrics at /metrics
        .build()
        .unwrap();

    // Initialize Redis and Wrap in `web::Data`
    init_redis().await;
    
    let db: Database = init_mongo_client().await;
    let db_data = web::Data::new(db.clone()); // Wrap DB in `web::Data`
    println!("Database initialized......");

    // Initialize AdminX components using the initializer
    let adminx_config = AdminxInitializer::initialize(db.clone()).await;

    //Print a startup message
    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8082".to_string());
    println!("🚀 Server started on: http://{}", server_address);


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(adminx_config.clone()))
            .wrap(Logger::default())
            .wrap(prometheus.clone())
            .wrap(AdminxInitializer::get_session_middleware(&adminx_config))
            .service(AdminxInitializer::get_routes_service())
    })
    .bind(server_address)?
    .run()
    .await
}
