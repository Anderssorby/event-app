//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;
use dioxus_fullstack::routing::Router;
use anyhow::anyhow;

#[cfg(feature = "server")]
pub mod db;
#[cfg(feature = "server")]
pub mod error;

pub mod models;

pub mod event;

/// Echo the user input on the server.
#[server]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    let reversed = input.chars().rev().collect::<String>();
    info!("Echoing back: {}", reversed);
    Ok(reversed)
}

#[cfg(feature = "server")]
pub async fn launch(component: fn() -> Element) -> Result<Router, anyhow::Error> {
    let mut router = dioxus::server::router(component);
    db::connect()
        .await
        .map_err(|e| anyhow!("Failed to connect to database: {}", e))?;
    println!("Connected to database successfully");

    return Ok(router);
}
