// src/components/mod.rs

pub mod video_upload;
pub mod document_import;
pub mod translation_panel;
pub mod project_timeline;
pub mod settings_panel;
pub mod common;

// 重新导出主要组件
pub use video_upload::VideoUploadComponent;
pub use document_import::DocumentImportComponent;
pub use translation_panel::TranslationPanelComponent;
pub use project_timeline::ProjectTimelineComponent;
// pub use settings_panel::SettingsPanel; // 暂时未使用，注释掉避免警告
