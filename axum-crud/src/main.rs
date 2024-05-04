use axum::{routing::get, serve, Json, Router};
use config::core_config;
use crud_fns::ModelController;
use error::Result;
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod config;
mod crud_fns;
mod crud_routes;
mod envs;
mod error;
mod store;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mm = ModelController::new().await?;

    let app = Router::new()
        .route("/", get(greet))
        .merge(crud_routes::routes_crud(mm.clone()));

    let app_addr = format!(
        "{}:{}",
        &core_config().SERVER_URL,
        &core_config().SERVER_PORT
    );
    let tcp_listener = TcpListener::bind(app_addr.clone()).await.unwrap();

    info!("{:<12} - Server is live!", format!("http://{}", app_addr));
    serve(tcp_listener, app).await.unwrap();

    Ok(())
}

async fn greet() -> Json<Value> {
    Json(json!({ "greet": "Hello World!"}))
}
