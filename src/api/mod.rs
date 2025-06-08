// src/api/mod.rs

use wasm_bindgen::prelude::*;
use crate::types::{TranslateVideoArgs, UploadVideoArgs};

// 为了方便在其他模块中使用，创建一个别名
pub mod tauri_api {
    pub use super::*;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// 上传视频 API
#[allow(dead_code)]
pub async fn upload_video(path: String) -> Result<String, String> {
    let args = serde_wasm_bindgen::to_value(&UploadVideoArgs { path })
        .map_err(|e| format!("序列化参数失败: {}", e))?;
    
    let result = invoke("upload_video", args).await;
    
    if let Ok(path) = serde_wasm_bindgen::from_value::<String>(result) {
        Ok(path)
    } else {
        Err("上传视频失败".to_string())
    }
}

// 翻译视频 API
#[allow(dead_code)]
pub async fn translate_video(video_path: String, source_language: String, target_language: String) -> Result<String, String> {
    let args = serde_wasm_bindgen::to_value(&TranslateVideoArgs {
        video_path,
        source_language,
        target_language,
    }).map_err(|e| format!("序列化参数失败: {}", e))?;
    
    let result = invoke("translate_video", args).await;
    
    if let Ok(task_id) = serde_wasm_bindgen::from_value::<String>(result) {
        Ok(task_id)
    } else {
        Err("开始翻译失败".to_string())
    }
}

// 检查任务输出 API
#[allow(dead_code)]
pub async fn check_task_output(task_id: String) -> Result<String, String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "task_id": task_id
    })).map_err(|e| format!("序列化参数失败: {}", e))?;
    
    let result = invoke("check_task_output", args).await;
    
    if let Ok(output) = serde_wasm_bindgen::from_value::<String>(result) {
        Ok(output)
    } else {
        Err("检查任务状态失败".to_string())
    }
}

// 获取应用配置 API
pub async fn get_app_config() -> Result<serde_json::Value, String> {
    let result = invoke("get_app_config", JsValue::NULL).await;
    
    if let Ok(config) = serde_wasm_bindgen::from_value::<serde_json::Value>(result) {
        Ok(config)
    } else {
        Err("获取配置失败".to_string())
    }
}

// 更新应用配置 API
#[allow(dead_code)]
pub async fn update_app_config(config: serde_json::Value) -> Result<(), String> {
    let args = serde_wasm_bindgen::to_value(&config)
        .map_err(|e| format!("序列化配置失败: {}", e))?;
    
    let result = invoke("update_app_config", args).await;
    
    if serde_wasm_bindgen::from_value::<bool>(result).unwrap_or(false) {
        Ok(())
    } else {
        Err("更新配置失败".to_string())
    }
}

// 获取AI模型状态 API
#[allow(dead_code)]
pub async fn get_ai_model_status() -> Result<serde_json::Value, String> {
    let result = invoke("get_ai_model_status", JsValue::NULL).await;
    
    if let Ok(status) = serde_wasm_bindgen::from_value::<serde_json::Value>(result) {
        Ok(status)
    } else {
        Err("获取AI模型状态失败".to_string())
    }
}

// 初始化AI模型 API
#[allow(dead_code)]
pub async fn initialize_ai_models() -> Result<(), String> {
    let result = invoke("initialize_ai_models", JsValue::NULL).await;
    
    if serde_wasm_bindgen::from_value::<bool>(result).unwrap_or(false) {
        Ok(())
    } else {
        Err("初始化AI模型失败".to_string())
    }
} 