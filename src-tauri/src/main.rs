// src-tauri/src/main.rs

mod auth;

use auth::commands as auth_commands;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
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

    let result = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init()) 
        .setup(|app| {
            let handle = app.handle();
            
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
        ])
        .run(tauri::generate_context!());

    match result {
        Ok(_) => tracing::info!("Application exited normally"),
        Err(e) => {
            tracing::error!("Application error: {}", e);
            std::process::exit(1);
        }
    }
}