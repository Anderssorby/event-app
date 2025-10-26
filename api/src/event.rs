use crate::models::{Event, NewEvent};
use dioxus::{prelude::*};
#[cfg(feature = "server")]
use crate::db;


#[server]
pub async fn get_event(id: String) -> Result<Event, ServerFnError> {
    let event = db::get_event(id).await?;
    Ok(event)
}

#[server]
pub async fn new_event(event: NewEvent) -> Result<Event, ServerFnError> {
    use surrealdb::Uuid;
    let id = Uuid::new_v4().to_string();
    let event = db::create_event(id,event).await?;
    Ok(event)
}

#[server]
pub async fn list_events() -> Result<Vec<Event>, ServerFnError> {
    let events = db::list_events().await?;
    Ok(events)
}