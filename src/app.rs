use yew::prelude::*;

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
    // 状态管理
    let app_state = use_state(|| AppState::Idle);
    let source_language = use_state(|| String::from("zh")); // 默认源语言为中文
    let target_language = use_state(|| String::from("en")); // 默认目标语言为英文

    // 重置应用状态
    let on_reset = {
        let app_state = app_state.clone();
        Callback::from(move |_: MouseEvent| {
            app_state.set(AppState::Idle);
        })
    };

    html! {
        <main class="app-container">
            // 应用头部
            <header class="app-header">
                <div class="header-content">
                    <h1 class="app-title">{"Multisay 视频创作与翻译工具"}</h1>
                    <p class="app-description">{"集成视频翻译、文档导入、时间线编辑的综合性视频创作平台"}</p>
                </div>
                <div class="header-actions">
                    <button class="reset-btn" onclick={on_reset}>{"重置"}</button>
                </div>
            </header>

            // 主要内容区域
            <div class="app-content">
                // 左侧面板 - 输入和控制
                <aside class="sidebar">
                    <div class="sidebar-section">
                        <VideoUploadComponent 
                            app_state={app_state.clone()}
                            source_language={source_language.clone()}
                            target_language={target_language.clone()}
                        />
                    </div>
                    
                    <div class="sidebar-section">
                        <DocumentImportComponent 
                            app_state={app_state.clone()}
                        />
                    </div>
                </aside>

                // 主要工作区域
                <main class="main-content">
                    <div class="content-tabs">
                        <div class="tab-content">
                            // 翻译面板
                            <div class="panel">
                                <TranslationPanelComponent 
                                    app_state={app_state.clone()}
                                />
                            </div>
                            
                            // 时间线面板
                            <div class="panel">
                                <ProjectTimelineComponent 
                                    app_state={app_state.clone()}
                                />
                            </div>
                        </div>
                    </div>
                </main>
            </div>

            // 状态栏
            <footer class="app-footer">
                <div class="status-info">
                    {
                        match &*app_state {
                            AppState::Idle => "就绪",
                            AppState::Uploading => "正在上传视频...",
                            AppState::Ready(_) => "视频已准备就绪",
                            AppState::Translating(_) => "正在翻译视频...",
                            AppState::Completed(_) => "翻译完成",
                            AppState::Error(_) => "发生错误",
                            AppState::DocumentImporting => "正在导入文档...",
                            AppState::DocumentReady(_) => "文档已导入",
                            AppState::CreatingProject => "正在创建项目...",
                        }
                    }
                </div>
                <div class="app-version">{"v0.1.0"}</div>
            </footer>
        </main>
    }
}
