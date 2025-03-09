use crate::chat::models::*;
use chrono::Utc;
use mongodb::{
    bson::{self, doc, to_bson},
    Collection, Database,
};
use futures::stream::TryStreamExt;
use std::collections::HashMap;
use uuid::Uuid;

// MongoDB集合包装
pub struct ChatDb {
    messages: Collection<Message>,
    conversations: Collection<Conversation>,
    events: Collection<WebSocketEvent>,
}

impl ChatDb {
    pub fn new(db: &Database) -> Self {
        Self {
            messages: db.collection("messages"),
            conversations: db.collection("conversations"),
            events: db.collection("chat_events"),
        }
    }

    // 创建新会话
    pub async fn create_conversation(&self, request: &CreateConversationRequest) -> Result<Conversation, mongodb::error::Error> {
        let now = Utc::now();
        
        let conversation = Conversation {
            id: Uuid::new_v4().to_string(),
            conversation_type: request.conversation_type.clone(),
            participants: request.participants.clone(),
            created_at: now,
            updated_at: now,
            last_message: None,
            encryption_key: None, // 应由加密模块设置
            name: request.name.clone(),
            avatar_url: request.avatar_url.clone(),
        };

        self.conversations.insert_one(&conversation, None).await?;
        Ok(conversation)
    }

    // 获取指定会话
    pub async fn get_conversation(&self, conversation_id: &str) -> Result<Option<Conversation>, mongodb::error::Error> {
        self.conversations.find_one(doc! { "id": conversation_id }, None).await
    }

    // 获取用户的所有会话
    pub async fn get_conversations_for_user(&self, user_id: &str) -> Result<Vec<Conversation>, mongodb::error::Error> {
        let filter = doc! { "participants": user_id };
        let mut cursor = self.conversations.find(filter, None).await?;
        
        let mut conversations = Vec::new();
        while let Some(conversation) = cursor.try_next().await? {
            conversations.push(conversation);
        }
        
        // 按最后消息时间排序（最新的在前）
        conversations.sort_by(|a, b| {
            let a_time = a.last_message.as_ref().map(|m| m.timestamp).unwrap_or(a.updated_at);
            let b_time = b.last_message.as_ref().map(|m| m.timestamp).unwrap_or(b.updated_at);
            b_time.cmp(&a_time)
        });
        
        Ok(conversations)
    }

    // 发送新消息
    pub async fn send_message(&self, user_id: &str, request: &SendMessageRequest) -> Result<Message, mongodb::error::Error> {
        // 首先检查会话是否存在且用户是参与者
        let conversation = match self.get_conversation(&request.conversation_id).await? {
            Some(c) => {
                if !c.participants.contains(&user_id.to_string()) {
                    return Err(mongodb::error::Error::custom("用户不是该会话的参与者"));
                }
                c
            },
            None => return Err(mongodb::error::Error::custom("会话不存在")),
        };
        
        let now = Utc::now();
        let message_id = Uuid::new_v4().to_string();
        
        // 创建已读状态，发送者自动标记为已读
        let mut read_by = HashMap::new();
        read_by.insert(user_id.to_string(), now);
        
        let message = Message {
            id: message_id,
            conversation_id: request.conversation_id.clone(),
            sender_id: user_id.to_string(),
            content: request.content.clone(),
            content_type: request.content_type.clone(),
            timestamp: now,
            read_status: ReadStatus { read_by },
            encrypted: false, // 应由加密模块设置
            encryption_info: None, // 应由加密模块设置
        };
        
        // 插入消息
        self.messages.insert_one(&message, None).await?;
        
        // 更新会话的最后一条消息和更新时间
        let last_message = LastMessagePreview {
            id: message.id.clone(),
            sender_id: message.sender_id.clone(),
            content: message.content.clone(),
            content_type: message.content_type.clone(),
            timestamp: message.timestamp,
            read_by_all: false,
        };
        
        self.conversations.update_one(
            doc! { "id": request.conversation_id.clone() },
            doc! { "$set": {
                "last_message": to_bson(&last_message)?,
                "updated_at": bson::DateTime::from_millis(now.timestamp_millis())
            }},
            None
        ).await?;
        
        Ok(message)
    }

