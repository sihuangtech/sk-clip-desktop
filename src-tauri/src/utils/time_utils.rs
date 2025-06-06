/// 格式化时长（秒）为可读字符串
pub fn format_duration(seconds: f64) -> String {
    let total_seconds = seconds as u64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let secs = total_seconds % 60;
    
    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{:02}:{:02}", minutes, secs)
    }
}

/// 解析时间字符串为秒数
pub fn parse_time_string(time_str: &str) -> Result<f64, String> {
    let parts: Vec<&str> = time_str.split(':').collect();
    
    match parts.len() {
        2 => {
            // MM:SS 格式
            let minutes: f64 = parts[0].parse().map_err(|_| "无效的分钟数")?;
            let seconds: f64 = parts[1].parse().map_err(|_| "无效的秒数")?;
            Ok(minutes * 60.0 + seconds)
        }
        3 => {
            // HH:MM:SS 格式
            let hours: f64 = parts[0].parse().map_err(|_| "无效的小时数")?;
            let minutes: f64 = parts[1].parse().map_err(|_| "无效的分钟数")?;
            let seconds: f64 = parts[2].parse().map_err(|_| "无效的秒数")?;
            Ok(hours * 3600.0 + minutes * 60.0 + seconds)
        }
        _ => Err("时间格式应为 MM:SS 或 HH:MM:SS".to_string()),
    }
}

/// 获取当前时间戳
pub fn current_timestamp() -> i64 {
    chrono::Utc::now().timestamp()
}

/// 获取当前时间字符串
pub fn current_time_string() -> String {
    chrono::Utc::now().to_rfc3339()
}

/// 格式化时间戳为可读字符串
pub fn format_timestamp(timestamp: i64) -> String {
    if let Some(datetime) = chrono::DateTime::from_timestamp(timestamp, 0) {
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    } else {
        "无效时间".to_string()
    }
} 