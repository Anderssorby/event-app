use dioxus::{logger::tracing::Level, prelude::*};
use ui::Route;


const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::logger::init(Level::INFO).expect("logger failed to init");
    #[cfg(feature = "server")]
    {
        use dotenv::dotenv;
        dotenv().ok();
    }
    #[cfg(feature = "mobile")]
    // Hydrate the application on the client
    dioxus::launch(App);

    // Launch axum on the server
    #[cfg(feature = "server")]
    {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                api::launch(App).await;
            });
    }}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}
