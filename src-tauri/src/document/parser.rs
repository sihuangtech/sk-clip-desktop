use std::path::PathBuf;
use crate::models::AppError;
use log::info;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::File;
use std::io::Read;
use zip::ZipArchive;

/// 文档内容结构
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DocumentContent {
    /// 文档类型
    pub document_type: DocumentType,
    /// 文档标题
    pub title: Option<String>,
    /// 文档作者
    pub author: Option<String>,
    /// 页面内容
    pub pages: Vec<PageContent>,
    /// 元数据
    pub metadata: DocumentMetadata,
}

/// 文档类型
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DocumentType {
    PDF,
    PowerPoint,
    Markdown,
    Word,
    Unknown,
}

/// 页面内容
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PageContent {
    /// 页面编号
    pub page_number: usize,
    /// 文本内容
    pub text: String,
    /// 图片列表
    pub images: Vec<ImageInfo>,
    /// 表格列表
    pub tables: Vec<TableInfo>,
    /// 布局信息
    pub layout: Option<LayoutInfo>,
}

/// 图片信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageInfo {
    /// 图片ID
    pub id: String,
    /// 图片路径（如果已提取）
    pub path: Option<PathBuf>,
    /// 图片描述
    pub description: Option<String>,
    /// 位置信息
    pub position: Option<Position>,
    /// 尺寸信息
    pub size: Option<Size>,
}

/// 表格信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TableInfo {
    /// 表格ID
    pub id: String,
    /// 行数
    pub rows: usize,
    /// 列数
    pub columns: usize,
    /// 表格数据
    pub data: Vec<Vec<String>>,
    /// 位置信息
    pub position: Option<Position>,
}

/// 布局信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LayoutInfo {
    /// 页面宽度
    pub width: f32,
    /// 页面高度
    pub height: f32,
    /// 边距
    pub margins: Margins,
}

/// 位置信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// 尺寸信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

/// 边距信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Margins {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

/// 文档元数据
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DocumentMetadata {
    /// 文件路径
    pub file_path: PathBuf,
    /// 文件大小
    pub file_size: u64,
    /// 创建时间
    pub created_at: Option<String>,
    /// 修改时间
    pub modified_at: Option<String>,
    /// 页面数量
    pub page_count: usize,
    /// 语言
    pub language: Option<String>,
}

/// 文档解析器
pub struct DocumentParser {
    // 可以添加解析器配置
}

impl DocumentParser {
    pub fn new() -> Self {
        Self {}
    }

    /// 解析文档
    pub async fn parse(&self, document_path: &PathBuf) -> Result<DocumentContent, AppError> {
        info!("解析文档: {}", document_path.display());
        
        // 检查文件是否存在
        if !document_path.exists() {
            return Err(AppError::FileError(format!("文档文件不存在: {}", document_path.display())));
        }

        // 获取文件信息
        let _file_metadata = std::fs::metadata(document_path)
            .map_err(|e| AppError::FileError(format!("无法获取文件信息: {}", e)))?;

        // 根据文件扩展名确定文档类型
        let document_type = self.detect_document_type(document_path);
        
        // 根据文档类型进行解析
        let content = match document_type {
            DocumentType::PDF => self.parse_pdf(document_path).await?,
            DocumentType::PowerPoint => self.parse_powerpoint(document_path).await?,
            DocumentType::Markdown => self.parse_markdown(document_path).await?,
            DocumentType::Word => self.parse_word(document_path).await?,
            DocumentType::Unknown => {
                return Err(AppError::UnsupportedFormat(
                    format!("不支持的文档格式: {}", document_path.display())
                ));
            }
        };

        info!("文档解析完成，共{}页", content.pages.len());
        Ok(content)
    }

    /// 检测文档类型
    fn detect_document_type(&self, document_path: &PathBuf) -> DocumentType {
        if let Some(extension) = document_path.extension() {
            if let Some(ext_str) = extension.to_str() {
                match ext_str.to_lowercase().as_str() {
                    "pdf" => DocumentType::PDF,
                    "pptx" | "ppt" => DocumentType::PowerPoint,
                    "md" | "markdown" => DocumentType::Markdown,
                    "docx" | "doc" => DocumentType::Word,
                    _ => DocumentType::Unknown,
                }
            } else {
                DocumentType::Unknown
            }
        } else {
            DocumentType::Unknown
        }
    }

    /// 解析PDF文档
    async fn parse_pdf(&self, document_path: &PathBuf) -> Result<DocumentContent, AppError> {
        info!("解析PDF文档: {}", document_path.display());

        let doc = lopdf::Document::load(document_path)
            .map_err(|e| AppError::DocumentParsingError(format!("读取PDF失败: {}", e)))?;
        let file_metadata = std::fs::metadata(document_path)
            .map_err(|e| AppError::FileError(format!("无法获取文件信息: {}", e)))?;

        let mut pages = Vec::new();
        for (index, page_number) in doc.get_pages().keys().enumerate() {
            let text = doc.extract_text(&[*page_number]).unwrap_or_default();
            pages.push(PageContent {
                page_number: index + 1,
                text,
                images: vec![],
                tables: vec![],
                layout: None,
            });
        }

        Ok(Self::build_document_content(
            DocumentType::PDF,
            document_path,
            file_metadata.len(),
            pages,
        ))
    }

