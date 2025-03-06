// src-tauri/src/auth/commands.rs

use crate::auth::{AuthManager, AuthError, models::*};
use tauri::{command, AppHandle, Manager};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

// Login request
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// Register request
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

// OAuth request
#[derive(Debug, Deserialize)]
pub struct OAuthRequest {
    pub provider: String,
}

// OAuth callback request
#[derive(Debug, Deserialize)]
pub struct OAuthCallbackRequest {
    pub provider: String,
    pub code: String,
    pub state: Option<String>,
}

// Login response
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
    pub token_expires: DateTime<Utc>,
}

// Convert string to OAuthProvider
fn parse_oauth_provider(provider: &str) -> Result<OAuthProvider, AuthError> {
    match provider.to_lowercase().as_str() {
        "google" => Ok(OAuthProvider::Google),
        "github" => Ok(OAuthProvider::GitHub),
        _ => Err(AuthError::OAuthError(format!("Unsupported OAuth provider: {}", provider))),
    }
}

// Generate JWT token (simplified version, use a professional library in real applications)
fn generate_token(user_id: &str) -> (String, DateTime<Utc>) {
    // In a real application, you should use a library like jsonwebtoken to generate JWT
    let token = format!("sample_token_{}", user_id);
    let expires = Utc::now() + chrono::Duration::hours(24);
    
    (token, expires)
}

// Register with email and password
#[command]
pub async fn register_user(
    app_handle: AppHandle,
    request: RegisterRequest,
) -> Result<LoginResponse, AuthError> {
    let auth_manager = AuthManager::get_instance();
    let mut auth_manager = auth_manager.lock().await;
    
    // Register the user
    let user = auth_manager.register_user(&request.email, &request.password, &request.name).await?;
    
    // Generate token
    let (token, expires) = generate_token(&user.id);
    
    Ok(LoginResponse {
        user,
        token,
        token_expires: expires,
    })
}

// Login with email and password
#[command]
pub async fn login_with_email(
    app_handle: AppHandle,
    request: LoginRequest,
) -> Result<LoginResponse, AuthError> {
    let auth_manager = AuthManager::get_instance();
    let mut auth_manager = auth_manager.lock().await;
    
    // Perform login
    let user = auth_manager.login_with_email(&request.email, &request.password).await?;
    
    // Generate token
    let (token, expires) = generate_token(&user.id);
    
    Ok(LoginResponse {
        user,
        token,
        token_expires: expires,
    })
}

// Get OAuth login URL
#[command]
pub async fn get_oauth_url(
    app_handle: AppHandle,
    request: OAuthRequest,
) -> Result<String, AuthError> {
    let provider = parse_oauth_provider(&request.provider)?;
    
    let auth_manager = AuthManager::get_instance();
    let mut auth_manager = auth_manager.lock().await;
    
    auth_manager.login_with_oauth(provider).await
}

// Handle OAuth callback
#[command]
pub async fn handle_oauth_callback(
    app_handle: AppHandle,
    request: OAuthCallbackRequest,
) -> Result<LoginResponse, AuthError> {
    let provider = parse_oauth_provider(&request.provider)?;
    
    let auth_manager = AuthManager::get_instance();
    let mut auth_manager = auth_manager.lock().await;
    
    // Handle OAuth callback
    let user = auth_manager.handle_oauth_callback(provider, &request.code).await?;
    
    // Generate token
    let (token, expires) = generate_token(&user.id);
    
    Ok(LoginResponse {
        user,
        token,
        token_expires: expires,
    })
}

// Get current user
#[command]
pub async fn get_current_user(
    app_handle: AppHandle,
) -> Result<User, AuthError> {
    let auth_manager = AuthManager::get_instance();
    let auth_manager = auth_manager.lock().await;
    
    auth_manager.get_current_user().await
}

// Logout
#[command]
pub async fn logout(
    app_handle: AppHandle,
) -> Result<(), AuthError> {
    let auth_manager = AuthManager::get_instance();
    let mut auth_manager = auth_manager.lock().await;
    
    auth_manager.logout().await
}

// Refresh token
#[command]
pub async fn refresh_token(
    app_handle: AppHandle,
) -> Result<LoginResponse, AuthError> {
    let auth_manager = AuthManager::get_instance();
    let auth_manager = auth_manager.lock().await;
    
    // Get current user
    let user = auth_manager.get_current_user().await?;
    
    // Generate new token
    let (token, expires) = generate_token(&user.id);
    
    Ok(LoginResponse {
        user,
        token,
        token_expires: expires,
    })
}

// Initialize the authentication system
pub fn init(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.app_handle().clone();

    // Read configuration from environment variables with defaults
    let mongo_uri = std::env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let google_client_id = std::env::var("GOOGLE_CLIENT_ID").unwrap_or_default();
    let google_client_secret = std::env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default();
    let github_client_id = std::env::var("GITHUB_CLIENT_ID").unwrap_or_default();
    let github_client_secret = std::env::var("GITHUB_CLIENT_SECRET").unwrap_or_default();
    let redirect_url = std::env::var("OAUTH_REDIRECT_URL").unwrap_or_else(|_| "http://localhost:3000/callback".to_string());

    // Initialize AuthManager
    tauri::async_runtime::block_on(async {
        AuthManager::init(app_handle.clone(), &mongo_uri)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        // Configure OAuth clients
        let auth_manager = AuthManager::get_instance();
        let mut auth_manager = auth_manager.lock().await;

        // Here we would configure the OAuthManager with the client IDs and secrets
        // For example:
        // auth_manager.oauth_manager.configure(
        //     OAuthConfig {
        //         google_client_id,
        //         google_client_secret,
        //         github_client_id,
        //         github_client_secret,
        //         redirect_url,
        //     }
        // );

        Ok(())
    })
}