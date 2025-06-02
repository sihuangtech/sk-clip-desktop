// src/components/video_upload.rs

use yew::prelude::*;
use web_sys::{FileList, HtmlInputElement};
use wasm_bindgen_futures::spawn_local;
use serde_wasm_bindgen;

use crate::types::{AppState, UploadVideoArgs, SUPPORTED_LANGUAGES};
use crate::components::common::LoadingSpinner;
use crate::api::invoke;

#[derive(Properties, PartialEq)]
pub struct VideoUploadProps {
    pub app_state: UseStateHandle<AppState>,
    pub source_language: UseStateHandle<String>,
    pub target_language: UseStateHandle<String>,
}

#[function_component(VideoUploadComponent)]
pub fn video_upload_component(props: &VideoUploadProps) -> Html {
    let file_input_ref = use_node_ref();

    // 处理视频文件选择
    let on_video_select = {
        let app_state = props.app_state.clone();
        
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let files: Option<FileList> = input.files();
            
            if let Some(files) = files {
                if let Some(file) = files.get(0) {
                    let file_name = file.name();
                    app_state.set(AppState::Uploading);
                    
                    // 调用Tauri API来处理文件上传
                    let app_state_clone = app_state.clone();
                    spawn_local(async move {
                        let args = serde_wasm_bindgen::to_value(&UploadVideoArgs {
                            path: file_name
                        }).unwrap();
                        
                        match invoke("upload_video", args).await.as_string() {
                            Some(path) => {
                                app_state_clone.set(AppState::Ready(path));
                            },
                            None => {
                                app_state_clone.set(AppState::Error("上传视频失败".to_string()));
                            }
                        }
                    });
                }
            }
        })
    };

    // 处理语言选择
    let on_source_language_change = {
        let source_language = props.source_language.clone();
        Callback::from(move |e: Event| {
            let select: HtmlInputElement = e.target_unchecked_into();
            source_language.set(select.value());
        })
    };

    let on_target_language_change = {
        let target_language = props.target_language.clone();
        Callback::from(move |e: Event| {
            let select: HtmlInputElement = e.target_unchecked_into();
            target_language.set(select.value());
        })
    };

    // 根据状态判断是否禁用上传
    let is_uploading = matches!(&*props.app_state, AppState::Uploading | AppState::Translating(_));

    html! {
        <div class="video-upload-section">
            <h2>{"视频上传"}</h2>
            
            if is_uploading {
                <LoadingSpinner message={"正在上传视频...".to_string()} />
            } else {
                <div class="upload-controls">
                    <input 
                        type="file" 
                        ref={file_input_ref.clone()} 
                        accept="video/*" 
                        onchange={on_video_select}
                        disabled={is_uploading}
                    />
                    
                    <div class="language-selection">
                        <div class="language-group">
                            <label for="source-language">{"源语言："}</label>
                            <select id="source-language" onchange={on_source_language_change}>
                                {
                                    SUPPORTED_LANGUAGES.iter().map(|(code, name)| {
                                        html! {
                                            <option 
                                                value={*code} 
                                                selected={&*props.source_language == code}
                                            >
                                                {name}
                                            </option>
                                        }
                                    }).collect::<Html>()
                                }
                            </select>
                        </div>
                        
                        <div class="language-group">
                            <label for="target-language">{"目标语言："}</label>
                            <select id="target-language" onchange={on_target_language_change}>
                                {
                                    SUPPORTED_LANGUAGES.iter().map(|(code, name)| {
                                        html! {
                                            <option 
                                                value={*code} 
                                                selected={&*props.target_language == code}
                                            >
                                                {name}
                                            </option>
                                        }
                                    }).collect::<Html>()
                                }
                            </select>
                        </div>
                    </div>
                </div>
            }
        </div>
    }
} 