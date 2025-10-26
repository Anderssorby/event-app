use dioxus::{prelude::*};
use api::models;
use time::{macros::format_description, PrimitiveDateTime};
// const BLOG_CSS: Asset = asset!("/assets/blog.css");

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

#[component]
pub fn ViewEvent(id: String) -> Element {
    //let get_event = move || async move { api::event::get_event(id.clone()).await };
    //let id_moved = id.clone();
    let event_res = use_resource(move || {
            let id_moved = id.clone();

        async move { api::event::get_event(id_moved.clone()).await }}
    );
    rsx! {

        div { id: "event",
            // Content
            if let Some(response) = &*event_res.read() {
                match response {
                    Ok(event) => rsx! {
                        "Event Title: {event.title}"
                        br {}
                        "Event Description: {event.description}"
                    },
                    Err(err) => rsx! { "Failed to fetch response: {err}" },
                }
            } else {
                "Loading..."
            }
        }
    }
}

#[component]
pub fn NewEvent() -> Element {
    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    let mut new_event = use_signal(||
        models::NewEvent { title: "".to_string(), description: "".to_string(), date_time: time::OffsetDateTime::now_utc() });
    rsx! {

        div { id: "event",

            // Content
            h1 { "Create a New Event" }
            p {
                "Here you can create a new event. This is just a placeholder for the new event creation form."
                form {
                    onsubmit: move |event| async move {
                        event.prevent_default();
                        // Handle form submission logic here
                        info!("New event form submitted!");
                        info!("Data: {:?}", new_event());
                        if let Ok(event) = api::event::new_event(new_event()).await {
                            info!("Created event: {:?}", event);
                        }
                    },
                    label { "Event Title: " }
                    input {
                        r#type: "text",
                        name: "title",
                        required: true,
                        value: new_event().title,
                        oninput: move |e| {
                            let mut event = new_event();

                            event.title = e.value();
                            new_event.set(event);

                        },
                    }
                    br {}
                    label { "Event Description: " }
                    textarea {
                        name: "description",
                        required: true,
                        value: new_event().description,
                        oninput: move |e| {
                            let mut event = new_event();
                            event.description = e.value();
                            new_event.set(event);
                        },
                    }
                    br {}
                    label { "Event Date and Time: " }
                    //date_picker::DatePicker {  }
                    input {
                        r#type: "datetime-local",
                        name: "date_time",
                        required: true,
                        value: new_event().date_time.to_string(),
                        oninput: move |e| {
                            if let Ok(date_time) = time::OffsetDateTime::parse(&e.value(), &format) {
                                let mut event = new_event();

                                event.date_time = date_time;
                                new_event.set(event);

                            }
                        },
                    }
                    br {}
                    button { r#type: "submit", "Create Event" }
                }
            }
        }
    }
}

#[component]
pub fn ChangeEvent(id: String) -> Element {
    rsx! {
        // document::Link { rel: "stylesheet", href: BLOG_CSS }

        div { id: "event",

            // Content
            h1 { "This is event #{id}!" }
            p {
                "In event #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
            }
        }
    }
}