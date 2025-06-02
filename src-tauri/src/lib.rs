// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// 声明项目模块
pub mod commands; // Tauri 命令处理模块
pub mod models;   // 数据结构和类型定义模块
pub mod file_manager; // 文件管理模块
pub mod video_processor; // 视频处理模块
pub mod document_parser; // 文档解析模块
// pub mod ai_models; // AI 模型集成模块 (待实现)
// pub mod task_manager; // 任务管理模块 (待实现)
// pub mod timeline; // 时间线数据管理模块 (待实现)

// 从 modules 中导入需要直接在 lib.rs 中使用的项
use models::{AppState, AppError}; // 从 models 模块导入 AppState 和 AppError
use log::info;
use anyhow;

// 实现从AppError到tauri::Error的转换
impl From<AppError> for tauri::Error {
    fn from(err: AppError) -> Self {
        tauri::Error::Anyhow(anyhow::anyhow!(err.to_string()))
    }
}

// 实现从anyhow::Error到AppError的转换
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Unknown(err.to_string())
    }
}

// 应用的入口点，由 Tauri 框架调用
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    info!("启动 Multisay 视频创作与翻译工具...");

    // Tauri 应用构建器
    tauri::Builder::default()
        // 初始化插件
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        // 管理应用状态，可以在多个命令中共享
        .manage(AppState::default())
        // 注册 Tauri 命令
        .invoke_handler(tauri::generate_handler![
            commands::greet, // 保留示例命令
            commands::upload_video,
            commands::translate_video,
            commands::get_translation_task,
            commands::check_task_output,
            commands::import_document,
            commands::get_supported_document_types,
            commands::convert_document_to_assets,
            // 在这里注册其他命令...
        ])
        // 运行应用
        .run(tauri::generate_context!())
        .expect("运行 Tauri 应用时出错");
}
