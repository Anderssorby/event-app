//! This crate contains all shared fullstack server functions.
use dioxus::{fullstack::Json, prelude::*};
use itertools::Itertools;

#[cfg(feature = "server")]
pub mod db;
#[cfg(feature = "server")]
pub mod error;

pub mod models;

#[server]
pub async fn load_data() -> Result<String, ServerFnError> {
    //server::db::connect().await?;
    let people = db::list_people().await?;
    Ok(people.into_iter().map(|p|p.name).join(", "))
}

/// Echo the user input on the server.
#[server]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    info!("Echoing back: {}", input);
    Ok(input.chars().rev().collect::<String>())
}
