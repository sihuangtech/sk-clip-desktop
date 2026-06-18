// src-tauri/src/commands/timeline.rs

// 时间线相关命令模块，处理时间线项目的创建、保存、加载和导出功能

use log::{info, error};
use std::path::PathBuf;
use std::fs;

use crate::models::{AppError, TimelineProject, TrackType};
use crate::file_manager;

// 创建时间线项目命令
#[tauri::command]
pub async fn create_timeline_project(
    project_name: String,
) -> Result<serde_json::Value, AppError> {
    info!("调用 create_timeline_project 命令，项目名: {}", project_name);

    let mut project = TimelineProject::new(project_name);

    // 添加默认轨道
    project.add_track(TrackType::Video, Some("视频轨道 1".to_string()));
    project.add_track(TrackType::Audio, Some("音频轨道 1".to_string()));

    // 保存项目文件
    let app_data_dir = file_manager::get_app_data_dir()?;
    let projects_dir = app_data_dir.join("projects");

    if !projects_dir.exists() {
        fs::create_dir_all(&projects_dir).map_err(|e| {
            error!("创建项目目录失败: {}: {}", projects_dir.display(), e);
            AppError::FileError(e.to_string())
        })?;
    }

    let project_file = projects_dir.join(format!("{}.json", project.id));
    let project_json = serde_json::to_string_pretty(&project)
        .map_err(|e| AppError::SerializationError(e.to_string()))?;

    fs::write(&project_file, project_json)
        .map_err(|e| AppError::FileError(format!("写入项目文件失败: {}", e)))?;

    info!("项目已创建: {} ({})", project.name, project.id);

    Ok(serde_json::to_value(project)
        .map_err(|e| AppError::SerializationError(e.to_string()))?)
}

// 保存时间线项目命令
#[tauri::command]
pub async fn save_timeline_project(
    project_id: String,
    project_data: serde_json::Value,
) -> Result<String, AppError> {
    info!("调用 save_timeline_project 命令，项目ID: {}", project_id);

    let app_data_dir = file_manager::get_app_data_dir()?;
    let project_file = app_data_dir.join("projects").join(format!("{}.json", project_id));

    // 验证并更新项目数据
    let mut project: TimelineProject = serde_json::from_value(project_data)
        .map_err(|e| AppError::SerializationError(format!("无效的项目数据: {}", e)))?;

    project.updated_at = chrono::Utc::now();
    project.update_duration();

    // 保存项目
    let project_json = serde_json::to_string_pretty(&project)
        .map_err(|e| AppError::SerializationError(e.to_string()))?;

    fs::write(&project_file, project_json)
        .map_err(|e| AppError::FileError(format!("写入项目文件失败: {}", e)))?;

    info!("项目已保存: {} ({})", project.name, project.id);

    Ok("项目保存成功".to_string())
}

// 加载时间线项目命令
#[tauri::command]
pub async fn load_timeline_project(
    project_id: String,
) -> Result<serde_json::Value, AppError> {
    info!("调用 load_timeline_project 命令，项目ID: {}", project_id);

    let app_data_dir = file_manager::get_app_data_dir()?;
    let project_file = app_data_dir.join("projects").join(format!("{}.json", project_id));

    if !project_file.exists() {
        return Err(AppError::FileError(format!("项目文件不存在: {}", project_id)));
    }

    let project_json = fs::read_to_string(&project_file)
        .map_err(|e| AppError::FileError(format!("读取项目文件失败: {}", e)))?;

    let project: TimelineProject = serde_json::from_str(&project_json)
        .map_err(|e| AppError::SerializationError(format!("解析项目数据失败: {}", e)))?;

    info!("项目已加载: {} ({})", project.name, project.id);

    Ok(serde_json::to_value(project)
        .map_err(|e| AppError::SerializationError(e.to_string()))?)
}

