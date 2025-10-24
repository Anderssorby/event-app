use dioxus::prelude::*;
use crate::components::{Calendar, ToastButton, ToastProvider};

const HERO_CSS: Asset = asset!("/assets/styling/hero.css");
const COMPONENTS_CSS: Asset = asset!("/assets/styling/dx-components-theme.css");

#[component]
pub fn Hero() -> Element {
    
    rsx! {
        document::Link { rel: "stylesheet", href: HERO_CSS }
        document::Link { rel: "stylesheet", href: COMPONENTS_CSS }

        div { id: "hero",
            h1 { "Welcome to Event App!" }
            p { "An example fullstack application built with Dioxus." }
            Calendar {}
            ToastProvider {
                ToastButton { button_text: "Open toast".to_string() }
            }
        }

    }
}
