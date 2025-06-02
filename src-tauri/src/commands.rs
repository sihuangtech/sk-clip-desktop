// src-tauri/src/commands.rs

// 该文件包含了所有前端可以调用的 Tauri 命令的实现

use tauri::{AppHandle, State, Manager};
use log::{info, error};
use std::path::{Path, PathBuf};
use std::fs;
use uuid::Uuid;

// 导入 models 模块中的数据结构和错误类型
use crate::models::{AppState, AppError, TaskStatus, TranslationTask, UploadVideoArgs, TranslateVideoArgs};
// 导入 file_manager 模块
use crate::file_manager;
// 导入 document_parser 模块
use crate::document_parser::{self, DocumentContent};

// TODO: 导入 ai_models 模块 (待实现)
// use crate::ai_models;

// 示例命令：打招呼
// 接收一个名字，返回一个问候字符串
#[tauri::command]
pub fn greet(name: &str) -> String {
    info!("调用 greet 命令，参数: {}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 检查语言代码是否支持（目前支持中文、英文、日语、韩语、西班牙语、俄语、德语、法语、葡萄牙语）
// 参数: lang_code - 语言代码字符串
// 返回值: 如果支持则为 true，否则为 false
pub fn is_language_supported(lang_code: &str) -> bool {
    matches!(lang_code, "zh" | "en" | "ja" | "ko" | "es" | "ru" | "de" | "fr" | "pt")
}

// 获取应用数据目录 (已移至 file_manager)
// pub fn get_app_data_dir() -> Result<PathBuf, AppError> { ... }

// 上传视频文件命令
// 接收用户选择的文件路径，将其复制到应用的数据目录中
// 参数: app_handle - Tauri 应用句柄
//       args - 包含视频文件路径的结构体
// 返回值: 复制后文件在应用内部的路径
#[tauri::command]
pub async fn upload_video(_app_handle: AppHandle, args: UploadVideoArgs) -> Result<String, AppError> {
    info!("调用 upload_video 命令，参数: {}", args.path);
    
    // 创建应用数据目录和视频存储子目录 (使用 file_manager 模块)
    let app_data_dir = file_manager::get_app_data_dir()?;
    let videos_dir = app_data_dir.join("videos");
    
    // 确保视频存储目录存在
    if !videos_dir.exists() {
        fs::create_dir_all(&videos_dir).map_err(|e| {
            error!("创建视频存储目录失败: {}: {}", videos_dir.display(), e);
            AppError::FileError(e.to_string())
        })?;
        info!("已创建视频存储目录: {}", videos_dir.display());
    }
    
    // 生成唯一文件名，保留原始文件扩展名
    let file_id = Uuid::new_v4().to_string();
    let original_path = Path::new(&args.path);
    let file_extension = original_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("mp4"); // 默认扩展名
    
    let target_path = videos_dir.join(format!("{}.{}", file_id, file_extension));
    
    // 将用户选择的文件复制到目标路径 (使用 file_manager 模块)
    file_manager::copy_file(&original_path.to_path_buf(), &target_path)?; // 将 Path 转换为 PathBuf
    
    info!("文件已成功复制到: {}", target_path.display());
    
    Ok(target_path.to_string_lossy().to_string())
}

// 翻译视频命令
// 接收视频路径、源语言和目标语言，创建一个新的翻译任务并开始处理（目前为模拟）
// 参数: app_handle - Tauri 应用句柄
//       state - 应用状态，用于访问任务列表
//       args - 包含翻译参数的结构体
// 返回值: 新创建任务的唯一ID
#[tauri::command]
pub async fn translate_video(
    app_handle: AppHandle, 
    state: State<'_, AppState>,
    args: TranslateVideoArgs
) -> Result<String, AppError> {
    info!("调用 translate_video 命令，参数: {:?}", args);
    
    // 检查源语言和目标语言是否支持
    if !is_language_supported(&args.source_language) {
        error!("不受支持的源语言: {}", args.source_language);
        return Err(AppError::UnsupportedLanguage(args.source_language));
    }
    if !is_language_supported(&args.target_language) {
        error!("不受支持的目标语言: {}", args.target_language);
        return Err(AppError::UnsupportedLanguage(args.target_language));
    }
    
    // 创建任务ID
    let task_id = Uuid::new_v4().to_string();
    info!("创建新的翻译任务，ID: {}", task_id);

    // 获取应用数据目录以确定输出路径 (使用 file_manager 模块)
    let app_data_dir = file_manager::get_app_data_dir()?;
    let output_dir = app_data_dir.join("outputs");
    
    // 确保输出目录存在
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir).map_err(|e| {
            error!("创建输出目录失败: {}: {}", output_dir.display(), e);
            AppError::FileError(e.to_string())
        })?;
        info!("已创建输出目录: {}", output_dir.display());
    }
    
    // 生成输出文件路径 (这里仅为示例，实际文件名和格式可能更复杂)
    let output_path = output_dir.join(format!("translated_{}.mp4", task_id));
    let output_path_str = output_path.to_string_lossy().to_string();
    
    // 创建任务并保存到状态中
    let task = TranslationTask {
        id: task_id.clone(),
        video_path: args.video_path.clone(),
        status: TaskStatus::Pending, // 初始状态为 Pending
        source_language: args.source_language.clone(),
        target_language: args.target_language.clone(),
        output_path: Some(output_path_str.clone()), // 预设输出路径
        error_message: None,
    };
    
    // 将任务添加到共享状态中
    state.tasks.lock().await.insert(task_id.clone(), task);
    info!("任务 {} 已添加到状态管理", task_id);
    
    // 克隆必要的数据以便在 spawn 中使用
    let task_id_clone = task_id.clone();
    let video_path_clone = args.video_path.clone();
    let source_language_clone = args.source_language.clone();
    let target_language_clone = args.target_language.clone();
    let output_path_clone = output_path.clone();

    tokio::spawn(async move {
        info!("任务 {}：开始处理流程", task_id_clone);
        
        // 通过 AppHandle 获取状态
        let app_state: State<AppState> = app_handle.state();
        
        // 更新任务状态为处理中
        {
            let mut tasks = app_state.tasks.lock().await;
            if let Some(task) = tasks.get_mut(&task_id_clone) {
                task.status = TaskStatus::Processing;
                info!("任务 {} 状态更新为 Processing", task_id_clone);
            }
        }

        // 调用实际的视频翻译处理函数
        let process_result = process_video_translation_flow(
            &video_path_clone,
            &output_path_clone,
            &source_language_clone,
            &target_language_clone,
        ).await;

        // 处理任务结果并更新状态
        let mut tasks = app_state.tasks.lock().await;
        if let Some(task) = tasks.get_mut(&task_id_clone) {
            match process_result {
                Ok(_) => {
                    task.status = TaskStatus::Completed;
                    info!("任务 {} 状态更新为 Completed", task_id_clone);
                },
                Err(e) => {
                    task.status = TaskStatus::Failed;
                    task.error_message = Some(e.to_string());
                    error!("任务 {} 失败: {}", task_id_clone, e);
                }
            }
        }
    });
    
    // 返回任务ID给前端
    Ok(task_id)
}

