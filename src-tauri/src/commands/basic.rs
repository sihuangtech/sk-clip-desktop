// src-tauri/src/commands/basic.rs

// 基础命令模块，包含一些通用的基础功能命令

use log::info;

// 示例命令：打招呼
#[tauri::command]
pub fn greet(name: &str) -> String {
    info!("调用 greet 命令，参数: {}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 检查语言代码是否支持
pub fn is_language_supported(lang_code: &str) -> bool {
    matches!(lang_code, "zh" | "en" | "ja" | "ko" | "es" | "ru" | "de" | "fr" | "pt")
} 