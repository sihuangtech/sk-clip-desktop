// src/components/mod.rs

pub mod video_upload;
pub mod document_import;
pub mod translation_panel;
pub mod project_timeline;
pub mod common;

// 重新导出主要组件
pub use video_upload::VideoUploadComponent;
pub use document_import::DocumentImportComponent;
pub use translation_panel::TranslationPanelComponent;
pub use project_timeline::ProjectTimelineComponent; 