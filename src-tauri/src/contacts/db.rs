// src-tauri/src/contacts/db.rs
use mongodb::{bson::{doc, oid::ObjectId}, Collection, Database};
use crate::error::Error;
use super::models::{User, FriendRequest, FriendRequestStatus, Contact};
use futures::stream::TryStreamExt;

// 获取用户联系人列表
pub async fn get_user_contacts(db: &Database, user_id: &str) -> Result<Vec<User>, Error> {
    let contacts_collection: Collection<Contact> = db.collection("contacts");
    let users_collection: Collection<User> = db.collection("users");
    
    // 查找用户的所有联系人记录
    let cursor = contacts_collection
        .find(doc! { "user_id": user_id }, None)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
    
    let contacts: Vec<Contact> = cursor
        .try_collect()
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
    
    // 获取联系人用户信息
    let mut result = Vec::new();
    for contact in contacts {
        // 先获取 Result<Option<User>, Error>
        let user_result = users_collection
            .find_one(doc! { "id": &contact.contact_id }, None)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;
        
        // 然后处理 Option<User>
        if let Some(mut user_data) = user_result {
            // 如果有标签，添加到用户信息中
            user_data.tags = contact.tags;
            result.push(user_data);
        }
    }
    
    Ok(result)
}

// 获取收藏的联系人
pub async fn get_favorite_contacts(db: &Database, user_id: &str) -> Result<Vec<User>, Error> {
    let contacts_collection: Collection<Contact> = db.collection("contacts");
    let users_collection: Collection<User> = db.collection("users");
    
    // 查找用户收藏的联系人
    let cursor = contacts_collection
        .find(doc! { "user_id": user_id, "is_favorite": true }, None)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
    
    let favorite_contacts: Vec<Contact> = cursor
        .try_collect()
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
    
    // 获取联系人用户信息
    let mut result = Vec::new();
    for contact in favorite_contacts {
        // 直接处理 Result<Option<User>, Error>
        let user_option = users_collection
            .find_one(doc! { "id": &contact.contact_id }, None)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;
        
        // 然后处理 Option<User>
        if let Some(mut user_data) = user_option {
            // 如果有标签，添加到用户信息中
            user_data.tags = contact.tags;
            result.push(user_data);
        }
    }
    
    Ok(result)
}

// 将联系人添加到收藏
pub async fn add_contact_to_favorites(db: &Database, user_id: &str, contact_id: &str) -> Result<(), Error> {
    let contacts_collection: Collection<Contact> = db.collection("contacts");
    
    let update_result = contacts_collection
        .update_one(
            doc! { "user_id": user_id, "contact_id": contact_id },
            doc! { "$set": { "is_favorite": true } },
            None,
        )
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
    
    if update_result.matched_count == 0 {
        return Err(Error::NotFound("Contact not found".to_string()));
    }
    
    Ok(())
}

// 从收藏中移除联系人
pub async fn remove_contact_from_favorites(db: &Database, user_id: &str, contact_id: &str) -> Result<(), Error> {
    let contacts_collection: Collection<Contact> = db.collection("contacts");
    
    let update_result = contacts_collection
        .update_one(
            doc! { "user_id": user_id, "contact_id": contact_id },
            doc! { "$set": { "is_favorite": false } },
            None,
        )
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
    
    if update_result.matched_count == 0 {
        return Err(Error::NotFound("Contact not found".to_string()));
    }
    
    Ok(())
}

// 搜索用户
pub async fn search_users(db: &Database, query: &str) -> Result<Vec<User>, Error> {
    let users_collection: Collection<User> = db.collection("users");
    
    // 创建一个正则表达式进行不区分大小写的搜索
    let query_regex = format!(".*{}.*", regex::escape(query));
    
    let cursor = users_collection
        .find(
            doc! {
                "$or": [
                    { "name": { "$regex": &query_regex, "$options": "i" } },
                    { "email": { "$regex": &query_regex, "$options": "i" } }
                ]
            },
            None,
        )
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
    
    let users: Vec<User> = cursor
        .try_collect()
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
    
    Ok(users)
}

