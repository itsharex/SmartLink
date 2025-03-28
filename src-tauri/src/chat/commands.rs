// src-tauri/src/chat/commands.rs
use crate::error::Error;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{Manager, State};
use tracing::{debug, info};

use super::db::ChatDatabase;
use super::manager::ChatManager;
use super::models::{Conversation, Message, MessageStatus, NewConversation, NewMessage, ConversationType};

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

/// 更新消息状态（已读/已送达）
#[tauri::command]
pub async fn update_message_status(
    message_id: String,
    user_id: String,
    status: MessageStatus,
    state: State<'_, ChatState>,
) -> Result<(), Error> {
    debug!("Updating message {} status by user {}", message_id, user_id);
    
    state.chat_manager.update_message_status(
        &message_id,
        &user_id,
        status,
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