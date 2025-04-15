// src-tauri/src/webrtc/manager.rs
use crate::error::Error;
use crate::webrtc::models::{CallSession, CallStatus};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct RTCManager {
    active_calls: Arc<RwLock<HashMap<String, CallSession>>>,
}

impl RTCManager {
    pub fn new() -> Self {
        Self {
            active_calls: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_call_session(
        &self,
        initiator_id: String,
        recipient_id: String,
        conversation_id: Option<String>,
    ) -> Result<CallSession, Error> {
        let session = CallSession {
            id: Uuid::new_v4().to_string(),
            initiator_id,
            recipient_id,
            conversation_id,
            start_time: std::time::SystemTime::now(),
            status: CallStatus::Ringing,
        };

        let mut calls = self.active_calls.write().await;
        calls.insert(session.id.clone(), session.clone());

        Ok(session)
    }

    pub async fn update_call_status(
        &self,
        call_id: &str,
        status: CallStatus,
    ) -> Result<(), Error> {
        let mut calls = self.active_calls.write().await;
        
        if let Some(session) = calls.get_mut(call_id) {
            session.status = status;
            Ok(())
        } else {
            Err(Error::NotFound("Call session not found".into()))
        }
    }

    pub async fn end_call(&self, call_id: &str) -> Result<(), Error> {
        let mut calls = self.active_calls.write().await;
        calls.remove(call_id);
        Ok(())
    }

    pub async fn get_active_call(
        &self,
        user_id: &str,
    ) -> Option<CallSession> {
        let calls = self.active_calls.read().await;
        
        calls.values()
            .find(|session| {
                session.initiator_id == user_id || session.recipient_id == user_id
            })
            .cloned()
    }
}