use crate::error::{Error};
use crate::models::NewPerson;
use std::env;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Wss};
use surrealdb::opt::auth::Root;
use surrealdb::RecordId;
use serde::{Serialize, Deserialize};
use axum::extract::{Path, Json};

use std::sync::LazyLock;
static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub id: RecordId,
}

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

const PERSON: &str = "person";

pub async fn create_person(
    id: Path<String>,
    Json(person): Json<NewPerson>,
) -> Result<Json<Option<Person>>, Error> {
    let person = DB.create((PERSON, &*id)).content(person).await?;
    Ok(Json(person))
}

pub async fn read_person(id: Json<String>) -> Result<Json<Option<Person>>, Error> {
    let person = DB.select((PERSON, &*id)).await?;
    Ok(Json(person))
}

pub async fn update_person(
    id: Path<String>,
    Json(person): Json<NewPerson>,
) -> Result<Json<Option<Person>>, Error> {
    let person = DB.update((PERSON, &*id)).content(person).await?;
    Ok(Json(person))
}

pub async fn delete_person(id: String) -> Result<Json<Option<Person>>, Error> {
    let person = DB.delete((PERSON, &*id)).await?;
    Ok(Json(person))
}

pub async fn list_people() -> Result<Vec<Person>, Error> {
    let people = DB.select(PERSON).await?;
    Ok(people)
}
