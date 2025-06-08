use yew::prelude::*;
use web_sys::window;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

// 导入组件和类型
use crate::components::{
    VideoUploadComponent, 
    DocumentImportComponent, 
    TranslationPanelComponent, 
    ProjectTimelineComponent,
};
use crate::types::AppState;

#[function_component(App)]
pub fn app() -> Html {
    let app_state = use_state(|| AppState::Idle);
    let source_language = use_state(|| "zh".to_string());
    let target_language = use_state(|| "en".to_string());
    let is_mobile = use_state(|| false);
    let selected_material_tab = use_state(|| "video".to_string());

    // 检测移动设备
    {
        let is_mobile = is_mobile.clone();
        use_effect_with((), move |_| {
            let closure = Closure::wrap(Box::new({
                let is_mobile = is_mobile.clone();
                move || {
                    let mobile = window()
                        .and_then(|w| w.inner_width().ok())
                        .and_then(|w| w.as_f64())
                        .map(|w| w < 768.0)
                        .unwrap_or(false);
                    is_mobile.set(mobile);
                }
            }) as Box<dyn Fn()>);
            
            if let Some(window) = window() {
                let _ = window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
            }
            
            move || drop(closure)
        });
    }

    // 素材标签切换
    let on_material_tab_change = {
        let selected_material_tab = selected_material_tab.clone();
        Callback::from(move |tab: String| {
            selected_material_tab.set(tab);
        })
    };

    // 重置应用状态
    let on_reset = {
        let app_state = app_state.clone();
        Callback::from(move |_: MouseEvent| {
            app_state.set(AppState::Idle);
        })
    };

    // 渲染素材库内容
    let render_material_content = || {
        match (*selected_material_tab).as_str() {
            "video" => html! {
                <VideoUploadComponent 
                    app_state={app_state.clone()}
                    source_language={source_language.clone()}
                    target_language={target_language.clone()}
                />
            },
            "document" => html! {
                <DocumentImportComponent 
                    app_state={app_state.clone()}
                />
            },
            "audio" => html! {
                <div class="material-placeholder">
                    <div class="placeholder-icon">{"🎵"}</div>
                    <h3>{"音频素材"}</h3>
                    <p>{"拖拽音频文件到此处或点击上传"}</p>
                    <button class="btn btn-outline">{"选择音频文件"}</button>
                </div>
            },
            "image" => html! {
                <div class="material-placeholder">
                    <div class="placeholder-icon">{"🖼️"}</div>
                    <h3>{"图片素材"}</h3>
                    <p>{"拖拽图片文件到此处或点击上传"}</p>
                    <button class="btn btn-outline">{"选择图片文件"}</button>
                </div>
            },
            _ => html! { <div>{"未知素材类型"}</div> }
        }
    };

    html! {
        <div class={classes!("editor-container", (*is_mobile).then(|| "mobile"))}>
            // 顶部工具栏
            <header class="editor-header">
                <div class="header-left">
                    <div class="app-logo">
                        <span class="logo-icon">{"🎬"}</span>
                        <h1 class="app-title">{"彩旗剪辑"}</h1>
                    </div>
                    <div class="project-info">
                        <span class="project-name">{"未命名项目"}</span>
                        <span class="project-status">
                            {
                                match &*app_state {
                                    AppState::Idle => "就绪",
                                    AppState::Uploading => "上传中...",
                                    AppState::Ready(_) => "素材已就绪",
                                    AppState::Translating(_) => "翻译中...",
                                    AppState::Completed(_) => "已完成",
                                    AppState::Error(_) => "错误",
                                    AppState::DocumentImporting => "导入中...",
                                    AppState::DocumentReady(_) => "文档已就绪",
                                    AppState::CreatingProject => "创建中...",
                                }
                            }
                        </span>
                    </div>
                </div>
                
                <div class="header-center">
                    <div class="playback-controls">
                        <button class="control-btn" title="播放/暂停">
                            <span>{"▶️"}</span>
                        </button>
                        <button class="control-btn" title="停止">
                            <span>{"⏹️"}</span>
                        </button>
                        <button class="control-btn" title="上一帧">
                            <span>{"⏮️"}</span>
                        </button>
                        <button class="control-btn" title="下一帧">
                            <span>{"⏭️"}</span>
                        </button>
                    </div>
                    <div class="timecode">{"00:00:00"}</div>
                </div>
                
                <div class="header-right">
                    <button class="header-btn" title="导出项目">
                        <span>{"📤"}</span>
                        <span>{"导出"}</span>
                    </button>
                    <button class="header-btn" title="项目设置">
                        <span>{"⚙️"}</span>
                        <span>{"设置"}</span>
                    </button>
                    <button class="reset-btn" onclick={on_reset} title="重置项目">
                        <span>{"🔄"}</span>
                    </button>
                </div>
            </header>

            <div class="editor-body">
                // 左侧素材库
                <aside class="materials-panel">
                    <div class="panel-header">
                        <h2 class="panel-title">{"📁 素材库"}</h2>
                    </div>
                    
                    <div class="material-tabs">
                        <button 
                            class={classes!("tab-btn", (*selected_material_tab == "video").then(|| "active"))}
                            onclick={
                                let on_change = on_material_tab_change.clone();
                                Callback::from(move |_: MouseEvent| on_change.emit("video".to_string()))
                            }
                        >
                            <span>{"🎬"}</span>
                            <span>{"视频"}</span>
                        </button>
                        <button 
                            class={classes!("tab-btn", (*selected_material_tab == "document").then(|| "active"))}
                            onclick={
                                let on_change = on_material_tab_change.clone();
                                Callback::from(move |_: MouseEvent| on_change.emit("document".to_string()))
                            }
                        >
                            <span>{"📄"}</span>
                            <span>{"文档"}</span>
                        </button>
                        <button 
                            class={classes!("tab-btn", (*selected_material_tab == "audio").then(|| "active"))}
                            onclick={
                                let on_change = on_material_tab_change.clone();
                                Callback::from(move |_: MouseEvent| on_change.emit("audio".to_string()))
                            }
                        >
                            <span>{"🎵"}</span>
                            <span>{"音频"}</span>
                        </button>
                        <button 
                            class={classes!("tab-btn", (*selected_material_tab == "image").then(|| "active"))}
                            onclick={
                                let on_change = on_material_tab_change.clone();
                                Callback::from(move |_: MouseEvent| on_change.emit("image".to_string()))
                            }
                        >
                            <span>{"🖼️"}</span>
                            <span>{"图片"}</span>
                        </button>
                    </div>
                    
                    <div class="material-content">
                        { render_material_content() }
                    </div>
                </aside>

                // 中间编辑预览区
                <main class="editor-main">
                    <div class="preview-area">
                        <div class="video-preview">
                            <div class="preview-placeholder">
                                <div class="placeholder-content">
                                    <span class="placeholder-icon">{"🎬"}</span>
                                    <h3>{"预览区域"}</h3>
                                    <p>{"从左侧素材库拖拽素材到时间线开始编辑"}</p>
                                </div>
                            </div>
                            <div class="preview-controls">
                                <div class="zoom-controls">
                                    <button class="zoom-btn">{"🔍-"}</button>
                                    <span class="zoom-level">{"100%"}</span>
                                    <button class="zoom-btn">{"🔍+"}</button>
                                </div>
                                <div class="view-controls">
                                    <button class="view-btn active">{"预览"}</button>
                                    <button class="view-btn">{"源码"}</button>
                                </div>
                            </div>
                        </div>
                    </div>
                </main>

                // 右侧翻译面板
                <aside class="translation-panel-sidebar">
                    <TranslationPanelComponent 
                        app_state={app_state.clone()}
                    />
                </aside>
            </div>

            // 底部时间线
            <footer class="timeline-footer">
                <ProjectTimelineComponent 
                    app_state={app_state.clone()}
                />
            </footer>
        </div>
    }
}
