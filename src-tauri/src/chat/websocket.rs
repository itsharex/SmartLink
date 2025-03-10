use std::sync::Arc;
use tokio::sync::mpsc;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, Window};

use super::manager::ChatManager;
use super::models::WebSocketEvent;

// 前端到后端的消息类型
#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    // 认证消息
    Authenticate { token: String },
    // 输入状态更新
    TypingStatus { conversation_id: String, is_typing: bool },
    // 接收确认
    AckEvent { event_id: String },
}

// WebSocket管理器
pub struct WebSocketManager {
    pub chat_manager: Arc<ChatManager>,
}

impl WebSocketManager {
    pub fn new(chat_manager: Arc<ChatManager>) -> Self {
        Self {
            chat_manager,
        }
    }

    // 设置前端事件监听，用于处理WebSocket通信
    pub fn setup_frontend_events<R: tauri::Runtime>(
        self: &Arc<Self>,
        window: Window<R>,
        user_id: String,
    ) {
        // 创建通道用于向前端发送事件
        let (tx, mut rx) = mpsc::channel::<WebSocketEvent>(100);
        
        // 将用户注册到聊天管理器
        let chat_manager = self.chat_manager.clone();
        let user_id_clone = user_id.clone();
        
        // 创建一个任务来处理用户注册
        tokio::spawn(async move {
            chat_manager.register_user(&user_id_clone, tx).await;
        });
        
        // 创建一个任务来处理从通道接收的事件并发送到前端
        let window_clone = window.clone();
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                // 将事件发送到前端
                if let Err(e) = window_clone.emit("chat_event", &event) {
                    eprintln!("Error sending event to frontend: {}", e);
                    break;
                }
            }
            
            println!("WebSocket channel for user {} closed", user_id);
        });
    }

    // 处理从前端接收的消息
    pub async fn handle_frontend_message(
        &self,
        user_id: &str,
        message: ClientMessage,
    ) -> Result<(), String> {
        match message {
            ClientMessage::TypingStatus { conversation_id, is_typing } => {
                self.chat_manager.update_typing_status(user_id, &conversation_id, is_typing).await?;
            },
            ClientMessage::AckEvent { event_id: _ } => {
                // 可以实现确认消息接收的逻辑
            },
            ClientMessage::Authenticate { token: _ } => {
                // 可以实现额外的认证逻辑
            }
        }
        
        Ok(())
    }
}

// 用于初始化WebSocket通信的Tauri命令
#[tauri::command]
pub async fn initialize_chat_connection(
    app_handle: AppHandle,
    window: Window,
    user_id: String,
) -> Result<(), String> {
    // 获取WebSocket管理器
    let ws_manager = app_handle.state::<Arc<WebSocketManager>>();
    
    // 设置前端事件监听
    ws_manager.setup_frontend_events(window, user_id);
    
    Ok(())
}

// 用于发送WebSocket消息的Tauri命令
#[tauri::command]
pub async fn send_chat_message(
    app_handle: AppHandle,
    user_id: String,
    message_str: String,
) -> Result<(), String> {
    // 获取WebSocket管理器
    let ws_manager = app_handle.state::<Arc<WebSocketManager>>();
    
    // 将消息字符串解析为ClientMessage
    let message: ClientMessage = serde_json::from_str(&message_str)
        .map_err(|e| format!("无效的消息格式: {}", e))?;
    
    // 处理消息
    ws_manager.handle_frontend_message(&user_id, message).await
}