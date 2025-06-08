// src/components/translation_panel.rs

use yew::prelude::*;
use web_sys::HtmlTextAreaElement;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;
use crate::types::{AppState, SUPPORTED_LANGUAGES};

#[derive(Properties, PartialEq)]
pub struct TranslationPanelProps {
    pub app_state: UseStateHandle<AppState>,
}

#[function_component(TranslationPanelComponent)]
pub fn translation_panel_component(props: &TranslationPanelProps) -> Html {
    let source_language = use_state(|| "zh".to_string()); // 默认中文
    let target_language = use_state(|| "en".to_string()); // 默认英文
    let source_text = use_state(|| "".to_string());
    let translated_text = use_state(|| "".to_string());
    let is_translating = use_state(|| false);
    let is_synthesizing = use_state(|| false);
    let voice_type = use_state(|| "female".to_string());
    let speech_speed = use_state(|| 1.0f32);
    let active_tab = use_state(|| "translate".to_string());

    // 处理文本翻译
    let on_translate_text = {
        let source_text = source_text.clone();
        let translated_text = translated_text.clone();
        let is_translating = is_translating.clone();
        let source_language = source_language.clone();
        let target_language = target_language.clone();
        
        Callback::from(move |_: MouseEvent| {
            if !source_text.is_empty() && !*is_translating {
                let source_text = source_text.clone();
                let translated_text = translated_text.clone();
                let is_translating = is_translating.clone();
                let source_lang = (*source_language).clone();
                let target_lang = (*target_language).clone();
                let text_to_translate = (*source_text).clone();
                
                is_translating.set(true);
                
                spawn_local(async move {
                    // 模拟翻译延迟
                    wasm_bindgen_futures::JsFuture::from(
                        js_sys::Promise::new(&mut |resolve, _| {
                            web_sys::window()
                                .unwrap()
                                .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 2000)
                                .unwrap();
                        })
                    ).await.unwrap();
                    
                    // 模拟翻译结果
                    let mock_translation = format!(
                        "[{}→{}] {}", 
                        source_lang, 
                        target_lang, 
                        text_to_translate
                    );
                    
                    translated_text.set(mock_translation);
                    is_translating.set(false);
                });
            }
        })
    };

    // 处理语音合成
    let on_synthesize_speech = {
        let source_text = source_text.clone();
        let translated_text = translated_text.clone();
        let is_synthesizing = is_synthesizing.clone();
        let target_language = target_language.clone();
        let voice_type = voice_type.clone();
        let speech_speed = speech_speed.clone();
        
        Callback::from(move |_: MouseEvent| {
            let text_to_synthesize = if translated_text.is_empty() {
                (*source_text).clone()
            } else {
                (*translated_text).clone()
            };
            
            if !text_to_synthesize.is_empty() && !*is_synthesizing {
                let is_synthesizing = is_synthesizing.clone();
                let target_lang = (*target_language).clone();
                let voice = (*voice_type).clone();
                let speed = *speech_speed;
                
                is_synthesizing.set(true);
                
                spawn_local(async move {
                    // 模拟语音合成过程
                    wasm_bindgen_futures::JsFuture::from(
                        js_sys::Promise::new(&mut |resolve, _| {
                            web_sys::window()
                                .unwrap()
                                .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 3000)
                                .unwrap();
                        })
                    ).await.unwrap();
                    
                    // 模拟合成完成
                    web_sys::console::log_1(&format!("语音合成完成: {} 语言, {} 音色, {}x 速度", target_lang, voice, speed).into());
                    is_synthesizing.set(false);
                });
            }
        })
    };

    // 处理视频翻译
    let on_translate_video = {
        let app_state = props.app_state.clone();
        
        Callback::from(move |_: MouseEvent| {
            if let AppState::Ready(video_path) = &*app_state {
                let app_state_clone = app_state.clone();
                let video_path = video_path.clone();
                
                app_state_clone.set(AppState::Translating(video_path.clone()));
                
                spawn_local(async move {
                    // 模拟视频翻译过程
                    wasm_bindgen_futures::JsFuture::from(
                        js_sys::Promise::new(&mut |resolve, _| {
                            web_sys::window()
                                .unwrap()
                                .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 5000)
                                .unwrap();
                        })
                    ).await.unwrap();
                    
                    app_state_clone.set(AppState::Completed(format!("translated_{}", video_path)));
                });
            }
        })
    };

    // 处理语言选择变化
    let on_source_language_change = {
        let source_language = source_language.clone();
        Callback::from(move |e: Event| {
            if let Some(target) = e.target() {
                if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                    source_language.set(select.value());
                }
            }
        })
    };

    let on_target_language_change = {
        let target_language = target_language.clone();
        Callback::from(move |e: Event| {
            if let Some(target) = e.target() {
                if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                    target_language.set(select.value());
                }
            }
        })
    };

    // 处理文本输入变化
    let on_source_text_change = {
        let source_text = source_text.clone();
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<HtmlTextAreaElement>();
            if let Some(textarea) = target {
                source_text.set(textarea.value());
            }
        })
    };

    // 处理音色选择变化
    let on_voice_type_change = {
        let voice_type = voice_type.clone();
        Callback::from(move |e: Event| {
            if let Some(target) = e.target() {
                if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                    voice_type.set(select.value());
                }
            }
        })
    };

    // 处理语速变化
    let on_speech_speed_change = {
        let speech_speed = speech_speed.clone();
        Callback::from(move |e: Event| {
            if let Some(target) = e.target() {
                if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                    if let Ok(speed) = input.value().parse::<f32>() {
                        speech_speed.set(speed);
                    }
                }
            }
        })
    };

    // 标签页切换
    let on_tab_change = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: String| {
            active_tab.set(tab);
        })
    };

    html! {
        <div class="translation-synthesis-panel">
            <div class="panel-header">
                <h2 class="panel-title">{"🎙️ 翻译与合成"}</h2>
            </div>
            
            <div class="panel-content">
                // 标签页导航
                <div class="panel-tabs">
                    <button 
                        class={classes!("tab-btn", (*active_tab == "translate").then(|| "active"))}
                        onclick={
                            let on_change = on_tab_change.clone();
                            Callback::from(move |_: MouseEvent| on_change.emit("translate".to_string()))
                        }
                    >
                        <span>{"🌐"}</span>
                        <span>{"翻译"}</span>
                    </button>
                    <button 
                        class={classes!("tab-btn", (*active_tab == "synthesis").then(|| "active"))}
                        onclick={
                            let on_change = on_tab_change.clone();
                            Callback::from(move |_: MouseEvent| on_change.emit("synthesis".to_string()))
                        }
                    >
                        <span>{"🎵"}</span>
                        <span>{"合成"}</span>
                    </button>
                </div>

                // 语言选择区域
                <div class="language-selection">
                    <div class="language-group">
                        <label class="form-label">{"源语言"}</label>
                        <select 
                            class="form-select"
                            value={(*source_language).clone()}
                            onchange={on_source_language_change}
                        >
                            {for SUPPORTED_LANGUAGES.iter().map(|(code, name)| {
                                html! {
                                    <option value={*code} selected={*code == *source_language}>
                                        {name}
                                    </option>
                                }
                            })}
                        </select>
                    </div>
                    
                    <div class="language-arrow">{"→"}</div>
                    
                    <div class="language-group">
                        <label class="form-label">{"目标语言"}</label>
                        <select 
                            class="form-select"
                            value={(*target_language).clone()}
                            onchange={on_target_language_change}
                        >
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

                // 内容区域
                {
                    match (*active_tab).as_str() {
                        "translate" => html! {
                            <div class="translate-content">
                                <div class="text-translation-section">
                                    <h3 class="section-title">{"📝 台词翻译"}</h3>
                                    
                                    <div class="translation-editor-compact">
                                        <div class="editor-group">
                                            <label class="editor-label">{"原文"}</label>
                                            <textarea
                                                class="translation-textarea source"
                                                placeholder="选择时间线上的台词进行翻译..."
                                                value={(*source_text).clone()}
                                                onchange={on_source_text_change}
                                                disabled={*is_translating}
                                                rows="3"
                                            />
                                        </div>
                                        
                                        <div class="translate-action">
                                            <button 
                                                class={classes!("translate-btn-compact", (*is_translating).then(|| "loading"))}
                                                onclick={on_translate_text}
                                                disabled={source_text.is_empty() || *is_translating}
                                            >
                                                if *is_translating {
                                                    <span class="loading-spinner-small"></span>
                                                    <span>{"翻译中"}</span>
                                                } else {
                                                    <span>{"🔄"}</span>
                                                    <span>{"翻译"}</span>
                                                }
                                            </button>
                                        </div>
                                        
                                        <div class="editor-group">
                                            <label class="editor-label">{"译文"}</label>
                                            <textarea
                                                class="translation-textarea target"
                                                placeholder="翻译结果..."
                                                value={(*translated_text).clone()}
                                                readonly=true
                                                rows="3"
                                            />
                                        </div>
                                    </div>
                                </div>
                            </div>
                        },
                        "synthesis" => html! {
                            <div class="synthesis-content">
                                <div class="voice-synthesis-section">
                                    <h3 class="section-title">{"🎵 语音合成"}</h3>
                                    
                                    <div class="synthesis-controls">
                                        <div class="control-group">
                                            <label class="form-label">{"音色选择"}</label>
                                            <select 
                                                class="form-select"
                                                value={(*voice_type).clone()}
                                                onchange={on_voice_type_change}
                                            >
                                                <option value="female">{"女声（温柔）"}</option>
                                                <option value="male">{"男声（沉稳）"}</option>
                                                <option value="child">{"童声（活泼）"}</option>
                                                <option value="elderly">{"老年（慈祥）"}</option>
                                            </select>
                                        </div>
                                        
                                        <div class="control-group">
                                            <label class="form-label">{format!("语速: {}x", *speech_speed)}</label>
                                            <input 
                                                type="range"
                                                class="speed-slider"
                                                min="0.5"
                                                max="2.0"
                                                step="0.1"
                                                value={speech_speed.to_string()}
                                                onchange={on_speech_speed_change}
                                            />
                                            <div class="speed-labels">
                                                <span>{"慢"}</span>
                                                <span>{"正常"}</span>
                                                <span>{"快"}</span>
                                            </div>
                                        </div>
                                        
                                                                <div class="synthesis-preview">
                            <label class="form-label">{"合成文本"}</label>
                            <div class="preview-text">
                                {
                                    if translated_text.is_empty() {
                                        if source_text.is_empty() {
                                            "请输入或翻译文本后进行语音合成".to_string()
                                        } else {
                                            (*source_text).clone()
                                        }
                                    } else {
                                        (*translated_text).clone()
                                    }
                                }
                            </div>
                        </div>
                                        
                                        <div class="synthesis-action">
                                                                        <button 
                                class={classes!("synthesis-btn", (*is_synthesizing).then(|| "loading"))}
                                onclick={on_synthesize_speech}
                                disabled={source_text.is_empty() && translated_text.is_empty() || *is_synthesizing}
                            >
                                                if *is_synthesizing {
                                                    <span class="loading-spinner-small"></span>
                                                    <span>{"合成中..."}</span>
                                                } else {
                                                    <span>{"🎤"}</span>
                                                    <span>{"生成语音"}</span>
                                                }
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        },
                        _ => html! { <div>{"未知标签页"}</div> }
                    }
                }

                // 视频翻译区域
                <div class="video-translation-section">
                    <h3 class="section-title">{"🎬 视频翻译"}</h3>
                    
                    {match &*props.app_state {
                        AppState::Idle => html! {
                            <div class="status-message">
                                <span class="status-icon">{"💡"}</span>
                                <p>{"请先在素材库中上传视频文件"}</p>
                            </div>
                        },
                        AppState::Uploading => html! {
                            <div class="status-message uploading">
                                <div class="progress-indicator"></div>
                                <p>{"正在上传视频文件..."}</p>
                            </div>
                        },
                        AppState::Ready(path) => html! {
                            <div class="video-ready">
                                <div class="status-message success">
                                    <span class="status-icon">{"✅"}</span>
                                    <p>{format!("视频已准备就绪: {}", path)}</p>
                                </div>
                                
                                <button 
                                    class="btn btn-primary"
                                    onclick={on_translate_video}
                                >
                                    <span>{"🚀"}</span>
                                    <span>{"开始视频翻译"}</span>
                                </button>
                            </div>
                        },
                        AppState::Translating(task_id) => html! {
                            <div class="status-message processing">
                                <div class="progress-indicator"></div>
                                <p>{format!("正在翻译视频... 任务ID: {}", task_id)}</p>
                            </div>
                        },
                        AppState::Completed(output_path) => html! {
                            <div class="translation-result">
                                <div class="status-message success">
                                    <span class="status-icon">{"🎉"}</span>
                                    <h4>{"翻译完成！"}</h4>
                                    <p>{format!("输出文件: {}", output_path)}</p>
                                </div>
                                <div class="result-actions">
                                    <button class="btn btn-secondary">
                                        <span>{"📁"}</span>
                                        <span>{"打开文件夹"}</span>
                                    </button>
                                    <button class="btn btn-primary">
                                        <span>{"▶️"}</span>
                                        <span>{"播放视频"}</span>
                                    </button>
                                </div>
                            </div>
                        },
                        AppState::Error(error_msg) => html! {
                            <div class="status-message error">
                                <span class="status-icon">{"❌"}</span>
                                <h4>{"翻译失败"}</h4>
                                <p>{error_msg}</p>
                                <button class="btn btn-secondary" onclick={
                                    let app_state = props.app_state.clone();
                                    Callback::from(move |_: MouseEvent| {
                                        app_state.set(AppState::Idle);
                                    })
                                }>
                                    {"重试"}
                                </button>
                            </div>
                        },
                        _ => html! {
                            <div class="status-message">
                                <p>{"当前状态不支持视频翻译操作"}</p>
                            </div>
                        }
                    }}
                </div>
            </div>
        </div>
    }
} 