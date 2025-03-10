// src-tauri/src/auth/models.rs

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use mongodb::bson::{self, doc, Bson, Document};

// User structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: Option<String>,
    pub name: String,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub oauth_connections: Vec<OAuthConnection>,
    pub settings: UserSettings,
}

// OAuth connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthConnection {
    pub provider: OAuthProvider,
    pub provider_user_id: String,
    pub created_at: DateTime<Utc>,
    pub last_used: DateTime<Utc>,
}

// OAuth provider enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OAuthProvider {
    Google,
    GitHub,
}

// User settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub theme: String,
    pub language: String,
    pub notifications_enabled: bool,
    pub encryption_enabled: bool,
}

// OAuth user information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthUserInfo {
    pub id: String,
    pub provider: OAuthProvider,
    pub email: Option<String>,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub raw_data: Document,
}

impl User {
    // Create a new user with email
    pub fn new_with_email(email: &str, name: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            email: Some(email.to_string()),
            name: name.to_string(),
            avatar_url: None,
            created_at: now,
            last_login: None,
            oauth_connections: Vec::new(),
            settings: UserSettings::default(),
        }
    }
    
    // Create a user with OAuth information
    pub fn new_with_oauth(oauth_info: &OAuthUserInfo) -> Self {
        let now = Utc::now();
        let oauth_connection = OAuthConnection {
            provider: oauth_info.provider,
            provider_user_id: oauth_info.id.clone(),
            created_at: now,
            last_used: now,
        };
        
        Self {
            id: Uuid::new_v4().to_string(),
            email: oauth_info.email.clone(),
            name: oauth_info.name.clone().unwrap_or_else(|| "User".to_string()),
            avatar_url: oauth_info.avatar_url.clone(),
            created_at: now,
            last_login: Some(now),
            oauth_connections: vec![oauth_connection],
            settings: UserSettings::default(),
        }
    }
    
    // Add an OAuth connection
    pub fn add_oauth_connection(&mut self, oauth_info: &OAuthUserInfo) {
        let now = Utc::now();
        let oauth_connection = OAuthConnection {
            provider: oauth_info.provider,
            provider_user_id: oauth_info.id.clone(),
            created_at: now,
            last_used: now,
        };
        
        self.oauth_connections.push(oauth_connection);
    }
    
    // Convert to MongoDB document
    pub fn to_document(&self) -> Document {
        let mut doc = bson::to_document(self).unwrap_or_else(|_| Document::new());
        
        // Ensure datetime fields are correctly converted
        if let Some(created_at) = doc.get_mut("created_at") {
            *created_at = crate::auth::utils::datetime_to_bson(self.created_at);
        }
        
        if let Some(last_login) = doc.get_mut("last_login") {
            if let Some(last_login_time) = self.last_login {
                *last_login = crate::auth::utils::datetime_to_bson(last_login_time);
            }
        }
        
        // Handle datetime fields in oauth_connections
        if let Some(connections) = doc.get_mut("oauth_connections") {
            if let Bson::Array(array) = connections {
                for connection in array {
                    if let Bson::Document(conn_doc) = connection {
                        if let Some(created_at) = conn_doc.get_mut("created_at") {
                            if let Ok(dt) = bson::from_bson::<DateTime<Utc>>(created_at.clone()) {
                                *created_at = crate::auth::utils::datetime_to_bson(dt);
                            }
                        }
                        
                        if let Some(last_used) = conn_doc.get_mut("last_used") {
                            if let Ok(dt) = bson::from_bson::<DateTime<Utc>>(last_used.clone()) {
                                *last_used = crate::auth::utils::datetime_to_bson(dt);
                            }
                        }
                    }
                }
            }
        }
        
        doc
    }
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            language: "zh-CN".to_string(),
            notifications_enabled: true,
            encryption_enabled: true,
        }
    }
}

// User login credentials response
#[derive(Debug, Clone, Serialize)]
pub struct UserCredentials {
    pub user: User,
    pub token: String,
    pub token_expires: DateTime<Utc>,
}