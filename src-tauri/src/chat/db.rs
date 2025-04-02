// db.rs
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, DateTime as BsonDateTime},
    options::{FindOptions, UpdateOptions},
    Collection, Database,
};
use uuid::Uuid;
use std::time::SystemTime;

use crate::error::Error;
use super::models::{Conversation, Message, MessageStatus, NewConversation, NewMessage};

pub struct ChatDatabase {
    pub messages_collection: Collection<Message>,
    pub conversations_collection: Collection<Conversation>,
}

impl ChatDatabase {
    pub fn new(db: Database) -> Self {
        Self {
            messages_collection: db.collection("messages"),
            conversations_collection: db.collection("conversations"),
        }
    }

    // 会话相关方法
    pub async fn create_conversation(&self, new_conversation: NewConversation) -> Result<Conversation, Error> {
        let now = Utc::now();
        
        let conversation = Conversation {
            id: Uuid::new_v4().to_string(),
            name: new_conversation.name,
            conversation_type: new_conversation.conversation_type,
            participants: new_conversation.participants,
            created_at: now,
            updated_at: now,
            last_message: None,
            encryption_enabled: new_conversation.encryption_enabled,
        };
        
        self.conversations_collection
            .insert_one(&conversation, None)
            .await
            .map_err(|e| Error::Database(format!("Failed to create conversation: {}", e)))?;
        
        Ok(conversation)
    }

    pub async fn get_conversation(&self, conversation_id: &str) -> Result<Option<Conversation>, Error> {
        let filter = doc! { "id": conversation_id };
        
        let conversation = self.conversations_collection
            .find_one(filter, None)
            .await
            .map_err(|e| Error::Database(format!("Failed to get conversation: {}", e)))?;
        
        Ok(conversation)
    }

    pub async fn get_conversations_for_user(&self, user_id: &str) -> Result<Vec<Conversation>, Error> {
        let filter = doc! { "participants": { "$in": [user_id] } };
        let options = FindOptions::builder()
            .sort(doc! { "updated_at": -1 })
            .build();
        
        let cursor = self.conversations_collection
            .find(filter, options)
            .await
            .map_err(|e| Error::Database(format!("Failed to get conversations: {}", e)))?;
        
        let conversations = cursor
            .try_collect()
            .await
            .map_err(|e| Error::Database(format!("Failed to collect conversations: {}", e)))?;
        
        Ok(conversations)
    }

    pub async fn update_conversation_last_message(&self, conversation_id: &str, message: &Message) -> Result<(), Error> {
        let filter = doc! { "id": conversation_id };
        
        // 使用辅助函数转换时间
        let bson_now = chrono_to_bson_datetime(Utc::now());
        
        let update = doc! {
            "$set": {
                "last_message": mongodb::bson::to_document(message)
                    .map_err(|e| Error::Database(format!("Failed to serialize message: {}", e)))?,
                "updated_at": bson_now
            }
        };
        
        self.conversations_collection
            .update_one(filter, update, None)
            .await
            .map_err(|e| Error::Database(format!("Failed to update conversation: {}", e)))?;
        
        Ok(())
    }

    // 消息相关方法
    pub async fn save_message(&self, new_message: NewMessage) -> Result<Message, Error> {
        // 首先检查会话是否存在
        let conversation = self.get_conversation(&new_message.conversation_id).await?
            .ok_or_else(|| Error::NotFound(format!("Conversation not found: {}", new_message.conversation_id)))?;
        
        let now = Utc::now();
        let message = Message {
            id: Uuid::new_v4().to_string(),
            conversation_id: new_message.conversation_id,
            sender_id: new_message.sender_id,
            content: new_message.content,
            content_type: new_message.content_type,
            timestamp: now,
            status: Some(MessageStatus::Sent),
            encrypted: new_message.encrypted,
            media_url: new_message.media_url,
        };
        
        self.messages_collection
            .insert_one(&message, None)
            .await
            .map_err(|e| Error::Database(format!("Failed to save message: {}", e)))?;
        
        Ok(message)
    }

