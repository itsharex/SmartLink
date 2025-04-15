// manager.rs
use super::{
    db::ChatDatabase,
    models::{Conversation, Message, MessageStatus, NewConversation, NewMessage, ConversationType},
    encryption::{Encryption, EncryptedMessage, KeyPair},
};
use crate::error::Error;
use std::{collections::HashMap, time::SystemTime};
use std::sync::{Arc, Mutex, RwLock};
use chrono::Utc;
use mongodb::{bson::{doc, DateTime}, options::UpdateOptions};
use rand::rngs::OsRng;
use tokio::sync::Mutex as TokioMutex;
use tracing::debug;
use serde_json;
use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret};

/// 安全的会话密钥存储
pub struct SessionKeyStore {
    // 会话ID -> 用户ID -> 对应的会话密钥
    keys: RwLock<HashMap<String, HashMap<String, Arc<SharedSecret>>>>,
}

impl SessionKeyStore {
    pub fn new() -> Self {
        Self {
            keys: RwLock::new(HashMap::new()),
        }
    }

    pub fn store_key(&self, conversation_id: &str, user_id: &str, secret: SharedSecret) -> Result<(), Error> {
        let mut keys = self.keys.write().map_err(|_| 
            Error::Internal("Failed to acquire write lock on session keys".to_string()))?;

        let user_keys = keys.entry(conversation_id.to_string())
            .or_insert_with(HashMap::new);
        
        user_keys.insert(user_id.to_string(), Arc::new(secret));
        Ok(())
    }

    pub fn get_key(&self, conversation_id: &str, user_id: &str) -> Result<Option<Arc<SharedSecret>>, Error> {
        let keys = self.keys.read().map_err(|_| 
            Error::Internal("Failed to acquire read lock on session keys".to_string()))?;

        if let Some(user_keys) = keys.get(conversation_id) {
            return Ok(user_keys.get(user_id).cloned());
        }
        
        Ok(None)
    }

    pub fn remove_conversation_keys(&self, conversation_id: &str) -> Result<(), Error> {
        let mut keys = self.keys.write().map_err(|_| 
            Error::Internal("Failed to acquire write lock on session keys".to_string()))?;
        
        keys.remove(conversation_id);
        Ok(())
    }

    pub fn remove_user_keys(&self, conversation_id: &str, user_id: &str) -> Result<(), Error> {
        let mut keys = self.keys.write().map_err(|_| 
            Error::Internal("Failed to acquire write lock on session keys".to_string()))?;
        
        if let Some(user_keys) = keys.get_mut(conversation_id) {
            user_keys.remove(user_id);
        }
        
        Ok(())
    }
}

/// 密钥管理器
pub struct KeyManager {
    encryption: Arc<Mutex<Encryption>>,
    key_store: Arc<TokioMutex<HashMap<String, KeyPair>>>,
}

impl KeyManager {
    pub fn new() -> Self {
        Self {
            encryption: Arc::new(Mutex::new(Encryption::new())),
            key_store: Arc::new(TokioMutex::new(HashMap::new())),
        }
    }

    pub async fn get_or_create_key_pair(&self, user_id: &str) -> Result<(PublicKey, bool), Error> {
        let mut store = self.key_store.lock().await;
        
        // 检查是否已有密钥对
        if let Some(key_pair) = store.get(user_id) {
            return Ok((key_pair.public_key.clone(), false));
        }
        
        // 生成新密钥对
        let mut encryption = self.encryption.lock().map_err(|_| 
            Error::Internal("Failed to lock encryption".to_string()))?;
        
        let key_pair = encryption.generate_key_pair()
            .map_err(|e| Error::Encryption(format!("Failed to generate key pair: {:?}", e)))?;
        
        let public_key = key_pair.public_key.clone();
        store.insert(user_id.to_string(), key_pair);
        
        Ok((public_key, true))
    }

    pub async fn derive_shared_secret(
        &self, 
        user_id: &str, 
        peer_public_key: &PublicKey
    ) -> Result<SharedSecret, Error> {
        let mut store = self.key_store.lock().await;
        
        let key_pair = store.get_mut(user_id).ok_or_else(|| 
            Error::Internal(format!("No key pair found for user {}", user_id)))?;
        
        let encryption = self.encryption.lock().map_err(|_| 
            Error::Internal("Failed to lock encryption".to_string()))?;
        
        let private_key = std::mem::replace(&mut key_pair.private_key, EphemeralSecret::random_from_rng(&mut OsRng));
        
        encryption.derive_shared_secret(private_key, peer_public_key)
            .map_err(|e| Error::Encryption(format!("Failed to derive shared secret: {:?}", e)))
    }

