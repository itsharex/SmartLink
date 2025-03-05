use std::io;
use std::fmt;
use rusqlite;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result};
use tauri::{AppHandle, Manager};
use std::collections::HashMap;
use super::types::{AppUsageRecord, AppUsageStats, DailyUsage};

#[derive(Debug)]
pub enum StorageError {
    Io(io::Error),
    Sqlite(rusqlite::Error),
}

impl From<io::Error> for StorageError {
    fn from(err: io::Error) -> Self {
        StorageError::Io(err)
    }
}

impl From<rusqlite::Error> for StorageError {
    fn from(err: rusqlite::Error) -> Self {
        StorageError::Sqlite(err)
    }
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StorageError::Io(err) => write!(f, "IO Error: {}", err),
            StorageError::Sqlite(err) => write!(f, "SQLite Error: {}", err),
        }
    }
}

pub struct Storage {
    conn: Connection,
}

impl Storage {
    pub fn new(app_handle: &AppHandle) -> Result<Self, StorageError> {
        let data_dir = app_handle
            .path().app_data_dir()
            .expect("Failed to get app data dir");
        
        tracing::info!("Creating data directory: {:?}", data_dir);
        std::fs::create_dir_all(&data_dir)?;
        
        let db_path = data_dir.join("usage_stats.db");
        tracing::info!("Database path: {:?}", db_path);
        
        let conn = Connection::open(&db_path)?;
        tracing::info!("Database connection established");
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS app_usage (
                id INTEGER PRIMARY KEY,
                timestamp TEXT NOT NULL,
                app_name TEXT NOT NULL,
                duration INTEGER NOT NULL
            )",
            [],
        )?;
        tracing::info!("Database table created/verified");
        
