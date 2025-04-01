// src-tauri/src/contacts/commands.rs
use mongodb::Database;
use tauri::{command, AppHandle, Manager};

use super::{db, models::{User, FriendRequest}};

// 获取用户的联系人列表
#[command]
pub async fn get_contacts(app_handle: AppHandle, user_id: String) -> Result<Vec<User>, String> {
    let db = app_handle.state::<Database>();
    
    db::get_user_contacts(&db, &user_id)
        .await
        .map_err(|e| e.to_string())
}

// 获取收藏的联系人
#[command]
pub async fn get_favorite_contacts(app_handle: AppHandle, user_id: String) -> Result<Vec<User>, String> {
    let db = app_handle.state::<Database>();
    
    db::get_favorite_contacts(&db, &user_id)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn search_users(app_handle: AppHandle, query: String) -> Result<Vec<User>, String> {
    let db = app_handle.state::<Database>();
    
    tracing::info!("search_users called with query: {}", query); // Log the query
    let results = db::search_users(&db, &query)
        .await
        .map_err(|e| e.to_string())?;
    
    tracing::info!("search_users found {} results", results.len()); // Log the number of results
    Ok(results)
}

// 发送好友请求
#[command]
pub async fn send_friend_request(app_handle: AppHandle, sender_id: String, recipient_id: String) -> Result<(), String> {
    tracing::info!("Received friend request from: {} to: {}", sender_id, recipient_id);
    let db = app_handle.state::<Database>();
    
    db::create_friend_request(&db, &sender_id, &recipient_id)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}

// 获取好友请求
#[command]
pub async fn get_friend_requests(app_handle: AppHandle, user_id: String) -> Result<Vec<FriendRequest>, String> {
    let db = app_handle.state::<Database>();
    
    db::get_friend_requests(&db, &user_id)
        .await
        .map_err(|e| e.to_string())
}

// 接受好友请求
#[command]
pub async fn accept_friend_request(app_handle: AppHandle, user_id: String, request_id: String) -> Result<(), String> {
    let db = app_handle.state::<Database>();
    
    db::accept_friend_request(&db, &user_id, &request_id)
        .await
        .map_err(|e| e.to_string())
}

// 拒绝好友请求
#[command]
pub async fn reject_friend_request(app_handle: AppHandle, user_id: String, request_id: String) -> Result<(), String> {
    let db = app_handle.state::<Database>();
    
    db::reject_friend_request(&db, &user_id, &request_id)
        .await
        .map_err(|e| e.to_string())
}

// 添加联系人到收藏
#[command]
pub async fn add_contact_to_favorites(app_handle: AppHandle, user_id: String, contact_id: String) -> Result<(), String> {
    let db = app_handle.state::<Database>();
    
    db::add_contact_to_favorites(&db, &user_id, &contact_id)
        .await
        .map_err(|e| e.to_string())
}

// 从收藏中移除联系人
#[command]
pub async fn remove_contact_from_favorites(app_handle: AppHandle, user_id: String, contact_id: String) -> Result<(), String> {
    let db = app_handle.state::<Database>();
    
    db::remove_contact_from_favorites(&db, &user_id, &contact_id)
        .await
        .map_err(|e| e.to_string())
}

// 提供初始化函数
pub fn init(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let _ = app;
    // 如果有需要的初始化代码，放在这里
    tracing::info!("Contacts module initialized");
    Ok(())
}