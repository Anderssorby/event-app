use dioxus::prelude::*;
use crate::event::{Event};

#[component]
pub fn EventView() -> Element {
    rsx! {
        Event {}
    }
}