// 实际视频翻译处理流程函数
// 协调调用各个模块的功能来完成视频翻译任务
// 参数: input_path - 输入视频路径
//       output_path - 输出视频路径
//       source_language - 源语言
//       target_language - 目标语言
//       app_handle - Tauri 应用句柄 (如果需要发送事件)
// 返回值: 处理成功或失败的结果
pub async fn process_video_translation_flow(
    input_path: &str,
    output_path: &PathBuf,
    source_language: &str,
    target_language: &str,
    // app_handle: &AppHandle // 如果需要发送事件
) -> Result<(), AppError> {
    info!("开始执行视频翻译处理流程: {} ({} -> {}) 到 {}", input_path, source_language, target_language, output_path.display());

    // TODO: 在这里集成实际的视频处理和 AI 模型调用流程
    // 1. 使用 video_processor 模块提取音频轨道
    //    let audio_path = ...; // 生成临时音频文件路径
    //    video_processor::extract_audio(&PathBuf::from(input_path), &audio_path).await?;

    // 2. 使用 ai_models 模块进行语音识别 (ASR)
    //    let transcript = ai_models::asr::recognize_speech(&audio_path, source_language).await?;

    // 3. 使用 ai_models 模块进行文本翻译 (MT)
    //    let translated_text = ai_models::mt::translate_text(&transcript, source_language, target_language).await?;

    // 4. 使用 ai_models 模块生成目标语言的语音 (TTS)
    //    let translated_audio_path = ...; // 生成临时翻译音频文件路径
    //    ai_models::tts::generate_speech(&translated_text, target_language, &translated_audio_path).await?;

    // 5. (可选) 使用口型同步技术，可能需要视频处理或单独的 AI 模型

    // 6. 使用 video_processor 模块将原始视频流和翻译后的音频流 (或字幕) 合成最终视频
    //    video_processor::merge_video_audio(&PathBuf::from(input_path), &translated_audio_path, output_path).await?;

    // TODO: 清理临时文件 (使用 file_manager 模块)
    // file_manager::delete_file(&audio_path)?;
    // file_manager::delete_file(&translated_audio_path)?;

    info!("视频翻译处理流程模拟完成");
    
    // 模拟成功，实际应根据实际处理结果返回
    Ok(())
}

