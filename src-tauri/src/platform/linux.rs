#[cfg(target_os = "linux")]
mod linux {
    use super::WindowInfo;
    use x11rb::connection::Connection;
    use x11rb::protocol::xproto::*;
    use std::process::Command;
    use std::path::PathBuf;

    pub struct Linux;
    pub struct LinuxMonitor;

    impl LinuxMonitor {
        pub fn new() -> Self {
            LinuxMonitor
        }
    }

    impl WindowInfo for LinuxMonitor {
        fn get_active_window(&self) -> Option<String> {
            let (conn, screen_num) = x11rb::connect(None).ok()?;
            let screen = &conn.setup().roots[screen_num];
            
            let active_window = conn.get_property(
                false,
                screen.root,
                conn.atom("_NET_ACTIVE_WINDOW")?,
                conn.atom("WINDOW")?,
                0,
                1
            ).ok()?.reply().ok()?;

            if active_window.value.len() >= 4 {
                let window_id = u32::from_ne_bytes(active_window.value[0..4].try_into().ok()?);
                
                let pid = conn.get_property(
                    false,
                    window_id,
                    conn.atom("_NET_WM_PID")?,
                    conn.atom("CARDINAL")?,
                    0,
                    1
                ).ok()?.reply().ok()?;

                if pid.value.len() >= 4 {
                    let pid = u32::from_ne_bytes(pid.value[0..4].try_into().ok()?);
                    
                    let output = Command::new("ps")
                        .arg("-p")
                        .arg(pid.to_string())
                        .arg("-o")
                        .arg("comm=")
                        .output()
                        .ok()?;
                    
                    return String::from_utf8(output.stdout).ok()
                        .map(|s| s.trim().to_string());
                }
            }
            None
        }
    }
}

