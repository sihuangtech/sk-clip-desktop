// src/components/common.rs

use yew::prelude::*;

// 加载动画组件
#[derive(Properties, PartialEq)]
pub struct LoadingSpinnerProps {
    pub message: String,
}

#[function_component(LoadingSpinner)]
pub fn loading_spinner(props: &LoadingSpinnerProps) -> Html {
    html! {
        <div class="loading-spinner">
            <div class="spinner"></div>
            <div class="loading-message">{&props.message}</div>
        </div>
    }
}

// 错误消息组件
#[derive(Properties, PartialEq)]
pub struct ErrorMessageProps {
    pub message: String,
}

#[function_component(ErrorMessage)]
pub fn error_message(props: &ErrorMessageProps) -> Html {
    html! {
        <div class="message error">
            <span class="error-icon">{"⚠️"}</span>
            <span class="error-text">{"错误：" }{&props.message}</span>
        </div>
    }
}

// 成功消息组件
#[derive(Properties, PartialEq)]
pub struct SuccessMessageProps {
    pub message: String,
}

#[function_component(SuccessMessage)]
pub fn success_message(props: &SuccessMessageProps) -> Html {
    html! {
        <div class="message success">
            <span class="success-icon">{"✅"}</span>
            <span class="success-text">{&props.message}</span>
        </div>
    }
}

// 按钮组件
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub text: String,
    pub onclick: Callback<MouseEvent>,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub class: String,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let class = format!("btn {}", props.class);
    
    html! {
        <button 
            class={class}
            onclick={&props.onclick}
            disabled={props.disabled}
        >
            {&props.text}
        </button>
    }
} 