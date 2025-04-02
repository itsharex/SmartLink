use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::{Mutex, RwLock};
use tokio_tungstenite::{
    connect_async,
    tungstenite::Message,
};
use tracing::{debug, error, info};
use url::Url;
use crate::chat::models::{Message as ClientMessage};

use super::models::MessageType;

/// WebSocket连接状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ConnectionStatus {
    /// 未连接
    Disconnected,
    /// 连接中
    Connecting,
    /// 已连接
    Connected,
}

/// WebSocket客户端配置
#[derive(Debug, Clone, Deserialize)]
pub struct WebSocketConfig {
    /// WebSocket服务器URL
    pub server_url: String,
    /// 心跳间隔（毫秒）
    pub heartbeat_interval_ms: u64,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            server_url: "wss://smartlink-server-production.up.railway.app".to_string(),
            heartbeat_interval_ms: 30000,
        }
    }
}
/// 本地消息缓存
#[derive(Debug, Default)]
pub struct MessageCache {
    // 待保存到数据库的消息
    pending_messages: Vec<ClientMessage>,
}

impl MessageCache {
    pub fn new() -> Self {
        Self {
            pending_messages: Vec::new(),
        }
    }

    // 添加消息到缓存
    pub fn add_message(&mut self, message: ClientMessage) {
        self.pending_messages.push(message);
    }

    // 获取并清空缓存
    pub fn take_pending_messages(&mut self) -> Vec<ClientMessage> {
        std::mem::take(&mut self.pending_messages)
    }
}

/// WebSocket客户端
pub struct WebSocketClient {
    config: WebSocketConfig,
    status: Arc<RwLock<ConnectionStatus>>,
    user_id: Arc<RwLock<Option<String>>>,
    app_handle: AppHandle,
    message_cache: Arc<Mutex<MessageCache>>,
    tx: Arc<Mutex<Option<tokio::sync::mpsc::UnboundedSender<Message>>>>,
}

impl WebSocketClient {
    /// 创建新的WebSocket客户端
    pub fn new(app_handle: AppHandle, config: WebSocketConfig) -> Self {
        Self {
            config,
            status: Arc::new(RwLock::new(ConnectionStatus::Disconnected)),
            user_id: Arc::new(RwLock::new(None)),
            app_handle,
            message_cache: Arc::new(Mutex::new(MessageCache::new())),
            tx: Arc::new(Mutex::new(None)),
        }
    }

