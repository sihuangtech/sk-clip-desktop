pub mod processor;
pub mod editor;
pub mod converter;
pub mod analyzer;

// 重新导出主要类型和功能
pub use processor::{VideoProcessor, VideoProcessingTask};
pub use editor::{VideoEditor, EditOperation};
pub use converter::{VideoConverter, ConversionOptions};
pub use analyzer::{VideoAnalyzer, VideoMetadata};

use crate::models::AppError;
use crate::config::VideoConfig;
use std::path::PathBuf;

/// 视频服务管理器
pub struct VideoServiceManager {
    processor: VideoProcessor,
    editor: VideoEditor,
    converter: VideoConverter,
    analyzer: VideoAnalyzer,
    config: VideoConfig,
}

impl VideoServiceManager {
    pub fn new(config: VideoConfig) -> Self {
        Self {
            processor: VideoProcessor::new(),
            editor: VideoEditor::new(),
            converter: VideoConverter::new(),
            analyzer: VideoAnalyzer::new(),
            config,
        }
    }

    /// 分析视频文件
    pub async fn analyze_video(&self, video_path: &PathBuf) -> Result<VideoMetadata, AppError> {
        self.analyzer.analyze(video_path).await
    }

    /// 转换视频格式
    pub async fn convert_video(
        &self,
        input_path: &PathBuf,
        output_path: &PathBuf,
        options: Option<ConversionOptions>,
    ) -> Result<(), AppError> {
        let conversion_options = options.unwrap_or_else(|| ConversionOptions {
            format: self.config.default_output_format.clone(),
            quality: self.config.default_quality.clone(),
            ..Default::default()
        });
        
        self.converter.convert(input_path, output_path, conversion_options).await
    }

    /// 编辑视频
    pub async fn edit_video(
        &self,
        input_path: &PathBuf,
        output_path: &PathBuf,
        operations: Vec<EditOperation>,
    ) -> Result<(), AppError> {
        self.editor.apply_operations(input_path, output_path, operations).await
    }

    /// 处理视频任务
    pub async fn process_video_task(&self, task: VideoProcessingTask) -> Result<PathBuf, AppError> {
        self.processor.process_task(task).await
    }

    /// 获取支持的视频格式
    pub fn get_supported_formats(&self) -> Vec<String> {
        vec![
            "mp4".to_string(),
            "avi".to_string(),
            "mov".to_string(),
            "mkv".to_string(),
            "wmv".to_string(),
            "flv".to_string(),
            "webm".to_string(),
        ]
    }

    /// 检查文件大小是否超过限制
    pub fn check_file_size(&self, file_path: &PathBuf) -> Result<bool, AppError> {
        let metadata = std::fs::metadata(file_path)
            .map_err(|e| AppError::FileError(format!("无法获取文件信息: {}", e)))?;
        
        let size_mb = metadata.len() / (1024 * 1024);
        Ok(size_mb <= self.config.max_file_size_mb)
    }
} 