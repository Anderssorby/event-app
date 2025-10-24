//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;


#[cfg(feature = "server")]
pub mod db;
#[cfg(feature = "server")]
pub mod error;

pub mod models;

pub mod event;

#[server]
pub async fn load_data() -> Result<Vec<models::Event>, ServerFnError> {
    let events = db::list_events().await?;
    Ok(events)
}

/// Echo the user input on the server.
#[server]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    let reversed = input.chars().rev().collect::<String>();
    info!("Echoing back: {}", reversed);
    Ok(reversed)
}
