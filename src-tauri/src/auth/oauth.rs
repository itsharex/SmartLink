// src-tauri/src/auth/oauth.rs

use crate::auth::{AuthError, models::*};
use oauth2::{
    basic::BasicClient, AuthUrl, TokenUrl, ClientId, ClientSecret,
    RedirectUrl, AuthorizationCode, CsrfToken, PkceCodeChallenge, Scope,
    TokenResponse,
};
use oauth2::reqwest::async_http_client;
use reqwest::Client as HttpClient;
use mongodb::bson::{self, doc, Document};
use std::sync::Arc;
use std::collections::HashMap;
use serde_json::Value;
use url::Url;

// OAuth管理器
pub struct OAuthManager {
    http_client: HttpClient,
    google_client: Option<BasicClient>,
    github_client: Option<BasicClient>,
    // 存储CSRF令牌和PKCE挑战
    pending_requests: HashMap<String, PendingOAuthRequest>,
}

// 待处理的OAuth请求
struct PendingOAuthRequest {
    provider: OAuthProvider,
    pkce_verifier: oauth2::PkceCodeVerifier,
}

impl OAuthManager {
    // 创建新的OAuth管理器
    pub fn new() -> Self {
        Self {
            http_client: HttpClient::new(),
            google_client: None,
            github_client: None,
            pending_requests: HashMap::new(),
        }
    }
    
