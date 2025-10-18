use dioxus::{fullstack::Transportable, prelude::*};

const ECHO_CSS: Asset = asset!("/assets/styling/echo.css");

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn Echo() -> Element {
    let mut response = use_signal(|| String::new());
    let data: String = use_server_future(api::load_data)?().unwrap_or_else(|| Ok("Loading...".into()))?;

    rsx! {
        document::Link { rel: "stylesheet", href: ECHO_CSS }
        div { id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput: move |event| async move {
                    info!("Sending to server: {}", event.value());
                    match api::echo(event.value()).await {
                        Err(err) => {
                            response.set(format!("Error: {}", err));
                        }
                        Ok(data) => response.set(data),
                    }
                },
            }

            if !response().is_empty() {
                p {
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        
        }
        p {
            h1 { "Persons in DataBase" }
            b { "{data}" }
        }
    }
}
