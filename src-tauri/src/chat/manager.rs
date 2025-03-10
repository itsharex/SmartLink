use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use crate::chat::db::ChatDb;
use crate::chat::models::*;

// 跟踪活跃用户连接的结构
#[derive(Debug, Clone)]
struct ActiveUser {
    user_id: String,
    tx: mpsc::Sender<WebSocketEvent>,
}

// 聊天管理器处理业务逻辑
pub struct ChatManager {
    db: ChatDb,
    active_users: Arc<RwLock<HashMap<String, ActiveUser>>>,
}

impl ChatManager {
    pub fn new(db: ChatDb) -> Self {
        Self {
            db,
            active_users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // 创建新会话
    pub async fn create_conversation(&self, user_id: &str, request: CreateConversationRequest) -> Result<Conversation, String> {
        // 验证创建者在参与者列表中
        if !request.participants.contains(&user_id.to_string()) {
            return Err("创建者必须是参与者".to_string());
        }
        
        // 创建会话
        let conversation = self.db.create_conversation(&request).await
            .map_err(|e| format!("创建会话失败: {}", e))?;
            
        // 通知所有在线参与者关于新会话
        let event = WebSocketEvent {
            id: Uuid::new_v4().to_string(),
            event_type: WebSocketEventType::ConversationUpdated,
            conversation_id: conversation.id.clone(),
            data: json!({
                "action": "created",
                "conversation": conversation,
            }),
            timestamp: Utc::now(),
        };
        
        self.broadcast_event(&event, &conversation.participants).await;
        
        Ok(conversation)
    }

    // 获取用户的所有会话
    pub async fn get_conversations(&self, user_id: &str) -> Result<Vec<Conversation>, String> {
        self.db.get_conversations_for_user(user_id).await
            .map_err(|e| format!("获取会话失败: {}", e))
    }

    // 发送消息
    pub async fn send_message(&self, user_id: &str, request: SendMessageRequest) -> Result<Message, String> {
        // 发送消息
        let message = self.db.send_message(user_id, &request).await
            .map_err(|e| format!("发送消息失败: {}", e))?;
            
        // 获取会话以知道需要通知谁
        let conversation = self.db.get_conversation(&request.conversation_id).await
            .map_err(|e| format!("获取会话失败: {}", e))?
            .ok_or_else(|| "会话不存在".to_string())?;
            
        // 创建并广播事件
        let event = WebSocketEvent {
            id: Uuid::new_v4().to_string(),
            event_type: WebSocketEventType::NewMessage,
            conversation_id: conversation.id.clone(),
            data: json!({
                "message": message,
            }),
            timestamp: Utc::now(),
        };
        
        self.broadcast_event(&event, &conversation.participants).await;
        
        Ok(message)
    }

    // 获取会话的消息
    pub async fn get_messages(&self, user_id: &str, request: GetMessagesRequest) -> Result<Vec<Message>, String> {
        // 验证用户是会话的参与者
        let conversation = self.db.get_conversation(&request.conversation_id).await
            .map_err(|e| format!("获取会话失败: {}", e))?
            .ok_or_else(|| "会话不存在".to_string())?;
            
        if !conversation.participants.contains(&user_id.to_string()) {
            return Err("用户不是该会话的参与者".to_string());
        }
        
        // 获取消息
        self.db.get_messages(&request).await
            .map_err(|e| format!("获取消息失败: {}", e))
    }

    // 标记消息为已读
    pub async fn mark_as_read(&self, user_id: &str, request: MarkAsReadRequest) -> Result<bool, String> {
        // 标记消息为已读
        let success = self.db.mark_as_read(user_id, &request).await
            .map_err(|e| format!("标记消息为已读失败: {}", e))?;
            
        if success {
            // 获取会话以知道需要通知谁
            let conversation = self.db.get_conversation(&request.conversation_id).await
                .map_err(|e| format!("获取会话失败: {}", e))?
                .ok_or_else(|| "会话不存在".to_string())?;
                
            // 创建并广播事件
            let event = WebSocketEvent {
                id: Uuid::new_v4().to_string(),
                event_type: WebSocketEventType::MessageRead,
                conversation_id: conversation.id.clone(),
                data: json!({
                    "message_id": request.message_id,
                    "user_id": user_id,
                    "timestamp": Utc::now(),
                }),
                timestamp: Utc::now(),
            };
            
            self.broadcast_event(&event, &conversation.participants).await;
        }
        
        Ok(success)
    }

    // 注册用户为活跃状态（WebSocket连接）
    pub async fn register_user(&self, user_id: &str, tx: mpsc::Sender<WebSocketEvent>) {
        let active_user = ActiveUser {
            user_id: user_id.to_string(),
            tx: tx.clone(),
        };
        
        // 注册用户
        {
            let mut users = self.active_users.write().await;
            users.insert(user_id.to_string(), active_user);
        }
        
        // 获取并发送任何待处理事件
        let pending_events = match self.db.get_pending_events(user_id).await {
            Ok(events) => events,
            Err(e) => {
                eprintln!("获取待处理事件失败: {}", e);
                Vec::new()
            }
        };
        
        if !pending_events.is_empty() {
            let users = self.active_users.read().await;
            if let Some(user) = users.get(user_id) {
                let mut event_ids = Vec::new();
                
                for event in pending_events {
                    if let Err(e) = user.tx.send(event.clone()).await {
                        eprintln!("发送待处理事件失败: {}", e);
                        break;
                    }
                    event_ids.push(event.id);
                }
                
                // 删除已投递的事件
                if !event_ids.is_empty() {
                    if let Err(e) = self.db.delete_events(&event_ids).await {
                        eprintln!("删除已投递事件失败: {}", e);
                    }
                }
            }
        }
        
        // 通知其他人该用户在线
        let conversations = match self.get_conversations(user_id).await {
            Ok(convs) => convs,
            Err(e) => {
                eprintln!("获取用户会话失败: {}", e);
                Vec::new()
            }
        };
        
        for conversation in conversations {
            let event = WebSocketEvent {
                id: Uuid::new_v4().to_string(),
                event_type: WebSocketEventType::ParticipantOnlineStatus,
                conversation_id: conversation.id.clone(),
                data: json!({
                    "user_id": user_id,
                    "status": "online",
                }),
                timestamp: Utc::now(),
            };
            
            self.broadcast_event(&event, &conversation.participants).await;
        }
    }

    // 用户断开连接时取消注册
    pub async fn unregister_user(&self, user_id: &str) {
        // 从活跃用户中移除
        {
            let mut users = self.active_users.write().await;
            users.remove(user_id);
        }
        
        // 通知其他人该用户离线
        let conversations = match self.get_conversations(user_id).await {
            Ok(convs) => convs,
            Err(e) => {
                eprintln!("获取用户会话失败: {}", e);
                Vec::new()
            }
        };
        
        for conversation in conversations {
            let event = WebSocketEvent {
                id: Uuid::new_v4().to_string(),
                event_type: WebSocketEventType::ParticipantOnlineStatus,
                conversation_id: conversation.id.clone(),
                data: json!({
                    "user_id": user_id,
                    "status": "offline",
                }),
                timestamp: Utc::now(),
            };
            
            self.broadcast_event(&event, &conversation.participants).await;
        }
    }

    // 向参与者广播事件
    async fn broadcast_event(&self, event: &WebSocketEvent, participants: &[String]) {
        let users = self.active_users.read().await;
        let mut offline_participants = Vec::new();
        
        // 尝试发送给所有在线参与者
        for participant_id in participants {
            if let Some(user) = users.get(participant_id) {
                if let Err(e) = user.tx.send(event.clone()).await {
                    eprintln!("发送事件失败: {}", e);
                    // 如果发送失败，视为用户离线
                    offline_participants.push(participant_id.clone());
                }
            } else {
                // 用户不活跃，添加到离线列表
                offline_participants.push(participant_id.clone());
            }
        }
        
        // 为离线参与者存储事件
        if !offline_participants.is_empty() {
            if let Err(e) = self.db.store_event(event).await {
                eprintln!("存储离线事件失败: {}", e);
            }
        }
    }

    // 更新输入状态
    pub async fn update_typing_status(&self, user_id: &str, conversation_id: &str, is_typing: bool) -> Result<(), String> {
        // 验证用户是会话的参与者
        let conversation = self.db.get_conversation(conversation_id).await
            .map_err(|e| format!("获取会话失败: {}", e))?
            .ok_or_else(|| "会话不存在".to_string())?;
            
        if !conversation.participants.contains(&user_id.to_string()) {
            return Err("用户不是该会话的参与者".to_string());
        }
        
        // 创建并广播事件
        let event = WebSocketEvent {
            id: Uuid::new_v4().to_string(),
            event_type: WebSocketEventType::ParticipantTyping,
            conversation_id: conversation_id.to_string(),
            data: json!({
                "user_id": user_id,
                "is_typing": is_typing,
            }),
            timestamp: Utc::now(),
        };
        
        self.broadcast_event(&event, &conversation.participants).await;
        
        Ok(())
    }
}