    // 配置OAuth客户端
    pub fn configure(&mut self, 
                   google_client_id: String, 
                   google_client_secret: String,
                   github_client_id: String, 
                   github_client_secret: String,
                   redirect_url: String) {
        // 配置Google OAuth客户端
        let google_client = BasicClient::new(
            ClientId::new(google_client_id),
            Some(ClientSecret::new(google_client_secret)),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
                .expect("Invalid Google auth URL"),
            Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
                .expect("Invalid Google token URL"))
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url.clone())
            .expect("Invalid redirect URL"));
            
        // 配置GitHub OAuth客户端
        let github_client = BasicClient::new(
            ClientId::new(github_client_id),
            Some(ClientSecret::new(github_client_secret)),
            AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
                .expect("Invalid GitHub auth URL"),
            Some(TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
                .expect("Invalid GitHub token URL"))
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url)
            .expect("Invalid redirect URL"));
            
        self.google_client = Some(google_client);
        self.github_client = Some(github_client);
    }
    
    // 获取授权URL
    pub fn get_authorization_url(&mut self, provider: OAuthProvider) -> Result<String, AuthError> {
        let client = match provider {
            OAuthProvider::Google => self.google_client.as_ref()
                .ok_or_else(|| AuthError::OAuthError("Google OAuth client not configured".to_string()))?,
            OAuthProvider::GitHub => self.github_client.as_ref()
                .ok_or_else(|| AuthError::OAuthError("GitHub OAuth client not configured".to_string()))?,
        };
        
        // 创建PKCE挑战
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        
        // 生成授权URL和CSRF令牌
        let mut auth_request = client.authorize_url(CsrfToken::new_random);
        
        // 添加提供商特定的Scope
        match provider {
            OAuthProvider::Google => {
                auth_request = auth_request
                    .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()))
                    .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()));
            },
            OAuthProvider::GitHub => {
                auth_request = auth_request
                    .add_scope(Scope::new("user:email".to_string()));
            },
        }
        
        // 设置PKCE挑战
        let (auth_url, csrf_token) = auth_request
            .set_pkce_challenge(pkce_challenge)
            .url();
            
        // 存储CSRF令牌和PKCE验证器，以便回调时使用
        self.pending_requests.insert(
            csrf_token.secret().clone(),
            PendingOAuthRequest {
                provider,
                pkce_verifier,
            }
        );
        
        Ok(auth_url.to_string())
    }
    
    // 处理OAuth回调
    pub async fn handle_callback(&mut self, provider: OAuthProvider, code: &str) -> Result<OAuthUserInfo, AuthError> {
        // 在实际应用中，你需要验证CSRF令牌
        // 这里简化处理，直接使用授权码
        
        let client = match provider {
            OAuthProvider::Google => self.google_client.as_ref()
                .ok_or_else(|| AuthError::OAuthError("Google OAuth client not configured".to_string()))?,
            OAuthProvider::GitHub => self.github_client.as_ref()
                .ok_or_else(|| AuthError::OAuthError("GitHub OAuth client not configured".to_string()))?,
        };
        
        // 使用授权码交换令牌
        // 注意：在实际应用中，应该使用存储的PKCE验证器
        let token_result = client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .request_async(async_http_client)
            .await
            .map_err(|e| AuthError::OAuthError(format!("Failed to exchange token: {}", e)))?;
            
        let access_token = token_result.access_token().secret();
        
        // 获取用户信息
        match provider {
            OAuthProvider::Google => self.fetch_google_user_info(access_token).await,
            OAuthProvider::GitHub => self.fetch_github_user_info(access_token).await,
        }
    }
    
    // 获取Google用户信息
    async fn fetch_google_user_info(&self, access_token: &str) -> Result<OAuthUserInfo, AuthError> {
        let user_info_url = "https://www.googleapis.com/oauth2/v3/userinfo";
        let response = self.http_client
            .get(user_info_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| AuthError::OAuthError(format!("Failed to fetch Google user info: {}", e)))?;
            
        let json: Value = response.json()
            .await
            .map_err(|e| AuthError::OAuthError(format!("Failed to parse Google user info: {}", e)))?;
            
        // 将JSON转换为BSON文档
        let raw_data = bson::to_document(&json)
            .map_err(|e| AuthError::OAuthError(format!("Failed to convert Google user info to BSON: {}", e)))?;
            
        // 提取用户ID
        let sub = json["sub"].as_str()
            .ok_or_else(|| AuthError::OAuthError("Google user info missing 'sub'".to_string()))?;
            
        // 提取电子邮件
        let email = json["email"].as_str().map(String::from);
        
        // 提取姓名
        let name = json["name"].as_str().map(String::from);
        
        // 提取头像URL
        let avatar_url = json["picture"].as_str().map(String::from);
        
        Ok(OAuthUserInfo {
            id: sub.to_string(),
            provider: OAuthProvider::Google,
            email,
            name,
            avatar_url,
            raw_data,
        })
    }
    
    // 获取GitHub用户信息
    async fn fetch_github_user_info(&self, access_token: &str) -> Result<OAuthUserInfo, AuthError> {
        // 获取用户基本信息
        let user_info_url = "https://api.github.com/user";
        let response = self.http_client
            .get(user_info_url)
            .header("Authorization", format!("token {}", access_token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "SmartLink-App")
            .send()
            .await
            .map_err(|e| AuthError::OAuthError(format!("Failed to fetch GitHub user info: {}", e)))?;
            
        let json: Value = response.json()
            .await
            .map_err(|e| AuthError::OAuthError(format!("Failed to parse GitHub user info: {}", e)))?;
            
        // 将JSON转换为BSON文档
        let raw_data = bson::to_document(&json)
            .map_err(|e| AuthError::OAuthError(format!("Failed to convert GitHub user info to BSON: {}", e)))?;
            
        // 提取用户ID
        let id = json["id"].as_u64()
            .ok_or_else(|| AuthError::OAuthError("GitHub user info missing 'id'".to_string()))?
            .to_string();
            
        // 提取姓名
        let name = json["name"].as_str().map(String::from);
        
        // 提取头像URL
        let avatar_url = json["avatar_url"].as_str().map(String::from);
        
        // GitHub API 可能不会直接返回邮箱，需要额外请求
        let email = match json["email"].as_str() {
            Some(email) if !email.is_empty() => Some(email.to_string()),
            _ => self.fetch_github_primary_email(access_token).await.ok(),
        };
        
        Ok(OAuthUserInfo {
            id,
            provider: OAuthProvider::GitHub,
            email,
            name,
            avatar_url,
            raw_data,
        })
    }
    
    // 获取GitHub主要电子邮件
    async fn fetch_github_primary_email(&self, access_token: &str) -> Result<String, AuthError> {
        let emails_url = "https://api.github.com/user/emails";
        let response = self.http_client
            .get(emails_url)
            .header("Authorization", format!("token {}", access_token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "SmartLink-App")
            .send()
            .await
            .map_err(|e| AuthError::OAuthError(format!("Failed to fetch GitHub emails: {}", e)))?;
            
        let emails: Vec<Value> = response.json()
            .await
            .map_err(|e| AuthError::OAuthError(format!("Failed to parse GitHub emails: {}", e)))?;
            
        // 寻找主要电子邮件
        for email_obj in emails {
            if let (Some(email), Some(primary)) = (email_obj["email"].as_str(), email_obj["primary"].as_bool()) {
                if primary {
                    return Ok(email.to_string());
                }
            }
        }
        
        Err(AuthError::OAuthError("No primary email found in GitHub account".to_string()))
    }
}