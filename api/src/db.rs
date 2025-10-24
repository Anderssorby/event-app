use crate::error::{Error};
use crate::models::{Event, NewEvent};
use std::env;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Wss};
use surrealdb::opt::auth::Root;
use surrealdb::RecordId;
use serde::{Serialize, Deserialize};
use axum::extract::{Path, };

use std::sync::LazyLock;
static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn connect() -> Result<(), Error> {
    let host: String = env::var("SURREAL_HOST")?;
    DB.connect::<Wss>(host).await?;

    DB.use_ns("demo").use_db("surreal_deal_store").await?;

    let username = env::var("SURREAL_USERNAME")?;
    let password = env::var("SURREAL_PASSWORD")?;
    // Authenticate
    DB.signin(Root {
        username: &username,
        password: &password,
    })
    .await?;

    return Ok(());
}

const EVENT: &str = "event";


pub async fn create_event(
    id: String,
    event: NewEvent,
) -> Result<Event, Error> {
    let event = DB.create((EVENT, &*id)).content(event).await?;
    event.ok_or_else(|| Error::Db)
}

pub async fn get_event(id: String) -> Result<Event, Error> {
    let event = DB.select((EVENT, &*id)).await?;
    event.ok_or_else(|| Error::NotFound(format!("Event not found: {}", id)))
}

pub async fn update_event(
    id: String,
    event: NewEvent,
) -> Result<Event, Error> {
    let event = DB.update((EVENT, &*id)).content(event).await?;
    event.ok_or_else(|| Error::NotFound(format!("Event not found: {}", id)))
}

pub async fn delete_event(id: String) -> Result<Event, Error> {
    let event = DB.delete((EVENT, &*id)).await?;
    event.ok_or_else(|| Error::NotFound(format!("Event not found: {}", id)))
}

pub async fn list_events() -> Result<Vec<Event>, Error> {
    let events = DB.select(EVENT).await?;
    Ok(events)
}
