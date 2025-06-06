// src-tauri/src/commands/mod.rs

// 命令模块的主入口文件，负责导出所有子模块的命令

pub mod basic;
pub mod config;
pub mod video;
pub mod document;
pub mod ai;
pub mod timeline;
pub mod proxy;

// 重新导出所有命令，方便在main.rs中使用
pub use basic::*;
pub use config::*;
pub use video::*;
pub use document::*;
pub use ai::*;
pub use timeline::*;
pub use proxy::*; 