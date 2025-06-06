use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::api::tauri_api;

#[derive(Properties, PartialEq)]
pub struct SettingsPanelProps {
    pub on_close: Callback<()>,
}

#[function_component(SettingsPanel)]
pub fn settings_panel(props: &SettingsPanelProps) -> Html {
    let config_state = use_state(|| None::<serde_json::Value>);
    let loading = use_state(|| false);
    let error_message = use_state(|| None::<String>);

    // 加载配置
    {
        let config_state = config_state.clone();
        let loading = loading.clone();
        let error_message = error_message.clone();
        
        use_effect_with((), move |_| {
            spawn_local(async move {
                loading.set(true);
                match tauri_api::get_app_config().await {
                    Ok(config) => {
                        config_state.set(Some(config));
                        error_message.set(None);
                    }
                    Err(e) => {
                        error_message.set(Some(format!("加载配置失败: {}", e)));
                    }
                }
                loading.set(false);
            });
            || ()
        });
    }

    let on_close = props.on_close.clone();
    let close_handler = Callback::from(move |_| {
        on_close.emit(());
    });

    html! {
        <div class="settings-panel">
            <div class="settings-header">
                <h2>{"应用设置"}</h2>
                <button class="close-btn" onclick={close_handler}>
                    {"✕"}
                </button>
            </div>
            
            <div class="settings-content">
                if *loading {
                    <div class="loading">
                        <div class="spinner"></div>
                        <p>{"加载配置中..."}</p>
                    </div>
                } else if let Some(error) = error_message.as_ref() {
                    <div class="error-message">
                        <p>{error}</p>
                    </div>
                } else if let Some(_config) = config_state.as_ref() {
                    <div class="settings-sections">
                        <div class="setting-section">
                            <h3>{"AI 模型设置"}</h3>
                            <div class="setting-item">
                                <label>{"默认语言:"}</label>
                                <select>
                                    <option value="zh">{"中文"}</option>
                                    <option value="en">{"英文"}</option>
                                    <option value="ja">{"日文"}</option>
                                    <option value="ko">{"韩文"}</option>
                                </select>
                            </div>
                            <div class="setting-item">
                                <label>{"Whisper 模型路径:"}</label>
                                <input type="text" placeholder="选择模型文件..." />
                                <button class="browse-btn">{"浏览"}</button>
                            </div>
                        </div>

                        <div class="setting-section">
                            <h3>{"视频处理设置"}</h3>
                            <div class="setting-item">
                                <label>{"默认输出格式:"}</label>
                                <select>
                                    <option value="mp4">{"MP4"}</option>
                                    <option value="avi">{"AVI"}</option>
                                    <option value="mov">{"MOV"}</option>
                                    <option value="mkv">{"MKV"}</option>
                                </select>
                            </div>
                            <div class="setting-item">
                                <label>{"默认视频质量:"}</label>
                                <select>
                                    <option value="low">{"低质量"}</option>
                                    <option value="medium">{"中等质量"}</option>
                                    <option value="high">{"高质量"}</option>
                                    <option value="ultra">{"超高质量"}</option>
                                </select>
                            </div>
                            <div class="setting-item">
                                <label>{"最大文件大小 (MB):"}</label>
                                <input type="number" min="1" max="10240" value="1024" />
                            </div>
                        </div>

                        <div class="setting-section">
                            <h3>{"文档处理设置"}</h3>
                            <div class="setting-item">
                                <label>{"PDF DPI 设置:"}</label>
                                <input type="number" min="72" max="600" value="300" />
                            </div>
                            <div class="setting-item">
                                <label>{"最大文档大小 (MB):"}</label>
                                <input type="number" min="1" max="1024" value="100" />
                            </div>
                        </div>

                        <div class="setting-section">
                            <h3>{"用户界面设置"}</h3>
                            <div class="setting-item">
                                <label>{"主题:"}</label>
                                <select>
                                    <option value="light">{"浅色主题"}</option>
                                    <option value="dark">{"深色主题"}</option>
                                    <option value="auto">{"跟随系统"}</option>
                                </select>
                            </div>
                            <div class="setting-item">
                                <label>{"界面语言:"}</label>
                                <select>
                                    <option value="zh">{"中文"}</option>
                                    <option value="en">{"English"}</option>
                                </select>
                            </div>
                        </div>
                    </div>

                    <div class="settings-actions">
                        <button class="save-btn">{"保存设置"}</button>
                        <button class="reset-btn">{"重置为默认"}</button>
                    </div>
                } else {
                    <div class="no-config">
                        <p>{"无法加载配置"}</p>
                    </div>
                }
            </div>
        </div>
    }
} 