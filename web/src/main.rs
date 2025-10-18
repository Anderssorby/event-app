use dioxus::{logger::tracing::Level, prelude::*};
use serde::{Deserialize, Serialize};

use ui::{HistoryNavigation, Navbar, NavbarItem, NavbarNav, NavbarTrigger};
use views::{Blog, Home};

mod views;

#[derive(Debug, Clone, Routable, PartialEq, Serialize, Deserialize)]
#[rustfmt::skip]
enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::logger::init(Level::INFO).expect("logger failed to init");
    #[cfg(feature = "server")]
    {
        use dotenv::dotenv;
        dotenv().ok();
    }
    #[cfg(feature = "web")]
    // Hydrate the application on the client
    dioxus::launch(App);

    // Launch axum on the server
    #[cfg(feature = "server")]
    {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                launch(App).await;
            });
    }
}

#[cfg(feature = "server")]
pub async fn launch(component: fn() -> Element) {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    // Get the address the server should run on. If the CLI is running, the CLI proxies fullstack into the main address
    // and we use the generated address the CLI gives us
    let ip =
        dioxus::cli_config::server_ip().unwrap_or_else(|| IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    let port = dioxus::cli_config::server_port().unwrap_or(8080);
    let address = SocketAddr::new(ip, port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    let config = ServeConfig::new();
    let router = axum::Router::new()
        // serve_dioxus_application adds routes to server side render the application, serve static assets, and register server functions
        .serve_dioxus_application(config, App)
        .into_make_service();
    match api::db::connect().await {
        Ok(_) => {
            println!("Connected to database successfully");
        }
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
        }
    }
    axum::serve(listener, router).await.unwrap();
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebNavbar() -> Element {
    rsx! {
        Navbar {
            // The NavbarNav contains the individual menus that can be opened.
            // NavbarNav {
            // The index of the menu, used to determine the order in which menus are displayed.
            // index: 0usize,

            // The menubar trigger is the element that will display the menu when activated.
            // NavbarTrigger {
            // The content of the trigger button
            // {children}

            // }
            // The menubar content contains all the items that will be displayed in the menu when it is opened.
            // NavbarContent {
            //     // Each menubar item represents an individual items in the menu.
            //     NavbarItem {

            //         // The value of the item which will be passed to the on_select callback when the item is selected.
            //         value: "",
            //         on_select: |value: String| {
            //             // This callback is triggered when the item is selected.
            //             // The value parameter contains the value of the selected item.
            //         },
            //     }
            // }

            NavbarItem { index: 0usize, to: Route::Home {}, value: "home", "Home" }
            NavbarItem { index: 1usize, to: Route::Blog { id: 1 }, value: "blog", "Blog" }
        }

        Outlet::<Route> {}
    }
}
