use crate::chat::models::{MessageStatus, MessageType};
// src-tauri/src/chat/commands.rs
use crate::error::Error;
use chrono::Utc;
use mongodb::Database;
use uuid::Uuid;
use std::sync::Arc;
use tauri::{Manager, State};
use tracing::{debug, info};

use super::db::ChatDatabase;
use super::manager::ChatManager;
use super::models::{Conversation, Message, NewConversation, NewMessage, ConversationType};
use super::websocket::{WebSocketConfig, WebSocketState};
use crate::auth::commands::validate_token;

/// 应用状态，包含聊天管理器
pub struct ChatState {
    pub chat_manager: Arc<ChatManager>,
}

/// 初始化聊天模块
pub fn init(app: &mut tauri::App, db: Arc<Database>) -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing chat module");
    
    // 创建聊天数据库和管理器
    let chat_db = ChatDatabase::new(db.as_ref().clone());
    let chat_manager = Arc::new(ChatManager::new(chat_db));
    
    // 创建并管理应用状态
    let chat_state = ChatState {
        chat_manager,
    };
    
    app.manage(chat_state);
    info!("Chat module initialized successfully");
    
    Ok(())
}

/// 获取用户的所有会话
#[tauri::command]
pub async fn get_conversations(
    user_id: String,
    state: State<'_, ChatState>,
) -> Result<Vec<Conversation>, Error> {
    info!("Getting conversations for user: {}", user_id);
    state.chat_manager.get_user_conversations(&user_id).await
}

/// 创建新的会话
#[tauri::command]
pub async fn create_conversation(
    token: String,
    name: Option<String>,
    participants: Vec<String>,
    encryptionEnabled: bool, 
    conversationType: ConversationType,
    state: State<'_, ChatState>,
) -> Result<Conversation, Error> {
    let claims = validate_token(&token)
        .map_err(|_| Error::Authentication("Invalid token".to_string()))?;
    
    let user_id = claims.sub;

    let mut all_participants = participants.clone();
    
    if !all_participants.contains(&user_id) {
        all_participants.push(user_id.clone());
    }
    
    info!("Creating new conversation with {} participants", all_participants.len());
    
    let new_conversation = NewConversation {
        name,
        conversation_type: conversationType,
        participants: all_participants,
        encryption_enabled: encryptionEnabled,
    };
    
    state.chat_manager.create_conversation(new_conversation).await
}

/// 获取会话详情
#[tauri::command]
pub async fn get_conversation(
    conversation_id: String,
    state: State<'_, ChatState>,
) -> Result<Option<Conversation>, Error> {
    info!("Getting conversation details: {}", conversation_id);
    state.chat_manager.get_conversation(&conversation_id).await
}

/// 发送消息
#[tauri::command]
pub async fn send_message(
    conversation_id: String,
    content: String,
    sender_id: String,
    content_type: super::models::MessageType,
    media_url: Option<String>,
    state: State<'_, ChatState>,
) -> Result<Message, Error> {
    debug!("Sending message from {} to conversation {}", sender_id, conversation_id);
    
    let new_message = NewMessage {
        conversation_id,
        sender_id: sender_id.clone(),
        content,
        content_type,
        encrypted: false,
        media_url,
    };
    
    state.chat_manager.send_message(new_message, &sender_id).await
}

/// 获取会话消息历史
#[tauri::command]
pub async fn get_messages(
    conversation_id: String,
    user_id: String,
    limit: Option<u32>,
    before_id: Option<String>,
    state: State<'_, ChatState>,
) -> Result<Vec<Message>, Error> {
    debug!("Getting messages for conversation {} by user {}", conversation_id, user_id);
    
    state.chat_manager.get_messages(
        &conversation_id,
        &user_id,
        limit,
        before_id.as_deref(),
    ).await
}

/// 将消息标记为已读
#[tauri::command]
pub async fn mark_message_read(
    message_id: String,
    user_id: String,
    state: State<'_, ChatState>,
) -> Result<(), Error> {
    debug!("Marking message {} as read by user {}", message_id, user_id);
    state.chat_manager.mark_message_read(&message_id, &user_id).await
}

/// 将会话中所有消息标记为已读
#[tauri::command]
pub async fn mark_conversation_read(
    conversation_id: String,
    user_id: String,
    state: State<'_, ChatState>,
) -> Result<u64, Error> {
    debug!("Marking all messages in conversation {} as read by user {}", 
           conversation_id, user_id);
    
    state.chat_manager.mark_conversation_read(
        &conversation_id,
        &user_id,
    ).await
}

/// 将会话中的消息标记为已送达
#[tauri::command]
pub async fn mark_conversation_delivered(
    conversation_id: String,
    user_id: String,
    state: State<'_, ChatState>,
) -> Result<u64, Error> {
    debug!("Marking messages as delivered in conversation {} for user {}", 
           conversation_id, user_id);
    
    state.chat_manager.mark_conversation_delivered(
        &conversation_id,
        &user_id,
    ).await
}

