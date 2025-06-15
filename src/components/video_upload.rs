// src/components/video_upload.rs

use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen_futures::spawn_local;
// 移除gloo_timers导入，使用JavaScript setTimeout

use crate::types::AppState;

#[derive(Properties, PartialEq)]
pub struct VideoUploadProps {
    pub app_state: UseStateHandle<AppState>,
    pub source_language: UseStateHandle<String>,
    pub target_language: UseStateHandle<String>,
}

#[function_component(VideoUploadComponent)]
pub fn video_upload_component(props: &VideoUploadProps) -> Html {
    let file_input_ref = use_node_ref();
    let selected_file = use_state(|| None::<String>);
    let is_uploading = use_state(|| false);

    // 处理文件选择
    let on_file_change = {
        let file_input_ref = file_input_ref.clone();
        let selected_file = selected_file.clone();
        let app_state = props.app_state.clone();
        let is_uploading = is_uploading.clone();

        Callback::from(move |_: Event| {
            let file_input = file_input_ref.cast::<HtmlInputElement>();
            if let Some(input) = file_input {
                if let Some(files) = input.files() {
                    if let Some(file) = files.get(0) {
                        let file_name = file.name();
                        selected_file.set(Some(file_name.clone()));
                        
                        // 模拟上传过程
                        let app_state = app_state.clone();
                        let is_uploading = is_uploading.clone();
                        
                        is_uploading.set(true);
                        app_state.set(AppState::Uploading);
                        
                        spawn_local(async move {
                            // 模拟上传延迟
                            wasm_bindgen_futures::JsFuture::from(
                                js_sys::Promise::new(&mut |resolve, _| {
                                    web_sys::window()
                                        .unwrap()
                                        .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 2000)
                                        .unwrap();
                                })
                            ).await.unwrap();
                            
                            is_uploading.set(false);
                            app_state.set(AppState::Ready(file_name));
                        });
                    }
                }
            }
        })
    };

    // 处理语言选择变化
    let on_source_language_change = {
        let source_language = props.source_language.clone();
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = target {
                source_language.set(input.value());
            }
        })
    };

    let on_target_language_change = {
        let target_language = props.target_language.clone();
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = target {
                target_language.set(input.value());
            }
        })
    };

    // 开始翻译
    let on_start_translation = {
        let app_state = props.app_state.clone();
        Callback::from(move |_: MouseEvent| {
            if let AppState::Ready(file_name) = &*app_state {
                let file_name = file_name.clone();
                let app_state = app_state.clone();
                
                app_state.set(AppState::Translating(file_name.clone()));
                
                spawn_local(async move {
                    // 模拟翻译过程
                    wasm_bindgen_futures::JsFuture::from(
                        js_sys::Promise::new(&mut |resolve, _| {
                            web_sys::window()
                                .unwrap()
                                .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 5000)
                                .unwrap();
                        })
                    ).await.unwrap();
                    app_state.set(AppState::Completed(format!("translated_{}", file_name)));
                });
            }
        })
    };

    html! {
<<<<<<< HEAD
        <div class="card">
            <div class="card-header">
                <h2 class="card-title">{"📹 视频上传"}</h2>
            </div>
            <div class="card-content">
                <div class="video-upload-section">
                    // 文件选择区域
                    <div class="upload-area">
                        <input
                            ref={file_input_ref}
                            type="file"
                            accept="video/*"
                            onchange={on_file_change}
                            disabled={*is_uploading}
                            class="file-input"
                        />
                        <div class="upload-hint">
                            <span class="upload-icon">{"📁"}</span>
                            <p class="upload-text">
                                if selected_file.is_some() {
                                    { format!("已选择: {}", selected_file.as_ref().unwrap()) }
                                } else {
                                    { "点击选择视频文件或拖拽到此处" }
                                }
                            </p>
                            <p class="upload-subtext">{"支持 MP4, AVI, MOV, MKV 等格式"}</p>
                        </div>
                    </div>

                    // 语言选择
                    <div class="language-selection">
                        <div class="language-group">
                            <label class="form-label">{"源语言"}</label>
                            <select 
                                class="form-select"
                                value={(*props.source_language).clone()}
                                onchange={on_source_language_change}
                            >
                                <option value="zh">{"中文"}</option>
                                <option value="en">{"English"}</option>
                                <option value="ja">{"日本語"}</option>
                                <option value="ko">{"한국어"}</option>
                                <option value="es">{"Español"}</option>
                                <option value="fr">{"Français"}</option>
                                <option value="de">{"Deutsch"}</option>
                                <option value="ru">{"Русский"}</option>
=======
        <div class="video-upload-section">
            <h2>
                <span class="section-icon">{"🎬"}</span>
                {"视频上传"}
            </h2>
            <p class="section-description">
                {"支持 MP4、AVI、MOV 等常见视频格式，文件大小不超过 2GB"}
            </p>
            
            if is_uploading {
                <LoadingSpinner message={"正在处理视频文件...".to_string()} />
            } else {
                <div class="upload-controls">
                    <div class="file-upload-area">
                        <input 
                            type="file" 
                            ref={file_input_ref.clone()} 
                            accept="video/*" 
                            onchange={on_video_select}
                            disabled={is_uploading}
                            id="video-file-input"
                        />
                        <label for="video-file-input" class="file-upload-label">
                            <div class="upload-icon">{"📁"}</div>
                            <div class="upload-text">
                                <span class="upload-primary">{"点击选择视频文件"}</span>
                                <span class="upload-secondary">{"或拖拽文件到此区域"}</span>
                            </div>
                        </label>
                    </div>
                    
                    <div class="language-selection">
                        <div class="language-group">
                            <label for="source-language">
                                <span class="label-icon">{"🗣️"}</span>
                                {"源语言"}
                            </label>
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
>>>>>>> 55e27c8 (重构样式文件，添加全局样式和组件样式，优化视频上传和翻译面板的用户界面，更新 API 以支持应用配置和 AI 模型状态，清理未使用的代码，增强响应式设计和动画效果。)
                            </select>
                        </div>
                        
                        <div class="language-arrow">{"→"}</div>
                        
                        <div class="language-group">
<<<<<<< HEAD
                            <label class="form-label">{"目标语言"}</label>
                            <select 
                                class="form-select"
                                value={(*props.target_language).clone()}
                                onchange={on_target_language_change}
                            >
                                <option value="en">{"English"}</option>
                                <option value="zh">{"中文"}</option>
                                <option value="ja">{"日本語"}</option>
                                <option value="ko">{"한국어"}</option>
                                <option value="es">{"Español"}</option>
                                <option value="fr">{"Français"}</option>
                                <option value="de">{"Deutsch"}</option>
                                <option value="ru">{"Русский"}</option>
=======
                            <label for="target-language">
                                <span class="label-icon">{"🎯"}</span>
                                {"目标语言"}
                            </label>
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
>>>>>>> 55e27c8 (重构样式文件，添加全局样式和组件样式，优化视频上传和翻译面板的用户界面，更新 API 以支持应用配置和 AI 模型状态，清理未使用的代码，增强响应式设计和动画效果。)
                            </select>
                        </div>
                    </div>

<<<<<<< HEAD
                    // 状态显示和操作按钮
                    <div class="upload-status">
                        {
                            match &*props.app_state {
                                AppState::Idle => html! {
                                    <div class="status-message">
                                        <span class="status-icon">{"💡"}</span>
                                        <p>{"请选择要处理的视频文件"}</p>
                                    </div>
                                },
                                AppState::Uploading => html! {
                                    <div class="status-message uploading">
                                        <div class="progress-indicator"></div>
                                        <p>{"正在上传视频文件..."}</p>
                                    </div>
                                },
                                AppState::Ready(filename) => html! {
                                    <div class="status-message success">
                                        <span class="status-icon">{"✅"}</span>
                                        <p>{format!("视频 \"{}\" 已准备就绪", filename)}</p>
                                        <button 
                                            class="btn btn-primary"
                                            onclick={on_start_translation}
                                        >
                                            <span>{"🚀"}</span>
                                            <span>{"开始翻译"}</span>
                                        </button>
                                    </div>
                                },
                                AppState::Translating(filename) => html! {
                                    <div class="status-message processing">
                                        <div class="progress-indicator"></div>
                                        <p>{format!("正在翻译视频 \"{}\"...", filename)}</p>
                                        <p class="processing-hint">{"这可能需要几分钟时间，请耐心等待"}</p>
                                    </div>
                                },
                                AppState::Completed(output_path) => html! {
                                    <div class="status-message completed">
                                        <span class="status-icon">{"🎉"}</span>
                                        <p>{"翻译完成！"}</p>
                                        <div class="output-info">
                                            <p class="output-label">{"输出文件："}</p>
                                            <p class="output-path">{output_path}</p>
                                        </div>
                                        <div class="action-buttons">
                                            <button class="btn btn-secondary">
                                                <span>{"📁"}</span>
                                                <span>{"打开文件夹"}</span>
                                            </button>
                                            <button class="btn btn-primary">
                                                <span>{"▶️"}</span>
                                                <span>{"预览视频"}</span>
                                            </button>
                                        </div>
                                    </div>
                                },
                                AppState::Error(error) => html! {
                                    <div class="status-message error">
                                        <span class="status-icon">{"❌"}</span>
                                        <p>{"处理失败"}</p>
                                        <p class="error-details">{error}</p>
                                        <button class="btn btn-secondary">
                                            <span>{"🔄"}</span>
                                            <span>{"重试"}</span>
                                        </button>
                                    </div>
                                },
                                _ => html! {
                                    <div class="status-message">
                                        <span class="status-icon">{"ℹ️"}</span>
                                        <p>{"准备中..."}</p>
                                    </div>
                                }
                            }
                        }
                    </div>
=======
                    // 显示当前选择的文件信息
                    {match &*props.app_state {
                        AppState::Ready(path) => html! {
                            <div class="file-info">
                                <div class="file-info-icon">{"✅"}</div>
                                <div class="file-info-content">
                                    <div class="file-info-title">{"视频文件已准备就绪"}</div>
                                    <div class="file-info-path">{path}</div>
                                </div>
                            </div>
                        },
                        AppState::Error(error) => html! {
                            <div class="file-error">
                                <div class="file-error-icon">{"❌"}</div>
                                <div class="file-error-content">
                                    <div class="file-error-title">{"上传失败"}</div>
                                    <div class="file-error-message">{error}</div>
                                </div>
                            </div>
                        },
                        _ => html! {}
                    }}
>>>>>>> 55e27c8 (重构样式文件，添加全局样式和组件样式，优化视频上传和翻译面板的用户界面，更新 API 以支持应用配置和 AI 模型状态，清理未使用的代码，增强响应式设计和动画效果。)
                </div>
            </div>
        </div>
    }
} 