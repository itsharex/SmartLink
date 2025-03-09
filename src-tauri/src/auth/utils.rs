use chrono::{DateTime, Utc};
use mongodb::bson::{Bson, Document};
use tauri::{AppHandle, Emitter, Manager};
use crate::auth::AuthError;
use std::sync::Mutex;

// Holds user state, with current_user_id wrapped in a Mutex for mutability
pub struct UserState {
    pub current_user_id: Mutex<Option<String>>, // Uses Mutex to enable mutability
}

// Converts chrono::DateTime<Utc> to bson::DateTime
pub fn datetime_to_bson(dt: DateTime<Utc>) -> Bson {
    let timestamp_millis = dt.timestamp_millis();
    Bson::DateTime(mongodb::bson::DateTime::from_millis(timestamp_millis))
}

// Converts bson::DateTime to chrono::DateTime<Utc>
pub fn bson_to_datetime(bson: Bson) -> Result<DateTime<Utc>, AuthError> {
    match bson {
        Bson::DateTime(dt) => {
            let millis = dt.timestamp_millis();
            let secs = millis / 1000;
            let nsecs = ((millis % 1000) * 1_000_000) as u32;
            DateTime::from_timestamp(secs, nsecs)
                .ok_or_else(|| AuthError::DatabaseError("Invalid timestamp for DateTime".to_string()))
        },
        _ => Err(AuthError::DatabaseError("Invalid BSON type for DateTime".to_string())),
    }
}

// Creates a BSON document containing a datetime
pub fn datetime_document(field: &str, dt: DateTime<Utc>) -> Result<Document, AuthError> {
    let mut doc = Document::new();
    doc.insert(field, datetime_to_bson(dt));
    Ok(doc)
}

// Safely adds a datetime to a query document
pub fn add_datetime_to_doc(doc: &mut Document, field: &str, dt: DateTime<Utc>) {
    doc.insert(field, datetime_to_bson(dt));
}

// Gets the current logged-in user's ID
pub fn get_current_user_id(app_handle: &AppHandle) -> Result<String, String> {
    let state = app_handle.state::<UserState>();
    let user_id_guard = state.current_user_id.lock().map_err(|e| e.to_string())?;
    user_id_guard
        .as_ref()
        .cloned()
        .ok_or_else(|| "User not logged in".to_string())
}

// Sets the current user ID
pub fn set_current_user_id(app_handle: &AppHandle, user_id: Option<String>) -> Result<(), String> {
    let state = app_handle.state::<UserState>();
    let mut user_id_guard = state.current_user_id.lock().map_err(|e| e.to_string())?;

    // Updates the user ID
    *user_id_guard = user_id.clone();

    // Emits corresponding events
    if let Some(id) = &user_id {
        app_handle.emit("user_logged_in", id).map_err(|e| e.to_string())?;
    } else if user_id_guard.is_some() {
        app_handle.emit("user_logged_out", user_id_guard.as_ref().unwrap()).map_err(|e| e.to_string())?;
    }

    Ok(())
}

// Checks if a user is logged in
pub fn is_user_logged_in(app_handle: &AppHandle) -> bool {
    let state = app_handle.state::<UserState>();
    let user_id_guard = state.current_user_id.lock().unwrap(); // Unwrap directly in a non-async function
    user_id_guard.is_some()
}