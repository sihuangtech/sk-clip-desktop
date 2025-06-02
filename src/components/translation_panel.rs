// src/components/translation_panel.rs

use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::types::{AppState, SUPPORTED_LANGUAGES};

#[derive(Properties, PartialEq)]
pub struct TranslationPanelProps {
    pub app_state: UseStateHandle<AppState>,
}

#[function_component(TranslationPanelComponent)]
pub fn translation_panel_component(props: &TranslationPanelProps) -> Html {
    let source_language = use_state(|| "zh".to_string());
    let target_language = use_state(|| "en".to_string());

    let on_translate = {
        let app_state = props.app_state.clone();
        let source_language = source_language.clone();
        let target_language = target_language.clone();
        
        Callback::from(move |_| {
            if let AppState::Ready(video_path) = &*app_state {
                let app_state_clone = app_state.clone();
                let source_lang = (*source_language).clone();
                let target_lang = (*target_language).clone();
                let video_path = video_path.clone();
                
                spawn_local(async move {
                    match crate::api::translate_video(video_path, source_lang, target_lang).await {
                        Ok(task_id) => {
                            app_state_clone.set(AppState::Translating(task_id));
                        }
                        Err(e) => {
                            app_state_clone.set(AppState::Error(format!("翻译失败: {}", e)));
                        }
                    }
                });
            }
        })
    };

    html! {
        <div class="translation-panel">
            <h2>{"视频翻译"}</h2>
            
            {match &*props.app_state {
                AppState::Idle => html! {
                    <p class="section-description">{"请先上传视频文件"}</p>
                },
                AppState::Uploading => html! {
                    <div class="loading-spinner">
                        <div class="spinner"></div>
                        <p>{"正在上传视频..."}</p>
                    </div>
                },
                AppState::Ready(path) => html! {
                    <div class="translation-controls">
                        <p class="section-description">{format!("视频已准备就绪: {}", path)}</p>
                        
                        <div class="language-selection">
                            <div class="language-group">
                                <label for="source-language">{"源语言:"}</label>
                                <select id="source-language">
                                    {for SUPPORTED_LANGUAGES.iter().map(|(code, name)| {
                                        html! {
                                            <option value={*code} selected={*code == *source_language}>
                                                {name}
                                            </option>
                                        }
                                    })}
                                </select>
                            </div>
                            
                            <div class="language-group">
                                <label for="target-language">{"目标语言:"}</label>
                                <select id="target-language">
                                    {for SUPPORTED_LANGUAGES.iter().map(|(code, name)| {
                                        html! {
                                            <option value={*code} selected={*code == *target_language}>
                                                {name}
                                            </option>
                                        }
                                    })}
                                </select>
                            </div>
                        </div>
                        
                        <button class="btn btn-primary" onclick={on_translate}>
                            {"开始翻译"}
                        </button>
                    </div>
                },
                AppState::Translating(_) => html! {
                    <div class="loading-spinner">
                        <div class="spinner"></div>
                        <p>{"正在翻译视频，请稍候..."}</p>
                    </div>
                },
                AppState::Completed(output_path) => html! {
                    <div class="translation-result">
                        <div class="message success">
                            <h3>{"翻译完成！"}</h3>
                            <p>{format!("输出文件: {}", output_path)}</p>
                        </div>
                        <button class="btn btn-secondary" onclick={
                            let app_state = props.app_state.clone();
                            Callback::from(move |_| {
                                app_state.set(AppState::Idle);
                            })
                        }>
                            {"重新开始"}
                        </button>
                    </div>
                },
                AppState::Error(error_msg) => html! {
                    <div class="message error">
                        <h3>{"翻译失败"}</h3>
                        <p>{error_msg}</p>
                        <button class="btn btn-secondary" onclick={
                            let app_state = props.app_state.clone();
                            Callback::from(move |_| {
                                app_state.set(AppState::Idle);
                            })
                        }>
                            {"重试"}
                        </button>
                    </div>
                },
                _ => html! {
                    <p>{"当前状态不支持翻译操作"}</p>
                }
            }}
        </div>
    }
} 