// src/components/project_timeline.rs

// 可视化时间线编辑器组件
// 提供拖拽式的多轨道时间线编辑功能

use yew::prelude::*;
use web_sys::{HtmlElement, MouseEvent, DragEvent};
use wasm_bindgen::JsCast;
use serde::{Deserialize, Serialize};

use crate::types::AppState;

/// 时间线轨道类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrackType {
    /// 视频轨道
    Video,
    /// 音频轨道
    Audio,
    /// 文档内容轨道
    Document,
    /// 字幕轨道
    Subtitle,
    /// 语音合成轨道
    TTS,
}

/// 时间线元素
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimelineElement {
    /// 元素唯一ID
    pub id: String,
    /// 元素名称
    pub name: String,
    /// 轨道类型
    pub track_type: TrackType,
    /// 开始时间（秒）
    pub start_time: f32,
    /// 持续时间（秒）
    pub duration: f32,
    /// 文件路径或内容
    pub content: String,
    /// 是否被选中
    pub selected: bool,
    /// 元素颜色（CSS颜色值）
    pub color: String,
}

/// 时间线轨道
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimelineTrack {
    /// 轨道ID
    pub id: String,
    /// 轨道名称
    pub name: String,
    /// 轨道类型
    pub track_type: TrackType,
    /// 轨道中的元素列表
    pub elements: Vec<TimelineElement>,
    /// 是否静音
    pub muted: bool,
    /// 是否锁定
    pub locked: bool,
    /// 轨道高度
    pub height: u32,
}

/// 时间线项目数据
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimelineProject {
    /// 项目名称
    pub name: String,
    /// 总时长（秒）
    pub total_duration: f32,
    /// 时间线轨道列表
    pub tracks: Vec<TimelineTrack>,
    /// 当前播放位置（秒）
    pub current_time: f32,
    /// 缩放级别（像素/秒）
    pub zoom_level: f32,
    /// 是否正在播放
    pub is_playing: bool,
}

