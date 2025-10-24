use dioxus::prelude::*;
use ui::{ChangeEvent, NewEvent, ViewEvent};

#[component]
pub fn Event() -> Element {
    let id = use_signal(|| String::new());
    rsx! {
        NewEvent {}
        if id().is_empty() {
            ViewEvent { id: id() }

            ChangeEvent { id: id() }
        }
    }
}