#[cfg(test)]
#[cfg(target_os = "linux")]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    use std::process::Command;
    use std::time::Duration;

    // 1. 基础窗口信息测试
    #[test]
    fn test_get_active_window() {
        let monitor = LinuxMonitor::new();
        let result = monitor.get_active_window();
        
        match result {
            Some(window_name) => {
                // Linux 下进程名通常不包含路径
                assert!(!window_name.is_empty());
                assert!(!window_name.contains('/'));
            }
            None => {} // 可能没有活动窗口，这是合法的
        }
    }

    #[test]
    fn test_multiple_queries() {
        let monitor = LinuxMonitor::new();
        
        // 连续多次查询窗口信息
        for _ in 0..5 {
            let result = monitor.get_active_window();
            match result {
                Some(name) => {
                    assert!(!name.is_empty());
                    assert!(!name.contains('\0')); // 不应包含空字符
                }
                None => continue,
            }
        }
    }

    // 2. X11 连接测试
    #[test]
    fn test_x11_connection() {
        if let Ok((conn, screen_num)) = x11rb::connect(None) {
            let screen = &conn.setup().roots[screen_num];
            assert!(screen.root != 0);
            
            // 测试根窗口属性获取
            if let Ok(atom) = conn.intern_atom(false, b"_NET_ACTIVE_WINDOW") {
                let atom_reply = atom.reply();
                assert!(atom_reply.is_ok());
            }
        }
    }

    // 3. 进程命令测试
    #[test]
    fn test_process_command() {
        // 测试 ps 命令
        let output = Command::new("ps")
            .arg("-e")
            .arg("-o")
            .arg("comm=")
            .output();
            
        assert!(output.is_ok());
        if let Ok(out) = output {
            assert!(!out.stdout.is_empty());
            let processes = String::from_utf8_lossy(&out.stdout);
            assert!(!processes.is_empty());
        }
    }

    // 4. 并发测试扩展
    #[test]
    fn test_concurrent_monitoring() {
        let monitor = Arc::new(LinuxMonitor::new());
        let mut handles = vec![];

        // 创建多个线程同时获取窗口信息
        for thread_id in 0..5 {
            let monitor_clone = Arc::clone(&monitor);
            let handle = thread::spawn(move || {
                let mut results = Vec::new();
                for i in 0..10 {
                    let window = monitor_clone.get_active_window();
                    results.push((thread_id, i, window.is_some()));
                    thread::sleep(Duration::from_millis(10));
                }
                results
            });
            handles.push(handle);
        }

        let mut all_results = Vec::new();
        for handle in handles {
            all_results.extend(handle.join().unwrap());
        }

        // 验证每个线程都完成了所有查询
        for thread_id in 0..5 {
            let thread_queries = all_results.iter()
                .filter(|(t, _, _)| *t == thread_id)
                .count();
            assert_eq!(thread_queries, 10);
        }
    }

    // 5. 错误处理测试扩展
    #[test]
    fn test_error_handling() {
        // 1. 测试无效的 pid
        let output = Command::new("ps")
            .arg("-p")
            .arg("999999999")
            .arg("-o")
            .arg("comm=")
            .output();
        assert!(output.is_ok());
        assert!(!output.unwrap().status.success());

        // 2. 测试错误的命令参数
        let output = Command::new("ps")
            .arg("--invalid-arg")
            .output();
        assert!(output.is_ok());
        assert!(!output.unwrap().status.success());

        // 3. 测试无效的环境变量
        std::env::remove_var("DISPLAY");
        let monitor = LinuxMonitor::new();
        let result = monitor.get_active_window();
        // 没有 DISPLAY 环境变量时应该返回 None
        assert!(result.is_none());
    }

    // 6. 进程名解析测试扩展
    #[test]
    fn test_process_name_parsing() {
        let test_cases = vec![
            // 基本测试
            ("simple", "simple"),
            ("process_name", "process_name"),
            ("process-name", "process-name"),
            ("process.name", "process.name"),
            
            // 空白字符测试
            ("  process  ", "process"),
            ("\tprocess\t", "process"),
            ("\nprocess\n", "process"),
            
            // 特殊字符测试
            ("process\0name", "processname"),
            ("process\u{200B}name", "processname"),
            
            // 长度边界测试
            ("a".repeat(1000).as_str(), "a".repeat(1000).as_str()),
            ("", ""),
            
            // Unicode 字符测试
            ("进程", "进程"),
            ("プロセス", "プロセス"),
            ("🔧process", "🔧process"),
        ];

        for (input, expected) in test_cases {
            let cleaned = input.trim()
                .replace('\0', "")
                .replace('\u{200B}', "");
            assert_eq!(cleaned, expected);
        }
    }

    // 7. 边界条件测试扩展
    #[test]
    fn test_edge_cases() {
        let monitor = LinuxMonitor::new();
        
        // 1. 快速切换测试
        let mut last_window = None;
        for _ in 0..10 {
            let current_window = monitor.get_active_window();
            if last_window.is_some() && current_window.is_some() {
                // 记录窗口切换
                assert!(last_window.as_ref() != current_window.as_ref());
            }
            last_window = current_window;
            thread::sleep(Duration::from_millis(100));
        }

        // 2. 资源限制测试
        let mut handles = vec![];
        for _ in 0..100 {
            let monitor = Arc::new(LinuxMonitor::new());
            handles.push(thread::spawn(move || {
                monitor.get_active_window()
            }));
        }
        
        for handle in handles {
            let _ = handle.join();
        }

        // 3. 内存压力测试
        let mut windows = Vec::new();
        for _ in 0..1000 {
            if let Some(window) = monitor.get_active_window() {
                windows.push(window);
            }
        }
    }

    // 8. 系统信息测试扩展
    #[test]
    fn test_system_info() {
        // 1. X11 环境检查
        let display = std::env::var("DISPLAY");
        if let Ok(display_val) = display {
            assert!(!display_val.is_empty());
        }

        // 2. 系统命令检查
        let required_commands = vec![
            "ps",
            "xwininfo",
            "xprop",
            "wmctrl",
            "xdotool",
        ];

        let mut missing_commands = Vec::new();
        for cmd in required_commands {
            let which_output = Command::new("which")
                .arg(cmd)
                .output();
                
            if which_output.is_err() || !which_output.unwrap().status.success() {
                missing_commands.push(cmd);
            }
        }

        if !missing_commands.is_empty() {
            println!("Warning: Missing commands: {:?}", missing_commands);
        }

        // 3. 桌面环境检查
        let desktop_env = std::env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| std::env::var("DESKTOP_SESSION"));
        if let Ok(env) = desktop_env {
            println!("Current desktop environment: {}", env);
        }
    }

    // 9. 集成测试扩展
    #[test]
    fn test_integrated_functionality() {
        let monitor = Arc::new(LinuxMonitor::new());
        
        // 1. 基本功能测试
        let mut window_stats = std::collections::HashMap::new();
        for _ in 0..10 {
            if let Some(window) = monitor.get_active_window() {
                *window_stats.entry(window).or_insert(0) += 1;
            }
            thread::sleep(Duration::from_millis(100));
        }

        // 2. 并发访问测试
        let monitor_clone = Arc::clone(&monitor);
        let handle = thread::spawn(move || {
            for _ in 0..5 {
                let _ = monitor_clone.get_active_window();
                thread::sleep(Duration::from_millis(50));
            }
        });

        for _ in 0..5 {
            let _ = monitor.get_active_window();
            thread::sleep(Duration::from_millis(50));
        }

        handle.join().unwrap();

        // 3. 打印统计信息
        for (window, count) in window_stats {
            println!("Window '{}' was active {} times", window, count);
        }
    }

    // 10. 性能测试扩展
    #[test]
    fn test_performance() {
        let monitor = LinuxMonitor::new();
        
        // 1. 单次查询性能
        let mut single_query_times = Vec::new();
        for _ in 0..100 {
            let start = std::time::Instant::now();
            let _ = monitor.get_active_window();
            single_query_times.push(start.elapsed());
        }

        // 计算统计信息
        let avg_time: Duration = single_query_times.iter().sum::<Duration>() / 100;
        let max_time = single_query_times.iter().max().unwrap();
        let min_time = single_query_times.iter().min().unwrap();

        println!("Performance statistics:");
        println!("Average query time: {:?}", avg_time);
        println!("Maximum query time: {:?}", max_time);
        println!("Minimum query time: {:?}", min_time);

        // 2. 并发性能测试
        let start = std::time::Instant::now();
        let monitor = Arc::new(monitor);
        let mut handles = vec![];

        for _ in 0..10 {
            let monitor_clone = Arc::clone(&monitor);
            handles.push(thread::spawn(move || {
                for _ in 0..10 {
                    let _ = monitor_clone.get_active_window();
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let concurrent_duration = start.elapsed();
        println!("100 concurrent queries took: {:?}", concurrent_duration);
        assert!(concurrent_duration.as_millis() < 5000);
    }

    // 11. 压力测试
    #[test]
    fn test_stress() {
        let monitor = Arc::new(LinuxMonitor::new());
        let duration = Duration::from_secs(2);
        let start = std::time::Instant::now();
        let mut handles = vec![];

        // 创建多个线程持续查询
        while start.elapsed() < duration {
            let monitor_clone = Arc::clone(&monitor);
            handles.push(thread::spawn(move || {
                let mut success_count = 0;
                let mut failure_count = 0;
                while start.elapsed() < duration {
                    match monitor_clone.get_active_window() {
                        Some(_) => success_count += 1,
                        None => failure_count += 1,
                    }
                }
                (success_count, failure_count)
            }));

            thread::sleep(Duration::from_millis(100));
        }

        // 收集结果
        let mut total_success = 0;
        let mut total_failure = 0;
        for handle in handles {
            let (success, failure) = handle.join().unwrap();
            total_success += success;
            total_failure += failure;
        }

        println!("Stress test results:");
        println!("Successful queries: {}", total_success);
        println!("Failed queries: {}", total_failure);
        println!("Success rate: {:.2}%", 
            (total_success as f64 / (total_success + total_failure) as f64) * 100.0);
    }
}