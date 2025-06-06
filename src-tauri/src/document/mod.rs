pub mod parser;
pub mod converter;
pub mod extractor;

// 重新导出主要类型和功能
pub use parser::{DocumentParser, DocumentContent};
pub use converter::{DocumentConverter, ConversionOptions};
pub use extractor::{ContentExtractor, ExtractedContent};

use crate::models::AppError;
use crate::config::DocumentConfig;
use std::path::PathBuf;

/// 文档服务管理器
pub struct DocumentServiceManager {
    parser: DocumentParser,
    converter: DocumentConverter,
    extractor: ContentExtractor,
    config: DocumentConfig,
}

impl DocumentServiceManager {
    pub fn new(config: DocumentConfig) -> Self {
        Self {
            parser: DocumentParser::new(),
            converter: DocumentConverter::new(),
            extractor: ContentExtractor::new(),
            config,
        }
    }

    /// 解析文档
    pub async fn parse_document(&self, document_path: &PathBuf) -> Result<DocumentContent, AppError> {
        // 检查文件大小
        if !self.check_file_size(document_path)? {
            return Err(AppError::ValidationError(
                format!("文档文件过大，超过{}MB限制", self.config.max_file_size_mb)
            ));
        }

        self.parser.parse(document_path).await
    }

    /// 转换文档格式
    pub async fn convert_document(
        &self,
        input_path: &PathBuf,
        output_path: &PathBuf,
        options: Option<ConversionOptions>,
    ) -> Result<(), AppError> {
        let conversion_options = options.unwrap_or_default();
        self.converter.convert(input_path, output_path, conversion_options).await
    }

    /// 提取文档内容
    pub async fn extract_content(&self, document_path: &PathBuf) -> Result<ExtractedContent, AppError> {
        self.extractor.extract(document_path).await
    }

    /// 获取支持的文档格式
    pub fn get_supported_formats(&self) -> Vec<String> {
        self.config.supported_formats.clone()
    }

    /// 检查文件大小是否超过限制
    pub fn check_file_size(&self, file_path: &PathBuf) -> Result<bool, AppError> {
        let metadata = std::fs::metadata(file_path)
            .map_err(|e| AppError::FileError(format!("无法获取文件信息: {}", e)))?;
        
        let size_mb = metadata.len() / (1024 * 1024);
        Ok(size_mb <= self.config.max_file_size_mb)
    }

    /// 检查文档格式是否支持
    pub fn is_format_supported(&self, file_path: &PathBuf) -> bool {
        if let Some(extension) = file_path.extension() {
            if let Some(ext_str) = extension.to_str() {
                return self.config.supported_formats.contains(&ext_str.to_lowercase());
            }
        }
        false
    }

    /// 批量处理文档
    pub async fn batch_process_documents(
        &self,
        document_paths: Vec<PathBuf>,
        output_dir: &PathBuf,
    ) -> Result<Vec<PathBuf>, AppError> {
        let mut results = Vec::new();
        
        for (i, doc_path) in document_paths.iter().enumerate() {
            log::info!("处理文档 {}/{}: {}", i + 1, document_paths.len(), doc_path.display());
            
            // 检查格式支持
            if !self.is_format_supported(doc_path) {
                log::warn!("跳过不支持的文档格式: {}", doc_path.display());
                continue;
            }

            // 生成输出路径
            let output_path = output_dir.join(
                doc_path.file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string() + "_processed.json"
            );

            // 处理文档
            match self.parse_document(doc_path).await {
                Ok(content) => {
                    // 保存处理结果
                    let content_json = serde_json::to_string_pretty(&content)
                        .map_err(|e| AppError::SerializationError(format!("序列化失败: {}", e)))?;
                    
                    std::fs::write(&output_path, content_json)
                        .map_err(|e| AppError::FileError(format!("保存文件失败: {}", e)))?;
                    
                    results.push(output_path);
                }
                Err(e) => {
                    log::error!("处理文档失败 {}: {}", doc_path.display(), e);
                }
            }
        }

        Ok(results)
    }
} 