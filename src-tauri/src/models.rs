// src-tauri/src/models.rs

// 该文件定义了应用程序中使用的各种数据结构和类型

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use std::collections::HashMap;
use thiserror::Error;

// 视频翻译任务状态枚举
// 表示一个视频翻译任务的当前处理阶段
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,      // 任务已创建，等待处理
    Processing,   // 任务正在进行中 (例如：语音识别、翻译、合成)
    Completed,    // 任务已成功完成
    Failed,       // 任务执行失败
}

// 视频翻译任务结构体
// 存储单个视频翻译任务的所有相关信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationTask {
    pub id: String,                  // 任务的唯一标识符
    pub video_path: String,          // 待翻译视频的原始路径
    pub status: TaskStatus,          // 任务的当前状态
    pub source_language: String,     // 视频的源语言代码 (例如: "en", "zh")
    pub target_language: String,     // 目标翻译语言代码 (例如: "zh", "en")
    pub output_path: Option<String>, // 翻译完成后的输出文件路径 (如果成功)
    pub error_message: Option<String>, // 任务失败时的错误信息 (如果失败)
}

// 应用状态结构体
// 存储整个应用需要共享的可变状态，如当前进行的任务列表、临时目录等
#[derive(Default)]
pub struct AppState {
    // 使用 Mutex 保护 HashMap，以便在多个异步任务中安全地访问和修改任务列表
    pub tasks: Mutex<HashMap<String, TranslationTask>>,
    // 临时目录，用于存放处理过程中的中间文件
    pub temp_dir: Mutex<Option<tempfile::TempDir>>,
    // TODO: 添加其他全局状态，如配置、AI模型加载状态等
}

// 应用程序自定义错误类型
// 定义了应用中可能发生的各种错误，方便错误处理和向前台报告
#[derive(Debug, Error, Serialize)]
pub enum AppError {
    #[error("视频处理失败: {0}")]
    VideoProcessingError(String), // 视频处理相关的错误
    
    #[error("文件操作失败: {0}")]
    FileError(String), // 文件系统操作错误，改为 String 以支持序列化
    
    #[error("目录操作失败: {0}")]
    DirectoryError(String), // 目录创建或访问错误
    
    #[error("任务不存在: {0}")]
    TaskNotFound(String), // 查找特定任务时，任务不存在
    
    #[error("命令执行失败: {0}")]
    CommandError(String), // 调用外部命令 (如 FFmpeg) 失败
    
    #[error("不受支持的语言: {0}")]
    UnsupportedLanguage(String), // 使用了应用不支持的语言代码

    #[error("AI 模型错误: {0}")]
    AiModelError(String), // AI 模型加载或推理错误

    #[error("文档解析失败: {0}")]
    DocumentParsingError(String), // 解析文档文件失败

    #[error("参数错误: {0}")]
    ArgumentError(String), // 函数或命令接收到的参数无效
    
    #[error("无效参数: {0}")]
    InvalidParameter(String), // 参数值无效或超出范围
    
    #[error("模型未初始化: {0}")]
    ModelNotInitialized(String), // AI模型未初始化
    
    #[error("模型初始化失败: {0}")]
    ModelInitializationError(String), // AI模型初始化失败
    
    #[error("配置错误: {0}")]
    ConfigError(String), // 配置文件相关错误
    
    #[error("验证错误: {0}")]
    ValidationError(String), // 数据验证错误
    
    #[error("不支持的格式: {0}")]
    UnsupportedFormat(String), // 不支持的文件格式
    
    #[error("序列化错误: {0}")]
    SerializationError(String), // 序列化/反序列化错误
    
    #[error("未知错误: {0}")]
    Unknown(String), // 其他未明确分类的错误
    
    // TODO: 根据实际需求细化错误类型
}

// 实现从 std::io::Error 到 AppError 的转换
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::FileError(err.to_string())
    }
}

// Tauri 命令的输入参数结构体
// 定义前端调用 Tauri 命令时需要传递的数据结构

// 视频上传命令的输入参数
#[derive(Debug, Deserialize)]
pub struct UploadVideoArgs {
    pub path: String, // 用户选择的视频文件路径
}