    pub fn encrypt_message(
        &self, 
        content: &str, 
        shared_secret: &SharedSecret
    ) -> Result<EncryptedMessage, Error> {
        let mut encryption = self.encryption.lock().map_err(|_| 
            Error::Internal("Failed to lock encryption".to_string()))?;
        
        encryption.encrypt_message(content, shared_secret)
            .map_err(|e| Error::Encryption(format!("Failed to encrypt message: {:?}", e)))
    }

    pub fn decrypt_message(
        &self, 
        encrypted: &EncryptedMessage, 
        shared_secret: &SharedSecret
    ) -> Result<String, Error> {
        let encryption = self.encryption.lock().map_err(|_| 
            Error::Internal("Failed to lock encryption".to_string()))?;
        
        encryption.decrypt_message(encrypted, shared_secret)
            .map_err(|e| Error::Encryption(format!("Failed to decrypt message: {:?}", e)))
    }
}

/// 聊天会话管理器
pub struct ChatManager {
    db: ChatDatabase,
    key_manager: Arc<KeyManager>,
    session_keys: Arc<SessionKeyStore>,
}

impl ChatManager {
    pub fn new(db: ChatDatabase) -> Self {
        Self {
            db,
            key_manager: Arc::new(KeyManager::new()),
            session_keys: Arc::new(SessionKeyStore::new()),
        }
    }

    /// 创建新的聊天会话
    pub async fn create_conversation(&self, new_conversation: NewConversation) -> Result<Conversation, Error> {
        debug!("Creating new conversation: {:?}", new_conversation);
        
        // 确保至少有两个参与者
        if new_conversation.participants.len() < 2 {
            return Err(Error::Validation("Conversation must have at least 2 participants".to_string()));
        }
        
        // 创建会话
        let conversation = self.db.create_conversation(new_conversation).await?;
        
        // 初始化会话密钥（如果启用加密）
        if conversation.encryption_enabled {
            self.initialize_conversation_keys(&conversation).await?;
        }
        
        Ok(conversation)
    }

    /// 获取指定会话信息
    pub async fn get_conversation(&self, conversation_id: &str) -> Result<Option<Conversation>, Error> {
        self.db.get_conversation(conversation_id).await
    }

    /// 获取用户的所有会话
    pub async fn get_user_conversations(&self, user_id: &str) -> Result<Vec<Conversation>, Error> {
        self.db.get_conversations_for_user(user_id).await
    }

    /// 发送消息
    pub async fn send_message(&self, new_message: NewMessage, user_id: &str) -> Result<Message, Error> {
        debug!("Sending message from user {} to conversation {}", 
               user_id, new_message.conversation_id);
        
        // 验证发送者
        if new_message.sender_id != user_id {
            return Err(Error::Authentication(
                "Sender ID does not match authenticated user".to_string()
            ));
        }
        
        // 获取会话
        let conversation = self.db.get_conversation(&new_message.conversation_id).await?
            .ok_or_else(|| Error::NotFound(format!("Conversation not found: {}", new_message.conversation_id)))?;
        
        // 验证发送者是会话参与者
        if !conversation.participants.contains(&user_id.to_string()) {
            return Err(Error::Authentication(
                format!("User {} is not a participant in conversation {}", 
                       user_id, conversation.id)
            ));
        }
        
        // 处理加密
        let processed_message = if conversation.encryption_enabled {
            self.process_outgoing_encrypted_message(new_message, &conversation).await?
        } else {
            new_message
        };
        
        // 保存消息
        let message = self.db.save_message(processed_message).await?;
        
        // 更新会话的最后一条消息
        self.db.update_conversation_last_message(&message.conversation_id, &message).await?;
        
        Ok(message)
    }