    /// 解析PowerPoint文档
    async fn parse_powerpoint(&self, document_path: &PathBuf) -> Result<DocumentContent, AppError> {
        info!("解析PowerPoint文档: {}", document_path.display());

        if document_path.extension().and_then(|ext| ext.to_str()) != Some("pptx") {
            return Err(AppError::UnsupportedFormat(
                "当前仅支持真实解析 PPTX；旧版 PPT 二进制格式尚未支持。".to_string(),
            ));
        }

        let file_metadata = std::fs::metadata(document_path)
            .map_err(|e| AppError::FileError(format!("无法获取文件信息: {}", e)))?;
        let mut archive = Self::open_zip(document_path)?;
        let mut slide_names = Vec::new();

        for index in 0..archive.len() {
            let file = archive
                .by_index(index)
                .map_err(|e| AppError::DocumentParsingError(format!("读取PPTX条目失败: {}", e)))?;
            let name = file.name().to_string();
            if name.starts_with("ppt/slides/slide") && name.ends_with(".xml") {
                slide_names.push(name);
            }
        }

        slide_names.sort_by_key(|name| Self::natural_number_key(name));

        let mut pages = Vec::new();
        for (index, name) in slide_names.iter().enumerate() {
            let xml = Self::read_zip_text(&mut archive, name)?;
            let text = Self::extract_xml_text(&xml);
            pages.push(PageContent {
                page_number: index + 1,
                text,
                images: vec![],
                tables: vec![],
                layout: Some(LayoutInfo {
                    width: 1280.0,
                    height: 720.0,
                    margins: Margins {
                        top: 0.0,
                        right: 0.0,
                        bottom: 0.0,
                        left: 0.0,
                    },
                }),
            });
        }

        Ok(Self::build_document_content(
            DocumentType::PowerPoint,
            document_path,
            file_metadata.len(),
            pages,
        ))
    }

    /// 解析Markdown文档
    async fn parse_markdown(&self, document_path: &PathBuf) -> Result<DocumentContent, AppError> {
        info!("解析Markdown文档: {}", document_path.display());
        
        // 读取文件内容
        let content = std::fs::read_to_string(document_path)
            .map_err(|e| AppError::FileError(format!("读取Markdown文件失败: {}", e)))?;
        
        let file_metadata = std::fs::metadata(document_path)
            .map_err(|e| AppError::FileError(format!("无法获取文件信息: {}", e)))?;

        // 简单解析：将整个内容作为一页
        let pages = vec![
            PageContent {
                page_number: 1,
                text: content,
                images: vec![],
                tables: vec![],
                layout: None,
            },
        ];

        Ok(DocumentContent {
            document_type: DocumentType::Markdown,
            title: document_path.file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string()),
            author: None,
            pages,
            metadata: DocumentMetadata {
                file_path: document_path.clone(),
                file_size: file_metadata.len(),
                created_at: Some(chrono::Utc::now().to_rfc3339()),
                modified_at: Some(chrono::Utc::now().to_rfc3339()),
                page_count: 1,
                language: Some("zh".to_string()),
            },
        })
    }

    /// 解析Word文档
    async fn parse_word(&self, document_path: &PathBuf) -> Result<DocumentContent, AppError> {
        info!("解析Word文档: {}", document_path.display());

        if document_path.extension().and_then(|ext| ext.to_str()) != Some("docx") {
            return Err(AppError::UnsupportedFormat(
                "当前仅支持真实解析 DOCX；旧版 DOC 二进制格式尚未支持。".to_string(),
            ));
        }

        let file_metadata = std::fs::metadata(document_path)
            .map_err(|e| AppError::FileError(format!("无法获取文件信息: {}", e)))?;
        let mut archive = Self::open_zip(document_path)?;
        let xml = Self::read_zip_text(&mut archive, "word/document.xml")?;
        let text = Self::extract_xml_text(&xml);

        let pages = vec![PageContent {
            page_number: 1,
            text,
            images: vec![],
            tables: vec![],
            layout: None,
        }];

        Ok(Self::build_document_content(
            DocumentType::Word,
            document_path,
            file_metadata.len(),
            pages,
        ))
    }

    fn build_document_content(
        document_type: DocumentType,
        document_path: &PathBuf,
        file_size: u64,
        pages: Vec<PageContent>,
    ) -> DocumentContent {
        let page_count = pages.len();
        DocumentContent {
            document_type,
            title: document_path
                .file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string()),
            author: None,
            pages,
            metadata: DocumentMetadata {
                file_path: document_path.clone(),
                file_size,
                created_at: Some(chrono::Utc::now().to_rfc3339()),
                modified_at: Some(chrono::Utc::now().to_rfc3339()),
                page_count,
                language: None,
            },
        }
    }

    fn open_zip(document_path: &PathBuf) -> Result<ZipArchive<File>, AppError> {
        let file = File::open(document_path)
            .map_err(|e| AppError::FileError(format!("打开压缩文档失败: {}", e)))?;
        ZipArchive::new(file)
            .map_err(|e| AppError::DocumentParsingError(format!("读取压缩文档失败: {}", e)))
    }

    fn read_zip_text(archive: &mut ZipArchive<File>, name: &str) -> Result<String, AppError> {
        let mut file = archive
            .by_name(name)
            .map_err(|e| AppError::DocumentParsingError(format!("读取文档内部文件 {} 失败: {}", name, e)))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| AppError::DocumentParsingError(format!("读取XML文本失败: {}", e)))?;
        Ok(content)
    }

    fn extract_xml_text(xml: &str) -> String {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut parts = Vec::new();
        loop {
            match reader.read_event() {
                Ok(Event::Text(text)) => {
                    if let Ok(decoded) = text.decode() {
                        let text = decoded.trim();
                        if !text.is_empty() {
                            parts.push(text.to_string());
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => break,
                _ => {}
            }
        }

        parts.join("\n")
    }

    fn natural_number_key(name: &str) -> usize {
        name.chars()
            .filter(|ch| ch.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap_or(usize::MAX)
    }
}

impl Default for DocumentParser {
    fn default() -> Self {
        Self::new()
    }
} 
