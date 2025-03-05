use super::{AutoStart, WindowInfo};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use windows::Win32::System::ProcessStatus::GetProcessImageFileNameA;
use windows::Win32::Foundation::BOOL;
use std::path::PathBuf;
use winreg::enums::*;
use winreg::RegKey;

pub struct Windows;
pub struct WindowsMonitor;

impl WindowsMonitor {
    pub fn new() -> Self {
        WindowsMonitor
    }
}

impl WindowInfo for WindowsMonitor {
    fn get_active_window(&self) -> Option<String> {
        unsafe {
            let hwnd = GetForegroundWindow();
            let mut process_id: u32 = 0;
            GetWindowThreadProcessId(hwnd, Some(&mut process_id));
            
            let process_handle = OpenProcess(
                PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                BOOL(0),
                process_id
            ).ok()?;

            if !process_handle.is_invalid() {
                let mut buffer = [0u8; 260];
                let result = GetProcessImageFileNameA(process_handle, &mut buffer);
                if result != 0 {
                    let path = String::from_utf8_lossy(&buffer)
                        .trim_matches(char::from(0))
                        .to_string();
                    return path.split('\\').last().map(String::from);
                }
            }
            None
        }
    }
}

fn get_app_info() -> Result<(String, PathBuf), String> {
    let context: tauri::Context<tauri::Wry> = tauri::generate_context!();
    let app_name = context.package_info().name.clone();
    let app_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get executable path: {}", e))?;
    
    Ok((app_name, app_path))
}

impl AutoStart for Windows {
    fn set_auto_start(&self, enable: bool) -> Result<(), String> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
        let (app_name, app_path) = get_app_info()?;
        
        let run_key = hkcu.open_subkey_with_flags(path, KEY_WRITE)
            .map_err(|e| format!("Failed to open registry key: {}", e))?;

        if enable {
            run_key.set_value(&app_name, &app_path.to_string_lossy().as_ref())
                .map_err(|e| format!("Failed to set registry value: {}", e))?;
        } else {
            run_key.delete_value(&app_name)
                .map_err(|e| format!("Failed to delete registry value: {}", e))?;
        }
        
        Ok(())
    }

    fn is_auto_start_enabled(&self) -> Result<bool, String> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
        let (app_name, _) = get_app_info()?;
        
        let run_key = hkcu.open_subkey(path)
            .map_err(|e| format!("Failed to open registry key: {}", e))?;
            
        match run_key.get_value::<String, &str>(&app_name) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    // 1. 测试窗口信息获取
    #[test]
    fn test_get_active_window() {
        let monitor = WindowsMonitor::new();
        // 这里我们只能测试返回的类型是否正确，因为实际窗口依赖于运行环境
        let result = monitor.get_active_window();
        match result {
            Some(window_name) => {
                assert!(!window_name.is_empty());
                assert!(window_name.ends_with(".exe"));
            }
            None => {} // 也可能没有活动窗口，这是合法的
        }
    }

    #[test]
    fn test_multiple_window_calls() {
        let monitor = WindowsMonitor::new();
        // 连续多次获取窗口信息，验证稳定性
        for _ in 0..5 {
            let _ = monitor.get_active_window();
        }
    }

    // 2. 测试路径处理
    #[test]
    fn test_path_processing() {
        let test_paths = vec![
            "C:\\Windows\\System32\\notepad.exe",
            "D:\\Program Files\\App\\app.exe",
            "\\Device\\HarddiskVolume1\\Windows\\explorer.exe",
        ];

        for path in test_paths {
            let result = path.split('\\').last();
            assert!(result.is_some());
            assert!(result.unwrap().ends_with(".exe"));
        }
    }

    // 3. 测试自启动相关功能
    #[test]
    fn test_auto_start() {
        let windows = Windows;
        
        // 测试获取自启动状态
        let status = windows.is_auto_start_enabled();
        assert!(status.is_ok());

        // 测试设置自启动
        match windows.set_auto_start(true) {
            Ok(_) => {
                // 验证设置成功
                assert!(windows.is_auto_start_enabled().unwrap_or(false));
            }
            Err(e) => {
                // 在某些环境下可能没有权限，这是可以接受的
                println!("Note: Auto start test skipped due to: {}", e);
            }
        }

        // 测试关闭自启动
        match windows.set_auto_start(false) {
            Ok(_) => {
                // 验证关闭成功
                assert!(!windows.is_auto_start_enabled().unwrap_or(true));
            }
            Err(e) => {
                println!("Note: Auto start disable test skipped due to: {}", e);
            }
        }
    }

    // 4. 测试并发场景
    #[test]
    fn test_concurrent_window_monitoring() {
        let monitor = Arc::new(WindowsMonitor::new());
        let mut handles = vec![];

        for _ in 0..5 {
            let monitor_clone = Arc::clone(&monitor);
            let handle = thread::spawn(move || {
                for _ in 0..10 {
                    let _ = monitor_clone.get_active_window();
                    thread::sleep(std::time::Duration::from_millis(10));
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    // 5. 测试错误处理
    #[test]
    fn test_error_handling() {
        let windows = Windows;

        // 测试自启动错误处理
        let temp_status = windows.is_auto_start_enabled();
        if let Err(e) = temp_status {
            assert!(!e.is_empty(), "错误信息不应为空");
        }

        // 验证错误信息的完整性
        if let Err(e) = windows.set_auto_start(true) {
            assert!(!e.is_empty());
            assert!(
                e.contains("Failed to") || 
                e.contains("Error") || 
                e.contains("Unable to")
            );
        }
    }

    // 6. 测试进程名称解析
    #[test]
    fn test_process_name_extraction() {
        let test_cases = vec![
            ("C:\\Windows\\System32\\notepad.exe", "notepad.exe"),
            ("D:\\Program Files\\App\\my app.exe", "my app.exe"),
            ("\\Device\\HarddiskVolume1\\test.exe", "test.exe"),
            ("test.exe", "test.exe"),
        ];

        for (input, expected) in test_cases {
            let result = input.split('\\').last().unwrap();
            assert_eq!(result, expected);
        }
    }

    // 7. 测试特殊情况
    #[test]
    fn test_edge_cases() {
        let windows = Windows;
        let monitor = WindowsMonitor::new();

        // 快速切换自启动状态
        for _ in 0..3 {
            let _ = windows.set_auto_start(true);
            let _ = windows.set_auto_start(false);
        }

        // 快速获取窗口信息
        for _ in 0..10 {
            let _ = monitor.get_active_window();
        }
    }

    // 8. 测试路径处理边界情况
    #[test]
    fn test_path_edge_cases() {
        let binding = "very_long_name_".repeat(100);
        let test_cases = vec![
            "",                     // 空字符串
            "no_extension",         // 无扩展名
            "multiple.dots.exe",    // 多个点
            ".exe",                // 只有扩展名
            binding.as_str(),   // 非常长的名称
        ];

        for case in test_cases {
            let result = case.split('\\').last();
            assert!(result.is_some());
        }
    }

    // 9. 集成测试
    #[test]
    fn test_integrated_functionality() {
        let windows = Windows;
        let monitor = WindowsMonitor::new();

        // 组合测试自启动和窗口监控
        let window_name = monitor.get_active_window();
        let auto_start_status = windows.is_auto_start_enabled();

        println!("Current window: {:?}", window_name);
        println!("Auto start status: {:?}", auto_start_status);
    }
}