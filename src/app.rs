use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use web_sys::{File, FileList, HtmlInputElement};
use std::rc::Rc;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// 定义用于上传视频文件的参数结构
#[derive(Serialize, Deserialize)]
struct UploadVideoArgs {
    path: String,
}

// 定义用于翻译视频的参数结构
#[derive(Serialize, Deserialize)]
struct TranslateVideoArgs {
    video_path: String,
    source_language: String,
    target_language: String,
}

// 翻译任务状态结构
#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct TranslationTask {
    id: String,
    video_path: String,
    status: String, // "Pending", "Processing", "Completed", "Failed"
    source_language: String,
    target_language: String,
    output_path: Option<String>,
    error_message: Option<String>,
}

// 定义应用状态
#[derive(Clone, PartialEq)]
enum AppState {
    Idle,
    Uploading,
    Ready(String), // 包含上传的视频路径
    Translating(String), // 包含任务ID
    Completed(String), // 包含翻译后的视频路径
    Error(String),     // 包含错误信息
}

#[function_component(App)]
pub fn app() -> Html {
    // 状态管理
    let app_state = use_state(|| AppState::Idle);
    let source_language = use_state(|| String::from("zh")); // 默认源语言为中文
    let target_language = use_state(|| String::from("en")); // 默认目标语言为英文
    let file_input_ref = use_node_ref();
    let video_ref = use_node_ref();

    // 定义任务状态检查效果
    {
        let app_state = app_state.clone();
        
        use_effect_with_deps(move |state| {
            if let AppState::Translating(task_id) = (**state).clone() {
                let task_id_clone = task_id.clone();
                let app_state_clone = app_state.clone();
                
                // 设置定时检查任务状态
                let interval_handle = Rc::new(std::cell::RefCell::new(None));
                let interval_handle_clone = interval_handle.clone();
                
                *interval_handle.borrow_mut() = Some(Interval::new(1000, move || {
                    let task_id = task_id_clone.clone();
                    let app_state = app_state_clone.clone();
                    let interval_handle = interval_handle_clone.clone();
                    
                    spawn_local(async move {
                        let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                            "task_id": task_id
                        })).unwrap();
                        
                        // 调用后端API获取任务状态
                        let result = invoke("check_task_output", args).await;
                        
                        if let Some(output_path) = result.as_string() {
                            // 任务完成，清除定时器并更新状态
                            *interval_handle.borrow_mut() = None;
                            app_state.set(AppState::Completed(output_path));
                        } else {
                            // 获取详细的任务状态
                            let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                                "task_id": task_id
                            })).unwrap();
                            
                            match invoke("get_translation_task", args).await.as_string() {
                                Some(task_json) => {
                                    // 尝试解析任务状态
                                    if let Ok(task) = serde_json::from_str::<TranslationTask>(&task_json) {
                                        if task.status == "Failed" {
                                            // 任务失败，清除定时器并显示错误
                                            *interval_handle.borrow_mut() = None;
                                            let error_msg = task.error_message.unwrap_or_else(|| "未知错误".to_string());
                                            app_state.set(AppState::Error(error_msg));
                                        } else if task.status == "Completed" && task.output_path.is_some() {
                                            // 任务完成，清除定时器并更新状态
                                            *interval_handle.borrow_mut() = None;
                                            app_state.set(AppState::Completed(task.output_path.unwrap()));
                                        }
                                        // 如果状态是Pending或Processing，继续等待
                                    }
                                },
                                None => {
                                    // API调用失败，继续等待
                                }
                            }
                        }
                    });
                }));
                
                // 清理函数，当组件卸载或依赖项变化时调用
                return move || {
                    *interval_handle.borrow_mut() = None;
                };
            }
            
            // 对于其他状态，不做任何事情
            || {}
        }, app_state.clone());
    }

    // 处理视频文件选择
    let on_video_select = {
        let app_state = app_state.clone();
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
        let source_language = source_language.clone();
        Callback::from(move |e: Event| {
            let select: HtmlInputElement = e.target_unchecked_into();
            source_language.set(select.value());
        })
    };

    let on_target_language_change = {
        let target_language = target_language.clone();
        Callback::from(move |e: Event| {
            let select: HtmlInputElement = e.target_unchecked_into();
            target_language.set(select.value());
        })
    };

    // 处理翻译请求
    let on_translate = {
        let app_state = app_state.clone();
        let source_language = source_language.clone();
        let target_language = target_language.clone();
        
        Callback::from(move |_: MouseEvent| {
            if let AppState::Ready(video_path) = (*app_state).clone() {
                let source_lang = (*source_language).clone();
                let target_lang = (*target_language).clone();
                let app_state_clone = app_state.clone();
                let video_path_clone = video_path.clone();
                
                spawn_local(async move {
                    let args = serde_wasm_bindgen::to_value(&TranslateVideoArgs {
                        video_path: video_path_clone,
                        source_language: source_lang,
                        target_language: target_lang,
                    }).unwrap();
                    
                    match invoke("translate_video", args).await.as_string() {
                        Some(task_id) => {
                            app_state_clone.set(AppState::Translating(task_id));
                        },
                        None => {
                            app_state_clone.set(AppState::Error("开始视频翻译失败".to_string()));
                        }
                    }
                });
            }
        })
    };

    // 渲染界面
    html! {
        <main class="container">
            <div class="header">
                <h1>{"视频翻译工具"}</h1>
                <p>{"上传视频并将其翻译成其他语言，保留原始音色和情感"}</p>
            </div>

            <div class="video-upload-section">
                <input 
                    type="file" 
                    ref={file_input_ref.clone()} 
                    accept="video/*" 
                    onchange={on_video_select}
                    disabled={matches!((*app_state), AppState::Uploading | AppState::Translating(_))}
                />
                <div class="language-selection">
                    <div class="language-group">
                        <label for="source-language">{"源语言："}</label>
                        <select id="source-language" onchange={on_source_language_change}>
                            <option value="zh" selected={*source_language == "zh"}>{"中文"}</option>
                            <option value="en" selected={*source_language == "en"}>{"英文"}</option>
                            <option value="ja" selected={*source_language == "ja"}>{"日语"}</option>
                            <option value="ko" selected={*source_language == "ko"}>{"韩语"}</option>
                        </select>
                    </div>
                    
                    <div class="language-group">
                        <label for="target-language">{"目标语言："}</label>
                        <select id="target-language" onchange={on_target_language_change}>
                            <option value="zh" selected={*target_language == "zh"}>{"中文"}</option>
                            <option value="en" selected={*target_language == "en"}>{"英文"}</option>
                            <option value="ja" selected={*target_language == "ja"}>{"日语"}</option>
                            <option value="ko" selected={*target_language == "ko"}>{"韩语"}</option>
                        </select>
                    </div>
                </div>
            </div>

            <div class="action-section">
                {
                    match (*app_state).clone() {
                        AppState::Idle => html! {
                            <div class="message">{"请选择一个视频文件进行翻译"}</div>
                        },
                        AppState::Uploading => html! {
                            <div class="message">{"正在上传视频..."}</div>
                        },
                        AppState::Ready(path) => html! {
                            <div class="ready-state">
                                <div class="message">{"视频已上传："}{&path}</div>
                                <button onclick={on_translate}>{"开始翻译"}</button>
                            </div>
                        },
                        AppState::Translating(_) => html! {
                            <div class="message">
                                <div>{"正在翻译视频，这可能需要一些时间..."}</div>
                                <div class="progress-indicator"></div>
                            </div>
                        },
                        AppState::Completed(output_path) => html! {
                            <div class="completed-state">
                                <div class="message success">{"翻译完成！"}</div>
                                <div>
                                    <video 
                                        ref={video_ref.clone()} 
                                        controls=true 
                                        width="640"
                                        height="360"
                                        src={format!("asset://{}", output_path)}
                                    ></video>
                                </div>
                                <div class="output-path">{"输出文件路径："}{&output_path}</div>
                            </div>
                        },
                        AppState::Error(message) => html! {
                            <div class="message error">{"错误："}{message}</div>
                        }
                    }
                }
            </div>
        </main>
    }
}

// 定义一个简单的轮询间隔计时器
struct Interval {
    callback: Closure<dyn FnMut()>,
    handle: i32,
}

impl Interval {
    fn new<F: 'static>(millis: u32, f: F) -> Self
    where
        F: FnMut(),
    {
        let callback = Closure::wrap(Box::new(f) as Box<dyn FnMut()>);
        let handle = web_sys::window()
            .unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                millis as i32,
            )
            .unwrap();

        Interval { callback, handle }
    }
}

impl Drop for Interval {
    fn drop(&mut self) {
        web_sys::window()
            .unwrap()
            .clear_interval_with_handle(self.handle);
    }
}
