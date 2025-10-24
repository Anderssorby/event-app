use crate::components::button::Button;

use dioxus::prelude::*;
use dioxus_primitives::toast::{self, use_toast, ToastOptions, ToastProviderProps};
use std::time::Duration;


const TOAST_CSS: Asset = asset!("/assets/styling/toast.css");
#[component]
pub fn ToastProvider(props: ToastProviderProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TOAST_CSS }
        toast::ToastProvider {
            default_duration: props.default_duration,
            max_toasts: props.max_toasts,
            render_toast: props.render_toast,
            {props.children}
        }
    }
}
#[derive(Props, PartialEq, Clone)]
pub struct ToastButtonProps {
    button_text: String,
}

#[component]
pub fn ToastButton(props: ToastButtonProps) -> Element {
    let toast_api = use_toast();

    rsx! {
        Button {
            r#type: "button",
            "data-style": "outline",
            onclick: move |_| {
                toast_api
                    .info(
                        "Custom Toast".to_string(),
                        ToastOptions::new()
                            .description("Some info you need")
                            .duration(Duration::from_secs(60))
                            .permanent(false),
                    );
            },
            {props.button_text}
        }
    }
}
