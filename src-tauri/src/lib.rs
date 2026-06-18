// 声明项目模块
pub mod commands;
pub mod models;
pub mod config;
pub mod file_manager;
pub mod ai;
pub mod video;
pub mod document;
pub mod utils;
pub mod model_manager;

use models::{AppState, AppError};
use log::info;

impl From<AppError> for tauri::Error {
    fn from(err: AppError) -> Self {
        tauri::Error::Anyhow(anyhow::anyhow!(err.to_string()))
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Unknown(err.to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    info!("启动彩旗剪辑视频创作工具...");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::basic::greet,
            commands::config::get_app_config,
            commands::config::update_app_config,
            commands::video::upload_video,
            commands::video::get_video_info,
            commands::video::extract_audio_from_video,
            commands::video::trim_video_command,
            commands::video::merge_videos_command,
            commands::video::add_subtitles_command,
            commands::video::resize_video_command,
            commands::video::create_video_thumbnail,
            commands::video::translate_video,
            commands::video::get_translation_task,
            commands::video::check_task_output,
            commands::ai::recognize_speech,
            commands::ai::translate_text,
            commands::ai::synthesize_speech,
            commands::ai::get_ai_model_status,
            commands::ai::initialize_ai_models,
            commands::document::import_document,
            commands::document::get_supported_document_types,
            commands::document::convert_document_to_assets,
            commands::timeline::create_timeline_project,
            commands::timeline::save_timeline_project,
            commands::timeline::load_timeline_project,
            commands::timeline::export_timeline_video,
            commands::timeline::list_timeline_projects,
            commands::timeline::delete_timeline_project,
            commands::proxy::get_proxy_config,
            commands::proxy::apply_proxy_profile,
            commands::proxy::disable_proxy,
            commands::proxy::test_proxy_connection,
            commands::proxy::auto_detect_proxy,
            commands::proxy::get_mirror_url,
            commands::proxy::test_download_connection,
        ])
        .run(tauri::generate_context!())
        .expect("运行 Tauri 应用时出错");
}
