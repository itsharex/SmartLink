// src-tauri/src/error.rs
use mongodb::error::Error as MongoError;
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Encryption error: {0}")]
    Encryption(String),
    
    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Key generation failed")]
    KeyGeneration,

    #[error("Encryption failed: {0}")]
    Encrypt(String),

    #[error("Decryption failed: {0}")]
    Decrypt(String),

    #[error("Invalid nonce")]
    InvalidNonce,
}

// 实现从MongoDB错误到自定义错误的转换
impl From<MongoError> for Error {
    fn from(err: MongoError) -> Self {
        Self::Database(err.to_string())
    }
}

// 实现从标准I/O错误到自定义错误的转换
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Internal(err.to_string())
    }
}