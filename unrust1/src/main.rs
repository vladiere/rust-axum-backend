#![allow(unused)]
// region: ---- Modules
mod config;
mod ctx;
mod error;
mod log;
mod model;
mod web;

// #[cfg(test)] // Commented during early development.
pub mod _dev_utils;

pub use self::error::{Error, Result};
pub use config::config; // to use crate::config;

use crate::{
    model::ModelManager,
    web::{
        middlewares::{
            auth::{mw_ctx_resolver, mw_requires_auth},
            res_map::main_response_mapper,
        },
        routes_login::routes,
        routes_static::serve_dir,
    },
};

use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let cors = CorsLayer::new().allow_origin(Any);

    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // -- FOR DEV ONLY
    _dev_utils::init_dev().await;

    //Initialize ModelManager.
    let mm = ModelManager::new().await;

    // let routes_rpc = rpc::routes(mm.clone())
    //     .route_layer(middleware::from_fn(mw_ctx_require));

    let app = Router::new()
        .merge(fn_routes())
        .merge(routes())
        // .nest("/api", routes_rpc)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mm.expect("Model Manager Failed").clone(),
            mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .layer(cors)
        .fallback_service(serve_dir());

    let host_addr = "127.0.0.1:3000";

    // start region ---- Start server
    let addr = tokio::net::TcpListener::bind(host_addr).await.unwrap();
    info!("Listening on http://{}", host_addr);

    axum::serve(addr, app).await.unwrap();
    // end region ---- Start server

    Ok(())
}

// region: -- Routes
fn fn_routes() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello/:name", get(handler_name))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g `/?name=vlad`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    debug!("{:<12} - handler_hello", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

// e.g `/id/12` or `/name/vlad`
async fn handler_name(Path(name): Path<String>) -> impl IntoResponse {
    debug!("{:<12} - handler_name - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}!!</strong>"))
}
