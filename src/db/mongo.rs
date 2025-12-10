// src/db/mongo.rs
use mongodb::{bson::doc, Client, options::IndexOptions, options::ClientOptions, Database, Collection, IndexModel};
use std::env;
use crate::models::{user::User};
use log::{info};
use once_cell::sync::OnceCell;

static DB_INSTANCE: OnceCell<Database> = OnceCell::new();
static DB_CLIENT: OnceCell<Client> = OnceCell::new();

/// ✅ Initializes MongoDB and sets global DB instance
pub async fn init_mongo_client() -> Database {
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let mongo_database_name = env::var("MONGO_DATABASE_NAME").expect("MONGO_DATABASE_NAME must be set");
    
    let client_options = ClientOptions::parse(&mongo_uri)
        .await
        .expect("Failed to parse MongoDB URI");
    
    let client = Client::with_options(client_options)
        .expect("Failed to initialize MongoDB client");
    
    let db = client.database(&mongo_database_name);
    
    info!("✅ Mongo client initialized: {}", mongo_uri);

    init_indexes_and_uniqness(&client, &mongo_database_name)
        .await
        .expect("Index creation failed");
    
    // Uncomment when ready to maintain indexes
    // MaintainMongoIndex::maintain(&client, &mongo_database_name).await.expect("Index creation failed");
    
    // Set global instances
    DB_INSTANCE.set(db.clone()).expect("Failed to set DB_INSTANCE");
    DB_CLIENT.set(client.clone()).expect("Failed to set DB_CLIENT");
    
    db
}

/// Returns the MongoDB client instance
pub fn get_client() -> Client {
    DB_CLIENT
        .get()
        .expect("Mongo Client is not initialized. Call init_mongo_client() first")
        .clone()
}

/// Returns the MongoDB database instance
pub fn get_db() -> Database {
    DB_INSTANCE
        .get()
        .expect("Mongo DB is not initialized. Call init_mongo_client() first")
        .clone()
}

/// Returns a typed collection from the database
pub fn get_collection<T>(name: &str) -> Collection<T>
where
    T: serde::de::DeserializeOwned + Unpin + Send + Sync,
{
    get_db().collection::<T>(name)
}




pub async fn init_indexes_and_uniqness(client: &Client, mongo_database_name: &str) -> mongodb::error::Result<()> {
    let db = client.database(mongo_database_name); // change if your DB name differs

    /*-----------------------------------------------------
    Start User
    -----------------------------------------------------*/
    let user_collection: Collection<User> = db.collection("users");

    // Email: Unique AND Required (Non-null)
    let email_index = IndexModel::builder()
        .keys(doc! { "email": 1 })
        .options(
            IndexOptions::builder()
                .unique(true)
                .sparse(false)
                .name(Some("user_email_unique".to_string()))
                .build()
        )
        .build();

    // Unique BUT Optional (Nullable)
    let official_email_index = IndexModel::builder()
    .keys(doc! { "official_email": 1 })
    .options(
        IndexOptions::builder()
            .unique(true)
            .sparse(true)
            .name(Some("user_official_email".to_string()))
            .build()
    )
    .build();

    // Phone Number: Unique BUT Optional (Nullable)
    let phone_index = IndexModel::builder()
        .keys(doc! { "phone_number": 1 })
        .options(
        IndexOptions::builder()
            .unique(true)
            .sparse(true)
            .name(Some("user_phone_unique".to_string()))
            .build()
        )
        .build();

    // Deleted: filter index (already present)
    let user_deleted_index = IndexModel::builder()
        .keys(doc! { "deleted": 1 })
        .options(
            IndexOptions::builder()
                .unique(false)
                .name(Some("user_deleted_index".to_string()))
                .build()
        )
        .build();

    user_collection.create_index(email_index, None).await?;
    user_collection.create_index(phone_index, None).await?;
    user_collection.create_index(official_email_index, None).await?;
    user_collection.create_index(user_deleted_index, None).await?;
    /*-----------------------------------------------------
    END User
    -----------------------------------------------------*/

    Ok(())
}