// 模拟视频翻译处理过程 (此函数将被上面的 process_video_translation_flow 替换)
// 保留此函数作为参考，后续将删除
/*
pub async fn simulate_video_translation_process(
    input_path: &str,
    output_path: &Path,
    source_language: &str,
    target_language: &str
) -> Result<(), AppError> {
    info!("模拟处理视频 {} ({} -> {}) 到 {}", input_path, source_language, target_language, output_path.display());
    sleep(std::time::Duration::from_secs(5)).await;
    file_manager::copy_file(&PathBuf::from(input_path), &output_path.to_path_buf())?;
    info!("模拟视频处理完成");
    Ok(())
}
*/

// 获取翻译任务状态命令
// 接收任务ID，返回该任务的当前状态及相关信息
// 参数: state - 应用状态，用于访问任务列表
//       task_id - 要查询的任务的唯一ID
// 返回值: TranslationTask 结构体，如果任务不存在则返回错误
#[tauri::command]
pub async fn get_translation_task(
    state: State<'_, AppState>,
    task_id: String
) -> Result<TranslationTask, AppError> {
    info!("调用 get_translation_task 命令，任务ID: {}", task_id);
    let tasks = state.tasks.lock().await;
    tasks.get(&task_id)
        .cloned() // 克隆任务数据以便返回
        .ok_or_else(|| {
            error!("任务 {} 不存在", task_id);
            AppError::TaskNotFound(task_id)
        })
}

