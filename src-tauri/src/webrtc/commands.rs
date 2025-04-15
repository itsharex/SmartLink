// src-tauri/src/webrtc/commands.rs
use crate::error::Error;
use crate::webrtc::models::{RTCSignalType, CallStatus};
use crate::webrtc::manager::RTCManager;
use tauri::{command, State};
use chrono::Utc;
use serde_json::json;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum MessageType {
    UserStatus,
    NewMessage,
    MessageStatusUpdate,
    WebRTCSignal,
    TypingIndicator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MessageType,
    sender_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recipient_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
    timestamp: String,
}

#[command]
pub async fn initiate_call(
    manager: State<'_, RTCManager>,
    recipient_id: String,
    conversation_id: Option<String>,
    user_id: String,
) -> Result<String, Error> {
    let session = manager
        .create_call_session(user_id.clone(), recipient_id.clone(), conversation_id.clone())
        .await
        .map_err(|e| Error::Internal(e.to_string()))?;

    let message = WebSocketMessage {
        message_type: MessageType::WebRTCSignal,
        sender_id: user_id,
        recipient_id: Some(recipient_id),
        conversation_id,
        message_id: None,
        data: Some(json!({
            "signal_type": "Offer",
            "session_id": session.id,
        })),
        timestamp: Utc::now().to_rfc3339(),
    };

    send_ws_message(message)
        .await
        .map_err(|_| Error::WebSocket)?;
    
    Ok(session.id)
}

#[command]
pub async fn send_webrtc_signal(
    signal_type: RTCSignalType,
    recipient_id: String,
    conversation_id: Option<String>,
    signal_data: serde_json::Value,
    sender_id: String,
) -> Result<(), Error> {
    let message = WebSocketMessage {
        message_type: MessageType::WebRTCSignal,
        sender_id,
        recipient_id: Some(recipient_id),
        conversation_id,
        message_id: None,
        data: Some(json!({
            "signal_type": signal_type,
            "signal_data": signal_data
        })),
        timestamp: Utc::now().to_rfc3339(),
    };

    send_ws_message(message)
        .await
        .map_err(|_| Error::WebSocket)
}

#[command]
pub async fn accept_call(
    manager: State<'_, RTCManager>,
    call_id: String,
    user_id: String,
    peer_id: String,
) -> Result<(), Error> {
    manager
        .update_call_status(&call_id, CallStatus::Connected)
        .await
        .map_err(|e| Error::Internal(e.to_string()))?;

    let message = WebSocketMessage {
        message_type: MessageType::WebRTCSignal,
        sender_id: user_id,
        recipient_id: Some(peer_id),
        conversation_id: None,
        message_id: None,
        data: Some(json!({
            "signal_type": "Answer",
            "session_id": call_id
        })),
        timestamp: Utc::now().to_rfc3339(),
    };

    send_ws_message(message)
        .await
        .map_err(|_| Error::WebSocket)
}

#[command]
pub async fn end_call(
    manager: State<'_, RTCManager>,
    call_id: String,
    user_id: String,
    peer_id: String,
) -> Result<(), Error> {
    manager
        .end_call(&call_id)
        .await
        .map_err(|e| Error::Internal(e.to_string()))?;

    let message = WebSocketMessage {
        message_type: MessageType::WebRTCSignal,
        sender_id: user_id,
        recipient_id: Some(peer_id),
        conversation_id: None,
        message_id: None,
        data: Some(json!({
            "signal_type": "Hangup",
            "session_id": call_id
        })),
        timestamp: Utc::now().to_rfc3339(),
    };

    send_ws_message(message)
        .await
        .map_err(|_| Error::WebSocket)
}

#[command]
pub async fn get_active_call(
    manager: State<'_, RTCManager>,
    user_id: String,
) -> Result<Option<String>, Error> {
    // 直接使用 Option 的 map
    let call = manager.get_active_call(&user_id).await;
    Ok(call.map(|session| session.id))
}

// WebSocket消息发送函数
async fn send_ws_message(message: WebSocketMessage) -> Result<(), Error> {
    // 需要确保全局WebSocket连接已建立
    if let Some(ws_sender) = get_ws_sender().await {
        let message_text = serde_json::to_string(&message)
            .map_err(|e| Error::Internal(e.to_string()))?;
        
        ws_sender
            .send(Message::Text(message_text))
            .map_err(|_| Error::WebSocket)?;
        
        Ok(())
    } else {
        Err(Error::WebSocket)
    }
}

// 获取WebSocket发送器
async fn get_ws_sender() -> Option<mpsc::UnboundedSender<Message>> {
    // 这部分需要在应用状态中维护WebSocket连接
    // 可以使用tauri::Manager来访问应用状态
    todo!("Implement WebSocket sender access")
}