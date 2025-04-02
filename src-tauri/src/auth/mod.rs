// src-tauri/src/auth/mod.rs

pub mod models;
pub mod oauth;
pub mod commands;
pub mod db;
pub mod utils;

use models::*;
use oauth::*;
use db::*;
use mongodb::bson::doc;

use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::AppHandle;
use thiserror::Error;
use serde::{Serialize, Deserialize};
use once_cell::sync::OnceCell;

// Global authentication state manager
static AUTH_MANAGER: OnceCell<Arc<Mutex<AuthManager>>> = OnceCell::new();

// Error types
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum AuthError {
    #[error("User not authenticated")]
    Unauthenticated,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("OAuth error: {0}")]
    OAuthError(String),
    #[error("Token error: {0}")]
    TokenError(String),
    #[error("Other error: {0}")]
    Other(String),
}

// Authentication manager
pub struct AuthManager {
    app_handle: Option<AppHandle>,
    db_client: Option<mongodb::Client>,
    current_user: Option<User>,
    oauth_manager: OAuthManager,
    user_db: UserDatabase,
}

impl AuthManager {
    // Initialize the authentication manager
    pub async fn init(app_handle: AppHandle, mongo_uri: &str) -> Result<(), AuthError> {
        let client = mongodb::Client::with_uri_str(mongo_uri)
            .await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
        
        let user_db = UserDatabase::new(client.clone());
        
        let oauth_manager = OAuthManager::new();
        
        let auth_manager = Self {
            app_handle: Some(app_handle),
            db_client: Some(client),
            current_user: None,
            oauth_manager,
            user_db,
        };
        
        let manager = Arc::new(Mutex::new(auth_manager));
        AUTH_MANAGER.set(manager);
        
        Ok(())
    }
    
    // Get AuthManager instance
    pub fn get_instance() -> Arc<Mutex<AuthManager>> {
        AUTH_MANAGER.get().expect("AuthManager not initialized").clone()
    }
    
    // Register a new user (email + password)
    pub async fn register_user(&mut self, email: &str, password: &str, name: &str) -> Result<User, AuthError> {
        // Check if the user already exists
        if self.user_db.user_exists(email).await? {
            return Err(AuthError::UserAlreadyExists);
        }
        
        // Create a new user
        let user = User::new_with_email(email, name);
        
        // Hash the password
        let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .map_err(|e| AuthError::Other(e.to_string()))?;
        
        // Store the user in the database
        self.user_db.create_user(&user, &password_hash).await?;
        
        Ok(user)
    }
    
    // Log in with email + password
    pub async fn login_with_email(&mut self, email: &str, password: &str) -> Result<User, AuthError> {
        // Retrieve the user from the database
        let (user, password_hash) = self.user_db.get_user_with_password(email).await?;
        
        // Verify the password
        let password_matches = bcrypt::verify(password, &password_hash)
            .map_err(|e| AuthError::Other(e.to_string()))?;
            
        if !password_matches {
            return Err(AuthError::InvalidCredentials);
        }
        
        // Set the current user
        self.current_user = Some(user.clone());
        
        // Record the login time
        self.user_db.update_last_login(&user.id).await?;
        
        Ok(user)
    }
    
    // Log in with an OAuth provider
    pub async fn login_with_oauth(&mut self, provider: OAuthProvider) -> Result<String, AuthError> {
        // Get the OAuth authorization URL
        let auth_url = self.oauth_manager.get_authorization_url(provider)?;
        
        // Return the authorization URL, which the frontend will use to open a browser
        Ok(auth_url)
    }
    
    // Handle OAuth callback
    pub async fn handle_oauth_callback(&mut self, provider: OAuthProvider, code: &str) -> Result<User, AuthError> {
        // Retrieve user info from the OAuth provider
        let oauth_info = self.oauth_manager.handle_callback(provider, code).await?;
        
        // Check if this OAuth ID is already associated with a user
        let existing_user = self.user_db.find_user_by_oauth_id(&oauth_info.provider, &oauth_info.id).await;
        
        let user = match existing_user {
            Ok(user) => {
                // Existing user, update OAuth info
                self.user_db.update_oauth_info(&user.id, &oauth_info).await?;
                user
            },
            Err(AuthError::UserNotFound) => {
                // New user, create an account
                let new_user = User::new_with_oauth(&oauth_info);
                self.user_db.create_user_with_oauth(&new_user, &oauth_info).await?;
                new_user
            },
            Err(e) => return Err(e),
        };
        
        // Set the current user
        self.current_user = Some(user.clone());
        
        // Record the login time
        self.user_db.update_last_login(&user.id).await?;
        
        Ok(user)
    }

    
    
    // Get the currently logged-in user
    pub async fn get_current_user(&self) -> Result<User, AuthError> {
        match &self.current_user {
            Some(user) => Ok(user.clone()),
            None => Err(AuthError::Unauthenticated),
        }
    }
    
    // Log out
    pub async fn logout(&mut self) -> Result<(), AuthError> {
        self.current_user = None;
        Ok(())
    }
}