    // 获取会话消息（带分页）
    pub async fn get_messages(&self, request: &GetMessagesRequest) -> Result<Vec<Message>, mongodb::error::Error> {
        let limit = request.limit.unwrap_or(20);
        
        let mut filter = doc! { "conversation_id": &request.conversation_id };
        
        // 如果提供了before_id，添加到筛选条件用于分页
        if let Some(before_id) = &request.before_id {
            let before_message = self.messages.find_one(doc! { "id": before_id }, None).await?;
            if let Some(msg) = before_message {
                filter.insert("timestamp", doc! { "$lt": bson::DateTime::from_millis(msg.timestamp.timestamp_millis()) });
            }
        }
        
        let options = mongodb::options::FindOptions::builder()
            .sort(doc! { "timestamp": -1 })
            .limit(limit as i64)
            .build();
        
        let mut cursor = self.messages.find(filter, options).await?;
        
        let mut messages = Vec::new();
        while let Some(message) = cursor.try_next().await? {
            messages.push(message);
        }
        
        // 反转以获得最旧的消息在前
        messages.reverse();
        
        Ok(messages)
    }

    // 标记消息为已读
    pub async fn mark_as_read(&self, user_id: &str, request: &MarkAsReadRequest) -> Result<bool, mongodb::error::Error> {
        let now = Utc::now();
        
        // 更新消息的已读状态
        let result = self.messages.update_one(
            doc! { 
                "id": &request.message_id,
                "conversation_id": &request.conversation_id
            },
            doc! { "$set": { format!("read_status.read_by.{}", user_id): bson::DateTime::from_millis(now.timestamp_millis()) } },
            None
        ).await?;
        
        // 如果消息已更新，还需检查是否需要更新会话的最后一条消息
        if result.modified_count > 0 {
            let conversation = self.get_conversation(&request.conversation_id).await?;
            if let Some(conv) = conversation {
                if let Some(last_msg) = &conv.last_message {
                    if last_msg.id == request.message_id {
                        // 获取更新后的消息，检查是否所有参与者都已读
                        let message = self.messages.find_one(doc! { "id": &request.message_id }, None).await?;
                        if let Some(msg) = message {
                            let all_read = conv.participants.iter().all(|p| msg.read_status.read_by.contains_key(p));
                            
                            // 如果需要，更新last_message.read_by_all
                            if all_read {
                                self.conversations.update_one(
                                    doc! { "id": &request.conversation_id },
                                    doc! { "$set": { "last_message.read_by_all": true } },
                                    None
                                ).await?;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(result.modified_count > 0)
    }

    // 存储离线用户的事件
    pub async fn store_event(&self, event: &WebSocketEvent) -> Result<(), mongodb::error::Error> {
        self.events.insert_one(event, None).await?;
        Ok(())
    }

    // 获取用户的待处理事件
    pub async fn get_pending_events(&self, user_id: &str) -> Result<Vec<WebSocketEvent>, mongodb::error::Error> {
        // 获取用户参与的所有会话
        let conversations = self.get_conversations_for_user(user_id).await?;
        let conversation_ids: Vec<String> = conversations.iter().map(|c| c.id.clone()).collect();
        
        if conversation_ids.is_empty() {
            return Ok(Vec::new());
        }
        
        // 查询这些会话的事件
        let filter = doc! { "conversation_id": { "$in": &conversation_ids } };
        let mut cursor = self.events.find(filter, None).await?;
        
        let mut events = Vec::new();
        while let Some(event) = cursor.try_next().await? {
            events.push(event);
        }
        
        Ok(events)
    }

    // 删除已投递的事件
    pub async fn delete_events(&self, event_ids: &[String]) -> Result<(), mongodb::error::Error> {
        if !event_ids.is_empty() {
            self.events.delete_many(doc! { "id": { "$in": event_ids } }, None).await?;
        }
        Ok(())
    }
}