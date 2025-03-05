#[cfg(target_os = "macos")]
mod macos {
    use super::WindowInfo;
    use core_foundation::string::CFString;
    use core_foundation::array::CFArray;
    use core_foundation::dictionary::CFDictionary;
    use core_graphics::window::CGWindowListOption;
    use core_graphics::window::CGWindow;
    use std::process::Command;

    pub struct MacOS;
    pub struct MacOSMonitor;

    impl MacOSMonitor {
        pub fn new() -> Self {
            MacOSMonitor
        }
    }

    impl WindowInfo for MacOSMonitor {
        fn get_active_window(&self) -> Option<String> {
            let options = CGWindowListOption::OPTION_ON_SCREEN | 
                         CGWindowListOption::OPTION_RELATIVE_TO_FRONT;
            let window_list = CGWindow::window_list_info(options, None)?;
            
            if let Some(window_info) = window_list.get(0) {
                if let Some(app_name) = window_info.get("kCGWindowOwnerName") {
                    return Some(app_name.to_string());
                }
            }
            None
        }
    }
}

impl AutoStart for MacOS {
    fn set_auto_start(&self, enable: bool) -> Result<(), String> {
        let (app_name, app_path) = get_app_info()?;
        let plist_path = format!("~/Library/LaunchAgents/{}.plist", app_name);
        
        if enable {
            let plist_content = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
                <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
                <plist version="1.0">
                <dict>
                    <key>Label</key>
                    <string>{}</string>
                    <key>ProgramArguments</key>
                    <array>
                        <string>{}</string>
                    </array>
                    <key>RunAtLoad</key>
                    <true/>
                </dict>
                </plist>"#,
                app_name,
                app_path.to_string_lossy()
            );
            
            std::fs::write(&plist_path, plist_content)
                .map_err(|e| format!("Failed to write plist file: {}", e))?;
                
            Command::new("launchctl")
                .args(&["load", &plist_path])
                .output()
                .map_err(|e| format!("Failed to load launch agent: {}", e))?;
        } else {
            Command::new("launchctl")
                .args(&["unload", &plist_path])
                .output()
                .map_err(|e| format!("Failed to unload launch agent: {}", e))?;
                
            std::fs::remove_file(&plist_path)
                .map_err(|e| format!("Failed to remove plist file: {}", e))?;
        }
        
        Ok(())
    }

    fn is_auto_start_enabled(&self) -> Result<bool, String> {
        let (app_name, _) = get_app_info()?;
        let plist_path = format!("~/Library/LaunchAgents/{}.plist", app_name);
        Ok(std::path::Path::new(&plist_path).exists())
    }
}

