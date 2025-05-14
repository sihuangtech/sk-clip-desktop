// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 导入所需的库
use std::{path::{Path, PathBuf}, fs, process::Command, sync::Arc};
use anyhow::{Result, anyhow};
use log::{info, error};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State, Manager};
use tempfile::tempdir;
use tokio::sync::Mutex;
use uuid::Uuid;
use std::collections::HashMap;

// 视频翻译任务状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

// 视频翻译任务结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationTask {
    id: String,
    video_path: String,
    status: TaskStatus,
    source_language: String,
    target_language: String,
    output_path: Option<String>,
    error_message: Option<String>,
}

// 应用状态结构体，用于在命令间共享状态
#[derive(Default)]
pub struct AppState {
    tasks: Mutex<HashMap<String, TranslationTask>>,
    temp_dir: Mutex<Option<tempfile::TempDir>>,
}

// 错误类型定义
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("视频处理失败: {0}")]
    VideoProcessingError(String),
    
    #[error("文件操作失败: {0}")]
    FileError(#[from] std::io::Error),
    
    #[error("任务不存在: {0}")]
    TaskNotFound(String),
    
    #[error("命令执行失败: {0}")]
    CommandError(String),
    
    #[error("不受支持的语言: {0}")]
    UnsupportedLanguage(String),
    
    #[error("未知错误: {0}")]
    Unknown(String),
}

// 实现从AppError到tauri::Error的转换
impl From<AppError> for tauri::Error {
    fn from(err: AppError) -> Self {
        tauri::Error::Runtime(err.to_string())
    }
}

// 实现从anyhow::Error到AppError的转换
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Unknown(err.to_string())
    }
}

// 视频上传命令的输入参数
#[derive(Debug, Deserialize)]
pub struct UploadVideoArgs {
    path: String,
}

// 翻译视频命令的输入参数
#[derive(Debug, Deserialize)]
pub struct TranslateVideoArgs {
    video_path: String,
    source_language: String,
    target_language: String,
}

// 检查语言代码是否支持（目前支持中文、英文、日语和韩语）
fn is_language_supported(lang_code: &str) -> bool {
    matches!(lang_code, "zh" | "en" | "ja" | "ko")
}

// 获取应用数据目录
fn get_app_data_dir() -> Result<PathBuf, AppError> {
    let data_dir = dirs::data_dir()
        .ok_or_else(|| AppError::Unknown("无法获取应用数据目录".to_string()))?
        .join("multisay");
    
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)?;
    }
    
    Ok(data_dir)
}

