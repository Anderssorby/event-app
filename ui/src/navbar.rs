use dioxus::prelude::*;
use crate::components::{self, NavbarItem, NavbarNav, NavbarContent, NavbarTrigger};
use crate::Route;


#[component]
pub fn Navbar() -> Element {
    rsx! {
        components::Navbar {
            NavbarItem { index: 0usize, to: Route::Home {}, value: "home", "Home" }
            NavbarItem { index: 1usize, to: Route::Blog { id: 1 }, value: "blog", "Blog" }
            // The NavbarNav contains the individual menus that can be opened.
            NavbarNav {
                // The index of the menu, used to determine the order in which menus are displayed.
                index: 2usize,

                // The menubar trigger is the element that will display the menu when activated.
                NavbarTrigger { "Events" }
                // The menubar content contains all the items that will be displayed in the menu when it is opened.
                NavbarContent { class: "navbar-content",
                    // Each menubar item represents an individual items in the menu.
                    NavbarItem {
                        index: 0usize,
                        to: Route::EventView {  },
                        // The value of the item which will be passed to the on_select callback when the item is selected.
                        value: "event",
                        "Event"
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
