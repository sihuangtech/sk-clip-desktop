use std::path::PathBuf;
use std::process::Command;
use crate::models::AppError;
use log::{error, info};

/// 视频转换选项
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConversionOptions {
    /// 输出格式
    pub format: String,
    /// 视频质量
    pub quality: String,
    /// 视频编码器
    pub video_codec: Option<String>,
    /// 音频编码器
    pub audio_codec: Option<String>,
    /// 比特率
    pub bitrate: Option<String>,
    /// 帧率
    pub framerate: Option<f32>,
    /// 分辨率
    pub resolution: Option<(u32, u32)>,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            format: "mp4".to_string(),
            quality: "high".to_string(),
            video_codec: Some("h264".to_string()),
            audio_codec: Some("aac".to_string()),
            bitrate: None,
            framerate: None,
            resolution: None,
        }
    }
}

/// 视频转换器
pub struct VideoConverter {
    // 可以添加FFmpeg配置
}

impl VideoConverter {
    pub fn new() -> Self {
        Self {}
    }

    /// 转换视频格式
    pub async fn convert(
        &self,
        input_path: &PathBuf,
        output_path: &PathBuf,
        options: ConversionOptions,
    ) -> Result<(), AppError> {
        info!("转换视频: {} -> {} (格式: {})", 
              input_path.display(), output_path.display(), options.format);
        
        // 检查输入文件
        if !input_path.exists() {
            return Err(AppError::FileError(format!("输入文件不存在: {}", input_path.display())));
        }

        // 创建输出目录
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::FileError(format!("创建输出目录失败: {}", e)))?;
        }

        // 验证转换选项
        self.validate_options(&options)?;

        let command = self.build_ffmpeg_command(input_path, output_path, &options);
        let output = Command::new(&command[0])
            .args(&command[1..])
            .output()
            .map_err(|e| AppError::VideoProcessingError(format!("FFmpeg执行失败: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("FFmpeg转换失败: {}", stderr);
            return Err(AppError::VideoProcessingError(format!("视频转换失败: {}", stderr)));
        }

        info!("视频转换完成");
        Ok(())
    }

    /// 验证转换选项
    fn validate_options(&self, options: &ConversionOptions) -> Result<(), AppError> {
        // 检查支持的格式
        let supported_formats = vec!["mp4", "avi", "mov", "mkv", "webm", "flv"];
        if !supported_formats.contains(&options.format.as_str()) {
            return Err(AppError::ValidationError(
                format!("不支持的输出格式: {}", options.format)
            ));
        }

        // 检查质量设置
        let supported_qualities = vec!["low", "medium", "high", "ultra"];
        if !supported_qualities.contains(&options.quality.as_str()) {
            return Err(AppError::ValidationError(
                format!("不支持的质量设置: {}", options.quality)
            ));
        }

        // 检查帧率
        if let Some(framerate) = options.framerate {
            if framerate <= 0.0 || framerate > 120.0 {
                return Err(AppError::ValidationError(
                    format!("无效的帧率: {}", framerate)
                ));
            }
        }

        // 检查分辨率
        if let Some((width, height)) = options.resolution {
            if width == 0 || height == 0 || width > 7680 || height > 4320 {
                return Err(AppError::ValidationError(
                    format!("无效的分辨率: {}x{}", width, height)
                ));
            }
        }

        Ok(())
    }

    /// 构建FFmpeg命令
    fn build_ffmpeg_command(
        &self,
        input_path: &PathBuf,
        output_path: &PathBuf,
        options: &ConversionOptions,
    ) -> Vec<String> {
        let mut command = vec![
            "ffmpeg".to_string(),
            "-i".to_string(),
            input_path.to_string_lossy().to_string(),
        ];

        // 添加视频编码器
        if let Some(codec) = &options.video_codec {
            command.extend_from_slice(&["-c:v".to_string(), codec.clone()]);
        }

        // 添加音频编码器
        if let Some(codec) = &options.audio_codec {
            command.extend_from_slice(&["-c:a".to_string(), codec.clone()]);
        }

        // 添加比特率
        if let Some(bitrate) = &options.bitrate {
            command.extend_from_slice(&["-b:v".to_string(), bitrate.clone()]);
        }

        // 添加帧率
        if let Some(framerate) = options.framerate {
            command.extend_from_slice(&["-r".to_string(), framerate.to_string()]);
        }

        // 添加分辨率
        if let Some((width, height)) = options.resolution {
            command.extend_from_slice(&[
                "-s".to_string(),
                format!("{}x{}", width, height),
            ]);
        }

        // 添加质量设置
        match options.quality.as_str() {
            "low" => command.extend_from_slice(&["-crf".to_string(), "28".to_string()]),
            "medium" => command.extend_from_slice(&["-crf".to_string(), "23".to_string()]),
            "high" => command.extend_from_slice(&["-crf".to_string(), "18".to_string()]),
            "ultra" => command.extend_from_slice(&["-crf".to_string(), "15".to_string()]),
            _ => {}
        }

        command.push("-y".to_string());

        // 添加输出文件
        command.push(output_path.to_string_lossy().to_string());

        command
    }

    /// 获取支持的输出格式
    pub fn get_supported_formats(&self) -> Vec<String> {
        vec![
            "mp4".to_string(),
            "avi".to_string(),
            "mov".to_string(),
            "mkv".to_string(),
            "webm".to_string(),
            "flv".to_string(),
            "wmv".to_string(),
        ]
    }

    /// 获取预设转换选项
    pub fn get_preset_options(&self, preset: &str) -> ConversionOptions {
        match preset {
            "web" => ConversionOptions {
                format: "mp4".to_string(),
                quality: "medium".to_string(),
                video_codec: Some("h264".to_string()),
                audio_codec: Some("aac".to_string()),
                resolution: Some((1280, 720)),
                framerate: Some(30.0),
                ..Default::default()
            },
            "mobile" => ConversionOptions {
                format: "mp4".to_string(),
                quality: "medium".to_string(),
                video_codec: Some("h264".to_string()),
                audio_codec: Some("aac".to_string()),
                resolution: Some((854, 480)),
                framerate: Some(24.0),
                ..Default::default()
            },
            "hd" => ConversionOptions {
                format: "mp4".to_string(),
                quality: "high".to_string(),
                video_codec: Some("h264".to_string()),
                audio_codec: Some("aac".to_string()),
                resolution: Some((1920, 1080)),
                framerate: Some(30.0),
                ..Default::default()
            },
            _ => ConversionOptions::default(),
        }
    }
}

impl Default for VideoConverter {
    fn default() -> Self {
        Self::new()
    }
} 