    /// 获取会话消息历史
    pub async fn get_messages(
        &self,
        conversation_id: &str,
        user_id: &str,
        limit: Option<u32>,
        before_id: Option<&str>,
    ) -> Result<Vec<Message>, Error> {
        debug!("Retrieving messages for conversation {} by user {}", 
               conversation_id, user_id);
        
        // 验证用户是会话参与者
        let conversation = self.db.get_conversation(conversation_id).await?
            .ok_or_else(|| Error::NotFound(format!("Conversation not found: {}", conversation_id)))?;
        
        if !conversation.participants.contains(&user_id.to_string()) {
            return Err(Error::Authentication(
                format!("User {} is not a participant in conversation {}", 
                       user_id, conversation_id)
            ));
        }
        
        // 获取消息
        let messages = self.db.get_messages(conversation_id, limit, before_id).await?;
        
        // 如果会话启用了加密，解密消息
        if conversation.encryption_enabled {
            self.process_incoming_encrypted_messages(messages, user_id, conversation_id).await
        } else {
            Ok(messages)
        }
    }

    /// 更新消息状态（已读/已送达）
    pub async fn update_message_status(
        &self,
        message_id: &str,
        user_id: &str,
        status: MessageStatus,
    ) -> Result<(), Error> {
        debug!("Updating message {} status to {:?} by user {}", 
               message_id, status, user_id);
        
        self.db.update_message_status(message_id, user_id, status).await
    }

    /// 标记会话中的所有消息为已送达
    pub async fn mark_conversation_delivered(
        &self, 
        conversation_id: &str, 
        user_id: &str
    ) -> Result<u64, Error> {
        debug!("Marking messages as delivered in conversation {} for user {}", 
               conversation_id, user_id);
        
        self.db.mark_messages_as_delivered(conversation_id, user_id).await
    }

    /// 获取用户未读消息数
    pub async fn get_unread_count(&self, user_id: &str) -> Result<u64, Error> {
        self.db.get_unread_message_count(user_id).await
    }

    /// 创建群聊
    pub async fn create_group_chat(
        &self,
        name: &str,
        creator_id: &str,
        members: Vec<String>,
        encryption_enabled: bool,
    ) -> Result<Conversation, Error> {
        debug!("Creating group chat '{}' by user {} with {} members", 
               name, creator_id, members.len());
        
        let mut participants = members;
        
        // 确保创建者在参与者列表中
        if !participants.contains(&creator_id.to_string()) {
            participants.push(creator_id.to_string());
        }
        
        // 创建会话
        let new_conversation = NewConversation {
            name: Some(name.to_string()),
            conversation_type: ConversationType::Group,
            participants,
            encryption_enabled,
        };
        
        self.create_conversation(new_conversation).await
    }

    /// 添加成员到群聊
    pub async fn add_group_member(
        &self,
        conversation_id: &str,
        user_id: &str,
        new_member_id: &str,
    ) -> Result<(), Error> {
        debug!("Adding member {} to group {} by user {}", 
            new_member_id, conversation_id, user_id);
        
        // 获取会话
        let conversation = self.db.get_conversation(conversation_id).await?
            .ok_or_else(|| Error::NotFound(format!("Conversation not found: {}", conversation_id)))?;
        
        // 验证是群聊
        if conversation.conversation_type != ConversationType::Group {
            return Err(Error::Validation("Not a group conversation".to_string()));
        }
        
        // 验证操作者是群成员
        if !conversation.participants.contains(&user_id.to_string()) {
            return Err(Error::Authentication(
                format!("User {} is not a participant in group {}", 
                    user_id, conversation_id)
            ));
        }
        
        // 检查新成员是否已在群中
        if conversation.participants.contains(&new_member_id.to_string()) {
            return Err(Error::Validation(
                format!("User {} is already a member of group {}", 
                    new_member_id, conversation_id)
            ));
        }
        
        // 更新会话的参与者列表
        let filter = doc! { "id": conversation_id };
        
        // 直接使用 MongoDB 的 BSON 日期时间类型
        let current_time = mongodb::bson::DateTime::from_system_time(std::time::SystemTime::now());
        
        let update = doc! {
            "$push": { "participants": new_member_id },
            "$set": { "updated_at": current_time }
        };
        
        self.db.conversations_collection
            .update_one(filter, update, None)
            .await
            .map_err(|e| Error::Database(format!("Failed to update conversation participants: {}", e)))?;
        
        // 如果启用了加密，需要为新成员建立密钥
        if conversation.encryption_enabled {
            // 为新成员与群中每个现有成员建立密钥
            for existing_member in &conversation.participants {
                if existing_member != new_member_id {
                    self.establish_secure_channel(existing_member, new_member_id, conversation_id).await?;
                }
            }
        }
        
        Ok(())
    }

