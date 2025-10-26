use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::navbar::Navbar;
use crate::views::{Blog, Home, EventView};

#[derive(Debug, Clone, Routable, PartialEq, Serialize, Deserialize)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },

    #[route("/event/")]
    EventView {},
}