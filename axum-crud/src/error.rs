use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    MissingENV(&'static str),
    ENVWrongFormat(&'static str),
    FailToConnectPool(String),
    FailedToCreateFood,
    CreateFailed(String),
    FailedToSelectFood,
    SelectFailed(String),
    FailedToUpdateFood,
    UpdateFailed(String),
    FailedToDeleteFood,
    DeleteFailed(String),
    FoodIdNotFound(String),
    FoodStampCodeNotFound(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        debug!("{:<12} - crud_fns error {self:?}", "INTO_RES");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(Arc::new(self));

        response
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