#[cfg(test)]
#[cfg(target_os = "macos")]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use std::process::Command;
    use std::sync::Arc;
    use std::thread;
    use std::time::{Duration, Instant};
    use std::collections::HashMap;

    // 1. 基础窗口信息测试扩展
    #[test]
    fn test_get_active_window() {
        let monitor = MacOSMonitor::new();
        let result = monitor.get_active_window();
        match result {
            Some(window_name) => {
                assert!(!window_name.is_empty());
                assert!(!window_name.ends_with(".app"));
                assert!(!window_name.contains('\0'));
                assert!(!window_name.contains('/'));
            }
            None => {} // 可能没有活动窗口
        }
    }

    #[test]
    fn test_multiple_window_queries() {
        let monitor = MacOSMonitor::new();
        let mut results = vec![];
        
        // 连续查询并记录结果
        for _ in 0..10 {
            let result = monitor.get_active_window();
            results.push(result);
            thread::sleep(Duration::from_millis(100));
        }

        // 分析结果
        let success_count = results.iter().filter(|r| r.is_some()).count();
        let unique_windows: HashSet<_> = results.iter().filter_map(|r| r.clone()).collect();
        
        println!("Success rate: {}%", (success_count as f64 / results.len() as f64) * 100.0);
        println!("Unique windows: {}", unique_windows.len());
    }

    // 2. plist 文件操作测试扩展
    #[test]
    fn test_plist_file_operations() {
        let (app_name, app_path) = get_app_info().unwrap();
        let plist_path = format!("~/Library/LaunchAgents/{}.plist", app_name);
        let path = Path::new(&plist_path);

        // 测试 plist 文件创建
        let macos = MacOS;
        macos.set_auto_start(true).unwrap();

        if path.exists() {
            // 验证 plist 文件内容
            let content = fs::read_to_string(path).unwrap();
            
            // 基本字段验证
            assert!(content.contains("Label"));
            assert!(content.contains("ProgramArguments"));
            assert!(content.contains("RunAtLoad"));
            assert!(content.contains(&app_name));
            
            // XML 格式验证
            assert!(content.starts_with("<?xml"));
            assert!(content.contains("version=\"1.0\""));
            assert!(content.contains("encoding=\"UTF-8\""));
            
            // 路径验证
            assert!(content.contains(&app_path.to_string_lossy()));
            
            // 权限验证
            let metadata = fs::metadata(path).unwrap();
            let permissions = metadata.permissions();
            
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                // 确保文件权限正确 (644)
                assert_eq!(permissions.mode() & 0o777, 0o644);
            }
        }

        // 清理
        macos.set_auto_start(false).unwrap();
    }

    // 3. 并发测试扩展
    #[test]
    fn test_concurrent_window_monitoring() {
        let monitor = Arc::new(MacOSMonitor::new());
        let mut handles = vec![];
        let start_time = Instant::now();
        let duration = Duration::from_secs(5);

        // 创建多个监控线程
        for thread_id in 0..5 {
            let monitor_clone = Arc::clone(&monitor);
            let handle = thread::spawn(move || {
                let mut thread_results = Vec::new();
                while start_time.elapsed() < duration {
                    let window = monitor_clone.get_active_window();
                    thread_results.push((thread_id, Instant::now(), window));
                    thread::sleep(Duration::from_millis(100));
                }
                thread_results
            });
            handles.push(handle);
        }

        // 收集结果
        let mut all_results = Vec::new();
        for handle in handles {
            all_results.extend(handle.join().unwrap());
        }

        // 分析结果
        let mut window_counts = HashMap::new();
        let mut thread_counts = HashMap::new();
        
        for (thread_id, time, window) in all_results {
            if let Some(name) = window {
                *window_counts.entry(name).or_insert(0) += 1;
            }
            *thread_counts.entry(thread_id).or_insert(0) += 1;
        }

        println!("Window statistics:");
        for (window, count) in window_counts {
            println!("- {}: {} times", window, count);
        }

        println!("Thread statistics:");
        for (thread_id, count) in thread_counts {
            println!("- Thread {}: {} queries", thread_id, count);
        }
    }

    // 4. 错误处理测试扩展
    #[test]
    fn test_error_handling() {
        let macos = MacOS;

        // 1. 文件系统错误
        let test_cases = vec![
            "/nonexistent/path/app.plist",
            "~/Library/LaunchAgents/",
            "",
            "/",
        ];

        for path in test_cases {
            if let Err(e) = std::fs::write(path, "test") {
                assert!(!e.to_string().is_empty());
                println!("Expected error for path {}: {}", path, e);
            }
        }

        // 2. launchctl 命令错误
        let invalid_commands = vec![
            vec!["invalid_command"],
            vec!["load", "/nonexistent/path"],
            vec!["unload", "/nonexistent/path"],
            vec!["list", "invalid_service"],
        ];

        for args in invalid_commands {
            let output = Command::new("launchctl")
                .args(&args)
                .output();
            
            match output {
                Ok(out) => {
                    assert!(!out.status.success());
                    println!("Expected error for args {:?}: {}", 
                            args, 
                            String::from_utf8_lossy(&out.stderr));
                }
                Err(e) => {
                    println!("Command failed as expected: {}", e);
                }
            }
        }

        // 3. 环境变量错误
        std::env::remove_var("HOME");
        assert!(macos.set_auto_start(true).is_err());
    }

    // 5. 性能测试
    #[test]
    fn test_performance() {
        let monitor = MacOSMonitor::new();
        let iterations = 100;
        
        // 1. 基准测试
        let mut durations = Vec::with_capacity(iterations);
        for _ in 0..iterations {
            let start = Instant::now();
            let _ = monitor.get_active_window();
            durations.push(start.elapsed());
        }

        // 计算统计信息
        let total_duration: Duration = durations.iter().sum();
        let avg_duration = total_duration / iterations as u32;
        let max_duration = durations.iter().max().unwrap();
        let min_duration = durations.iter().min().unwrap();

        println!("Performance Statistics:");
        println!("Average query time: {:?}", avg_duration);
        println!("Maximum query time: {:?}", max_duration);
        println!("Minimum query time: {:?}", min_duration);

        // 确保性能在可接受范围内
        assert!(avg_duration < Duration::from_millis(50));
        
        // 2. 并发性能测试
        let monitor = Arc::new(monitor);
        let mut handles = vec![];
        let start = Instant::now();

        for _ in 0..10 {
            let monitor_clone = Arc::clone(&monitor);
            handles.push(thread::spawn(move || {
                let mut thread_times = Vec::with_capacity(10);
                for _ in 0..10 {
                    let query_start = Instant::now();
                    let _ = monitor_clone.get_active_window();
                    thread_times.push(query_start.elapsed());
                }
                thread_times
            }));
        }

        let mut all_times = Vec::new();
        for handle in handles {
            all_times.extend(handle.join().unwrap());
        }

        let total_time = start.elapsed();
        println!("Concurrent test total time: {:?}", total_time);
        println!("Average time per query: {:?}", total_time / (100));
    }

    // 6. 压力测试
    #[test]
    fn test_stress() {
        let monitor = Arc::new(MacOSMonitor::new());
        let test_duration = Duration::from_secs(5);
        let start_time = Instant::now();
        let mut handles = vec![];

        // 创建多个压力测试线程
        for thread_id in 0..10 {
            let monitor_clone = Arc::clone(&monitor);
            let handle = thread::spawn(move || {
                let mut stats = QueryStats {
                    successful_queries: 0,
                    failed_queries: 0,
                    total_duration: Duration::new(0, 0),
                    min_duration: Duration::from_secs(999_999),
                    max_duration: Duration::new(0, 0),
                };

                while start_time.elapsed() < test_duration {
                    let query_start = Instant::now();
                    let result = monitor_clone.get_active_window();
                    let duration = query_start.elapsed();

                    stats.total_duration += duration;
                    stats.min_duration = stats.min_duration.min(duration);
                    stats.max_duration = stats.max_duration.max(duration);

                    match result {
                        Some(_) => stats.successful_queries += 1,
                        None => stats.failed_queries += 1,
                    }
                }

                stats
            });
            handles.push(handle);
        }

        // 收集并分析结果
        let mut total_stats = QueryStats::default();
        for handle in handles {
            let stats = handle.join().unwrap();
            total_stats.combine(&stats);
        }

        println!("Stress Test Results:");
        println!("Total queries: {}", 
            total_stats.successful_queries + total_stats.failed_queries);
        println!("Successful queries: {}", total_stats.successful_queries);
        println!("Failed queries: {}", total_stats.failed_queries);
        println!("Success rate: {:.2}%", 
            (total_stats.successful_queries as f64 / 
            (total_stats.successful_queries + total_stats.failed_queries) as f64) * 100.0);
        println!("Average query time: {:?}", 
            total_stats.total_duration / 
            (total_stats.successful_queries + total_stats.failed_queries) as u32);
        println!("Minimum query time: {:?}", total_stats.min_duration);
        println!("Maximum query time: {:?}", total_stats.max_duration);
    }

    // 7. 稳定性测试
    #[test]
    fn test_stability() {
        let monitor = MacOSMonitor::new();
        let macos = MacOS;
        let iterations = 50;

        // 1. 反复切换自启动状态
        for i in 0..iterations {
            let enable = i % 2 == 0;
            assert!(macos.set_auto_start(enable).is_ok());
            assert_eq!(macos.is_auto_start_enabled().unwrap(), enable);
            thread::sleep(Duration::from_millis(10));
        }

        // 2. 长时间运行测试
        let duration = Duration::from_secs(10);
        let start = Instant::now();
        let mut query_count = 0;
        let mut error_count = 0;

        while start.elapsed() < duration {
            match monitor.get_active_window() {
                Ok(_) => query_count += 1,
                Err(_) => error_count += 1,
            }
            thread::sleep(Duration::from_millis(100));
        }

        println!("Stability test results:");
        println!("Total queries: {}", query_count);
        println!("Error count: {}", error_count);
        println!("Error rate: {:.2}%", 
            (error_count as f64 / (query_count + error_count) as f64) * 100.0);
    }

    // 辅助结构体
    #[derive(Default)]
    struct QueryStats {
        successful_queries: u32,
        failed_queries: u32,
        total_duration: Duration,
        min_duration: Duration,
        max_duration: Duration,
    }

    impl QueryStats {
        fn combine(&mut self, other: &QueryStats) {
            self.successful_queries += other.successful_queries;
            self.failed_queries += other.failed_queries;
            self.total_duration += other.total_duration;
            self.min_duration = self.min_duration.min(other.min_duration);
            self.max_duration = self.max_duration.max(other.max_duration);
        }
    }
}