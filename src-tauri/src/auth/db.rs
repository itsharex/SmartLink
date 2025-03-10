// src-tauri/src/auth/db.rs

use crate::auth::{AuthError, models::*};
use mongodb::{Client, Collection, Database};
use mongodb::bson::{self, doc, Document};
use chrono::Utc;
use serde::{Serialize, Deserialize};

// User database view
pub struct UserDatabase {
    client: Client,
    db_name: String,
}

// User record (including password hash)
#[derive(Debug, Serialize, Deserialize)]
struct UserRecord {
    #[serde(flatten)]
    user: User,
    password_hash: Option<String>,
}

impl UserDatabase {
    // Create a new user database connection
    pub fn new(client: Client) -> Self {
        Self {
            client,
            db_name: "smartlink".to_string(),
        }
    }
    
    // Get the users collection
    fn get_users_collection(&self) -> Collection<UserRecord> {
        self.client.database(&self.db_name).collection("users")
    }
    
    // Check if a user exists
    pub async fn user_exists(&self, email: &str) -> Result<bool, AuthError> {
        let collection = self.get_users_collection();
        
        let count = collection
            .count_documents(doc! { "email": email }, None)
            .await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
            
        Ok(count > 0)
    }
    
    // Create a new user
    pub async fn create_user(&self, user: &User, password_hash: &str) -> Result<(), AuthError> {
        let collection = self.get_users_collection();
        
        let user_record = UserRecord {
            user: user.clone(),
            password_hash: Some(password_hash.to_string()),
        };
        
        collection
            .insert_one(user_record, None)
            .await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
            
        Ok(())
    }
    
    // Create a user with OAuth information
    pub async fn create_user_with_oauth(&self, user: &User, oauth_info: &OAuthUserInfo) -> Result<(), AuthError> {
        let collection = self.get_users_collection();
        
        let user_record = UserRecord {
            user: user.clone(),
            password_hash: None,
        };
        
        collection
            .insert_one(user_record, None)
            .await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
            
        Ok(())
    }
    
    // Get a user and their password hash
    pub async fn get_user_with_password(&self, email: &str) -> Result<(User, String), AuthError> {
        let collection = self.get_users_collection();
        
        let user_record = collection
            .find_one(doc! { "email": email }, None)
            .await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?
            .ok_or(AuthError::UserNotFound)?;
            
        let password_hash = user_record.password_hash
            .ok_or(AuthError::InvalidCredentials)?;
            
        Ok((user_record.user, password_hash))
    }
    
    // Get a user
    pub async fn get_user(&self, user_id: &str) -> Result<User, AuthError> {
        let collection = self.get_users_collection();
        
        let user_record = collection
            .find_one(doc! { "user.id": user_id }, None)
            .await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?
            .ok_or(AuthError::UserNotFound)?;
            
        Ok(user_record.user)
    }
    
    // Find a user by OAuth ID
    pub async fn find_user_by_oauth_id(&self, provider: &OAuthProvider, provider_user_id: &str) -> Result<User, AuthError> {
        let collection = self.get_users_collection();
        
        let filter = doc! {
            "user.oauth_connections": {
                "$elemMatch": {
                    "provider": bson::to_bson(provider).unwrap(),
                    "provider_user_id": provider_user_id
                }
            }
        };
        
        let user_record = collection
            .find_one(filter, None)
            .await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?
            .ok_or(AuthError::UserNotFound)?;
            
        Ok(user_record.user)
    }
    
    // Update the user's last login time
    pub async fn update_last_login(&self, user_id: &str) -> Result<(), AuthError> {
        let collection = self.get_users_collection();
        
        let now = Utc::now();
        let bson_now = crate::auth::utils::datetime_to_bson(now);
        
        collection
            .update_one(
                doc! { "user.id": user_id },
                doc! { "$set": { "user.last_login": bson_now } },
                None
            )
            .await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
            
        Ok(())
    }
    
    // Update OAuth information
    pub async fn update_oauth_info(&self, user_id: &str, oauth_info: &OAuthUserInfo) -> Result<(), AuthError> {
        let collection = self.get_users_collection();
        
        // Check if this OAuth connection already exists
        let filter = doc! {
            "user.id": user_id,
            "user.oauth_connections": {
                "$elemMatch": {
                    "provider": bson::to_bson(&oauth_info.provider).unwrap(),
                    "provider_user_id": &oauth_info.id
                }
            }
        };
        
        let user_exists = collection
            .count_documents(filter, None)
            .await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
            
        if user_exists > 0 {
            // Update existing connection
            let now = Utc::now();
            let bson_now = crate::auth::utils::datetime_to_bson(now);
            
            collection
                .update_one(
                    doc! {
                        "user.id": user_id,
                        "user.oauth_connections.provider": bson::to_bson(&oauth_info.provider).unwrap(),
                        "user.oauth_connections.provider_user_id": &oauth_info.id
                    },
                    doc! {
                        "$set": {
                            "user.oauth_connections.$.last_used": bson_now
                        }
                    },
                    None
                )
                .await
                .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
        } else {
            // Add new connection
            let now = Utc::now();
            let new_connection = OAuthConnection {
                provider: oauth_info.provider,
                provider_user_id: oauth_info.id.clone(),
                created_at: now,
                last_used: now,
            };
            
            // Convert to BSON (using serialization)
            let connection_bson = bson::to_bson(&new_connection)
                .map_err(|e| AuthError::DatabaseError(format!("Failed to convert OAuth connection to BSON: {}", e)))?;
            
            collection
                .update_one(
                    doc! { "user.id": user_id },
                    doc! {
                        "$push": {
                            "user.oauth_connections": connection_bson
                        }
                    },
                    None
                )
                .await
                .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
        }
        
        // Update basic user information (if needed)
        if oauth_info.email.is_some() || oauth_info.name.is_some() || oauth_info.avatar_url.is_some() {
            let mut update_doc = Document::new();
            
            if let Some(email) = &oauth_info.email {
                update_doc.insert("user.email", email);
            }
            
            if let Some(name) = &oauth_info.name {
                update_doc.insert("user.name", name);
            }
            
            if let Some(avatar_url) = &oauth_info.avatar_url {
                update_doc.insert("user.avatar_url", avatar_url);
            }
            
            if !update_doc.is_empty() {
                collection
                    .update_one(
                        doc! { "user.id": user_id },
                        doc! { "$set": update_doc },
                        None
                    )
                    .await
                    .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
            }
        }
        
        Ok(())
    }
    
    // Delete a user
    pub async fn delete_user(&self, user_id: &str) -> Result<(), AuthError> {
        let collection = self.get_users_collection();
        
        collection
            .delete_one(doc! { "user.id": user_id }, None)
            .await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
            
        Ok(())
    }
    
    // Update user settings
    pub async fn update_user_settings(&self, user_id: &str, settings: &UserSettings) -> Result<(), AuthError> {
        let collection = self.get_users_collection();
        
        collection
            .update_one(
                doc! { "user.id": user_id },
                doc! { "$set": { "user.settings": bson::to_bson(settings).unwrap() } },
                None
            )
            .await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
            
        Ok(())
    }
}