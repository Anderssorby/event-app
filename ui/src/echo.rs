use std::result::Result;

use api::models::Event;
use dioxus::prelude::*;

const ECHO_CSS: Asset = asset!("/assets/styling/echo.css");

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn Echo() -> Element {
    let mut response = use_signal(|| String::new());
    let events: Resource<Result<Vec<Event>, ServerFnError>> =
        use_server_future(api::load_data)?;

    rsx! {
        document::Link { rel: "stylesheet", href: ECHO_CSS }
        div { id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput: move |event| async move {
                    println!("Input changed: {}", event.value());
                    event.prevent_default();
                    info!("Sending to server: {}", event.value());
                    match api::echo(event.value()).await {
                        Err(err) => {
                            response.set(format!("Error: {}", err));
                        }
                        Ok(data) => response.set(data),
                    }
                },
            }

            if !response.read().is_empty() {
                p {
                    "Server echoed: "
                    i { "{response.read()}" }
                }
            }
        }
        p {
            h1 { "Events in DataBase" }
            div {
                if let Some(Ok(events)) = &*events.read() {
                    for event in events {
                        div { key: "{event.id}",
                            Link { to: format!("/event/{}", event.id),
                                "ID: {event.id}, Title: {event.title}, Description: {event.description}"
                            }
                        }
                    }
                } else {
                    "Loading events..."
                }
            }
        }
    }
}
