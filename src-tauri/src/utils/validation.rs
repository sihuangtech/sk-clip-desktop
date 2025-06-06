use std::path::Path;

/// 验证文件扩展名是否在支持列表中
pub fn validate_file_extension(file_path: &Path, supported_extensions: &[&str]) -> bool {
    if let Some(extension) = file_path.extension() {
        if let Some(ext_str) = extension.to_str() {
            return supported_extensions.contains(&ext_str.to_lowercase().as_str());
        }
    }
    false
}

/// 验证文件大小是否在限制范围内
pub fn validate_file_size(file_path: &Path, max_size_mb: u64) -> Result<bool, std::io::Error> {
    let metadata = std::fs::metadata(file_path)?;
    let size_mb = metadata.len() / (1024 * 1024);
    Ok(size_mb <= max_size_mb)
}

/// 验证视频文件格式
pub fn is_video_file(file_path: &Path) -> bool {
    const VIDEO_EXTENSIONS: &[&str] = &["mp4", "avi", "mov", "mkv", "wmv", "flv", "webm"];
    validate_file_extension(file_path, VIDEO_EXTENSIONS)
}

/// 验证音频文件格式
pub fn is_audio_file(file_path: &Path) -> bool {
    const AUDIO_EXTENSIONS: &[&str] = &["mp3", "wav", "aac", "flac", "ogg", "m4a"];
    validate_file_extension(file_path, AUDIO_EXTENSIONS)
}

/// 验证图片文件格式
pub fn is_image_file(file_path: &Path) -> bool {
    const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "gif", "bmp", "webp", "svg"];
    validate_file_extension(file_path, IMAGE_EXTENSIONS)
}

/// 验证文档文件格式
pub fn is_document_file(file_path: &Path) -> bool {
    const DOCUMENT_EXTENSIONS: &[&str] = &["pdf", "doc", "docx", "ppt", "pptx", "md", "txt"];
    validate_file_extension(file_path, DOCUMENT_EXTENSIONS)
}

/// 验证语言代码
pub fn validate_language_code(lang_code: &str) -> bool {
    const SUPPORTED_LANGUAGES: &[&str] = &["zh", "en", "ja", "ko", "es", "fr", "de", "ru", "pt", "it"];
    SUPPORTED_LANGUAGES.contains(&lang_code)
}

/// 验证时间范围
pub fn validate_time_range(start_time: f64, end_time: f64, max_duration: f64) -> bool {
    start_time >= 0.0 && end_time > start_time && end_time <= max_duration
}

/// 验证分辨率
pub fn validate_resolution(width: u32, height: u32) -> bool {
    width > 0 && height > 0 && width <= 7680 && height <= 4320 // 最大支持8K
}

/// 验证帧率
pub fn validate_framerate(fps: f32) -> bool {
    fps > 0.0 && fps <= 120.0
} 