// 导出时间线视频命令
#[tauri::command]
pub async fn export_timeline_video(
    project_id: String,
    output_path: String,
    export_settings: serde_json::Value,
) -> Result<String, AppError> {
    info!("调用 export_timeline_video 命令，项目ID: {}", project_id);

    // 加载项目
    let app_data_dir = file_manager::get_app_data_dir()?;
    let project_file = app_data_dir.join("projects").join(format!("{}.json", project_id));

    if !project_file.exists() {
        return Err(AppError::FileError(format!("项目文件不存在: {}", project_id)));
    }

    let project_json = fs::read_to_string(&project_file)
        .map_err(|e| AppError::FileError(format!("读取项目文件失败: {}", e)))?;

    let project: TimelineProject = serde_json::from_str(&project_json)
        .map_err(|e| AppError::SerializationError(format!("解析项目数据失败: {}", e)))?;

    // 解析导出设置
    let resolution = export_settings.get("resolution")
        .and_then(|r| r.as_str())
        .unwrap_or("1920x1080");
    let quality = export_settings.get("quality")
        .and_then(|q| q.as_str())
        .unwrap_or("high");
    let format = export_settings.get("format")
        .and_then(|f| f.as_str())
        .unwrap_or("mp4");

    info!("导出设置: 分辨率={}, 质量={}, 格式={}", resolution, quality, format);

    // 创建输出目录
    let output = PathBuf::from(&output_path);
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| AppError::FileError(format!("创建输出目录失败: {}", e)))?;
    }

    // 收集所有视频片段
    let video_clips: Vec<(String, f64, f64)> = project.tracks.iter()
        .filter(|t| t.track_type == TrackType::Video)
        .flat_map(|t| t.clips.iter())
        .map(|c| {
            let asset = project.assets.iter().find(|a| a.id == c.asset_id);
            let path = asset.map(|a| a.path.clone()).unwrap_or_default();
            (path, c.start_time, c.duration)
        })
        .collect();

    if video_clips.is_empty() {
        return Err(AppError::VideoProcessingError("没有视频片段可导出".to_string()));
    }

    // 使用FFmpeg进行导出
    use std::process::Command;

    if video_clips.len() == 1 {
        // 单个片段直接复制
        let (clip_path, _, _) = &video_clips[0];

        // 解析分辨率
        let (width, height): (u32, u32) = {
            let parts: Vec<&str> = resolution.split('x').collect();
            if parts.len() == 2 {
                (parts[0].parse().unwrap_or(1920), parts[1].parse().unwrap_or(1080))
            } else {
                (1920, 1080)
            }
        };

        // 质量对应的CRF值
        let crf = match quality {
            "low" => "28",
            "medium" => "23",
            "high" => "18",
            "ultra" => "15",
            _ => "18",
        };

        let output_result = Command::new("ffmpeg")
            .arg("-i").arg(clip_path)
            .arg("-vf").arg(format!("scale={}:{}", width, height))
            .arg("-c:v").arg("libx264")
            .arg("-crf").arg(crf)
            .arg("-preset").arg("medium")
            .arg("-c:a").arg("aac")
            .arg("-b:a").arg("192k")
            .arg("-y")
            .arg(&output_path)
            .output();

        match output_result {
            Ok(output) if output.status.success() => {
                info!("视频导出成功: {}", output_path);
                Ok(output_path)
            },
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                error!("FFmpeg导出失败: {}", stderr);
                Err(AppError::VideoProcessingError(format!("视频导出失败: {}", stderr)))
            },
            Err(e) => {
                error!("FFmpeg执行失败: {}", e);
                Err(AppError::VideoProcessingError(format!("FFmpeg执行失败: {}", e)))
            }
        }
    } else {
        // 多个片段需要合并
        let temp_dir = std::env::temp_dir();
        let list_file = temp_dir.join(format!("ffmpeg_export_{}.txt", uuid::Uuid::new_v4()));

        // 写入文件列表
        let list_content: String = video_clips.iter()
            .filter(|(path, _, _)| !path.is_empty())
            .map(|(path, _, _)| format!("file '{}'", path))
            .collect::<Vec<_>>()
            .join("\n");

        fs::write(&list_file, list_content)
            .map_err(|e| AppError::FileError(format!("创建文件列表失败: {}", e)))?;

        // 解析分辨率
        let (width, height): (u32, u32) = {
            let parts: Vec<&str> = resolution.split('x').collect();
            if parts.len() == 2 {
                (parts[0].parse().unwrap_or(1920), parts[1].parse().unwrap_or(1080))
            } else {
                (1920, 1080)
            }
        };

        let crf = match quality {
            "low" => "28",
            "medium" => "23",
            "high" => "18",
            "ultra" => "15",
            _ => "18",
        };

        let output_result = Command::new("ffmpeg")
            .arg("-f").arg("concat")
            .arg("-safe").arg("0")
            .arg("-i").arg(&list_file)
            .arg("-vf").arg(format!("scale={}:{}", width, height))
            .arg("-c:v").arg("libx264")
            .arg("-crf").arg(crf)
            .arg("-preset").arg("medium")
            .arg("-c:a").arg("aac")
            .arg("-b:a").arg("192k")
            .arg("-y")
            .arg(&output_path)
            .output();

        // 清理临时文件
        let _ = fs::remove_file(&list_file);

        match output_result {
            Ok(output) if output.status.success() => {
                info!("视频导出成功: {}", output_path);
                Ok(output_path)
            },
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                error!("FFmpeg导出失败: {}", stderr);
                Err(AppError::VideoProcessingError(format!("视频导出失败: {}", stderr)))
            },
            Err(e) => {
                error!("FFmpeg执行失败: {}", e);
                Err(AppError::VideoProcessingError(format!("FFmpeg执行失败: {}", e)))
            }
        }
    }
}

