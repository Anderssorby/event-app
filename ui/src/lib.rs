//! This crate contains all shared UI for the workspace.

mod navbar;
pub use navbar::Navbar;

mod hero;
pub use hero::Hero;

pub mod components;

mod echo;
pub use echo::Echo;

mod event;
pub use event::Event;

pub mod views;

mod route;
pub use route::Route;