// src/components/document_import.rs

use yew::prelude::*;
use web_sys::{FileList, HtmlInputElement};
use wasm_bindgen_futures::spawn_local;
use serde_wasm_bindgen;

use crate::types::{AppState, DocumentContent};
use crate::components::common::{LoadingSpinner, SuccessMessage};
use crate::api::invoke;

#[derive(Properties, PartialEq)]
pub struct DocumentImportProps {
    pub app_state: UseStateHandle<AppState>,
}

#[function_component(DocumentImportComponent)]
pub fn document_import_component(props: &DocumentImportProps) -> Html {
    let file_input_ref = use_node_ref();

    // 处理文档文件选择
    let on_document_select = {
        let app_state = props.app_state.clone();
        
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let files: Option<FileList> = input.files();
            
            if let Some(files) = files {
                if let Some(file) = files.get(0) {
                    let file_name = file.name();
                    app_state.set(AppState::DocumentImporting);
                    
                    // 调用Tauri API来处理文档导入
                    let app_state_clone = app_state.clone();
                    spawn_local(async move {
                        let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                            "file_path": file_name
                        })).unwrap();
                        
                        match invoke("import_document", args).await {
                            result => {
                                if let Ok(document_content) = serde_wasm_bindgen::from_value::<DocumentContent>(result) {
                                    app_state_clone.set(AppState::DocumentReady(document_content));
                                } else {
                                    app_state_clone.set(AppState::Error("文档导入失败".to_string()));
                                }
                            }
                        }
                    });
                }
            }
        })
    };

    // 根据状态判断是否禁用导入
    let is_importing = matches!(&*props.app_state, AppState::DocumentImporting);

    html! {
        <div class="document-import-section">
            <h2>{"文档导入"}</h2>
            <p class="section-description">{"支持导入 PowerPoint (.pptx)、Markdown (.md) 和 PDF 文档"}</p>
            
            if is_importing {
                <LoadingSpinner message={"正在导入文档...".to_string()} />
            } else {
                <div class="import-controls">
                    <input 
                        type="file" 
                        ref={file_input_ref.clone()} 
                        accept=".pptx,.ppt,.md,.markdown,.pdf" 
                        onchange={on_document_select}
                        disabled={is_importing}
                    />
                    
                    <div class="supported-formats">
                        <h4>{"支持的文档格式："}</h4>
                        <ul>
                            <li>{"PowerPoint 演示文稿 (.pptx, .ppt)"}</li>
                            <li>{"Markdown 文档 (.md, .markdown)"}</li>
                            <li>{"PDF 文档 (.pdf)"}</li>
                        </ul>
                    </div>
                </div>
            }
            
            // 显示文档内容预览
            if let AppState::DocumentReady(document) = &*props.app_state {
                <div class="document-preview">
                    <SuccessMessage message={"文档导入成功！".to_string()} />
                    
                    <div class="document-info">
                        <h3>{&document.title}</h3>
                        <div class="document-meta">
                            <span class="document-type">{"类型: "}{format!("{:?}", document.document_type)}</span>
                            <span class="document-pages">{"页数: "}{document.total_pages}</span>
                        </div>
                    </div>
                    
                    <div class="document-pages">
                        <h4>{"文档内容预览："}</h4>
                        {
                            document.pages.iter().take(3).map(|page| {
                                html! {
                                    <div class="document-page-preview" key={page.page_number}>
                                        <div class="page-header">
                                            <span class="page-number">{"第 "}{page.page_number}{" 页"}</span>
                                            {
                                                if let Some(title) = &page.title {
                                                    html! { <span class="page-title">{title}</span> }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                        </div>
                                        <div class="page-content">
                                            {
                                                if page.text_content.len() > 100 {
                                                    format!("{}...", &page.text_content[..100])
                                                } else {
                                                    page.text_content.clone()
                                                }
                                            }
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                        
                        if document.total_pages > 3 {
                            <div class="more-pages-indicator">
                                {"还有 "}{document.total_pages - 3}{" 页内容..."}
                            </div>
                        }
                    </div>
                </div>
            }
        </div>
    }
} 