    /// 连接到WebSocket服务器
    pub async fn connect(&self, user_id: String) -> Result<(), String> {
        // 设置状态为连接中
        *self.status.write().await = ConnectionStatus::Connecting;
        *self.user_id.write().await = Some(user_id.clone());
        
        // 构建URL
        let connect_url = format!("{}?user_id={}", self.config.server_url, user_id);
        let url = Url::parse(&connect_url).map_err(|e| format!("Invalid URL: {}", e))?;
        
        // 连接到WebSocket服务器
        debug!("Connecting to WebSocket server: {}", url);
        let ws_stream = match connect_async(url).await {
            Ok((ws_stream, _)) => ws_stream,
            Err(e) => {
                error!("Failed to connect to WebSocket server: {}", e);
                *self.status.write().await = ConnectionStatus::Disconnected;
                return Err(format!("Failed to connect: {}", e));
            }
        };
        
        // 设置状态为已连接
        debug!("Connected to WebSocket server");
        *self.status.write().await = ConnectionStatus::Connected;
        
        // 分离WebSocket读写流
        let (mut write, mut read) = ws_stream.split();
        
        // 发送认证消息
        let auth_message = self.create_auth_message(&user_id);
        if let Err(e) = write.send(Message::Text(auth_message)).await {
            error!("Failed to send authentication message: {}", e);
            *self.status.write().await = ConnectionStatus::Disconnected;
            return Err(format!("Failed to send auth message: {}", e));
        }
        
        // 创建发送通道
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();
        
        // 保存发送通道
        {
            let mut sender = self.tx.lock().await;
            *sender = Some(tx.clone());
        }
        
        // 克隆引用
        let app_handle = self.app_handle.clone();
        let status = self.status.clone();
        
        // 发送任务
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Err(e) = write.send(msg).await {
                    error!("Failed to send WebSocket message: {}", e);
                    break;
                }
            }
            debug!("Send task terminated");
        });
        
        // 接收任务
        tokio::spawn(async move {
            while let Some(message_result) = read.next().await {
                match message_result {
                    Ok(msg) => {
                        match msg {
                            Message::Text(text) => {
                                // 收到消息，直接发送到前端
                                debug!("Received text message");
                                if let Err(e) = app_handle.emit("chat_event", text) {
                                    error!("Failed to emit chat event: {}", e);
                                }
                            }
                            Message::Ping(data) => {
                                // 响应Ping
                                debug!("Received ping");
                                if tx.send(Message::Pong(data)).is_err() {
                                    break;
                                }
                            }
                            Message::Close(_) => {
                                // 连接关闭
                                break;
                            }
                            _ => {} // 忽略其他消息类型
                        }
                    }
                    Err(e) => {
                        error!("WebSocket error: {}", e);
                        break;
                    }
                }
            }
            
            // 连接关闭，更新状态
            *status.write().await = ConnectionStatus::Disconnected;
            debug!("Connection closed");
        });
        
        // 启动心跳
        self.start_heartbeat();
        
        Ok(())
    }
    
    /// 启动心跳
    fn start_heartbeat(&self) {
        let tx = self.tx.clone();
        let status = self.status.clone();
        let interval = tokio::time::Duration::from_millis(self.config.heartbeat_interval_ms);
        
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(interval);
            loop {
                ticker.tick().await;
                
                // 检查连接状态
                if *status.read().await != ConnectionStatus::Connected {
                    break;
                }
                
                // 发送心跳
                let sender = tx.lock().await;
                if let Some(sender) = &*sender {
                    if sender.send(Message::Ping(vec![])).is_err() {
                        break;
                    }
                } else {
                    break;
                }
            }
        });
    }
    
    /// 断开连接
    pub async fn disconnect(&self) -> Result<(), String> {
        if *self.status.write().await == ConnectionStatus::Disconnected {
            return Err("Already disconnected".to_string());
        }
        
        // 设置状态
        *self.status.write().await = ConnectionStatus::Disconnected;
        
        // 关闭发送通道
        let mut sender = self.tx.lock().await;
        *sender = None;
        
        Ok(())
    }
    
    /// 发送消息
    pub async fn send_message(&self, message: String) -> Result<(), String> {
        // 检查连接状态
        if *self.status.read().await != ConnectionStatus::Connected {
            return Err("Not connected".to_string());
        }
        
        // 获取发送通道
        let sender = self.tx.lock().await;
        if let Some(tx) = &*sender {
            tx.send(Message::Text(message))
                .map_err(|e| format!("Failed to send message: {}", e))?;
            Ok(())
        } else {
            Err("No active connection".to_string())
        }
    }
    
    /// 发送聊天消息
    pub async fn send_chat_message(&self, message: ClientMessage) -> Result<(), String> {
        // 检查连接状态
        if *self.status.read().await != ConnectionStatus::Connected {
            return Err("Not connected to WebSocket server".to_string());
        }
        
        // 创建WebSocket消息
        let ws_message = serde_json::json!({
            "message_type": "NewMessage",
            "sender_id": message.sender_id,
            "conversation_id": message.conversation_id,
            "message_id": message.id,
            "data": {
                "content": message.content,
                "content_type": message.content_type,
                "encrypted": message.encrypted,
                "media_url": message.media_url,
            },
            "timestamp": message.timestamp.to_rfc3339()
        }).to_string();
        
        // 添加到本地缓存
        let mut cache = self.message_cache.lock().await;
        cache.add_message(message);
        
        // 发送消息
        self.send_message(ws_message).await
    }
    
    /// 获取当前状态
    pub async fn get_status(&self) -> ConnectionStatus {
        *self.status.read().await
    }
    
    /// 创建认证消息
    fn create_auth_message(&self, user_id: &str) -> String {
        serde_json::json!({
            "message_type": "UserStatus",
            "sender_id": user_id,
            "data": {
                "status": "online"
            },
            "timestamp": chrono::Utc::now().to_rfc3339()
        }).to_string()
    }
}

/// WebSocket状态，用于Tauri状态管理
pub struct WebSocketState {
    client: Arc<Mutex<Option<WebSocketClient>>>,
}

impl WebSocketState {
    /// 创建新的状态
    pub fn new() -> Self {
        Self {
            client: Arc::new(Mutex::new(None)),
        }
    }

    /// 初始化WebSocket客户端
    pub async fn initialize(&self, app_handle: AppHandle, config: WebSocketConfig) {
        let mut client = self.client.lock().await;
        *client = Some(WebSocketClient::new(app_handle, config));
    }

    /// 连接WebSocket服务器
    pub async fn connect(&self, user_id: String) -> Result<(), String> {
        let client = self.client.lock().await;
        match &*client {
            Some(ws_client) => ws_client.connect(user_id).await,
            None => Err("WebSocket client not initialized".to_string()),
        }
    }

    /// 断开WebSocket连接
    pub async fn disconnect(&self) -> Result<(), String> {
        let client = self.client.lock().await;
        match &*client {
            Some(ws_client) => ws_client.disconnect().await,
            None => Err("WebSocket client not initialized".to_string()),
        }
    }

    /// 获取当前连接状态
    pub async fn get_status(&self) -> Result<ConnectionStatus, String> {
        let client = self.client.lock().await;
        match &*client {
            Some(ws_client) => Ok(ws_client.get_status().await),
            None => Err("WebSocket client not initialized".to_string()),
        }
    }

    pub async fn send_message(&self, message: String) -> Result<(), String> {
        let client = self.client.lock().await;
        match &*client {
            Some(ws_client) => ws_client.send_message(message).await,
            None => Err("WebSocket client not initialized".to_string()),
        }
    }

    /// 发送WebSocket消息
    pub async fn send_chat_message(&self, message: ClientMessage) -> Result<(), String> {
        let client = self.client.lock().await;
        match &*client {
            Some(ws_client) => ws_client.send_chat_message(message).await,
            None => Err("WebSocket client not initialized".to_string()),
        }
    }
    
    /// 保存缓存的消息到数据库
    pub async fn save_pending_messages(&self) -> Result<(), String> {
        // 此处应实现保存逻辑，例如在程序退出时调用
        // 简化实现，仅返回成功
        Ok(())
    }
}