// models.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageType {
    Text,
    Image,
    File,
    Voice,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum MessageStatus {
    Sent,
    Delivered,
    Read,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub sender_id: String,
    pub content: String,
    pub content_type: MessageType,
    pub timestamp: DateTime<Utc>,
    pub status: Option<MessageStatus>,
    pub encrypted: bool,
    pub media_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ConversationType {
    Direct,
    Group,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Conversation {
    pub id: String,
    pub name: Option<String>, 
    pub conversation_type: ConversationType,
    pub participants: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_message: Option<Message>,
    pub encryption_enabled: bool,
}

// 用于创建新消息的简化结构
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewMessage {
    pub conversation_id: String,
    pub sender_id: String,
    pub content: String,
    pub content_type: MessageType,
    pub media_url: Option<String>,
    pub encrypted: bool,
}

// 用于创建新会话的简化结构
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewConversation {
    pub name: Option<String>,
    pub conversation_type: ConversationType,
    pub participants: Vec<String>,
    pub encryption_enabled: bool,
}

// 用于更新消息状态的结构
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageStatusUpdate {
    pub message_id: String,
    pub user_id: String,
    pub status: MessageStatus,
}

impl std::fmt::Display for MessageStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sent => write!(f, "Sent"),
            Self::Delivered => write!(f, "Delivered"),
            Self::Read => write!(f, "Read"),
        }
    }
}