    /// 从群聊中移除成员
    pub async fn remove_group_member(
        &self,
        conversation_id: &str,
        user_id: &str,
        member_to_remove: &str,
    ) -> Result<(), Error> {
        debug!("Removing member {} from group {} by user {}", 
            member_to_remove, conversation_id, user_id);
        
        // 获取会话
        let conversation = self.db.get_conversation(conversation_id).await?
            .ok_or_else(|| Error::NotFound(format!("Conversation not found: {}", conversation_id)))?;
        
        // 验证是群聊
        if conversation.conversation_type != ConversationType::Group {
            return Err(Error::Validation("Not a group conversation".to_string()));
        }
        
        // 验证操作者是群成员
        if !conversation.participants.contains(&user_id.to_string()) {
            return Err(Error::Authentication(
                format!("User {} is not a participant in group {}", 
                    user_id, conversation_id)
            ));
        }
        
        // 检查要移除的成员是否在群中
        if !conversation.participants.contains(&member_to_remove.to_string()) {
            return Err(Error::Validation(
                format!("User {} is not a member of group {}", 
                    member_to_remove, conversation_id)
            ));
        }
        
        // 更新会话的参与者列表
        let filter = doc! { "id": conversation_id };
        let update = doc! {
            "$pull": { "participants": member_to_remove },
            "$set": { "updated_at": chrono_to_bson_datetime(Utc::now()) }
        };
        
        self.db.conversations_collection
            .update_one(filter, update, None)
            .await
            .map_err(|e| Error::Database(format!("Failed to update conversation participants: {}", e)))?;
        
        // 如果启用了加密，吊销该成员的密钥
        if conversation.encryption_enabled {
            // 移除该用户在此会话中的所有密钥
            self.session_keys.remove_user_keys(conversation_id, member_to_remove)?;
            
            // 为安全起见，在实际应用中应该为剩余成员重新生成密钥
            // 这需要密钥轮换机制，更复杂的实现会超出示例范围
        }
        
        Ok(())
    }

    /// 初始化会话密钥
    async fn initialize_conversation_keys(&self, conversation: &Conversation) -> Result<(), Error> {
        debug!("Initializing encryption keys for conversation {}", conversation.id);
        
        // 为会话中的每对用户创建共享密钥
        for (i, user_a) in conversation.participants.iter().enumerate() {
            for user_b in conversation.participants.iter().skip(i + 1) {
                // 为每对用户创建共享密钥
                self.establish_secure_channel(user_a, user_b, &conversation.id).await?;
            }
        }
        
        Ok(())
    }

    /// 为两个用户建立安全通道
    async fn establish_secure_channel(
        &self, 
        user_a: &str, 
        user_b: &str, 
        conversation_id: &str
    ) -> Result<(), Error> {
        debug!("Establishing secure channel between {} and {} for conversation {}", 
               user_a, user_b, conversation_id);
        
        // 获取或创建用户A的密钥对
        let (public_key_a, _) = self.key_manager.get_or_create_key_pair(user_a).await?;
        
        // 获取或创建用户B的密钥对
        let (public_key_b, _) = self.key_manager.get_or_create_key_pair(user_b).await?;
        
        // 计算用户A -> 用户B的共享密钥
        let shared_secret_a = self.key_manager.derive_shared_secret(user_a, &public_key_b).await?;
        
        // 存储用户A的会话密钥
        self.session_keys.store_key(conversation_id, user_a, shared_secret_a)?;
        
        // 计算用户B -> 用户A的共享密钥
        let shared_secret_b = self.key_manager.derive_shared_secret(user_b, &public_key_a).await?;
        
        // 存储用户B的会话密钥
        self.session_keys.store_key(conversation_id, user_b, shared_secret_b)?;
        
        Ok(())
    }

    /// 处理传出的加密消息
    async fn process_outgoing_encrypted_message(
        &self, 
        new_message: NewMessage, 
        conversation: &Conversation
    ) -> Result<NewMessage, Error> {
        let sender_id = &new_message.sender_id;
        let content = &new_message.content;
        
        // 获取发送者的会话密钥
        let shared_secret = self.session_keys.get_key(&conversation.id, sender_id)?
            .ok_or_else(|| Error::Encryption(format!(
                "No session key found for user {} in conversation {}", 
                sender_id, conversation.id
            )))?;
        
        // 加密消息
        let encrypted = self.key_manager.encrypt_message(content, &shared_secret)?;
        
        // 序列化加密消息
        let encrypted_json = serde_json::to_string(&encrypted)
            .map_err(|e| Error::Internal(format!("Failed to serialize encrypted message: {}", e)))?;
        
        // 创建含加密内容的新消息
        let mut encrypted_message = new_message;
        encrypted_message.content = encrypted_json;
        encrypted_message.encrypted = true;
        
        Ok(encrypted_message)
    }

