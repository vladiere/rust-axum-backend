#![allow(unused)]

pub use self::error::{Error, Result};

mod ctx;
mod error;
mod log;
mod model;
mod web;

use crate::ctx::Ctx;
use std::net::SocketAddr;

use crate::{log::log_request, model::ModelController};
use axum::{
    extract::{Path, Query},
    http::{Method, Uri},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    //Initialize ModelController.
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_ticket::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_requires_auth));

    let app = Router::new()
        .merge(fn_routes())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(fn_static());
    let host_addr = "127.0.0.1:3000";

    // start region ---- Start server
    let addr = tokio::net::TcpListener::bind(host_addr).await.unwrap();
    println!("Listening on http://{}", host_addr);

    axum::serve(addr, app).await.unwrap();
    // end region ---- Start server

    Ok(())
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    let uuid = Uuid::new_v4();

    // ---- Get the eventual response error.
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // ---- If client error, build the new response.
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            println!("    ->> client_error_body: {client_error_body}");

            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    // ---- TODD: Build and log the server log line.
    let client_error = client_status_error.unzip().1;
    log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!();
    error_response.unwrap_or(res)
}

// region: ---- Static Routes
fn fn_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// region: -- Routes
fn fn_routes() -> Router {
    Router::new()
        .route("/", get(handler_hello))
        .route("/:name", get(handler_name))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g `/?name=vlad`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");

    let name = params.name.as_deref().unwrap_or("Worl!");
    Html(format!("Hello <strong>{name}!!</strong>"))
}

// e.g `/id/12` or `/name/vlad`
async fn handler_name(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_name - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}!!</strong>"))
}
