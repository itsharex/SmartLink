// src-tauri/src/chat/commands.rs
use crate::error::Error;
use mongodb::Database;
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};
use tracing::{debug, info};

use super::db::ChatDatabase;
use super::manager::ChatManager;
use super::models::{Conversation, Message, NewConversation, NewMessage, ConversationType};
use super::websocket::{ConnectionStatus, WebSocketConfig, WebSocketState};

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
    debug!("Getting conversations for user: {}", user_id);
    state.chat_manager.get_user_conversations(&user_id).await
}

/// 创建新的会话
#[tauri::command]
pub async fn create_conversation(
    name: Option<String>,
    participants: Vec<String>,
    encryption_enabled: bool,
    conversation_type: ConversationType,
    state: State<'_, ChatState>,
) -> Result<Conversation, Error> {
    debug!("Creating new conversation with {} participants", participants.len());
    
    let new_conversation = NewConversation {
        name,
        conversation_type,
        participants,
        encryption_enabled,
    };
    
    state.chat_manager.create_conversation(new_conversation).await
}

/// 获取会话详情
#[tauri::command]
pub async fn get_conversation(
    conversation_id: String,
    state: State<'_, ChatState>,
) -> Result<Option<Conversation>, Error> {
    debug!("Getting conversation details: {}", conversation_id);
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
        encrypted: false, // 由管理器根据会话设置决定是否加密
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
    creator_id: String,
    members: Vec<String>,
    encryption_enabled: bool,
    state: State<'_, ChatState>,
) -> Result<Conversation, Error> {
    debug!("Creating group chat '{}' by user {}", name, creator_id);
    
    state.chat_manager.create_group_chat(
        &name,
        &creator_id,
        members,
        encryption_enabled,
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

/// 初始化WebSocket连接
#[tauri::command]
pub async fn initialize_websocket(
    app_handle: AppHandle,
    server_url: Option<String>,
    websocket_state: State<'_, WebSocketState>,
) -> Result<(), String> {
    let config = WebSocketConfig {
        server_url: server_url.unwrap_or_else(|| "ws://localhost:8080/ws".to_string()),
        ..WebSocketConfig::default()
    };
    
    websocket_state.initialize(app_handle, config).await;
    Ok(())
}

/// 连接到WebSocket服务器
#[tauri::command]
pub async fn connect_websocket(
    user_id: String,
    websocket_state: State<'_, WebSocketState>,
) -> Result<(), String> {
    websocket_state.connect(user_id).await
}

/// 断开WebSocket连接
#[tauri::command]
pub async fn disconnect_websocket(
    websocket_state: State<'_, WebSocketState>,
) -> Result<(), String> {
    websocket_state.disconnect().await
}

/// 获取WebSocket连接状态
#[tauri::command]
pub async fn get_websocket_status(
    websocket_state: State<'_, WebSocketState>,
) -> Result<ConnectionStatus, String> {
    websocket_state.get_status().await
}

/// 发送WebSocket消息
#[tauri::command]
pub async fn send_websocket_message(
    message: String,
    websocket_state: State<'_, WebSocketState>,
) -> Result<(), String> {
    websocket_state.send_message(message).await
}

/// 发送聊天消息 - 便捷包装器
#[tauri::command]
pub async fn send_chat_message(
    conversation_id: String,
    recipient_id: Option<String>,
    content: String,
    sender_id: String,
    message_type: String,
    websocket_state: State<'_, WebSocketState>,
) -> Result<(), String> {
    // 构建标准化的聊天消息格式
    let message = serde_json::json!({
        "message_type": "NewMessage",
        "sender_id": sender_id,
        "conversation_id": conversation_id,
        "recipient_id": recipient_id,
        "data": {
            "content": content,
            "content_type": message_type,
            "timestamp": chrono::Utc::now().to_rfc3339()
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    // 发送消息
    websocket_state.send_message(message.to_string()).await
}

/// 发送通话信令 - 用于WebRTC信令交换
#[tauri::command]
pub async fn send_webrtc_signal(
    recipient_id: String,
    conversation_id: Option<String>,
    signal_type: String,
    signal_data: serde_json::Value,
    sender_id: String,
    websocket_state: State<'_, WebSocketState>,
) -> Result<(), String> {
    // 构建WebRTC信令消息
    let message = serde_json::json!({
        "message_type": "WebRTCSignal",
        "sender_id": sender_id,
        "recipient_id": recipient_id,
        "conversation_id": conversation_id,
        "data": {
            "signal_type": signal_type,
            "signal_data": signal_data,
            "timestamp": chrono::Utc::now().to_rfc3339()
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    // 发送信令
    websocket_state.send_message(message.to_string()).await
}

/// 更新消息状态（已读/已送达）
#[tauri::command]
pub async fn update_message_status(
    message_id: String,
    conversation_id: String,
    original_sender_id: String,
    status: String, // "read" or "delivered"
    user_id: String,
    websocket_state: State<'_, WebSocketState>,
) -> Result<(), String> {
    // 确定消息类型
    let message_type = match status.as_str() {
        "read" => "MessageRead",
        "delivered" => "MessageDelivered",
        _ => return Err(format!("Invalid status: {}", status)),
    };
    
    // 构建状态更新消息
    let message = serde_json::json!({
        "message_type": message_type,
        "sender_id": user_id,
        "recipient_id": original_sender_id,
        "conversation_id": conversation_id,
        "message_id": message_id,
        "data": {
            "originalSenderId": original_sender_id,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    // 发送状态更新
    websocket_state.send_message(message.to_string()).await
}

/// 发送"正在输入"状态
#[tauri::command]
pub async fn send_typing_indicator(
    conversation_id: String,
    is_typing: bool,
    user_id: String,
    recipients: Vec<String>,
    websocket_state: State<'_, WebSocketState>,
) -> Result<(), String> {
    // 构建"正在输入"状态消息
    let message = serde_json::json!({
        "message_type": "TypingIndicator",
        "sender_id": user_id,
        "conversation_id": conversation_id,
        "data": {
            "is_typing": is_typing,
            "recipients": recipients,
            "timestamp": chrono::Utc::now().to_rfc3339()
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    // 发送状态
    websocket_state.send_message(message.to_string()).await
}