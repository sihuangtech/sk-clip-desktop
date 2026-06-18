// src-tauri/src/commands/video.rs

// 视频相关命令模块，处理视频上传、翻译、编辑等功能

use tauri::{AppHandle, State};
use log::{info, error};
use std::path::{Path, PathBuf};
use std::fs;
use uuid::Uuid;

use crate::models::{AppState, AppError, TranslationTask};
use crate::file_manager;
use crate::video::{VideoMetadata};
use crate::commands::basic::is_language_supported;

// 上传视频文件命令
#[tauri::command]
pub async fn upload_video(_app_handle: AppHandle, path: String) -> Result<String, AppError> {
    info!("调用 upload_video 命令，参数: {}", path);
    
    let app_data_dir = file_manager::get_app_data_dir()?;
    let videos_dir = app_data_dir.join("videos");
    
    if !videos_dir.exists() {
        fs::create_dir_all(&videos_dir).map_err(|e| {
            error!("创建视频存储目录失败: {}: {}", videos_dir.display(), e);
            AppError::FileError(e.to_string())
        })?;
        info!("已创建视频存储目录: {}", videos_dir.display());
    }
    
    let file_id = Uuid::new_v4().to_string();
    let original_path = Path::new(&path);
    let file_extension = original_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("mp4");
    
    let target_path = videos_dir.join(format!("{}.{}", file_id, file_extension));
    
    file_manager::copy_file(&original_path.to_path_buf(), &target_path)?;
    
    info!("文件已成功复制到: {}", target_path.display());
    
    Ok(target_path.to_string_lossy().to_string())
}

// 翻译视频命令
#[tauri::command]
pub async fn translate_video(
    _app_handle: AppHandle, 
    _state: State<'_, AppState>,
    video_path: String,
    source_language: String,
    target_language: String,
) -> Result<String, AppError> {
    info!(
        "调用 translate_video 命令，视频: {}, 语言: {} -> {}",
        video_path,
        source_language,
        target_language
    );
    
    if !is_language_supported(&source_language) {
        error!("不受支持的源语言: {}", source_language);
        return Err(AppError::UnsupportedLanguage(source_language));
    }
    if !is_language_supported(&target_language) {
        error!("不受支持的目标语言: {}", target_language);
        return Err(AppError::UnsupportedLanguage(target_language));
    }

    Err(AppError::AiModelError(
        "真实视频翻译流水线尚未配置：需要先接入真实 ASR、翻译、TTS 和音视频合成引擎。".to_string(),
    ))
}

// 实际视频翻译处理流程函数（简化版本）
pub async fn process_video_translation_flow(
    input_path: &str,
    output_path: &PathBuf,
    source_language: &str,
    target_language: &str,
) -> Result<(), AppError> {
    info!("开始视频翻译流程: {} -> {}", input_path, output_path.display());
    info!("语言: {} -> {}", source_language, target_language);

    Err(AppError::AiModelError(
        "真实视频翻译流水线尚未配置：需要接入真实 ASR、翻译、TTS 和音视频合成引擎后才能执行。".to_string(),
    ))
}

// 获取翻译任务状态命令
#[tauri::command]
pub async fn get_translation_task(
    state: State<'_, AppState>,
    task_id: String
) -> Result<TranslationTask, AppError> {
    info!("调用 get_translation_task 命令，任务ID: {}", task_id);
    
    let tasks = state.tasks.lock().await;
    match tasks.get(&task_id) {
        Some(task) => {
            info!("找到任务 {}，状态: {:?}", task_id, task.status);
            Ok(task.clone())
        },
        None => {
            error!("任务不存在: {}", task_id);
            Err(AppError::TaskNotFound(task_id))
        }
    }
}

// 检查任务输出命令
#[tauri::command]
pub async fn check_task_output(
    state: State<'_, AppState>,
    task_id: String
) -> Result<String, AppError> {
    info!("调用 check_task_output 命令，任务ID: {}", task_id);
    
    let tasks = state.tasks.lock().await;
    match tasks.get(&task_id) {
        Some(task) => {
            match &task.output_path {
                Some(path) => {
                    info!("任务 {} 输出路径: {}", task_id, path);
                    Ok(path.clone())
                },
                None => {
                    error!("任务 {} 没有输出路径", task_id);
                    Err(AppError::TaskNotFound(format!("任务 {} 没有输出", task_id)))
                }
            }
        },
        None => {
            error!("任务不存在: {}", task_id);
            Err(AppError::TaskNotFound(task_id))
        }
    }
}