    /// 处理传入的加密消息
    async fn process_incoming_encrypted_messages(
        &self, 
        messages: Vec<Message>, 
        user_id: &str, 
        conversation_id: &str
    ) -> Result<Vec<Message>, Error> {
        let mut processed_messages = Vec::with_capacity(messages.len());
        
        for mut message in messages {
            if message.encrypted {
                // 获取接收者的会话密钥
                let shared_secret = self.session_keys.get_key(conversation_id, user_id)?
                    .ok_or_else(|| Error::Encryption(format!(
                        "No session key found for user {} in conversation {}", 
                        user_id, conversation_id
                    )))?;
                
                // 反序列化加密消息
                let encrypted: EncryptedMessage = serde_json::from_str(&message.content)
                    .map_err(|e| Error::Internal(format!("Failed to deserialize encrypted message: {}", e)))?;
                
                // 解密消息
                let decrypted_content = self.key_manager.decrypt_message(&encrypted, &shared_secret)?;
                
                // 更新消息内容为解密后的文本
                message.content = decrypted_content;
            }
            
            processed_messages.push(message);
        }
        
        Ok(processed_messages)
    }

    /// 将消息标记为已读
    pub async fn mark_message_read(
        &self, 
        message_id: &str, 
        user_id: &str
    ) -> Result<(), Error> {
        debug!("Marking message {} as read by user {}", message_id, user_id);
        
        self.db.update_message_status(message_id, user_id, MessageStatus::Read).await
    }

    /// 将会话中所有消息标记为已读
    pub async fn mark_conversation_read(
        &self, 
        conversation_id: &str, 
        user_id: &str
    ) -> Result<u64, Error> {
        debug!("Marking all messages in conversation {} as read by user {}", 
            conversation_id, user_id);
        
        // 获取会话
        let conversation = self.db.get_conversation(conversation_id).await?
            .ok_or_else(|| Error::NotFound(format!("Conversation not found: {}", conversation_id)))?;
        
        // 验证用户是会话参与者
        if !conversation.participants.contains(&user_id.to_string()) {
            return Err(Error::Authentication(
                format!("User {} is not a participant in conversation {}", 
                    user_id, conversation_id)
            ));
        }
        
        // 更新所有未读消息的状态
        let filter = doc! {
            "conversation_id": conversation_id,
            "sender_id": { "$ne": user_id }, // 只处理不是用户自己发送的消息
            "$or": [
                { "status": "Sent" },
                { "status": "Delivered" }
            ]
        };
        
        let update = doc! {
            "$set": { "status": MessageStatus::Read.to_string() }
        };
        
        let options = UpdateOptions::builder().build();
        
        let result = self.db.messages_collection
            .update_many(filter, update, Some(options))
            .await
            .map_err(|e| Error::Database(format!("Failed to mark messages as read: {}", e)))?;
        
        Ok(result.modified_count)
    }


    /// 获取会话中的在线用户
    pub async fn get_online_participants(
        &self, 
        conversation_id: &str, 
        user_id: &str
    ) -> Result<Vec<String>, Error> {
        debug!("Getting online participants in conversation {} for user {}", 
               conversation_id, user_id);
        
        // 获取会话
        let conversation = self.db.get_conversation(conversation_id).await?
            .ok_or_else(|| Error::NotFound(format!("Conversation not found: {}", conversation_id)))?;
        
        // 验证用户是会话参与者
        if !conversation.participants.contains(&user_id.to_string()) {
            return Err(Error::Authentication(
                format!("User {} is not a participant in conversation {}", 
                       user_id, conversation_id)
            ));
        }
        
        // 实际实现中应该查询哪些用户在线
        // 这里应该集成在线状态服务
        
        // 返回所有参与者（假设都在线）
        Ok(conversation.participants.clone())
    }
}

fn chrono_to_bson_datetime(dt: chrono::DateTime<Utc>) -> DateTime {
    let system_time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(dt.timestamp() as u64)
        + std::time::Duration::from_nanos(dt.timestamp_subsec_nanos() as u64);
    
    DateTime::from_system_time(system_time)
}