    pub async fn get_messages(&self, conversation_id: &str, limit: Option<u32>, before_id: Option<&str>) -> Result<Vec<Message>, Error> {
        let mut filter = doc! { "conversation_id": conversation_id };
        
        if let Some(before_id) = before_id {
            // 获取指定消息的时间戳
            let before_message = self.messages_collection
                .find_one(doc! { "id": before_id }, None)
                .await
                .map_err(|e| Error::Database(format!("Failed to get reference message: {}", e)))?
                .ok_or_else(|| Error::NotFound(format!("Reference message not found: {}", before_id)))?;
            
            // 使用辅助函数转换时间
            let bson_timestamp = chrono_to_bson_datetime(before_message.timestamp);
            filter.insert("timestamp", doc! { "$lt": bson_timestamp });
        }
        
        let limit_value = limit.unwrap_or(50) as i64;
        let options = FindOptions::builder()
            .sort(doc! { "timestamp": -1 })
            .limit(limit_value)
            .build();
        
        let cursor = self.messages_collection
            .find(filter, options)
            .await
            .map_err(|e| Error::Database(format!("Failed to get messages: {}", e)))?;
        
        let mut messages: Vec<Message> = cursor
            .try_collect()
            .await
            .map_err(|e| Error::Database(format!("Failed to collect messages: {}", e)))?;
        
        // 按时间顺序排序
        messages.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        
        Ok(messages)
    }

    pub async fn update_message_status(&self, message_id: &str, user_id: &str, status: MessageStatus) -> Result<(), Error> {
        // 确保只有消息的接收者可以更新状态
        let message = self.messages_collection
            .find_one(doc! { "id": message_id }, None)
            .await
            .map_err(|e| Error::Database(format!("Failed to find message: {}", e)))?
            .ok_or_else(|| Error::NotFound(format!("Message not found: {}", message_id)))?;
        
        // 获取会话以检查用户是否是参与者
        let conversation = self.get_conversation(&message.conversation_id).await?
            .ok_or_else(|| Error::NotFound(format!("Conversation not found: {}", message.conversation_id)))?;
        
        // 确保用户是会话参与者
        if !conversation.participants.contains(&user_id.to_string()) {
            return Err(Error::Authentication(format!(
                "User {} is not a participant in conversation {}",
                user_id, message.conversation_id
            )));
        }
        
        // 不允许发送者更改已读状态
        if message.sender_id == user_id && status == MessageStatus::Read {
            return Err(Error::Validation("Sender cannot mark their own message as read".to_string()));
        }
        
        // 更新消息状态
        let filter = doc! { "id": message_id };
        let update = doc! {
            "$set": { "status": status.to_string() }
        };
        
        self.messages_collection
            .update_one(filter, update, None)
            .await
            .map_err(|e| Error::Database(format!("Failed to update message status: {}", e)))?;
        
        Ok(())
    }

    pub async fn delete_message(&self, message_id: &str) -> Result<(), Error> {
        let filter = doc! { "id": message_id };
        
        let result = self.messages_collection
            .delete_one(filter, None)
            .await
            .map_err(|e| Error::Database(format!("Failed to delete message: {}", e)))?;
        
        if result.deleted_count == 0 {
            return Err(Error::NotFound(format!("Message not found: {}", message_id)));
        }
        
        Ok(())
    }
    
    // 额外添加的实用方法
    pub async fn mark_messages_as_delivered(&self, conversation_id: &str, user_id: &str) -> Result<u64, Error> {
        // 标记所有发给用户但尚未标记为已送达的消息
        let filter = doc! {
            "conversation_id": conversation_id,
            "sender_id": { "$ne": user_id }, // 非用户发送的消息
            "status": { "$eq": "Sent" } // 仅更新已发送但未送达的消息
        };
        
        let update = doc! {
            "$set": { "status": "Delivered" }
        };
        
        let options = UpdateOptions::builder()
            .build();
        
        let result = self.messages_collection
            .update_many(filter, update, Some(options))
            .await
            .map_err(|e| Error::Database(format!("Failed to mark messages as delivered: {}", e)))?;
        
        Ok(result.modified_count)
    }
    
    pub async fn get_unread_message_count(&self, user_id: &str) -> Result<u64, Error> {
        // 获取所有发给用户但尚未阅读的消息数量
        let filter = doc! {
            "receiver_id": user_id,
            "status": { "$ne": "Read" }
        };
        
        let count = self.messages_collection
            .count_documents(filter, None)
            .await
            .map_err(|e| Error::Database(format!("Failed to count unread messages: {}", e)))?;
        
        Ok(count)
    }
}

fn chrono_to_bson_datetime(dt: chrono::DateTime<Utc>) -> BsonDateTime {
    let system_time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(dt.timestamp() as u64)
        + std::time::Duration::from_nanos(dt.timestamp_subsec_nanos() as u64);
    
    BsonDateTime::from_system_time(system_time)
}
