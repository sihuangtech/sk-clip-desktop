use std::path::PathBuf;
use std::process::Command;
use crate::models::AppError;
use log::{info, error};

/// 视频处理任务类型
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum VideoProcessingTask {
    /// 提取音频
    ExtractAudio {
        input_path: PathBuf,
        output_path: PathBuf,
    },
    /// 生成缩略图
    GenerateThumbnail {
        input_path: PathBuf,
        output_path: PathBuf,
        timestamp: f64,
    },
    /// 合并视频
    MergeVideos {
        input_paths: Vec<PathBuf>,
        output_path: PathBuf,
    },
    /// 添加字幕
    AddSubtitles {
        video_path: PathBuf,
        subtitle_path: PathBuf,
        output_path: PathBuf,
    },
    /// 裁剪视频
    TrimVideo {
        input_path: PathBuf,
        output_path: PathBuf,
        start_time: f64,
        end_time: f64,
    },
    /// 调整视频大小
    ResizeVideo {
        input_path: PathBuf,
        output_path: PathBuf,
        width: u32,
        height: u32,
    },
}

/// FFmpeg处理器
pub struct VideoProcessor {
    ffmpeg_path: String,
    ffprobe_path: String,
}

impl VideoProcessor {
    pub fn new() -> Self {
        Self {
            ffmpeg_path: "ffmpeg".to_string(),
            ffprobe_path: "ffprobe".to_string(),
        }
    }

    /// 使用自定义FFmpeg路径创建处理器
    pub fn with_ffmpeg_path(ffmpeg_path: String, ffprobe_path: String) -> Self {
        Self { ffmpeg_path, ffprobe_path }
    }

    /// 检查FFmpeg是否可用
    pub fn check_ffmpeg_available(&self) -> Result<(), AppError> {
        let output = Command::new(&self.ffmpeg_path)
            .arg("-version")
            .output()
            .map_err(|e| {
                error!("FFmpeg不可用: {}", e);
                AppError::VideoProcessingError(format!("FFmpeg不可用: {}. 请确保FFmpeg已安装并添加到PATH环境变量中。", e))
            })?;

        if !output.status.success() {
            return Err(AppError::VideoProcessingError("FFmpeg版本检查失败".to_string()));
        }

        info!("FFmpeg已就绪");
        Ok(())
    }

    /// 处理视频任务
    pub async fn process_task(&self, task: VideoProcessingTask) -> Result<PathBuf, AppError> {
        self.check_ffmpeg_available()?;

        match task {
            VideoProcessingTask::ExtractAudio { input_path, output_path } => {
                self.extract_audio(&input_path, &output_path).await?;
                Ok(output_path)
            }
            VideoProcessingTask::GenerateThumbnail { input_path, output_path, timestamp } => {
                self.generate_thumbnail(&input_path, &output_path, timestamp).await?;
                Ok(output_path)
            }
            VideoProcessingTask::MergeVideos { input_paths, output_path } => {
                self.merge_videos(&input_paths, &output_path).await?;
                Ok(output_path)
            }
            VideoProcessingTask::AddSubtitles { video_path, subtitle_path, output_path } => {
                self.add_subtitles(&video_path, &subtitle_path, &output_path).await?;
                Ok(output_path)
            }
            VideoProcessingTask::TrimVideo { input_path, output_path, start_time, end_time } => {
                self.trim_video(&input_path, &output_path, start_time, end_time).await?;
                Ok(output_path)
            }
            VideoProcessingTask::ResizeVideo { input_path, output_path, width, height } => {
                self.resize_video(&input_path, &output_path, width, height).await?;
                Ok(output_path)
            }
        }
    }

