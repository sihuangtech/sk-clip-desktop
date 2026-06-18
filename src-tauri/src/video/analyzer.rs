use std::path::PathBuf;
use crate::models::AppError;
use log::info;
use super::processor::VideoProcessor;

/// 视频元数据
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VideoMetadata {
    /// 文件路径
    pub file_path: PathBuf,
    /// 文件大小（字节）
    pub file_size: u64,
    /// 视频时长（秒）
    pub duration: f64,
    /// 视频宽度
    pub width: u32,
    /// 视频高度
    pub height: u32,
    /// 帧率
    pub framerate: f32,
    /// 视频编码器
    pub video_codec: String,
    /// 音频编码器
    pub audio_codec: Option<String>,
    /// 比特率
    pub bitrate: u64,
    /// 音频采样率
    pub sample_rate: Option<u32>,
    /// 音频通道数
    pub audio_channels: Option<u32>,
    /// 视频格式
    pub format: String,
    /// 创建时间
    pub created_at: Option<String>,
    /// 是否有音频轨道
    pub has_audio: bool,
    /// 是否有视频轨道
    pub has_video: bool,
}

/// 视频分析器
pub struct VideoAnalyzer {
    // 可以添加FFprobe配置
}

impl VideoAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    /// 分析视频文件
    pub async fn analyze(&self, video_path: &PathBuf) -> Result<VideoMetadata, AppError> {
        info!("分析视频文件: {}", video_path.display());
        
        // 检查文件是否存在
        if !video_path.exists() {
            return Err(AppError::FileError(format!("视频文件不存在: {}", video_path.display())));
        }

        // 获取文件基本信息
        let file_metadata = std::fs::metadata(video_path)
            .map_err(|e| AppError::FileError(format!("无法获取文件信息: {}", e)))?;

        let metadata = self.extract_metadata(video_path, file_metadata.len()).await?;

        info!("视频分析完成: {}x{}, 时长: {:.2}s", metadata.width, metadata.height, metadata.duration);
        Ok(metadata)
    }

    /// 提取视频元数据
    async fn extract_metadata(&self, video_path: &PathBuf, file_size: u64) -> Result<VideoMetadata, AppError> {
        let processor = VideoProcessor::new();
        let info = processor.get_video_info(video_path).await?;

        let extension = video_path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown")
            .to_lowercase();
        let has_audio = !info.audio_codec.is_empty();

        Ok(VideoMetadata {
            file_path: video_path.clone(),
            file_size,
            duration: info.duration,
            width: info.width,
            height: info.height,
            framerate: info.fps as f32,
            video_codec: info.video_codec,
            audio_codec: if has_audio { Some(info.audio_codec) } else { None },
            bitrate: info.bitrate,
            sample_rate: None,
            audio_channels: None,
            format: extension,
            created_at: Some(chrono::Utc::now().to_rfc3339()),
            has_audio,
            has_video: info.width > 0 && info.height > 0,
        })
    }

    /// 检查视频是否损坏
    pub async fn check_integrity(&self, video_path: &PathBuf) -> Result<bool, AppError> {
        info!("检查视频完整性: {}", video_path.display());
        
        let metadata = std::fs::metadata(video_path)
            .map_err(|e| AppError::FileError(format!("无法获取文件信息: {}", e)))?;
        
        Ok(metadata.len() > 0)
    }

    /// 获取视频缩略图时间点
    pub fn get_thumbnail_timestamps(&self, duration: f64, count: usize) -> Vec<f64> {
        if count == 0 {
            return vec![];
        }
        
        if count == 1 {
            return vec![duration / 2.0]; // 中间点
        }
        
        let mut timestamps = Vec::new();
        let interval = duration / (count + 1) as f64;
        
        for i in 1..=count {
            timestamps.push(interval * i as f64);
        }
        
        timestamps
    }

    /// 检测视频中的场景变化
    pub async fn detect_scene_changes(&self, video_path: &PathBuf) -> Result<Vec<f64>, AppError> {
        info!("检测场景变化: {}", video_path.display());
        
        Err(AppError::VideoProcessingError(
            "真实场景检测尚未实现，不能返回模拟场景时间点。".to_string(),
        ))
    }

    /// 分析音频特征
    pub async fn analyze_audio(&self, video_path: &PathBuf) -> Result<AudioAnalysis, AppError> {
        info!("分析音频特征: {}", video_path.display());
        
        Err(AppError::VideoProcessingError(
            "真实音频特征分析尚未实现，不能返回模拟音频分析。".to_string(),
        ))
    }
}

/// 音频分析结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AudioAnalysis {
    /// 是否包含语音
    pub has_speech: bool,
    /// 平均音量 (0.0 - 1.0)
    pub average_volume: f32,
    /// 峰值音量 (0.0 - 1.0)
    pub peak_volume: f32,
    /// 静音时段 (开始时间, 结束时间)
    pub silence_periods: Vec<(f64, f64)>,
    /// 主要频率
    pub dominant_frequency: f32,
}

impl Default for VideoAnalyzer {
    fn default() -> Self {
        Self::new()
    }
} 
