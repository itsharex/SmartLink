use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppUsageRecord {
    pub timestamp: DateTime<Utc>,
    pub app_name: String,
    pub duration: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppUsageStats {
    pub name: String,
    pub total_time: u64,
    pub daily_usage: Vec<DailyUsage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DailyUsage {
    pub date: DateTime<Utc>,
    pub duration: u64,
}