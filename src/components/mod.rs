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
<<<<<<< HEAD
// pub use settings_panel::SettingsPanel; // 暂时注释掉未使用的导入 
=======
// pub use settings_panel::SettingsPanel; // 暂时未使用，注释掉避免警告 
>>>>>>> 55e27c8 (重构样式文件，添加全局样式和组件样式，优化视频上传和翻译面板的用户界面，更新 API 以支持应用配置和 AI 模型状态，清理未使用的代码，增强响应式设计和动画效果。)
