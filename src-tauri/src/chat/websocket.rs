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

/// WebSocket连接状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ConnectionStatus {
    /// 未连接
    Disconnected,
    /// 正在连接
    Connecting,
    /// 已连接
    Connected,
    /// 重新连接中
    Reconnecting,
}

/// WebSocket客户端配置
#[derive(Debug, Clone, Deserialize)]
pub struct WebSocketConfig {
    /// WebSocket服务器URL
    pub server_url: String,
    /// 最大重连尝试次数
    pub max_reconnect_attempts: u32,
    /// 重连延迟（毫秒）
    pub reconnect_delay_ms: u64,
    /// 心跳间隔（毫秒）
    pub heartbeat_interval_ms: u64,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            server_url: "ws://localhost:8080/ws".to_string(),
            max_reconnect_attempts: 5,
            reconnect_delay_ms: 2000,
            heartbeat_interval_ms: 30000,
        }
    }
}

/// WebSocket客户端状态
pub struct WebSocketClient {
    config: WebSocketConfig,
    status: Arc<RwLock<ConnectionStatus>>,
    user_id: Arc<RwLock<Option<String>>>,
    app_handle: AppHandle,
}

impl WebSocketClient {
    /// 创建新的WebSocket客户端
    pub fn new(app_handle: AppHandle, config: WebSocketConfig) -> Self {
        Self {
            config,
            status: Arc::new(RwLock::new(ConnectionStatus::Disconnected)),
            user_id: Arc::new(RwLock::new(None)),
            app_handle,
        }
    }

