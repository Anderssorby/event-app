use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use dioxus::logger::tracing::{Level, debug, error, info, warn};
use dioxus::prelude::ServerFnError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error")]
    Db,

    #[error("environment variable error: {0}")]
    EnvVar(#[from] std::env::VarError),

    #[error("not found error: {0}")]
    NotFound(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response()
    }
}

impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Self {
        error!("{error}");
        Self::Db
    }
}

impl From<Error> for ServerFnError {
    fn from(error: Error) -> Self {
        error!("{error}");
        ServerFnError::Response("Error in service".to_owned())
    }
}
