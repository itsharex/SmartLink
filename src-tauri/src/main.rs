// src-tauri/src/main.rs
mod auth;
mod chat;
mod db_init;
mod error;

use std::sync::Arc;

use auth::commands as auth_commands;
use chat::commands as chat_commands;
use mongodb::Database;
use tauri::{Listener, Manager};

// Set up the chat module
fn setup_chat_module(app: &mut tauri::App, db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    // 初始化聊天模块
    chat_commands::init(app, Arc::new(db.clone()))?;

    // WebSocketState管理
    app.manage(chat::websocket::WebSocketState::new());
    
    // Get app handle
    let app_handle = app.app_handle();
    
    // Handle payload
    app_handle.listen_any("user_logged_in", move |event| {
        // Handle user login event here
        let payload = event.payload();
        println!("User logged in: {}", payload);
        
    });
    
    let app_handle = app.app_handle();
    app_handle.listen_any("user_logged_out", move |event| {
        // Handle user logout event here
        let payload= event.payload();
        println!("User logged out: {}", payload);
        
    });
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_file(true)
        .with_line_number(true)
        .init();
    
    tracing::info!("Application starting...");
    
    let runtime = match tokio::runtime::Runtime::new() {
        Ok(rt) => {
            tracing::info!("Tokio runtime created successfully");
            rt
        }
        Err(e) => {
            tracing::error!("Failed to create Tokio runtime: {}", e);
            std::process::exit(1);
        }
    };
    
    let db = runtime.block_on(db_init::init_database())?;
    
    let db = Arc::new(db);
    let db_for_setup = db.clone();
    
    let result = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        
        .setup(move |app| {
            let handle = app.handle();
            
            // Initialize chat module
            match setup_chat_module(app, &db_for_setup) {
                Ok(_) => tracing::info!("Chat module initialized successfully"),
                Err(e) => tracing::error!("Failed to initialize chat module: {}", e),
            }
            
            // Initialize authentication module
            match auth_commands::init(app) {
                Ok(_) => tracing::info!("Authentication module initialized successfully"),
                Err(e) => tracing::error!("Failed to initialize authentication module: {}", e),
            }
            
            tracing::info!("App state initialized successfully");
            
            tracing::info!("Tauri setup started");
            tracing::info!("Tauri setup completed");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Registration and authentication related commands
            auth_commands::register_user,
            auth_commands::login_with_email,
            auth_commands::get_oauth_url,
            auth_commands::handle_oauth_callback,
            auth_commands::get_current_user,
            auth_commands::logout,
            auth_commands::refresh_token,
            
            // Chat related commands
            chat_commands::get_conversations,
            chat_commands::create_conversation,
            chat_commands::get_conversation,
            chat_commands::send_message,
            chat_commands::get_messages,
            chat_commands::update_message_status,
            chat_commands::mark_message_read,
            chat_commands::mark_conversation_read,
            chat_commands::mark_conversation_delivered,
            chat_commands::create_group_chat,
            chat_commands::add_group_member,
            chat_commands::remove_group_member,
            chat_commands::get_unread_count,
            chat_commands::get_online_participants,
            chat_commands::initialize_websocket,
            chat_commands::connect_websocket,
            chat_commands::disconnect_websocket,
            chat_commands::get_websocket_status,
            chat_commands::send_websocket_message,
            chat_commands::send_chat_message,
            chat_commands::send_webrtc_signal,
            chat_commands::send_typing_indicator,

        ])
        .run(tauri::generate_context!());
    
    match result {
        Ok(_) => tracing::info!("Application exited normally"),
        Err(e) => {
            tracing::error!("Application error: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}