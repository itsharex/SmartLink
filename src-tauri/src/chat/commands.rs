use crate::auth::utils::get_current_user_id;
use crate::chat::manager::ChatManager;
use crate::chat::models::*;
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};

// 持有聊天相关状态的结构
pub struct ChatState {
    pub manager: Arc<ChatManager>,
}

// 创建新会话
#[tauri::command]
pub async fn create_conversation(
    request: CreateConversationRequest,
    state: State<'_, ChatState>,
    app_handle: AppHandle,
) -> Result<Conversation, String> {
    let user_id = get_current_user_id(&app_handle)?;
    state.manager.create_conversation(&user_id, request).await
}

// 获取当前用户的所有会话
#[tauri::command]
pub async fn get_conversations(
    state: State<'_, ChatState>,
    app_handle: AppHandle,
) -> Result<Vec<Conversation>, String> {
    let user_id = get_current_user_id(&app_handle)?;
    state.manager.get_conversations(&user_id).await
}

// 在会话中发送消息
#[tauri::command]
pub async fn send_message(
    request: SendMessageRequest,
    state: State<'_, ChatState>,
    app_handle: AppHandle,
) -> Result<Message, String> {
    let user_id = get_current_user_id(&app_handle)?;
    state.manager.send_message(&user_id, request).await
}

// 获取会话消息（带分页）
#[tauri::command]
pub async fn get_messages(
    request: GetMessagesRequest,
    state: State<'_, ChatState>,
    app_handle: AppHandle,
) -> Result<Vec<Message>, String> {
    let user_id = get_current_user_id(&app_handle)?;
    state.manager.get_messages(&user_id, request).await
}

// 标记消息为已读
#[tauri::command]
pub async fn mark_as_read(
    request: MarkAsReadRequest,
    state: State<'_, ChatState>,
    app_handle: AppHandle,
) -> Result<bool, String> {
    let user_id = get_current_user_id(&app_handle)?;
    state.manager.mark_as_read(&user_id, request).await
}

// 更新输入状态
#[tauri::command]
pub async fn update_typing_status(
    conversation_id: String,
    is_typing: bool,
    state: State<'_, ChatState>,
    app_handle: AppHandle,
) -> Result<(), String> {
    let user_id = get_current_user_id(&app_handle)?;
    state.manager.update_typing_status(&user_id, &conversation_id, is_typing).await
}