    /// 连接到WebSocket服务器
    pub async fn connect(&self, user_id: String) -> Result<(), String> {
        // 更新状态为连接中
        *self.status.write().await = ConnectionStatus::Connecting;
        *self.user_id.write().await = Some(user_id.clone());

        // 发射状态变更事件
        self.emit_status_change(ConnectionStatus::Connecting).await;

        // 构建连接URL，添加用户ID参数
        let connect_url = format!("{}?user_id={}", self.config.server_url, user_id);
        let url = Url::parse(&connect_url).map_err(|e| format!("Invalid URL: {}", e))?;

        // 连接到WebSocket服务器
        debug!("Connecting to WebSocket server: {}", url);
        let ws_stream = match connect_async(url).await {
            Ok((ws_stream, _)) => ws_stream,
            Err(e) => {
                error!("Failed to connect to WebSocket server: {}", e);
                *self.status.write().await = ConnectionStatus::Disconnected;
                self.emit_status_change(ConnectionStatus::Disconnected).await;
                return Err(format!("Failed to connect: {}", e));
            }
        };

        // 更新状态为已连接
        debug!("Connected to WebSocket server");
        *self.status.write().await = ConnectionStatus::Connected;
        self.emit_status_change(ConnectionStatus::Connected).await;

        // 分离WebSocket读写流
        let (mut write, mut read) = ws_stream.split();

        // 发送初始认证消息
        let auth_message = self.create_auth_message(&user_id);
        if let Err(e) = write.send(Message::Text(auth_message)).await {
            error!("Failed to send authentication message: {}", e);
            *self.status.write().await = ConnectionStatus::Disconnected;
            self.emit_status_change(ConnectionStatus::Disconnected).await;
            return Err(format!("Failed to send auth message: {}", e));
        }

        // 克隆用于消息处理的引用
        let app_handle = self.app_handle.clone();
        let status = self.status.clone();
        let user_id_clone = self.user_id.clone();
        let config = self.config.clone();

        // 创建发送队列和处理器
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();
        let tx_clone = tx.clone();

        // 消息发送处理任务
        let send_task = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Err(e) = write.send(msg).await {
                    error!("Failed to send WebSocket message: {}", e);
                    break;
                }
            }
            debug!("Send task terminated");
        });

        // 心跳发送任务
        let heartbeat_interval = tokio::time::Duration::from_millis(config.heartbeat_interval_ms);
        let status_clone_for_heartbeat = status.clone();
        let heartbeat_task = tokio::spawn(async move {
            let mut interval = tokio::time::interval(heartbeat_interval);
            loop {
                interval.tick().await;
                // 使用克隆的status_clone_for_heartbeat
                if *status_clone_for_heartbeat.read().await == ConnectionStatus::Connected {
                    let ping_message = Message::Ping(vec![]);
                    if tx_clone.send(ping_message).is_err() {
                        break;
                    }
                } else {
                    // 如果不是连接状态，停止心跳
                    break;
                }
            }
            debug!("Heartbeat task terminated");
        });
        

        // 消息接收处理任务
        tokio::spawn(async move {
            while let Some(message_result) = read.next().await {
                match message_result {
                    Ok(msg) => {
                        match msg {
                            Message::Text(text) => {
                                // 处理文本消息
                                debug!("Received text message");
                                // 发送消息到前端
                                if let Err(e) = app_handle.emit("chat_event", text) {
                                    error!("Failed to emit chat event: {}", e);
                                }
                            }
                            Message::Binary(_) => {
                                // 忽略二进制消息
                                debug!("Received binary message (ignored)");
                            }
                            Message::Ping(data) => {
                                // 响应Ping消息
                                debug!("Received ping");
                                if tx.send(Message::Pong(data)).is_err() {
                                    break;
                                }
                            }
                            Message::Pong(_) => {
                                // 处理Pong响应
                                debug!("Received pong");
                            }
                            Message::Close(frame) => {
                                // 处理关闭连接
                                info!("Received close frame: {:?}", frame);
                                break;
                            }
                            Message::Frame(_) => {
                                // 忽略原始帧
                                debug!("Received raw frame (ignored)");
                            }
                        }
                    }
                    Err(e) => {
                        error!("WebSocket error: {}", e);
                        break;
                    }
                }
            }

            // 连接已关闭
            *status.write().await = ConnectionStatus::Disconnected;
            // 发送断开连接事件
            if let Err(e) = app_handle.emit("ws_connection_change", ConnectionStatus::Disconnected) {
                error!("Failed to emit connection change event: {}", e);
            }
            
            // 尝试重新连接
            let user_id_value = user_id_clone.read().await.clone();
            if let Some(uid) = user_id_value {
                // 尝试重新连接
                Self::attempt_reconnect(
                    app_handle.clone(),
                    status.clone(),
                    config.clone(),
                    uid,
                    0,
                );
            }
            
            debug!("Receive task terminated");
        });

        Ok(())
    }

    /// 尝试重新连接到WebSocket服务器
    fn attempt_reconnect(
        app_handle: AppHandle,
        status: Arc<RwLock<ConnectionStatus>>,
        config: WebSocketConfig,
        user_id: String,
        attempt: u32,
    ) {
        if attempt >= config.max_reconnect_attempts {
            error!("Max reconnect attempts reached");
            return;
        }

        let delay = tokio::time::Duration::from_millis(config.reconnect_delay_ms);
        
        tokio::spawn(async move {
            // 等待一段时间后尝试重连
            tokio::time::sleep(delay).await;
            
            // 设置重连状态
            *status.write().await = ConnectionStatus::Reconnecting;
            if let Err(e) = app_handle.emit("ws_connection_change", ConnectionStatus::Reconnecting) {
                error!("Failed to emit connection change event: {}", e);
            }
            
            // 构建连接URL
            let connect_url = format!("{}?user_id={}", config.server_url, user_id);
            let url = match Url::parse(&connect_url) {
                Ok(url) => url,
                Err(e) => {
                    error!("Invalid URL: {}", e);
                    // 递归重试
                    Self::attempt_reconnect(app_handle, status, config, user_id, attempt + 1);
                    return;
                }
            };
            
            // 尝试重新连接
            match connect_async(url).await {
                Ok((ws_stream, _)) => {
                    // 连接成功，更新状态
                    *status.write().await = ConnectionStatus::Connected;
                    if let Err(e) = app_handle.emit("ws_connection_change", ConnectionStatus::Connected) {
                        error!("Failed to emit connection change event: {}", e);
                    }
                    
                    // 处理WebSocket流
                    let (mut write, mut read) = ws_stream.split();
                    
                    // 发送认证消息
                    let auth_message = serde_json::json!({
                        "message_type": "UserStatus",
                        "sender_id": user_id,
                        "data": {
                            "status": "online"
                        },
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    }).to_string();
                    
                    if let Err(e) = write.send(Message::Text(auth_message)).await {
                        error!("Failed to send authentication message: {}", e);
                        *status.write().await = ConnectionStatus::Disconnected;
                        if let Err(e) = app_handle.emit("ws_connection_change", ConnectionStatus::Disconnected) {
                            error!("Failed to emit connection change event: {}", e);
                        }
                        // 递归重试
                        Self::attempt_reconnect(app_handle, status.clone(), config, user_id, attempt + 1);
                        return;
                    }
                    
                    // 创建消息传递通道
                    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();
                    
                    // 消息发送任务
                    let tx_clone_for_send = tx.clone(); // 为发送任务克隆tx
                    let send_task = {
                        tokio::spawn(async move {
                            while let Some(msg) = rx.recv().await {
                                if let Err(e) = write.send(msg).await {
                                    error!("Failed to send WebSocket message: {}", e);
                                    break;
                                }
                            }
                        })
                    };

                    // 定义心跳间隔 - 从config中获取
                    let heartbeat_interval = tokio::time::Duration::from_millis(config.heartbeat_interval_ms);
                    
                    // 心跳任务
                    let status_clone = status.clone();
                    let tx_clone_for_heartbeat = tx.clone(); // 为心跳任务克隆tx
                    let heartbeat_task = tokio::spawn(async move {
                        let mut interval = tokio::time::interval(heartbeat_interval);
                        loop {
                            interval.tick().await;
                            if *status_clone.read().await == ConnectionStatus::Connected {
                                // 使用克隆的tx
                                if tx_clone_for_heartbeat.send(Message::Ping(vec![])).is_err() {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    });

                    // 消息接收任务
                    let status_clone_for_receiver = status.clone(); // 为接收任务克隆status
                    let tx_clone_for_receive = tx.clone(); // 为接收任务克隆tx
                    let app_handle_clone = app_handle.clone(); // 为接收任务克隆app_handle
                    let config_clone = config.clone(); // 为接收任务克隆config
                    let user_id_clone = user_id.clone(); // 为接收任务克隆user_id
                    
                    tokio::spawn(async move {
                        while let Some(message_result) = read.next().await {
                            match message_result {
                                Ok(msg) => {
                                    match msg {
                                        Message::Text(text) => {
                                            // 处理文本消息
                                            if let Err(e) = app_handle_clone.emit("chat_event", text) {
                                                error!("Failed to emit chat event: {}", e);
                                            }
                                        }
                                        Message::Ping(data) => {
                                            // 响应Ping - 使用克隆的tx
                                            if tx_clone_for_receive.send(Message::Pong(data)).is_err() {
                                                break;
                                            }
                                        }
                                        Message::Close(_) => {
                                            // 处理关闭连接
                                            break;
                                        }
                                        _ => {} // 忽略其他类型的消息
                                    }
                                }
                                Err(e) => {
                                    error!("WebSocket error: {}", e);
                                    break;
                                }
                            }
                        }
                        
                        // 连接已关闭
                        *status_clone_for_receiver.write().await = ConnectionStatus::Disconnected;
                        if let Err(e) = app_handle_clone.emit("ws_connection_change", ConnectionStatus::Disconnected) {
                            error!("Failed to emit connection change event: {}", e);
                        }
                        
                        // 重新尝试连接
                        Self::attempt_reconnect(app_handle_clone, status_clone_for_receiver, config_clone, user_id_clone, attempt + 1);
                    });
                }
                Err(e) => {
                    // 连接失败
                    error!("Reconnection attempt {} failed: {}", attempt + 1, e);
                    // 递归重试
                    Self::attempt_reconnect(app_handle, status, config, user_id, attempt + 1);
                }
            }
        });
    }

    /// 断开WebSocket连接
    pub async fn disconnect(&self) -> Result<(), String> {
        if *self.status.read().await == ConnectionStatus::Connected {
            // 更新状态
            *self.status.write().await = ConnectionStatus::Disconnected;
            // 发送状态变更事件
            self.emit_status_change(ConnectionStatus::Disconnected).await;
            
            // 注：实际断开连接是在消息接收循环中检测到并处理的
            // 这里我们只更新状态，实际连接会在下一个消息循环中关闭
            
            Ok(())
        } else {
            Err("Not connected".to_string())
        }
    }

    /// 获取当前连接状态
    pub async fn get_status(&self) -> ConnectionStatus {
        *self.status.read().await
    }

    /// 发送WebSocket消息
    pub async fn send_message(&self, message: String) -> Result<(), String> {
        // 检查连接状态
        if *self.status.read().await != ConnectionStatus::Connected {
            return Err("Not connected".to_string());
        }
        
        // 发送消息（通过事件）
        // 注：实际实现中，我们应该有一个专门的发送通道
        // 简化实现，通过事件触发发送
        self.app_handle.emit("ws_send", message)
            .map_err(|e| format!("Failed to emit send event: {}", e))
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

    /// 发送连接状态变更事件
    async fn emit_status_change(&self, status: ConnectionStatus) {
        if let Err(e) = self.app_handle.emit("ws_connection_change", status) {
            error!("Failed to emit connection change event: {}", e);
        }
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

    /// 发送WebSocket消息
    pub async fn send_message(&self, message: String) -> Result<(), String> {
        let client = self.client.lock().await;
        match &*client {
            Some(ws_client) => ws_client.send_message(message).await,
            None => Err("WebSocket client not initialized".to_string()),
        }
    }
}