// 翻译视频命令的输入参数
#[derive(Debug, Deserialize)]
pub struct TranslateVideoArgs {
    pub video_path: String,      // 待翻译视频在应用内部存储的路径
    pub source_language: String, // 源语言代码
    pub target_language: String, // 目标语言代码
}

// 获取翻译任务状态命令的输入参数 (如果需要，这里示例不需要)
// #[derive(Debug, Deserialize)]
// pub struct GetTranslationTaskArgs {
//     pub task_id: String,
// }

// TODO: 根据实际需要为其他命令定义输入参数结构体

// ==================== 时间线项目数据结构 ====================

/// 媒体类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Video,
    Audio,
    Image,
    Text,
    Document,
}

/// 媒体素材
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaAsset {
    pub id: String,
    pub name: String,
    pub path: String,
    pub media_type: MediaType,
    pub duration: f64,  // 时长（秒）
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// 时间线片段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineClip {
    pub id: String,
    pub asset_id: String,
    pub track_id: String,
    pub start_time: f64,      // 在时间线上的开始时间
    pub duration: f64,         // 片段时长
    pub in_point: f64,         // 素材入点
    pub out_point: f64,        // 素材出点
    pub opacity: f32,          // 透明度
    pub volume: f32,           // 音量
    pub muted: bool,           // 是否静音
    pub locked: bool,          // 是否锁定
    pub effects: Vec<String>,  // 应用的效果
}

/// 轨道类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TrackType {
    Video,
    Audio,
    Text,
}

/// 时间线轨道
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineTrack {
    pub id: String,
    pub name: String,
    pub track_type: TrackType,
    pub index: u32,            // 轨道索引（0为最上方）
    pub visible: bool,         // 是否可见
    pub locked: bool,          // 是否锁定
    pub muted: bool,           // 是否静音
    pub clips: Vec<TimelineClip>,
}

/// 时间线项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineProject {
    pub id: String,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub duration: f64,         // 总时长
    pub width: u32,            // 画布宽度
    pub height: u32,           // 画布高度
    pub frame_rate: f64,       // 帧率
    pub tracks: Vec<TimelineTrack>,
    pub assets: Vec<MediaAsset>,
    pub markers: Vec<TimelineMarker>,
}

/// 时间线标记
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineMarker {
    pub id: String,
    pub time: f64,
    pub name: String,
    pub color: String,
}

impl TimelineProject {
    pub fn new(name: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            created_at: now,
            updated_at: now,
            duration: 0.0,
            width: 1920,
            height: 1080,
            frame_rate: 30.0,
            tracks: Vec::new(),
            assets: Vec::new(),
            markers: Vec::new(),
        }
    }

    pub fn add_track(&mut self, track_type: TrackType, name: Option<String>) -> String {
        let track_id = Uuid::new_v4().to_string();
        let track_name = name.unwrap_or_else(|| {
            let type_name = match track_type {
                TrackType::Video => "视频",
                TrackType::Audio => "音频",
                TrackType::Text => "文本",
            };
            format!("{}轨道 {}", type_name, self.tracks.len() + 1)
        });

        let track = TimelineTrack {
            id: track_id.clone(),
            name: track_name,
            track_type,
            index: self.tracks.len() as u32,
            visible: true,
            locked: false,
            muted: false,
            clips: Vec::new(),
        };

        self.tracks.push(track);
        self.updated_at = chrono::Utc::now();
        track_id
    }

    pub fn add_asset(&mut self, asset: MediaAsset) -> String {
        let id = asset.id.clone();
        self.assets.push(asset);
        self.updated_at = chrono::Utc::now();
        id
    }

    pub fn update_duration(&mut self) {
        let mut max_end = 0.0;
        for track in &self.tracks {
            for clip in &track.clips {
                let clip_end = clip.start_time + clip.duration;
                if clip_end > max_end {
                    max_end = clip_end;
                }
            }
        }
        self.duration = max_end;
        self.updated_at = chrono::Utc::now();
    }
} 