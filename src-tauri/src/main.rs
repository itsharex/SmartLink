mod platform;
mod db;

use db::storage::StorageError;
use platform::windows::Windows;
use platform::AutoStart;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter, Manager};
use crate::db::{storage::Storage, types::{AppUsageRecord, AppUsageStats}};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppUsage {
    pub name: String,
    pub total_time: u64,
    pub last_active: u64,
}

pub struct AppState {
    usage_data: Mutex<HashMap<String, AppUsage>>,
    storage: Mutex<Storage>,
}

impl AppState {
    fn new(app_handle: &AppHandle) -> Result<Self, StorageError> {
        let storage = Storage::new(app_handle)?;
        Ok(Self {
            usage_data: Mutex::new(HashMap::new()),
            storage: Mutex::new(storage),
        })
    }
}

#[tauri::command]
async fn get_app_usage(state: tauri::State<'_, AppState>) -> Result<HashMap<String, AppUsage>, String> {
    state.usage_data
        .lock()
        .map(|data: std::sync::MutexGuard<'_, HashMap<String, AppUsage>>| data.clone())
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn toggle_auto_start(enable: bool) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    let platform = Windows;
    #[cfg(target_os = "macos")]
    let platform = MacOS;
    #[cfg(target_os = "linux")]
    let platform = Linux;
    
    platform.set_auto_start(enable)
}

#[tauri::command]
async fn get_auto_start_status() -> Result<bool, String> {
    #[cfg(target_os = "windows")]
    let platform = Windows;
    #[cfg(target_os = "macos")]
    let platform = MacOS;
    #[cfg(target_os = "linux")]
    let platform = Linux;
    
    platform.is_auto_start_enabled()
}

#[tauri::command]
async fn get_app_usage_stats(app_handle: tauri::AppHandle, range: String) -> Result<Vec<AppUsageStats>, String> {
    let storage = Storage::new(&app_handle).map_err(|e| e.to_string())?;
    storage.get_usage_stats(&range)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn record_app_usage(app_handle: tauri::AppHandle, record: AppUsageRecord) -> Result<(), String> {
    let storage = Storage::new(&app_handle).map_err(|e| e.to_string())?;
    storage.record_usage(record)
        .map_err(|e| e.to_string())
}

async fn monitor_active_window(handle: tauri::AppHandle) {
    tracing::info!("Starting window monitor...");
    let window_monitor = platform::create_window_monitor();
    
    loop {
        if let Some(process_name) = window_monitor.get_active_window() {
            tracing::debug!("Detected active window: {}", process_name);
            
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let state = handle.state::<AppState>();
            {
                let mut data = state.usage_data.lock().unwrap();
                let app_usage = data.entry(process_name.clone())
                    .or_insert(AppUsage {
                        name: process_name.clone(),
                        total_time: 0,
                        last_active: current_time,
                    });

                if current_time - app_usage.last_active <= 2 {
                    app_usage.total_time += 1;
                    tracing::debug!("Updating usage for {}: {} seconds", process_name, app_usage.total_time);
                    
                    let mut storage = state.storage.lock().unwrap();
                    let record = AppUsageRecord {
                        timestamp: chrono::Utc::now(),
                        app_name: process_name.clone(),
                        duration: 1,
                    };
                    
                    if let Err(e) = storage.record_usage(record) {
                        tracing::error!("Failed to record usage: {}", e);
                    } else {
                        tracing::info!("Successfully recorded usage for: {}", process_name);
                    }
                }
                app_usage.last_active = current_time;

                let data_clone = data.clone();
                drop(data);
                let _ = handle.emit("usage_updated", data_clone);
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

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
        .setup(|app| {
            let handle = app.handle();
            
            // 在setup中初始化AppState
            let app_state = AppState::new(&handle)
                .expect("Failed to initialize app state");
            app.manage(app_state);
            
            tracing::info!("App state initialized successfully");
            
            // 启动监控任务
            let handle_clone = handle.clone();
            tauri::async_runtime::spawn(async move {
                monitor_active_window(handle_clone).await;
            });
            
            tracing::info!("Tauri setup started");
            #[cfg(debug_assertions)]
            {
                handle.plugin(tauri_plugin_shell::init())?;
                tracing::info!("Debug plugins initialized");
            }
            tracing::info!("Tauri setup completed");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_app_usage,
            toggle_auto_start,
            get_auto_start_status,
            get_app_usage_stats,
            record_app_usage
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