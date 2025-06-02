// src/types/mod.rs

use serde::{Deserialize, Serialize};

// 视频上传参数
#[derive(Serialize, Deserialize)]
pub struct UploadVideoArgs {
    pub path: String,
}

// 视频翻译参数
#[derive(Serialize, Deserialize)]
pub struct TranslateVideoArgs {
    pub video_path: String,
    pub source_language: String,
    pub target_language: String,
}

// 翻译任务状态
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct TranslationTask {
    pub id: String,
    pub video_path: String,
    pub status: String, // "Pending", "Processing", "Completed", "Failed"
    pub source_language: String,
    pub target_language: String,
    pub output_path: Option<String>,
    pub error_message: Option<String>,
}

// 文档类型枚举
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum DocumentType {
    PowerPoint,
    Markdown,
    Pdf,
}

// 文档页面结构
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct DocumentPage {
    pub page_number: usize,
    pub title: Option<String>,
    pub text_content: String,
    pub image_paths: Vec<String>,
    pub notes: Option<String>,
}

// 文档内容结构
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct DocumentContent {
    pub document_type: DocumentType,
    pub title: String,
    pub pages: Vec<DocumentPage>,
    pub total_pages: usize,
    pub source_path: String,
}

// 应用状态枚举
#[derive(Clone, Debug, PartialEq)]
pub enum AppState {
    Idle,                           // 空闲状态
    Uploading,                      // 正在上传视频
    Ready(String),                  // 视频已上传，准备翻译（包含视频路径）
    Translating(String),            // 正在翻译（包含任务ID）
    #[allow(dead_code)]
    Completed(String),              // 翻译完成（包含输出路径）
    Error(String),                  // 错误状态
    DocumentImporting,              // 正在导入文档
    DocumentReady(DocumentContent), // 文档已导入，准备处理（包含文档内容）
    #[allow(dead_code)]
    CreatingProject,                // 正在创建项目
}

// 支持的语言列表
pub const SUPPORTED_LANGUAGES: &[(&str, &str)] = &[
    ("zh", "中文"),
    ("en", "英文"),
    ("ja", "日语"),
    ("ko", "韩语"),
    ("es", "西班牙语"),
    ("ru", "俄语"),
    ("de", "德语"),
    ("fr", "法语"),
]; 