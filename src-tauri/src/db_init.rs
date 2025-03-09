use dotenv::dotenv;
use std::env;
use mongodb::{Client, Database};
use mongodb::options::ClientOptions;
use mongodb::bson::doc;

// Initialize database connection
pub async fn init_database() -> Result<Database, Box<dyn std::error::Error>> {
    // Load .env file
    dotenv().ok();
    
    // Get connection string from environment variable
    let mongodb_uri = env::var("MONGO_URI")
        .expect("MONGO_URI must be set in .env file");
    
    // Parse connection options
    let client_options = ClientOptions::parse(&mongodb_uri).await?;
    
    // Connect to MongoDB
    let client = Client::with_options(client_options)?;
    
    // Specify database name
    let db = client.database("smartlink");
    
    // Test connection
    client.list_database_names(None, None).await?;
    println!("Connected to MongoDB successfully!");
    
    Ok(db)
}

// Ensure required collections and indexes exist
pub async fn ensure_indexes(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure index for the conversations collection
    db.collection::<mongodb::bson::Document>("conversations")
        .create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "participants": 1 })
                .build(),
            None,
        )
        .await?;
    
    // Ensure index for the messages collection
    db.collection::<mongodb::bson::Document>("messages")
        .create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "conversation_id": 1, "timestamp": -1 })
                .build(),
            None,
        )
        .await?;
    
    // Ensure index for the chat_events collection (for offline messages)
    db.collection::<mongodb::bson::Document>("chat_events")
        .create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "conversation_id": 1 })
                .build(),
            None,
        )
        .await?;
    
    println!("MongoDB indexes created successfully!");
    Ok(())
}