impl Default for TimelineProject {
    fn default() -> Self {
        Self {
            name: "新项目".to_string(),
            total_duration: 300.0, // 默认5分钟
            tracks: vec![
                TimelineTrack {
                    id: "video_track_1".to_string(),
                    name: "视频轨道 1".to_string(),
                    track_type: TrackType::Video,
                    elements: Vec::new(),
                    muted: false,
                    locked: false,
                    height: 80,
                },
                TimelineTrack {
                    id: "audio_track_1".to_string(),
                    name: "音频轨道 1".to_string(),
                    track_type: TrackType::Audio,
                    elements: Vec::new(),
                    muted: false,
                    locked: false,
                    height: 60,
                },
                TimelineTrack {
                    id: "document_track_1".to_string(),
                    name: "文档轨道 1".to_string(),
                    track_type: TrackType::Document,
                    elements: Vec::new(),
                    muted: false,
                    locked: false,
                    height: 60,
                },
                TimelineTrack {
                    id: "subtitle_track_1".to_string(),
                    name: "字幕轨道 1".to_string(),
                    track_type: TrackType::Subtitle,
                    elements: Vec::new(),
                    muted: false,
                    locked: false,
                    height: 40,
                },
            ],
            current_time: 0.0,
            zoom_level: 20.0, // 20像素/秒
            is_playing: false,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ProjectTimelineProps {
    pub app_state: UseStateHandle<AppState>,
}

#[function_component(ProjectTimelineComponent)]
pub fn project_timeline_component(_props: &ProjectTimelineProps) -> Html {
    // 时间线项目状态
    let timeline_project = use_state(|| TimelineProject::default());
    
    // 拖拽状态
    let dragging_element = use_state(|| None::<String>);
    let drag_start_x = use_state(|| 0.0);
    let drag_start_time = use_state(|| 0.0);
    
    // 选中的元素
    let _selected_element = use_state(|| None::<String>);

    // 播放/暂停控制
    let toggle_playback = {
        let timeline_project = timeline_project.clone();
        Callback::from(move |_: MouseEvent| {
            let mut project = (*timeline_project).clone();
            project.is_playing = !project.is_playing;
            timeline_project.set(project);
        })
    };

    // 时间线缩放控制
    let zoom_in = {
        let timeline_project = timeline_project.clone();
        Callback::from(move |_: MouseEvent| {
            let mut project = (*timeline_project).clone();
            project.zoom_level = (project.zoom_level * 1.2).min(100.0);
            timeline_project.set(project);
        })
    };

    let zoom_out = {
        let timeline_project = timeline_project.clone();
        Callback::from(move |_: MouseEvent| {
            let mut project = (*timeline_project).clone();
            project.zoom_level = (project.zoom_level / 1.2).max(5.0);
            timeline_project.set(project);
        })
    };

    // 添加新轨道
    let add_track = {
        let timeline_project = timeline_project.clone();
        Callback::from(move |track_type: TrackType| {
            let mut project = (*timeline_project).clone();
            let track_id = format!("{:?}_track_{}", track_type, project.tracks.len() + 1);
            let track_name = match &track_type {
                TrackType::Video => format!("视频轨道 {}", project.tracks.iter().filter(|t| t.track_type == TrackType::Video).count() + 1),
                TrackType::Audio => format!("音频轨道 {}", project.tracks.iter().filter(|t| t.track_type == TrackType::Audio).count() + 1),
                TrackType::Document => format!("文档轨道 {}", project.tracks.iter().filter(|t| t.track_type == TrackType::Document).count() + 1),
                TrackType::Subtitle => format!("字幕轨道 {}", project.tracks.iter().filter(|t| t.track_type == TrackType::Subtitle).count() + 1),
                TrackType::TTS => format!("语音合成轨道 {}", project.tracks.iter().filter(|t| t.track_type == TrackType::TTS).count() + 1),
            };
            
            let height = match &track_type {
                TrackType::Video => 80,
                TrackType::Audio | TrackType::Document | TrackType::TTS => 60,
                TrackType::Subtitle => 40,
            };
            
            let new_track = TimelineTrack {
                id: track_id,
                name: track_name,
                track_type,
                elements: Vec::new(),
                muted: false,
                locked: false,
                height,
            };
            
            project.tracks.push(new_track);
            timeline_project.set(project);
        })
    };

    // 添加示例元素到轨道
    let add_sample_element = {
        let timeline_project = timeline_project.clone();
        Callback::from(move |track_id: String| {
            let mut project = (*timeline_project).clone();
            
            if let Some(track) = project.tracks.iter_mut().find(|t| t.id == track_id) {
                let element_id = format!("element_{}_{}", track_id, track.elements.len() + 1);
                let (name, color, content) = match track.track_type {
                    TrackType::Video => ("示例视频.mp4", "#4CAF50", "video_content"),
                    TrackType::Audio => ("示例音频.wav", "#2196F3", "audio_content"),
                    TrackType::Document => ("示例文档.pptx", "#FF9800", "document_content"),
                    TrackType::Subtitle => ("字幕文本", "#9C27B0", "subtitle_content"),
                    TrackType::TTS => ("语音合成", "#F44336", "tts_content"),
                };
                
                let new_element = TimelineElement {
                    id: element_id,
                    name: name.to_string(),
                    track_type: track.track_type.clone(),
                    start_time: track.elements.len() as f32 * 10.0, // 每个元素间隔10秒
                    duration: 15.0, // 默认15秒
                    content: content.to_string(),
                    selected: false,
                    color: color.to_string(),
                };
                
                track.elements.push(new_element);
            }
            
            timeline_project.set(project);
        })
    };

    // 元素拖拽开始
    let on_element_drag_start = {
        let dragging_element = dragging_element.clone();
        let drag_start_x = drag_start_x.clone();
        let drag_start_time = drag_start_time.clone();
        
        Callback::from(move |e: DragEvent| {
            if let Some(target) = e.target() {
                if let Ok(element) = target.dyn_into::<HtmlElement>() {
                    if let Some(element_id) = element.get_attribute("data-element-id") {
                        dragging_element.set(Some(element_id));
                        drag_start_x.set(e.client_x() as f32);
                        
                        // 从元素属性中获取开始时间
                        if let Some(start_time_str) = element.get_attribute("data-start-time") {
                            if let Ok(start_time) = start_time_str.parse::<f32>() {
                                drag_start_time.set(start_time);
                            }
                        }
                    }
                }
            }
        })
    };

    // 元素拖拽结束
    let on_element_drag_end = {
        let dragging_element = dragging_element.clone();
        let timeline_project = timeline_project.clone();
        let drag_start_x = drag_start_x.clone();
        let drag_start_time = drag_start_time.clone();
        
        Callback::from(move |e: DragEvent| {
            if let Some(element_id) = (*dragging_element).clone() {
                let current_x = e.client_x() as f32;
                let delta_x = current_x - *drag_start_x;
                let delta_time = delta_x / (*timeline_project).zoom_level;
                let new_start_time = (*drag_start_time + delta_time).max(0.0);
                
                // 更新元素位置
                let mut project = (*timeline_project).clone();
                for track in &mut project.tracks {
                    for element in &mut track.elements {
                        if element.id == element_id {
                            element.start_time = new_start_time;
                            break;
                        }
                    }
                }
                timeline_project.set(project);
            }
            
            dragging_element.set(None);
        })
    };

    // 渲染时间线标尺
    let render_timeline_ruler = {
        let project = &*timeline_project;
        let ruler_width = project.total_duration * project.zoom_level;
        
        let time_markers: Vec<Html> = (0..=(project.total_duration as u32 / 10))
            .map(|i| {
                let time = i as f32 * 10.0;
                let x = time * project.zoom_level;
                html! {
                    <div
                        class="time-marker"
                        style={format!("left: {}px;", x)}
                    >
                        <div class="time-label">{format!("{}:{:02}", (time as u32) / 60, (time as u32) % 60)}</div>
                        <div class="time-line"></div>
                    </div>
                }
            })
            .collect();

        html! {
            <div class="timeline-ruler" style={format!("width: {}px;", ruler_width)}>
                {for time_markers}
                <div 
                    class="playhead" 
                    style={format!("left: {}px;", project.current_time * project.zoom_level)}
                ></div>
            </div>
        }
    };

    // 渲染轨道
    let render_tracks = {
        let project = &*timeline_project;
        
        project.tracks.iter().map(|track| {
            let track_id = track.id.clone();
            let add_element_callback = {
                let add_sample_element = add_sample_element.clone();
                let track_id = track_id.clone();
                Callback::from(move |_: MouseEvent| {
                    add_sample_element.emit(track_id.clone());
                })
            };

            // 渲染轨道中的元素
            let track_elements: Vec<Html> = track.elements.iter().map(|element| {
                let element_width = element.duration * project.zoom_level;
                let element_left = element.start_time * project.zoom_level;

                html! {
                    <div
                        class={classes!("timeline-element", if element.selected { Some("selected") } else { None })}
                        style={format!(
                            "left: {}px; width: {}px; background-color: {}; opacity: 0.8;",
                            element_left, element_width, element.color
                        )}
                        data-element-id={element.id.clone()}
                        data-start-time={element.start_time.to_string()}
                        draggable="true"
                        ondragstart={on_element_drag_start.clone()}
                        ondragend={on_element_drag_end.clone()}
                    >
                        <div class="element-name">{&element.name}</div>
                        <div class="element-duration">{format!("{:.1}s", element.duration)}</div>
                    </div>
                }
            }).collect();

            html! {
                <div class="timeline-track" style={format!("height: {}px;", track.height)}>
                    <div class="track-header">
                        <div class="track-name">{&track.name}</div>
                        <div class="track-controls">
                            <button
                                class={classes!("track-mute-btn", if track.muted { Some("active") } else { None })}
                                title="静音"
                            >{"M"}</button>
                            <button
                                class={classes!("track-lock-btn", if track.locked { Some("active") } else { None })}
                                title="锁定"
                            >{"L"}</button>
                            <button
                                class="track-add-btn"
                                onclick={add_element_callback}
                                title="添加元素"
                            >{"+"}</button>
                        </div>
                    </div>
                    <div class="track-content">
                        {for track_elements}
                    </div>
                </div>
            }
        }).collect::<Vec<Html>>()
    };

    html! {
        <div class="project-timeline">
            <div class="timeline-header">
                <h2>{"项目时间线编辑器"}</h2>
                <div class="timeline-controls">
                    <button 
                        class={classes!("play-btn", if timeline_project.is_playing { Some("playing") } else { None })}
                        onclick={toggle_playback}
                    >
                        {if timeline_project.is_playing { "⏸" } else { "▶" }}
                    </button>
                    <div class="time-display">
                        {format!("{}:{:02} / {}:{:02}", 
                            (timeline_project.current_time as u32) / 60, 
                            (timeline_project.current_time as u32) % 60,
                            (timeline_project.total_duration as u32) / 60, 
                            (timeline_project.total_duration as u32) % 60
                        )}
                    </div>
                    <div class="zoom-controls">
                        <button onclick={zoom_out}>{"🔍-"}</button>
                        <span class="zoom-level">{format!("{:.0}%", timeline_project.zoom_level * 5.0)}</span>
                        <button onclick={zoom_in}>{"🔍+"}</button>
                    </div>
                </div>
            </div>

            <div class="timeline-content">
                <div class="timeline-ruler-container">
                    {render_timeline_ruler}
                </div>
                
                <div class="timeline-tracks">
                    {for render_tracks}
                </div>
            </div>

            <div class="timeline-footer">
                <div class="track-add-controls">
                    <span>{"添加轨道："}</span>
                    <button onclick={let cb = add_track.clone(); Callback::from(move |_| cb.emit(TrackType::Video))}>
                        {"视频轨道"}
                    </button>
                    <button onclick={let cb = add_track.clone(); Callback::from(move |_| cb.emit(TrackType::Audio))}>
                        {"音频轨道"}
                    </button>
                    <button onclick={let cb = add_track.clone(); Callback::from(move |_| cb.emit(TrackType::Document))}>
                        {"文档轨道"}
                    </button>
                    <button onclick={let cb = add_track.clone(); Callback::from(move |_| cb.emit(TrackType::Subtitle))}>
                        {"字幕轨道"}
                    </button>
                    <button onclick={let cb = add_track.clone(); Callback::from(move |_| cb.emit(TrackType::TTS))}>
                        {"语音合成轨道"}
                    </button>
                </div>
                
                <div class="timeline-info">
                    <span>{"项目：{}"}</span>
                    <span>{format!("轨道数：{}", timeline_project.tracks.len())}</span>
                    <span>{format!("总时长：{:.1}秒", timeline_project.total_duration)}</span>
                </div>
            </div>
        </div>
    }
} 
