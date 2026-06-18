// src-tauri/src/commands/document.rs

// 文档相关命令模块，处理文档导入、解析、转换等功能

use tauri::AppHandle;
use log::info;
use std::path::PathBuf;

use crate::models::AppError;
use crate::document::{DocumentContent, DocumentServiceManager};

// 导入文档命令
#[tauri::command]
pub async fn import_document(
    _app_handle: AppHandle,
    file_path: String
) -> Result<DocumentContent, AppError> {
    info!("调用 import_document 命令，文件路径: {}", file_path);
    
    let path = PathBuf::from(file_path);
    let config = crate::config::AppConfig::default();
    let document_service = DocumentServiceManager::new(config.document);
    
    document_service.parse_document(&path).await
}

// 获取支持的文档类型命令
#[tauri::command]
pub fn get_supported_document_types() -> Vec<String> {
    info!("调用 get_supported_document_types 命令");
    vec![
        "pdf".to_string(),
        "pptx".to_string(),
        "md".to_string(),
        "docx".to_string(),
    ]
}

// 转换文档为资源命令
#[tauri::command]
pub async fn convert_document_to_assets(
    _app_handle: AppHandle,
    document_content: DocumentContent
) -> Result<Vec<String>, AppError> {
    info!("调用 convert_document_to_assets 命令，文档: {:?}", document_content.title);

    Err(AppError::DocumentParsingError(
        "真实文档转视频资源功能尚未实现，不能返回模拟资源路径。".to_string(),
    ))
} 
