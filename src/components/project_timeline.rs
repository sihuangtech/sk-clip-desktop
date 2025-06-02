use yew::prelude::*;

use crate::types::AppState;

#[derive(Properties, PartialEq)]
pub struct ProjectTimelineProps {
    pub app_state: UseStateHandle<AppState>,
}

#[function_component(ProjectTimelineComponent)]
pub fn project_timeline_component(_props: &ProjectTimelineProps) -> Html {
    html! {
        <div class="project-timeline">
            <h2>{"项目时间线"}</h2>
            <div class="timeline-placeholder">
                <p>{"时间线编辑器功能正在开发中..."}</p>
                <div class="timeline-features">
                    <h4>{"计划功能："}</h4>
                    <ul>
                        <li>{"可视化时间线编辑器"}</li>
                        <li>{"拖拽式素材排列"}</li>
                        <li>{"视频片段剪辑"}</li>
                        <li>{"文档内容时间轴"}</li>
                        <li>{"语音合成同步"}</li>
                        <li>{"字幕编辑"}</li>
                    </ul>
                </div>
            </div>
        </div>
    }
} 