        Ok(Self { conn })
    }
    
    pub fn record_usage(&self, record: AppUsageRecord) -> Result<(), StorageError> {
        tracing::info!("Recording usage for: {} at {}", record.app_name, record.timestamp);
        self.conn.execute(
            "INSERT INTO app_usage (timestamp, app_name, duration) VALUES (?1, ?2, ?3)",
            (
                record.timestamp.to_rfc3339(),
                &record.app_name,
                record.duration,
            ),
        )?;
        tracing::info!("Usage recorded successfully");
        Ok(())
    }
    
    pub fn get_usage_stats(&self, range: &str) -> Result<Vec<AppUsageStats>, StorageError> {
        let sql = match range {
            "daily" => "
                SELECT app_name, 
                       strftime('%Y-%m-%d', timestamp) as date,
                       SUM(duration) as daily_duration
                FROM app_usage 
                WHERE date(timestamp) = date('now', 'localtime')
                GROUP BY app_name, date
                ORDER BY daily_duration DESC
            ",
            "3days" => "
                SELECT app_name, 
                       strftime('%Y-%m-%d', timestamp) as date,
                       SUM(duration) as daily_duration
                FROM app_usage 
                WHERE timestamp >= datetime('now', '-3 days', 'localtime')
                GROUP BY app_name, date
                ORDER BY daily_duration DESC
            ",
            "weekly" => "
                SELECT app_name, 
                       strftime('%Y-%m-%d', timestamp) as date,
                       SUM(duration) as daily_duration
                FROM app_usage 
                WHERE timestamp >= datetime('now', '-7 days', 'localtime')
                GROUP BY app_name, date
                ORDER BY daily_duration DESC
            ",
            "monthly" => "
                SELECT app_name, 
                       strftime('%Y-%m-%d', timestamp) as date,
                       SUM(duration) as daily_duration
                FROM app_usage 
                WHERE timestamp >= datetime('now', '-30 days', 'localtime')
                GROUP BY app_name, date
                ORDER BY daily_duration DESC
            ",
            _ => "
                SELECT app_name, 
                       strftime('%Y-%m-%d', timestamp) as date,
                       SUM(duration) as daily_duration
                FROM app_usage 
                WHERE date(timestamp) = date('now', 'localtime')
                GROUP BY app_name, date
                ORDER BY daily_duration DESC
            ",
        };

        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
            ))
        })?;
        
        // 组织数据
        let mut app_stats: HashMap<String, AppUsageStats> = HashMap::new();
        
        for row in rows {
            let (app_name, date_str, duration) = row?;
            let date = DateTime::parse_from_rfc3339(&format!("{}T00:00:00Z", date_str))
                .unwrap()
                .with_timezone(&Utc);
                
            app_stats.entry(app_name.clone())
                .or_insert_with(|| AppUsageStats {
                    name: app_name.clone(),
                    total_time: 0,
                    daily_usage: Vec::new(),
                })
                .daily_usage
                .push(DailyUsage {
                    date,
                    duration: duration as u64,
                });
        }
        
        // 计算总时长并排序
        let mut stats: Vec<AppUsageStats> = app_stats.into_values()
            .map(|mut stat| {
                stat.total_time = stat.daily_usage.iter()
                    .map(|d| d.duration)
                    .sum();
                stat
            })
            .collect();
            
        stats.sort_by(|a, b| b.total_time.cmp(&a.total_time));
        
        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Duration, Timelike, Utc};
    use rusqlite::Connection;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS app_usage (
                id INTEGER PRIMARY KEY,
                timestamp TEXT NOT NULL,
                app_name TEXT NOT NULL,
                duration INTEGER NOT NULL
            )",
            [],
        ).unwrap();
        conn
    }

    // 1. 基础插入和查询测试
    #[test]
    fn test_basic_record_insert() {
        let conn = setup_test_db();
        conn.execute(
            "INSERT INTO app_usage (timestamp, app_name, duration) VALUES (?1, ?2, ?3)",
            [
                Utc::now().to_rfc3339(),
                "test_app".to_string(),
                3600i64.to_string(),
            ],
        ).unwrap();

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM app_usage",
            [],
            |row| row.get(0)
        ).unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_multiple_records() {
        let conn = setup_test_db();
        let now = Utc::now();
        
        // 插入多条记录
        let records = vec![
            (now, "app1", 3600),
            (now, "app2", 7200),
            (now, "app3", 1800),
        ];

        for (timestamp, app_name, duration) in records {
            conn.execute(
                "INSERT INTO app_usage (timestamp, app_name, duration) VALUES (?1, ?2, ?3)",
                [
                    timestamp.to_rfc3339(),
                    app_name.to_string(),
                    duration.to_string(),
                ],
            ).unwrap();
        }

        // 验证总记录数
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM app_usage",
            [],
            |row| row.get(0)
        ).unwrap();
        assert_eq!(count, 3);
    }

    // 2. 复杂查询测试
    #[test]
    fn test_daily_aggregation() {
        let conn = setup_test_db();
        let now = Utc::now();
        
        // 插入同一天不同时间的记录
        let records = vec![
            (now, "app1", 3600),
            (now + Duration::hours(1), "app1", 1800),
            (now + Duration::hours(2), "app1", 2400),
        ];

        for (timestamp, app_name, duration) in records {
            conn.execute(
                "INSERT INTO app_usage (timestamp, app_name, duration) VALUES (?1, ?2, ?3)",
                [
                    timestamp.to_rfc3339(),
                    app_name.to_string(),
                    duration.to_string(),
                ],
            ).unwrap();
        }

        // 验证日统计
        let total_duration: i64 = conn.query_row(
            "SELECT SUM(duration) FROM app_usage 
             WHERE date(timestamp) = date(?1)",
            [now.to_rfc3339()],
            |row| row.get(0)
        ).unwrap();
        assert_eq!(total_duration, 7800); // 3600 + 1800 + 2400
    }

    #[test]
    fn test_weekly_stats() {
        let conn = setup_test_db();
        let now = Utc::now();
        
        // 插入一周内的记录
        let records = vec![
            (now, "app1", 3600),                           // 今天
            (now - Duration::days(1), "app1", 7200),      // 昨天
            (now - Duration::days(2), "app1", 5400),      // 前天
            (now - Duration::days(6), "app1", 1800),      // 6天前
            (now - Duration::days(8), "app1", 3600),      // 8天前（不应计入）
        ];

        for (timestamp, app_name, duration) in records {
            conn.execute(
                "INSERT INTO app_usage (timestamp, app_name, duration) VALUES (?1, ?2, ?3)",
                [
                    timestamp.to_rfc3339(),
                    app_name.to_string(),
                    duration.to_string(),
                ],
            ).unwrap();
        }

        // 验证7天内的统计
        let total_duration: i64 = conn.query_row(
            "SELECT SUM(duration) FROM app_usage 
             WHERE timestamp >= datetime(?1, '-7 days')",
            [now.to_rfc3339()],
            |row| row.get(0)
        ).unwrap();
        assert_eq!(total_duration, 18000); // 所有7天内的记录总和
    }

    #[test]
    fn test_app_ranking() {
        let conn = setup_test_db();
        let now = Utc::now();
        
        // 插入多个应用的使用记录
        let records = vec![
            ("app1", 3600),  // 第二
            ("app2", 7200),  // 第一
            ("app3", 1800),  // 第四
            ("app2", 3600),  // app2 的另一条记录
            ("app4", 2400),  // 第三
        ];

        for (app_name, duration) in records {
            conn.execute(
                "INSERT INTO app_usage (timestamp, app_name, duration) VALUES (?1, ?2, ?3)",
                [
                    now.to_rfc3339(),
                    app_name.to_string(),
                    duration.to_string(),
                ],
            ).unwrap();
        }

        // 验证应用排名
        let mut stmt = conn.prepare(
            "SELECT app_name, SUM(duration) as total_duration 
             FROM app_usage 
             GROUP BY app_name 
             ORDER BY total_duration DESC"
        ).unwrap();

        let app_stats: Vec<(String, i64)> = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

        assert_eq!(app_stats.len(), 4);
        assert_eq!(app_stats[0].0, "app2");  // 总时长 10800
        assert_eq!(app_stats[1].0, "app1");  // 总时长 3600
        assert_eq!(app_stats[2].0, "app4");  // 总时长 2400
        assert_eq!(app_stats[3].0, "app3");  // 总时长 1800
    }

    #[test]
    fn test_time_window_boundaries() {
        let conn = setup_test_db();
        let now = Utc::now();
        
        // 插入边界时间的记录
        let records = vec![
            (now, "app1", 1000),                                      // 现在
            (now - Duration::days(2), "app1", 2000),                 // 2天前
            (now - Duration::days(3), "app1", 3000),                 // 3天前
            (now - Duration::days(3) + Duration::hours(23), "app1", 4000),  // 刚好在3天内
            (now - Duration::days(3) - Duration::minutes(1), "app1", 5000), // 刚好超过3天
        ];

        for (timestamp, app_name, duration) in records {
            conn.execute(
                "INSERT INTO app_usage (timestamp, app_name, duration) VALUES (?1, ?2, ?3)",
                [
                    timestamp.to_rfc3339(),
                    app_name.to_string(),
                    duration.to_string(),
                ],
            ).unwrap();
        }

        // 验证3天内的记录
        let total_duration: i64 = conn.query_row(
            "SELECT SUM(duration) FROM app_usage 
             WHERE timestamp >= datetime(?1, '-3 days')",
            [now.to_rfc3339()],
            |row| row.get(0)
        ).unwrap();
        assert_eq!(total_duration, 10000); // 1000 + 2000 + 3000 + 4000
    }

    #[test]
    fn test_complex_daily_patterns() {
        let conn = setup_test_db();
        let now = Utc::now();
        
        // 模拟一天中不同时段的使用模式
        let patterns = vec![
            (now.with_hour(9).unwrap(), "work_app", 3600),     // 上午
            (now.with_hour(13).unwrap(), "lunch_app", 1800),   // 午餐
            (now.with_hour(15).unwrap(), "work_app", 7200),    // 下午
            (now.with_hour(19).unwrap(), "game_app", 3600),    // 晚上
            (now.with_hour(22).unwrap(), "social_app", 1800),  // 深夜
        ];

        for (timestamp, app_name, duration) in patterns {
            conn.execute(
                "INSERT INTO app_usage (timestamp, app_name, duration) VALUES (?1, ?2, ?3)",
                [
                    timestamp.to_rfc3339(),
                    app_name.to_string(),
                    duration.to_string(),
                ],
            ).unwrap();
        }

        // 按时段统计
        let mut stmt = conn.prepare(
            "SELECT 
                app_name,
                strftime('%H', timestamp) as hour,
                SUM(duration) as total_duration
             FROM app_usage 
             GROUP BY app_name, hour
             ORDER BY hour, total_duration DESC"
        ).unwrap();

        let time_patterns: Vec<(String, String, i64)> = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

        assert_eq!(time_patterns.len(), 5);
    }

    #[test]
    fn test_monthly_trends() {
        let conn = setup_test_db();
        let now = Utc::now();
        
        // 插入一个月内的使用记录
        let mut records = Vec::new();
        for days_ago in 0..30 {
            let timestamp = now - Duration::days(days_ago);
            records.push((
                timestamp,
                "daily_app".to_string(),
                1800 + (days_ago as i64 * 100), // 时长呈现趋势
            ));
        }

        for (timestamp, app_name, duration) in records {
            conn.execute(
                "INSERT INTO app_usage (timestamp, app_name, duration) VALUES (?1, ?2, ?3)",
                [
                    timestamp.to_rfc3339(),
                    app_name,
                    duration.to_string(),
                ],
            ).unwrap();
        }

        // 按周统计趋势
        let mut stmt = conn.prepare(
            "SELECT 
                strftime('%W', timestamp) as week,
                AVG(duration) as avg_duration,
                MIN(duration) as min_duration,
                MAX(duration) as max_duration
             FROM app_usage 
             GROUP BY week
             ORDER BY week DESC"
        ).unwrap();

        let weekly_stats: Vec<(String, f64, i64, i64)> = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

        assert!(!weekly_stats.is_empty());
        
        // 验证趋势（每周的平均时长应该呈现变化）
        for i in 1..weekly_stats.len() {
            assert!(weekly_stats[i-1].1 != weekly_stats[i].1);
        }
    }

    #[test]
    fn test_usage_patterns() {
        let conn = setup_test_db();
        let now = Utc::now();
        
        // 模拟一周的使用模式
        let weekdays = vec!["Mon", "Tue", "Wed", "Thu", "Fri"];
        let weekend = vec!["Sat", "Sun"];
        
        // 工作日模式
        for day in weekdays {
            conn.execute(
                "INSERT INTO app_usage (timestamp, app_name, duration) VALUES 
                 (?1, 'work_app', 28800),
                 (?1, 'browser', 3600),
                 (?1, 'email', 1800)",
                [format!("2024-01-{} 09:00:00Z", day)],
            ).unwrap();
        }
        
        // 周末模式
        for day in weekend {
            conn.execute(
                "INSERT INTO app_usage (timestamp, app_name, duration) VALUES 
                 (?1, 'game_app', 14400),
                 (?1, 'browser', 7200),
                 (?1, 'media_player', 10800)",
                [format!("2024-01-{} 10:00:00Z", day)],
            ).unwrap();
        }

        // 分析工作日vs周末的使用模式
        let mut stmt = conn.prepare(
            "SELECT 
                app_name,
                CASE 
                    WHEN strftime('%w', timestamp) IN ('0', '6') THEN 'weekend'
                    ELSE 'weekday'
                END as day_type,
                AVG(duration) as avg_duration
             FROM app_usage 
             GROUP BY app_name, day_type
             ORDER BY day_type, avg_duration DESC"
        ).unwrap();

        let patterns: Vec<(String, String, f64)> = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

        // 验证工作日和周末的使用模式不同
        let weekday_apps: Vec<_> = patterns.iter()
            .filter(|p| p.1 == "weekday")
            .map(|p| &p.0)
            .collect();
        let weekend_apps: Vec<_> = patterns.iter()
            .filter(|p| p.1 == "weekend")
            .map(|p| &p.0)
            .collect();

        assert!(weekday_apps.contains(&&"work_app".to_string()));
        assert!(weekend_apps.contains(&&"game_app".to_string()));
    }

    #[test]
    fn test_concurrent_usage() {
        let conn = setup_test_db();
        let now = Utc::now();
        
        // 模拟同时运行的多个应用
        let concurrent_apps = vec![
            ("browser", now, now + Duration::hours(2)),
            ("editor", now, now + Duration::hours(1)),
            ("terminal", now + Duration::minutes(30), now + Duration::hours(2)),
            ("chat_app", now + Duration::hours(1), now + Duration::hours(3)),
        ];

        for (app_name, start_time, end_time) in concurrent_apps {
            let duration = (end_time - start_time).num_seconds() as i64;
            conn.execute(
                "INSERT INTO app_usage (timestamp, app_name, duration) VALUES (?1, ?2, ?3)",
                [
                    start_time.to_rfc3339(),
                    app_name.to_string(),
                    duration.to_string(),
                ],
            ).unwrap();
        }

        // 分析重叠时间段的应用数量
        let mut stmt = conn.prepare(
            "WITH RECURSIVE 
             time_points AS (
                SELECT DISTINCT timestamp as point
                FROM app_usage
                UNION
                SELECT datetime(point, '+30 minutes')
                FROM time_points
                WHERE datetime(point, '+30 minutes') <= (
                    SELECT MAX(timestamp) FROM app_usage
                )
             )
             SELECT 
                t.point,
                COUNT(DISTINCT a.app_name) as active_apps
             FROM time_points t
             LEFT JOIN app_usage a ON datetime(t.point) >= datetime(a.timestamp)
                AND datetime(t.point) <= datetime(a.timestamp, '+' || (a.duration/60) || ' minutes')
             GROUP BY t.point
             ORDER BY t.point"
        ).unwrap();

        let time_slices: Vec<(String, i64)> = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

        // 验证存在同时运行多个应用的时间段
        assert!(time_slices.iter().any(|slice| slice.1 > 1));
    }

    #[test]
    fn test_application_switching() {
        let conn = setup_test_db();
        let now = Utc::now();
        
        // 模拟用户在应用间切换的行为
        let switches = vec![
            ("browser", 600),    // 10分钟
            ("editor", 300),     // 5分钟
            ("browser", 900),    // 15分钟
            ("chat", 180),       // 3分钟
            ("editor", 1200),    // 20分钟
            ("browser", 300),    // 5分钟
        ];

        let mut current_time = now;
        for (app_name, duration) in switches {
            conn.execute(
                "INSERT INTO app_usage (timestamp, app_name, duration) VALUES (?1, ?2, ?3)",
                [
                    current_time.to_rfc3339(),
                    app_name.to_string(),
                    duration.to_string(),
                ],
            ).unwrap();
            current_time = current_time + Duration::seconds(duration);
        }

        // 分析应用切换模式
        let mut stmt = conn.prepare(
            "WITH ordered_usage AS (
                SELECT 
                    app_name,
                    timestamp,
                    duration,
                    LAG(app_name) OVER (ORDER BY timestamp) as prev_app
                FROM app_usage
            )
            SELECT 
                prev_app,
                app_name as next_app,
                COUNT(*) as switch_count
            FROM ordered_usage
            WHERE prev_app IS NOT NULL
            GROUP BY prev_app, next_app
            ORDER BY switch_count DESC"
        ).unwrap();

        let switches: Vec<(String, String, i64)> = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

        // 验证应用切换的存在
        assert!(!switches.is_empty());
        
        // 验证最频繁的切换
        let most_frequent = &switches[0];
        assert_eq!(most_frequent.2, 2); // browser -> editor 出现2次
    }

    #[test]
    fn test_long_term_trends() {
        let conn = setup_test_db();
        
        // 模拟半年的使用数据
        let start_date = Utc::now() - Duration::days(180);
        let mut current_date = start_date;
        let end_date = Utc::now();

        let apps = vec!["browser", "editor", "terminal", "chat"];
        
        while current_date < end_date {
            for app in &apps {
                // 模拟随时间变化的使用模式
                let base_duration = 3600; // 基础1小时
                let day_factor = current_date.day() as f64 / 31.0; // 根据月份日期变化
                let month_factor = current_date.month() as f64 / 12.0; // 根据月份变化
                let duration = (base_duration as f64 * (1.0 + day_factor + month_factor)) as i64;

                conn.execute(
                    "INSERT INTO app_usage (timestamp, app_name, duration) VALUES (?1, ?2, ?3)",
                    [
                        current_date.to_rfc3339(),
                        app.to_string(),
                        duration.to_string(),
                    ],
                ).unwrap();
            }
            current_date = current_date + Duration::days(1);
        }

        // 分析月度趋势
        let mut stmt = conn.prepare(
            "SELECT 
                strftime('%Y-%m', timestamp) as month,
                app_name,
                SUM(duration) as total_duration,
                AVG(duration) as avg_duration,
                COUNT(*) as usage_days
             FROM app_usage 
             GROUP BY month, app_name
             ORDER BY month, total_duration DESC"
        ).unwrap();

        let trends: Vec<(String, String, i64, f64, i64)> = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?))
        })
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

        // 验证每个月都有数据
        let months: Vec<_> = trends.iter()
            .map(|t| &t.0)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        assert!(months.len() >= 6);

        // 验证所有应用都有数据
        let recorded_apps: Vec<_> = trends.iter()
            .map(|t| &t.1)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        assert_eq!(recorded_apps.len(), apps.len());
    }
}