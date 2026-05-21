use crate::api::tauri_api;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SettingsPanelProps {
    pub on_close: Callback<()>,
}

fn tts_engine_id(config: &serde_json::Value) -> String {
    config
        .pointer("/ai/selected_tts_engine")
        .and_then(|value| value.as_str())
        .unwrap_or("piper")
        .to_string()
}

#[function_component(SettingsPanel)]
pub fn settings_panel(props: &SettingsPanelProps) -> Html {
    let config_state = use_state(|| None::<serde_json::Value>);
    let selected_tts_engine = use_state(|| "piper".to_string());
    let loading = use_state(|| false);
    let saving = use_state(|| false);
    let error_message = use_state(|| None::<String>);
    let success_message = use_state(|| None::<String>);

    {
        let config_state = config_state.clone();
        let selected_tts_engine = selected_tts_engine.clone();
        let loading = loading.clone();
        let error_message = error_message.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                loading.set(true);
                match tauri_api::get_app_config().await {
                    Ok(config) => {
                        selected_tts_engine.set(tts_engine_id(&config));
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

    let close_handler = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    let on_tts_engine_change = {
        let selected_tts_engine = selected_tts_engine.clone();
        let success_message = success_message.clone();
        Callback::from(move |event: Event| {
            if let Some(target) = event.target() {
                if let Ok(select) = target.dyn_into::<HtmlSelectElement>() {
                    selected_tts_engine.set(select.value());
                    success_message.set(None);
                }
            }
        })
    };

    let save_handler = {
        let config_state = config_state.clone();
        let selected_tts_engine = selected_tts_engine.clone();
        let saving = saving.clone();
        let error_message = error_message.clone();
        let success_message = success_message.clone();

        Callback::from(move |_| {
            let Some(mut config) = (*config_state).clone() else {
                error_message.set(Some("当前没有可保存的配置".to_string()));
                return;
            };

            if let Some(ai_config) = config.get_mut("ai").and_then(|value| value.as_object_mut()) {
                ai_config.insert(
                    "selected_tts_engine".to_string(),
                    serde_json::Value::String((*selected_tts_engine).clone()),
                );
            }

            let config_state = config_state.clone();
            let saving = saving.clone();
            let error_message = error_message.clone();
            let success_message = success_message.clone();

            spawn_local(async move {
                saving.set(true);
                match tauri_api::update_app_config(config.clone()).await {
                    Ok(()) => {
                        config_state.set(Some(config));
                        error_message.set(None);
                        success_message.set(Some("设置已保存".to_string()));
                    }
                    Err(e) => {
                        error_message.set(Some(format!("保存配置失败: {}", e)));
                    }
                }
                saving.set(false);
            });
        })
    };

    let render_tts_options = |config: &serde_json::Value| -> Vec<Html> {
        config
            .pointer("/ai/available_tts_engines")
            .and_then(|value| value.as_array())
            .map(|engines| {
                engines
                    .iter()
                    .filter_map(|engine| {
                        let id = engine.get("id")?.as_str()?.to_string();
                        let name = engine.get("name")?.as_str().unwrap_or(&id).to_string();
                        let footprint = engine
                            .get("footprint")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        Some(html! {
                            <option value={id.clone()} selected={id == *selected_tts_engine}>
                                {format!("{} - {}", name, footprint)}
                            </option>
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    };

    let render_selected_tts_detail = |config: &serde_json::Value| -> Html {
        let selected = (*selected_tts_engine).clone();
        let engine = config
            .pointer("/ai/available_tts_engines")
            .and_then(|value| value.as_array())
            .and_then(|engines| {
                engines.iter().find(|engine| {
                    engine.get("id").and_then(|value| value.as_str()) == Some(selected.as_str())
                })
            });

        if let Some(engine) = engine {
            let category = engine
                .get("category")
                .and_then(|v| v.as_str())
                .unwrap_or("-");
            let license = engine
                .get("license")
                .and_then(|v| v.as_str())
                .unwrap_or("-");
            let recommended_use = engine
                .get("recommended_use")
                .and_then(|v| v.as_str())
                .unwrap_or("-");
            let repository_url = engine
                .get("repository_url")
                .and_then(|v| v.as_str())
                .unwrap_or("-");

            html! {
                <div class="setting-hint">
                    <p>{format!("类型：{}，许可证：{}", category, license)}</p>
                    <p>{format!("适用：{}", recommended_use)}</p>
                    <p>{format!("仓库：{}", repository_url)}</p>
                </div>
            }
        } else {
            html! { <div class="setting-hint"><p>{"未找到当前TTS引擎详情"}</p></div> }
        }
    };

    html! {
        <div class="modal-overlay">
            <div class="settings-panel modal">
                <div class="settings-header modal-header">
                    <h2 class="modal-title">{"应用设置"}</h2>
                    <button class="modal-close" onclick={close_handler}>{"✕"}</button>
                </div>

                <div class="settings-content modal-content">
                    if *loading {
                        <div class="loading">
                            <div class="spinner"></div>
                            <p>{"加载配置中..."}</p>
                        </div>
                    } else if let Some(config) = config_state.as_ref() {
                        <div class="settings-sections">
                            if let Some(error) = error_message.as_ref() {
                                <div class="error-message"><p>{error}</p></div>
                            }
                            if let Some(success) = success_message.as_ref() {
                                <div class="success-message"><p>{success}</p></div>
                            }

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
                                    <label>{"TTS 引擎:"}</label>
                                    <select onchange={on_tts_engine_change}>
                                        {for render_tts_options(config)}
                                    </select>
                                </div>
                                {render_selected_tts_detail(config)}
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
                        </div>

                        <div class="settings-actions modal-footer">
                            <button class="save-btn btn btn-primary" onclick={save_handler} disabled={*saving}>
                                {if *saving { "保存中..." } else { "保存设置" }}
                            </button>
                        </div>
                    } else if let Some(error) = error_message.as_ref() {
                        <div class="error-message"><p>{error}</p></div>
                    } else {
                        <div class="no-config"><p>{"无法加载配置"}</p></div>
                    }
                </div>
            </div>
        </div>
    }
}
