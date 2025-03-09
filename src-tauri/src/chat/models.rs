use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 消息内容类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageContentType {
    Text,
    Image,
    File,
    Voice,
    Video,
    Location,
}

// 会话类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConversationType {
    Direct,
    Group,
}

// 消息阅读状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadStatus {
    // 用户ID到阅读时间的映射
    pub read_by: HashMap<String, DateTime<Utc>>,
}

// 加密元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionInfo {
    pub algorithm: String,
    pub key_id: String,
}

// 消息模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub sender_id: String,
    pub content: String,
    pub content_type: MessageContentType,
    pub timestamp: DateTime<Utc>,
    pub read_status: ReadStatus,
    pub encrypted: bool,
    pub encryption_info: Option<EncryptionInfo>,
}

// 会话最后一条消息预览
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastMessagePreview {
    pub id: String,
    pub sender_id: String,
    pub content: String,
    pub content_type: MessageContentType,
    pub timestamp: DateTime<Utc>,
    pub read_by_all: bool,
}

// 会话模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub conversation_type: ConversationType,
    pub participants: Vec<String>, // 用户ID列表
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_message: Option<LastMessagePreview>,
    pub encryption_key: Option<String>,
    pub name: Option<String>, // 群聊名称
    pub avatar_url: Option<String>, // 群聊头像
}

// WebSocket事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebSocketEventType {
    NewMessage,
    MessageRead,
    ConversationUpdated,
    ParticipantTyping,
    ParticipantOnlineStatus,
}

// WebSocket事件负载
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketEvent {
    pub id: String,
    pub event_type: WebSocketEventType,
    pub conversation_id: String,
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

// 请求和响应结构

// 创建会话请求
#[derive(Debug, Deserialize)]
pub struct CreateConversationRequest {
    pub conversation_type: ConversationType,
    pub participants: Vec<String>,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
}

// 发送消息请求
#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub conversation_id: String,
    pub content: String,
    pub content_type: MessageContentType,
}

// 获取消息请求（带分页）
#[derive(Debug, Deserialize)]
pub struct GetMessagesRequest {
    pub conversation_id: String,
    pub limit: Option<usize>,
    pub before_id: Option<String>, // 用于分页
}

// 标记已读请求
#[derive(Debug, Deserialize)]
pub struct MarkAsReadRequest {
    pub conversation_id: String,
    pub message_id: String,
}