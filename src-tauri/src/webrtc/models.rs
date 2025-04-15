// src-tauri/src/webrtc/models.rs
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RTCSignalType {
    Offer,
    Answer,
    IceCandidate,
    Hangup,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RTCSignal {
    pub signal_type: RTCSignalType,
    pub sender_id: String,
    pub recipient_id: String,
    pub conversation_id: Option<String>,
    pub data: serde_json::Value,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallSession {
    pub id: String,
    pub initiator_id: String,
    pub recipient_id: String,
    pub conversation_id: Option<String>,
    pub start_time: SystemTime,
    pub status: CallStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CallStatus {
    Ringing,
    Connected,
    Ended,
}