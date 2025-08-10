// src/db/mongo.rs
use mongodb::{bson::doc, Client, options::IndexOptions, options::ClientOptions, Database, Collection, IndexModel};
use dotenv::dotenv;
use std::env;
use log::{info};
use once_cell::sync::OnceCell;

static DB_INSTANCE: OnceCell<Database> = OnceCell::new();

/// ✅ Initializes MongoDB and sets global DB instance
pub async fn init_mongo_client() -> Database {
    dotenv().ok(); // Load env vars

    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let mongo_database_name = env::var("MONGO_DATABASE_NAME").expect("MONGO_DATABASE_NAME must be set");
    let client_options = ClientOptions::parse(&mongo_uri)
        .await
        .expect("Failed to parse MongoDB URI");

    let client = Client::with_options(client_options)
        .expect("Failed to initialize MongoDB client");

    let db = client.database(&mongo_database_name);

    info!("✅ Mongo client initialized: {}", mongo_uri);

    // init_indexes_and_uniqness(&client, &mongo_database_name)
    //     .await
    //     .expect("⚠️ Index creation failed");

    DB_INSTANCE.set(db.clone()).ok(); // Set globally

    db
}

/// ✅ Get the global Mongo DB instance (after init)
pub fn get_db() -> Database {
    DB_INSTANCE
        .get()
        .expect("Mongo DB is not initialized. Call init_mongo_client() first.")
        .clone()
}


pub fn get_collection<T>(name: &str) -> Collection<T>
where
    T: serde::de::DeserializeOwned + Unpin + Send + Sync,
{
    get_db().collection::<T>(name)
}