    /// 提取音频
    pub async fn extract_audio(&self, input_path: &PathBuf, output_path: &PathBuf) -> Result<(), AppError> {
        info!("提取音频: {} -> {}", input_path.display(), output_path.display());

        if !input_path.exists() {
            return Err(AppError::FileError(format!("输入文件不存在: {}", input_path.display())));
        }

        // 创建输出目录
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::FileError(format!("创建输出目录失败: {}", e)))?;
        }

        // 确定输出格式
        let output_ext = output_path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("mp3");

        let codec = match output_ext.to_lowercase().as_str() {
            "mp3" => "libmp3lame",
            "wav" => "pcm_s16le",
            "aac" => "aac",
            "ogg" => "libvorbis",
            "flac" => "flac",
            _ => "copy",
        };

        let output = Command::new(&self.ffmpeg_path)
            .arg("-i").arg(input_path)
            .arg("-vn")  // 不包含视频
            .arg("-acodec").arg(codec)
            .arg("-y")  // 覆盖输出文件
            .arg(output_path)
            .output()
            .map_err(|e| {
                error!("FFmpeg执行失败: {}", e);
                AppError::VideoProcessingError(format!("FFmpeg执行失败: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("FFmpeg错误: {}", stderr);
            return Err(AppError::VideoProcessingError(format!("音频提取失败: {}", stderr)));
        }

        info!("音频提取完成: {}", output_path.display());
        Ok(())
    }

    /// 生成缩略图
    pub async fn generate_thumbnail(&self, input_path: &PathBuf, output_path: &PathBuf, timestamp: f64) -> Result<(), AppError> {
        info!("生成缩略图: {} -> {} (时间: {}s)", input_path.display(), output_path.display(), timestamp);

        if !input_path.exists() {
            return Err(AppError::FileError(format!("输入文件不存在: {}", input_path.display())));
        }

        // 创建输出目录
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::FileError(format!("创建输出目录失败: {}", e)))?;
        }

        let timestamp_str = format!("{:.2}", timestamp);

        let output = Command::new(&self.ffmpeg_path)
            .arg("-ss").arg(&timestamp_str)  // 定位到时间点
            .arg("-i").arg(input_path)
            .arg("-vframes").arg("1")  // 只提取一帧
            .arg("-q:v").arg("2")  // 高质量
            .arg("-y")  // 覆盖输出文件
            .arg(output_path)
            .output()
            .map_err(|e| {
                error!("FFmpeg执行失败: {}", e);
                AppError::VideoProcessingError(format!("FFmpeg执行失败: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("FFmpeg错误: {}", stderr);
            return Err(AppError::VideoProcessingError(format!("缩略图生成失败: {}", stderr)));
        }

        info!("缩略图生成完成: {}", output_path.display());
        Ok(())
    }

    /// 合并视频
    pub async fn merge_videos(&self, input_paths: &[PathBuf], output_path: &PathBuf) -> Result<(), AppError> {
        info!("合并视频: {} 个文件 -> {}", input_paths.len(), output_path.display());

        if input_paths.is_empty() {
            return Err(AppError::VideoProcessingError("没有输入文件".to_string()));
        }

        // 检查所有输入文件
        for path in input_paths {
            if !path.exists() {
                return Err(AppError::FileError(format!("输入文件不存在: {}", path.display())));
            }
        }

        // 创建输出目录
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::FileError(format!("创建输出目录失败: {}", e)))?;
        }

        // 创建临时文件列表用于concat
        let temp_dir = std::env::temp_dir();
        let list_file = temp_dir.join(format!("ffmpeg_concat_{}.txt", uuid::Uuid::new_v4()));

        // 写入文件列表
        let list_content: String = input_paths
            .iter()
            .map(|p| format!("file '{}'", p.canonicalize()
                .unwrap_or_else(|_| p.clone())
                .to_string_lossy()
                .replace("\\", "\\\\")
                .replace("'", "'\\''")))
            .collect::<Vec<_>>()
            .join("\n");

        std::fs::write(&list_file, list_content)
            .map_err(|e| AppError::FileError(format!("创建文件列表失败: {}", e)))?;

        info!("临时文件列表: {}", list_file.display());

        // 使用concat demuxer合并
        let output = Command::new(&self.ffmpeg_path)
            .arg("-f").arg("concat")
            .arg("-safe").arg("0")
            .arg("-i").arg(&list_file)
            .arg("-c").arg("copy")  // 直接复制，不重新编码
            .arg("-y")  // 覆盖输出文件
            .arg(output_path)
            .output();

        // 清理临时文件
        let _ = std::fs::remove_file(&list_file);

        let output = output.map_err(|e| {
            error!("FFmpeg执行失败: {}", e);
            AppError::VideoProcessingError(format!("FFmpeg执行失败: {}", e))
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("FFmpeg错误: {}", stderr);
            return Err(AppError::VideoProcessingError(format!("视频合并失败: {}", stderr)));
        }

        info!("视频合并完成: {}", output_path.display());
        Ok(())
    }

    /// 添加字幕（硬字幕/烧录）
    pub async fn add_subtitles(&self, video_path: &PathBuf, subtitle_path: &PathBuf, output_path: &PathBuf) -> Result<(), AppError> {
        info!("添加字幕: {} + {} -> {}", video_path.display(), subtitle_path.display(), output_path.display());

        if !video_path.exists() {
            return Err(AppError::FileError(format!("视频文件不存在: {}", video_path.display())));
        }
        if !subtitle_path.exists() {
            return Err(AppError::FileError(format!("字幕文件不存在: {}", subtitle_path.display())));
        }

        // 创建输出目录
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::FileError(format!("创建输出目录失败: {}", e)))?;
        }

        // 字幕文件路径需要转义特殊字符
        let subtitle_path_str = subtitle_path
            .canonicalize()
            .unwrap_or_else(|_| subtitle_path.clone())
            .to_string_lossy()
            .replace("\\", "/")
            .replace(":", "\\:")
            .replace("'", "'\\''");

        let filter = format!("subtitles='{}'", subtitle_path_str);

        let output = Command::new(&self.ffmpeg_path)
            .arg("-i").arg(video_path)
            .arg("-vf").arg(&filter)
            .arg("-c:a").arg("copy")  // 音频直接复制
            .arg("-y")  // 覆盖输出文件
            .arg(output_path)
            .output()
            .map_err(|e| {
                error!("FFmpeg执行失败: {}", e);
                AppError::VideoProcessingError(format!("FFmpeg执行失败: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("FFmpeg错误: {}", stderr);
            return Err(AppError::VideoProcessingError(format!("字幕添加失败: {}", stderr)));
        }

        info!("字幕添加完成: {}", output_path.display());
        Ok(())
    }

    /// 裁剪视频
    pub async fn trim_video(&self, input_path: &PathBuf, output_path: &PathBuf, start_time: f64, end_time: f64) -> Result<(), AppError> {
        info!("裁剪视频: {} -> {} ({}s - {}s)", input_path.display(), output_path.display(), start_time, end_time);

        if !input_path.exists() {
            return Err(AppError::FileError(format!("输入文件不存在: {}", input_path.display())));
        }

        if start_time >= end_time {
            return Err(AppError::VideoProcessingError("开始时间必须小于结束时间".to_string()));
        }

        // 创建输出目录
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::FileError(format!("创建输出目录失败: {}", e)))?;
        }

        let start_str = format!("{:.2}", start_time);
        let duration = end_time - start_time;
        let duration_str = format!("{:.2}", duration);

        let output = Command::new(&self.ffmpeg_path)
            .arg("-ss").arg(&start_str)  // 开始时间
            .arg("-i").arg(input_path)
            .arg("-t").arg(&duration_str)  // 持续时间
            .arg("-c").arg("copy")  // 直接复制，快速裁剪
            .arg("-y")  // 覆盖输出文件
            .arg(output_path)
            .output()
            .map_err(|e| {
                error!("FFmpeg执行失败: {}", e);
                AppError::VideoProcessingError(format!("FFmpeg执行失败: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("FFmpeg错误: {}", stderr);
            return Err(AppError::VideoProcessingError(format!("视频裁剪失败: {}", stderr)));
        }

        info!("视频裁剪完成: {}", output_path.display());
        Ok(())
    }

    /// 调整视频大小
    pub async fn resize_video(&self, input_path: &PathBuf, output_path: &PathBuf, width: u32, height: u32) -> Result<(), AppError> {
        info!("调整视频大小: {} -> {} ({}x{})", input_path.display(), output_path.display(), width, height);

        if !input_path.exists() {
            return Err(AppError::FileError(format!("输入文件不存在: {}", input_path.display())));
        }

        // 创建输出目录
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::FileError(format!("创建输出目录失败: {}", e)))?;
        }

        let scale_filter = format!("scale={}:{}", width, height);

        let output = Command::new(&self.ffmpeg_path)
            .arg("-i").arg(input_path)
            .arg("-vf").arg(&scale_filter)
            .arg("-c:a").arg("copy")  // 音频直接复制
            .arg("-y")  // 覆盖输出文件
            .arg(output_path)
            .output()
            .map_err(|e| {
                error!("FFmpeg执行失败: {}", e);
                AppError::VideoProcessingError(format!("FFmpeg执行失败: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("FFmpeg错误: {}", stderr);
            return Err(AppError::VideoProcessingError(format!("视频大小调整失败: {}", stderr)));
        }

        info!("视频大小调整完成: {}", output_path.display());
        Ok(())
    }

    /// 获取视频信息
    pub async fn get_video_info(&self, input_path: &PathBuf) -> Result<VideoInfo, AppError> {
        info!("获取视频信息: {}", input_path.display());

        if !input_path.exists() {
            return Err(AppError::FileError(format!("输入文件不存在: {}", input_path.display())));
        }

        let output = Command::new(&self.ffprobe_path)
            .arg("-v").arg("quiet")
            .arg("-print_format").arg("json")
            .arg("-show_format")
            .arg("-show_streams")
            .arg(input_path)
            .output()
            .map_err(|e| {
                error!("FFprobe执行失败: {}", e);
                AppError::VideoProcessingError(format!("FFprobe执行失败: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("FFprobe错误: {}", stderr);
            return Err(AppError::VideoProcessingError(format!("获取视频信息失败: {}", stderr)));
        }

        let json_str = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|e| AppError::VideoProcessingError(format!("解析视频信息失败: {}", e)))?;

        let format = json.get("format").ok_or_else(|| AppError::VideoProcessingError("无法获取格式信息".to_string()))?;
        let streams = json.get("streams").and_then(|s| s.as_array()).ok_or_else(|| AppError::VideoProcessingError("无法获取流信息".to_string()))?;

        let duration: f64 = format.get("duration")
            .and_then(|d| d.as_str())
            .and_then(|d| d.parse().ok())
            .unwrap_or(0.0);

        let size: u64 = format.get("size")
            .and_then(|s| s.as_str())
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let bitrate: u64 = format.get("bit_rate")
            .and_then(|b| b.as_str())
            .and_then(|b| b.parse().ok())
            .unwrap_or(0);

        let mut width = 0u32;
        let mut height = 0u32;
        let mut fps = 0.0f64;
        let mut video_codec = String::new();
        let mut audio_codec = String::new();

        for stream in streams {
            let codec_type = stream.get("codec_type").and_then(|t| t.as_str()).unwrap_or("");
            let codec_name = stream.get("codec_name").and_then(|n| n.as_str()).unwrap_or("");

            if codec_type == "video" {
                width = stream.get("width").and_then(|w| w.as_u64()).unwrap_or(0) as u32;
                height = stream.get("height").and_then(|h| h.as_u64()).unwrap_or(0) as u32;
                video_codec = codec_name.to_string();

                // 解析帧率
                if let Some(r_frame_rate) = stream.get("r_frame_rate").and_then(|r| r.as_str()) {
                    if let Some((num, den)) = r_frame_rate.split_once('/') {
                        let num: f64 = num.parse().unwrap_or(0.0);
                        let den: f64 = den.parse().unwrap_or(1.0);
                        if den > 0.0 {
                            fps = num / den;
                        }
                    }
                }
            } else if codec_type == "audio" {
                audio_codec = codec_name.to_string();
            }
        }

        let info = VideoInfo {
            duration,
            width,
            height,
            fps,
            size,
            bitrate,
            video_codec,
            audio_codec,
        };

        info!("视频信息: {:?}", info);
        Ok(info)
    }
}

impl Default for VideoProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// 视频信息结构
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VideoInfo {
    pub duration: f64,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub size: u64,
    pub bitrate: u64,
    pub video_codec: String,
    pub audio_codec: String,
}