// 列出所有项目
#[tauri::command]
pub async fn list_timeline_projects() -> Result<Vec<serde_json::Value>, AppError> {
    info!("调用 list_timeline_projects 命令");

    let app_data_dir = file_manager::get_app_data_dir()?;
    let projects_dir = app_data_dir.join("projects");

    if !projects_dir.exists() {
        return Ok(Vec::new());
    }

    let mut projects = Vec::new();

    let entries = fs::read_dir(&projects_dir)
        .map_err(|e| AppError::FileError(format!("读取项目目录失败: {}", e)))?;

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(project) = serde_json::from_str::<TimelineProject>(&content) {
                        projects.push(serde_json::json!({
                            "id": project.id,
                            "name": project.name,
                            "created_at": project.created_at,
                            "updated_at": project.updated_at,
                            "duration": project.duration,
                        }));
                    }
                }
            }
        }
    }

    // 按更新时间排序
    projects.sort_by(|a, b| {
        let a_time = a.get("updated_at").and_then(|t| t.as_str()).unwrap_or("");
        let b_time = b.get("updated_at").and_then(|t| t.as_str()).unwrap_or("");
        b_time.cmp(a_time)
    });

    info!("找到 {} 个项目", projects.len());
    Ok(projects)
}

// 删除项目
#[tauri::command]
pub async fn delete_timeline_project(
    project_id: String,
) -> Result<String, AppError> {
    info!("调用 delete_timeline_project 命令，项目ID: {}", project_id);

    let app_data_dir = file_manager::get_app_data_dir()?;
    let project_file = app_data_dir.join("projects").join(format!("{}.json", project_id));

    if !project_file.exists() {
        return Err(AppError::FileError(format!("项目文件不存在: {}", project_id)));
    }

    fs::remove_file(&project_file)
        .map_err(|e| AppError::FileError(format!("删除项目文件失败: {}", e)))?;

    info!("项目已删除: {}", project_id);
    Ok("项目已删除".to_string())
}