// 上传视频文件命令
#[tauri::command]
async fn upload_video(app_handle: AppHandle, args: UploadVideoArgs) -> Result<String, AppError> {
    info!("上传视频: {}", args.path);
    
    // 创建应用数据目录
    let app_data_dir = get_app_data_dir()?;
    let videos_dir = app_data_dir.join("videos");
    if !videos_dir.exists() {
        fs::create_dir_all(&videos_dir)?;
    }
    
    // 生成唯一文件名
    let file_id = Uuid::new_v4().to_string();
    let file_extension = Path::new(&args.path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("mp4");
    
    let target_path = videos_dir.join(format!("{}.{}", file_id, file_extension));
    
    // 在实际应用中，这里应该从前端复制上传的文件到 target_path
    // 由于我们在这个示例中无法真正接收文件，所以我们只返回目标路径
    // 实际实现中，需要使用 tauri 的 fs API 来复制文件
    
    Ok(target_path.to_string_lossy().to_string())
}

// 翻译视频命令
#[tauri::command]
async fn translate_video(
    app_handle: AppHandle, 
    state: State<'_, AppState>,
    args: TranslateVideoArgs
) -> Result<String, AppError> {
    info!("开始翻译视频: {}", args.video_path);
    
    // 检查源语言和目标语言是否支持
    if !is_language_supported(&args.source_language) {
        return Err(AppError::UnsupportedLanguage(args.source_language));
    }
    if !is_language_supported(&args.target_language) {
        return Err(AppError::UnsupportedLanguage(args.target_language));
    }
    
    // 创建任务ID
    let task_id = Uuid::new_v4().to_string();
    
    // 创建任务并保存到状态中
    let task = TranslationTask {
        id: task_id.clone(),
        video_path: args.video_path.clone(),
        status: TaskStatus::Pending,
        source_language: args.source_language.clone(),
        target_language: args.target_language.clone(),
        output_path: None,
        error_message: None,
    };
    
    state.tasks.lock().await.insert(task_id.clone(), task);
    
    // 获取应用数据目录
    let app_data_dir = get_app_data_dir()?;
    let output_dir = app_data_dir.join("outputs");
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir)?;
    }
    
    // 生成输出文件路径
    let output_path = output_dir.join(format!("translated_{}.mp4", task_id));
    
    // 在实际应用中，这里应该启动异步任务来处理视频翻译
    // 但在这个示例中，我们只模拟一下过程
    
    // 更新任务状态为处理中
    {
        let mut tasks = state.tasks.lock().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            task.status = TaskStatus::Processing;
        }
    }
    
    // 模拟异步处理视频翻译过程
    // 实际应用中，这里应该使用 FFmpeg 进行音视频分离，然后使用 Whisper 进行语音识别，
    // 接着进行翻译和语音合成，最后使用 FFmpeg 进行视频重建
    tokio::spawn(async move {
        // 在此处理实际翻译逻辑
        match process_video_translation(&args.video_path, &output_path, &args.source_language, &args.target_language).await {
            Ok(_) => {
                // 更新任务状态为完成
                let mut tasks = state.tasks.lock().await;
                if let Some(task) = tasks.get_mut(&task_id) {
                    task.status = TaskStatus::Completed;
                    task.output_path = Some(output_path.to_string_lossy().to_string());
                }
            },
            Err(e) => {
                error!("视频翻译失败: {}", e);
                // 更新任务状态为失败
                let mut tasks = state.tasks.lock().await;
                if let Some(task) = tasks.get_mut(&task_id) {
                    task.status = TaskStatus::Failed;
                    task.error_message = Some(e.to_string());
                }
            }
        }
    });
    
    // 返回任务ID
    Ok(task_id)
}

// 获取翻译任务状态命令
#[tauri::command]
async fn get_translation_task(
    state: State<'_, AppState>,
    task_id: String
) -> Result<TranslationTask, AppError> {
    let tasks = state.tasks.lock().await;
    tasks.get(&task_id)
        .cloned()
        .ok_or_else(|| AppError::TaskNotFound(task_id))
}

// 模拟视频翻译处理过程
async fn process_video_translation(
    input_path: &str,
    output_path: &Path,
    source_language: &str,
    target_language: &str
) -> Result<(), AppError> {
    // 在实际应用中，这里应该是实际的视频处理逻辑
    // 1. 使用 FFmpeg 分离音频
    // 2. 使用 Whisper 进行语音识别
    // 3. 翻译文本
    // 4. 使用 TTS 生成目标语言语音
    // 5. 使用口型同步技术同步视频
    // 6. 使用 FFmpeg 合成最终视频
    
    // 这里我们只模拟一下过程，实际上只是复制输入文件到输出路径
    // 在真实应用中应替换为实际的处理逻辑
    
    // 为了模拟异步处理的时间，添加一个延迟
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    
    // 假设我们已经成功处理了视频，返回输出路径
    // 注意：在实际应用中，应该实现真正的视频处理逻辑
    // 这里仅作为示例
    fs::copy(input_path, output_path)?;
    
    info!("视频翻译处理完成: {}", output_path.display());
    Ok(())
}

// 检查任务并获取输出路径命令
#[tauri::command]
async fn check_task_output(
    state: State<'_, AppState>,
    task_id: String
) -> Result<String, AppError> {
    let tasks = state.tasks.lock().await;
    let task = tasks.get(&task_id)
        .ok_or_else(|| AppError::TaskNotFound(task_id.clone()))?;
    
    match &task.status {
        TaskStatus::Completed => {
            task.output_path.clone()
                .ok_or_else(|| AppError::Unknown(format!("任务 {} 没有输出路径", task_id)))
        },
        TaskStatus::Failed => {
            Err(AppError::VideoProcessingError(
                task.error_message.clone().unwrap_or_else(|| "未知错误".to_string())
            ))
        },
        _ => Err(AppError::VideoProcessingError(format!("任务 {} 仍在处理中", task_id))),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化应用状态
    let app_state = AppState::default();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            upload_video,
            translate_video,
            get_translation_task,
            check_task_output
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
