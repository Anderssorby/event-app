use dioxus::prelude::*;
use crate::{Calendar, ToastButton, ToastProvider};

const HERO_CSS: Asset = asset!("/assets/styling/hero.css");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");
const COMPONENTS_CSS: Asset = asset!("/assets/styling/dx-components-theme.css");

#[component]
pub fn Hero() -> Element {
    
    rsx! {
        document::Link { rel: "stylesheet", href: HERO_CSS }
        document::Link { rel: "stylesheet", href: COMPONENTS_CSS }

        div { id: "hero",
            // img { src: HEADER_SVG, id: "header" }
            // div { id: "links",
            //     a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
            //     a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
            // }
            h1 { "Welcome to Event App!" }
            p { "An example fullstack application built with Dioxus." }

            Calendar {}
            ToastProvider {
                ToastButton { button_text: "Open toast".to_string() }
            }
        }
    }
}
