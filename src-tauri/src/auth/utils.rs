// src-tauri/src/auth/utils.rs

use chrono::{DateTime, Utc};
use mongodb::bson::{Bson, Document};
use crate::auth::AuthError;

// Convert chrono::DateTime<Utc> to bson::DateTime
pub fn datetime_to_bson(dt: DateTime<Utc>) -> Bson {
    // In newer versions, we need to use the from_millis method
    let timestamp_millis = dt.timestamp_millis();
    Bson::DateTime(mongodb::bson::DateTime::from_millis(timestamp_millis))
}

// Convert bson::DateTime to chrono::DateTime<Utc>
pub fn bson_to_datetime(bson: Bson) -> Result<DateTime<Utc>, AuthError> {
    match bson {
        Bson::DateTime(dt) => {
            // Use timestamp_millis for conversion
            let millis = dt.timestamp_millis();
            let secs = millis / 1000;
            let nsecs = ((millis % 1000) * 1_000_000) as u32;
            
            match DateTime::from_timestamp(secs, nsecs) {
                Some(datetime) => Ok(datetime),
                None => Err(AuthError::DatabaseError("Invalid timestamp for DateTime".to_string())),
            }
        },
        _ => Err(AuthError::DatabaseError("Invalid BSON type for DateTime".to_string())),
    }
}

// Create a BSON document containing a datetime
pub fn datetime_document(field: &str, dt: DateTime<Utc>) -> Result<Document, AuthError> {
    let mut doc = Document::new();
    doc.insert(field, datetime_to_bson(dt));
    Ok(doc)
}

// Safely add a datetime to a query using a helper function
pub fn add_datetime_to_doc(doc: &mut Document, field: &str, dt: DateTime<Utc>) {
    doc.insert(field, datetime_to_bson(dt));
}