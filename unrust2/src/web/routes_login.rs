use crate::{Error, Result};
use axum::Json;
use serde::Deserialize;
use serde_json::Value;

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    unimplemented!();
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