// 获取视频信息命令
#[tauri::command]
pub async fn get_video_info(video_path: String) -> Result<VideoMetadata, AppError> {
    info!("调用 get_video_info 命令，视频路径: {}", video_path);
    
    let path = PathBuf::from(video_path);
    let processor = crate::video::processor::VideoProcessor::new();
    let info = processor.get_video_info(&path).await?;
    let file_metadata = std::fs::metadata(&path)
        .map_err(|e| AppError::FileError(format!("无法获取文件信息: {}", e)))?;

    Ok(VideoMetadata {
        file_path: path,
        file_size: info.size,
        duration: info.duration,
        width: info.width,
        height: info.height,
        framerate: info.fps as f32,
        video_codec: info.video_codec,
        audio_codec: if info.audio_codec.is_empty() { None } else { Some(info.audio_codec) },
        bitrate: info.bitrate,
        sample_rate: None,
        audio_channels: None,
        format: file_metadata
            .file_type()
            .is_file()
            .then(|| "media".to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        created_at: None,
        has_audio: true,
        has_video: true,
    })
}

// 从视频提取音频命令
#[tauri::command]
pub async fn extract_audio_from_video(
    video_path: String,
    output_audio_path: String,
) -> Result<String, AppError> {
    info!("调用 extract_audio_from_video 命令");

    use crate::video::processor::VideoProcessor;
    use std::path::PathBuf;

    let processor = VideoProcessor::new();
    let input = PathBuf::from(video_path);
    let output = PathBuf::from(&output_audio_path);

    processor.extract_audio(&input, &output).await?;

    Ok(output_audio_path)
}

// 裁剪视频命令
#[tauri::command]
pub async fn trim_video_command(
    input_path: String,
    output_path: String,
    start_time: f32,
    end_time: f32,
    _preserve_quality: Option<bool>,
) -> Result<String, AppError> {
    info!("调用 trim_video_command 命令");

    use crate::video::processor::VideoProcessor;
    use std::path::PathBuf;

    let processor = VideoProcessor::new();
    let input = PathBuf::from(input_path);
    let output = PathBuf::from(&output_path);

    processor.trim_video(&input, &output, start_time as f64, end_time as f64).await?;

    Ok(output_path)
}

// 合并视频命令
#[tauri::command]
pub async fn merge_videos_command(
    video_paths: Vec<String>,
    output_path: String,
    _add_transitions: Option<bool>,
    _transition_duration: Option<f32>,
) -> Result<String, AppError> {
    info!("调用 merge_videos_command 命令，合并 {} 个视频", video_paths.len());

    use crate::video::processor::VideoProcessor;
    use std::path::PathBuf;

    let processor = VideoProcessor::new();
    let inputs: Vec<PathBuf> = video_paths.iter().map(PathBuf::from).collect();
    let output = PathBuf::from(&output_path);

    processor.merge_videos(&inputs, &output).await?;

    Ok(output_path)
}

// 添加字幕命令
#[tauri::command]
pub async fn add_subtitles_command(
    input_path: String,
    output_path: String,
    subtitle_path: String,
) -> Result<String, AppError> {
    info!("调用 add_subtitles_command 命令");

    use crate::video::processor::VideoProcessor;
    use std::path::PathBuf;

    let processor = VideoProcessor::new();
    let video = PathBuf::from(input_path);
    let subtitle = PathBuf::from(subtitle_path);
    let output = PathBuf::from(&output_path);

    processor.add_subtitles(&video, &subtitle, &output).await?;

    Ok(output_path)
}

// 调整视频大小命令
#[tauri::command]
pub async fn resize_video_command(
    input_path: String,
    output_path: String,
    width: u32,
    height: u32,
    _maintain_aspect_ratio: Option<bool>,
) -> Result<String, AppError> {
    info!("调用 resize_video_command 命令，目标尺寸: {}x{}", width, height);

    use crate::video::processor::VideoProcessor;
    use std::path::PathBuf;

    let processor = VideoProcessor::new();
    let input = PathBuf::from(input_path);
    let output = PathBuf::from(&output_path);

    processor.resize_video(&input, &output, width, height).await?;

    Ok(output_path)
}

// 创建视频缩略图命令
#[tauri::command]
pub async fn create_video_thumbnail(
    video_path: String,
    output_path: String,
    timestamp: f32,
) -> Result<String, AppError> {
    info!("调用 create_video_thumbnail 命令，时间戳: {}s", timestamp);

    use crate::video::processor::VideoProcessor;
    use std::path::PathBuf;

    let processor = VideoProcessor::new();
    let input = PathBuf::from(video_path);
    let output = PathBuf::from(&output_path);

    processor.generate_thumbnail(&input, &output, timestamp as f64).await?;

    Ok(output_path)
} 
