// src/api/mod.rs

use wasm_bindgen::prelude::*;
use crate::types::{TranslateVideoArgs, UploadVideoArgs};

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