// 发送好友请求
pub async fn create_friend_request(db: &Database, sender_id: &str, recipient_id: &str) -> Result<FriendRequest, Error> {
    // 检查是否已经是好友
    let contacts_collection: Collection<Contact> = db.collection("contacts");
    let existing_contact = contacts_collection
        .find_one(doc! { "user_id": sender_id, "contact_id": recipient_id }, None)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
    
    if existing_contact.is_some() {
        return Err(Error::Validation("Already friends".to_string()));
    }
    
    // 检查是否已经有待处理的请求
    let requests_collection: Collection<FriendRequest> = db.collection("friend_requests");
    let existing_request = requests_collection
        .find_one(
            doc! { 
                "sender_id": sender_id, 
                "recipient_id": recipient_id,
                "status": "pending"
            },
            None,
        )
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
    
    if existing_request.is_some() {
        return Err(Error::Validation("Friend request already sent".to_string()));
    }
    
    // 创建新的好友请求
    let request_id = ObjectId::new().to_hex();
    let new_request = FriendRequest {
        id: request_id,
        sender_id: sender_id.to_string(),
        recipient_id: recipient_id.to_string(),
        status: FriendRequestStatus::Pending,
        created_at: chrono::Utc::now(),
        sender: None,
    };
    
    requests_collection
        .insert_one(&new_request, None)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
    
    Ok(new_request)
}

// 获取好友请求
pub async fn get_friend_requests(db: &Database, user_id: &str) -> Result<Vec<FriendRequest>, Error> {
    let requests_collection: Collection<FriendRequest> = db.collection("friend_requests");
    let users_collection: Collection<User> = db.collection("users");
    
    // 查找发送给用户的待处理请求
    let cursor = requests_collection
        .find(
            doc! { "recipient_id": user_id, "status": "pending" },
            None,
        )
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
    
    let requests: Vec<FriendRequest> = cursor
        .try_collect()
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
    
    // 为每个请求添加发送者信息
    let mut result = Vec::new();
    for request in requests {
        let sender = users_collection
            .find_one(doc! { "id": &request.sender_id }, None)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;
        
        let mut request_with_sender = request;
        request_with_sender.sender = sender;
        result.push(request_with_sender);
    }
    
    Ok(result)
}

// 接受好友请求
pub async fn accept_friend_request(db: &Database, user_id: &str, request_id: &str) -> Result<(), Error> {
    let requests_collection: Collection<FriendRequest> = db.collection("friend_requests");
    let contacts_collection: Collection<Contact> = db.collection("contacts");
    
    // 查找并更新请求状态
    let request = requests_collection
        .find_one_and_update(
            doc! { 
                "id": request_id, 
                "recipient_id": user_id,
                "status": "pending"
            },
            doc! { "$set": { "status": "accepted" } },
            None,
        )
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
    
    if let Some(request) = request {
        // 创建双向联系人关系
        let contact1 = Contact {
            user_id: user_id.to_string(),
            contact_id: request.sender_id.clone(),
            is_favorite: false,
            tags: None,
        };
        
        let contact2 = Contact {
            user_id: request.sender_id,
            contact_id: user_id.to_string(),
            is_favorite: false,
            tags: None,
        };
        
        contacts_collection
            .insert_one(&contact1, None)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;
        
        contacts_collection
            .insert_one(&contact2, None)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;
        
        Ok(())
    } else {
        Err(Error::NotFound("Friend request not found".to_string()))
    }
}

// 拒绝好友请求
pub async fn reject_friend_request(db: &Database, user_id: &str, request_id: &str) -> Result<(), Error> {
    let requests_collection: Collection<FriendRequest> = db.collection("friend_requests");
    
    // 查找并更新请求状态
    let update_result = requests_collection
        .update_one(
            doc! { 
                "id": request_id, 
                "recipient_id": user_id,
                "status": "pending"
            },
            doc! { "$set": { "status": "rejected" } },
            None,
        )
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
    
    if update_result.matched_count == 0 {
        return Err(Error::NotFound("Friend request not found".to_string()));
    }
    
    Ok(())
}
