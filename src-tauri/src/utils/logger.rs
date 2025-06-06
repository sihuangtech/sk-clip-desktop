use log::{info, warn, error, debug};

/// 初始化日志系统
pub fn init_logger() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
}

/// 记录操作开始
pub fn log_operation_start(operation: &str, details: &str) {
    info!("开始操作: {} - {}", operation, details);
}

/// 记录操作完成
pub fn log_operation_complete(operation: &str, duration_ms: u64) {
    info!("操作完成: {} - 耗时: {}ms", operation, duration_ms);
}

/// 记录操作失败
pub fn log_operation_error(operation: &str, error: &str) {
    error!("操作失败: {} - 错误: {}", operation, error);
}

/// 记录警告信息
pub fn log_warning(message: &str) {
    warn!("{}", message);
}

/// 记录调试信息
pub fn log_debug(message: &str) {
    debug!("{}", message);
}

/// 记录文件操作
pub fn log_file_operation(operation: &str, file_path: &str) {
    info!("文件操作: {} - 文件: {}", operation, file_path);
}

/// 记录性能指标
pub fn log_performance(metric_name: &str, value: f64, unit: &str) {
    info!("性能指标: {} = {:.2} {}", metric_name, value, unit);
} 