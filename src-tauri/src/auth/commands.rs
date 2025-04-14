// src-tauri/src/auth/commands.rs

use std::env;

use crate::auth::{AuthManager, AuthError, models::*};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use tauri::{command, AppHandle, Manager};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Duration, Utc};

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

// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,    // Subject (user_id)
    exp: i64,       // Expiration time (Unix timestamp)
    iat: i64,       // Issued at (Unix timestamp)
}

// Convert string to OAuthProvider
fn parse_oauth_provider(provider: &str) -> Result<OAuthProvider, AuthError> {
    match provider.to_lowercase().as_str() {
        "google" => Ok(OAuthProvider::Google),
        "github" => Ok(OAuthProvider::GitHub),
        _ => Err(AuthError::OAuthError(format!("Unsupported OAuth provider: {}", provider))),
    }
}

/// Generate a secure JWT token
fn generate_token(user_id: &str) -> (String, DateTime<Utc>) {
    // Get JWT secret from environment variable or use a default for development
    let secret = env::var("JWT_SECRET")
        .unwrap_or_else(|_| "my-secret-key-for-dev-only".to_string());
    
    // Set token expiration (24 hours from now)
    let expiration = Utc::now() + Duration::hours(24);
    
    // Create the claims
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration.timestamp(),
        iat: Utc::now().timestamp(),
    };
    
    // Create the JWT
    let token = encode(
        &Header::new(Algorithm::HS256),  // Using HMAC SHA-256 algorithm
        &claims,
        &EncodingKey::from_secret(secret.as_bytes())
    ).expect("Failed to generate JWT");  // In production, handle this error properly
    
    (token, expiration)
}

fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET")
        .unwrap_or_else(|_| "my-secret-key-for-dev-only".to_string());
    
    let validation = jsonwebtoken::Validation::new(Algorithm::HS256);
    
    jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
        &validation
    ).map(|data| data.claims)
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
    token: String,
) -> Result<User, AuthError> {
    // 验证token
    let claims = validate_token(&token)
        .map_err(|_| AuthError::Unauthenticated)?;
    
    // 从claims中获取用户ID
    let user_id = claims.sub;
    
    // 从数据库获取用户
    let auth_manager = AuthManager::get_instance();
    let mut auth_manager = auth_manager.lock().await;
    
    // 需要添加一个从ID获取用户的方法
    auth_manager.user_db.find_by_id(&user_id).await
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
        auth_manager.oauth_manager.configure(
                google_client_id,
                google_client_secret,
                github_client_id,
                github_client_secret,
                redirect_url

        );

        Ok(())
    })
}