// 检查任务输出文件命令
// 接收任务ID，如果任务完成则返回输出文件路径，如果失败则返回错误信息，否则返回状态（如 processing）
// 参数: state - 应用状态，用于访问任务列表
//       task_id - 要检查的任务的唯一ID
// 返回值: 输出文件路径字符串，错误信息字符串，或状态字符串
#[tauri::command]
pub async fn check_task_output(
    state: State<'_, AppState>,
    task_id: String
) -> Result<String, AppError> {
    info!("调用 check_task_output 命令，任务ID: {}", task_id);
    let tasks = state.tasks.lock().await;
    let task_id_clone = task_id.clone(); // 克隆 task_id 以避免移动错误
    let task = tasks.get(&task_id)
        .cloned() // 克隆任务数据以便访问
        .ok_or_else(|| {
            error!("任务 {} 不存在", task_id_clone);
            AppError::TaskNotFound(task_id_clone)
        })?;
    
    // 根据任务状态返回不同的结果
    match task.status {
        TaskStatus::Completed => {
            task.output_path
                .ok_or_else(|| {
                    error!("任务 {} 已完成但输出路径缺失", task_id);
                    AppError::Unknown(format!("任务 {} 完成但输出路径缺失", task_id))
                })
        },
        TaskStatus::Failed => {
            let error_msg = task.error_message.unwrap_or_else(|| "未知错误".to_string());
            error!("任务 {} 失败: {}", task_id, error_msg);
            // 返回一个特定的字符串或结构体来表示失败及错误信息，
            // 或者直接返回一个包含错误信息的 Result::Err。
            // 这里我们返回一个包含错误信息的 Result::Err，与前端的错误处理机制对应。
            Err(AppError::VideoProcessingError(error_msg))
        },
        TaskStatus::Pending | TaskStatus::Processing => {
            info!("任务 {} 仍在处理中或等待中", task_id);
            Ok("processing".to_string()) // 返回一个表示进行中的状态字符串
        },
    }
}

// TODO: 添加其他 Tauri 命令的实现，如:
// - import_document (导入文档)
// - add_video_to_timeline (添加视频到时间线)
// - add_text_to_timeline (添加文本到时间线)
// - generate_tts (生成语音合成)
// - export_video (导出最终视频)
// - ... 其他编辑操作相关的命令

// 文档导入命令
// 接收文档文件路径，解析文档内容并返回解析结果
// 参数: app_handle - Tauri 应用句柄
//       file_path - 文档文件路径
// 返回值: 解析后的文档内容
#[tauri::command]
pub async fn import_document(
    _app_handle: AppHandle,
    file_path: String
) -> Result<DocumentContent, AppError> {
    info!("调用 import_document 命令，文件路径: {}", file_path);
    
    let path = Path::new(&file_path);
    
    // 检查文件是否存在
    if !path.exists() {
        error!("文档文件不存在: {}", file_path);
        return Err(AppError::FileError(format!("文档文件不存在: {}", file_path)));
    }
    
    // 解析文档
    let document_content = document_parser::parse_document(path).await?;
    
    info!("文档导入成功: {} (共 {} 页)", document_content.title, document_content.total_pages);
    Ok(document_content)
}

// 获取支持的文档类型列表命令
// 返回应用支持的文档格式列表
#[tauri::command]
pub fn get_supported_document_types() -> Vec<String> {
    info!("调用 get_supported_document_types 命令");
    vec![
        "pptx".to_string(),
        "ppt".to_string(),
        "md".to_string(),
        "markdown".to_string(),
        "pdf".to_string(),
    ]
}

// 将文档转换为视频素材命令
// 将解析后的文档内容转换为可用于视频制作的素材
// 参数: app_handle - Tauri 应用句柄
//       document_content - 文档内容
// 返回值: 生成的素材文件路径列表
#[tauri::command]
pub async fn convert_document_to_assets(
    _app_handle: AppHandle,
    document_content: DocumentContent
) -> Result<Vec<String>, AppError> {
    info!("调用 convert_document_to_assets 命令，文档: {}", document_content.title);
    
    // 获取应用数据目录
    let app_data_dir = file_manager::get_app_data_dir()?;
    let assets_dir = app_data_dir.join("assets");
    
    // 确保素材目录存在
    if !assets_dir.exists() {
        fs::create_dir_all(&assets_dir).map_err(|e| {
            error!("创建素材目录失败: {}: {}", assets_dir.display(), e);
            AppError::FileError(e.to_string())
        })?;
        info!("已创建素材目录: {}", assets_dir.display());
    }
    
    // 转换文档为视频素材
    let asset_paths = document_parser::convert_document_to_video_assets(&document_content, &assets_dir).await?;
    
    info!("文档转换完成，生成 {} 个素材文件", asset_paths.len());
    Ok(asset_paths)
} 