/// 创建群聊
#[tauri::command]
pub async fn create_group_chat(
    name: String,
    creatorId: String,
    members: Vec<String>,
    encryptionEnabled: bool,
    state: State<'_, ChatState>,
) -> Result<Conversation, Error> {
    debug!("Creating group chat '{}' by user {}", name, creatorId);
    
    state.chat_manager.create_group_chat(
        &name,
        &creatorId,
        members,
        encryptionEnabled,
    ).await
}

/// 添加成员到群聊
#[tauri::command]
pub async fn add_group_member(
    conversation_id: String,
    user_id: String,
    member_id: String,
    state: State<'_, ChatState>,
) -> Result<(), Error> {
    debug!("Adding member {} to group {}", member_id, conversation_id);
    
    state.chat_manager.add_group_member(
        &conversation_id,
        &user_id,
        &member_id,
    ).await
}

/// 从群聊中移除成员
#[tauri::command]
pub async fn remove_group_member(
    conversation_id: String,
    user_id: String,
    member_id: String,
    state: State<'_, ChatState>,
) -> Result<(), Error> {
    debug!("Removing member {} from group {}", member_id, conversation_id);
    
    state.chat_manager.remove_group_member(
        &conversation_id,
        &user_id,
        &member_id,
    ).await
}

/// 获取用户未读消息数
#[tauri::command]
pub async fn get_unread_count(
    user_id: String,
    state: State<'_, ChatState>,
) -> Result<u64, Error> {
    debug!("Getting unread message count for user {}", user_id);
    state.chat_manager.get_unread_count(&user_id).await
}

/// 获取会话中的在线用户
#[tauri::command]
pub async fn get_online_participants(
    conversation_id: String,
    user_id: String,
    state: State<'_, ChatState>,
) -> Result<Vec<String>, Error> {
    debug!("Getting online participants in conversation {}", conversation_id);
    
    state.chat_manager.get_online_participants(&conversation_id, &user_id).await
}

/// 初始化WebSocket客户端
#[tauri::command]
pub async fn initialize_websocket(
    app: tauri::AppHandle,
    serverUrl: String,
    heartbeatIntervalMs: Option<u64>,
    websocket_state: State<'_, WebSocketState>,
) -> Result<(), String> {
    debug!("Initializing WebSocket client with server URL: {}", serverUrl);
    
    let config = WebSocketConfig {
        serverUrl,
        heartbeatIntervalMs: heartbeatIntervalMs.unwrap_or(30000),
    };
    
    websocket_state.initialize(app, config).await;
    Ok(())
}

/// 连接到WebSocket服务器
#[tauri::command]
pub async fn connect_websocket(
    userId: String,
    websocket_state: State<'_, WebSocketState>,
) -> Result<(), String> {
    debug!("Connecting to WebSocket server as user: {}", userId);
    websocket_state.connect(userId).await
}

/// 断开WebSocket连接
#[tauri::command]
pub async fn disconnect_websocket(
    websocket_state: State<'_, WebSocketState>,
) -> Result<(), String> {
    debug!("Disconnecting from WebSocket server");
    websocket_state.disconnect().await
}

/// 获取WebSocket连接状态
#[tauri::command]
pub async fn get_websocket_status(
    websocket_state: State<'_, WebSocketState>,
) -> Result<String, String> {
    let status = websocket_state.get_status().await?;
    Ok(format!("{:?}", status))
}

/// 发送WebSocket消息
#[tauri::command]
pub async fn send_websocket_message(
    message: String,
    websocket_state: State<'_, WebSocketState>,
) -> Result<(), String> {
    debug!("Sending WebSocket message");
    websocket_state.inner().send_message(message).await
}

/// 发送聊天消息
#[tauri::command]
pub async fn send_chat_message(
    conversation_id: String,
    content: String,
    content_type: String,
    encrypted: Option<bool>,
    media_url: Option<String>,
    user_id: String,
    websocket_state: State<'_, WebSocketState>,
) -> Result<Message, String> {
    debug!("Sending chat message to conversation: {}", conversation_id);
    
    // 将字符串转换为 MessageType 枚举
    let message_type = match content_type.to_lowercase().as_str() {
        "text" => MessageType::Text,
        "image" => MessageType::Image,
        "file" => MessageType::File,
        "voice" => MessageType::Voice,
        _ => {
            debug!("Unknown content type: {}, defaulting to Text", content_type);
            MessageType::Text
        }
    };
    
    // 创建消息
    let message = Message {
        id: Uuid::new_v4().to_string(),
        conversation_id,
        sender_id: user_id,
        content,
        content_type: message_type,
        timestamp: Utc::now(),
        status: Some(MessageStatus::Sent),
        encrypted: encrypted.unwrap_or(false),
        media_url,
    };
    
    // 发送消息
    websocket_state.send_chat_message(message.clone()).await?;
    
    Ok(message)
}

/// 应用退出前保存消息
#[tauri::command]
pub async fn before_exit(
    app_handle: tauri::AppHandle,
    websocket_state: tauri::State<'_, WebSocketState>
) -> Result<(), String> {
    let db = app_handle.state::<ChatDatabase>();
    
    websocket_state.save_pending_messages(&db).await?;
    
    websocket_state.disconnect().await?;
    
    Ok(())
}