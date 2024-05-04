use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use tower_http::services::ServeDir;

mod error;
pub use self::error::{Error, Result};
mod web;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello/:name", get(handler_name))
        .fallback_service(routes_static());

    // region: ---- Start service
    let host_addr = "127.0.0.1:4000";

    let tcp_listener = tokio::net::TcpListener::bind(host_addr).await.unwrap();
    println!("->> Listening on http://{host_addr}");
    axum::serve(tcp_listener, app).await.unwrap();
    // endregion: ---- Start service
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// region: ---- Handler hello
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello params {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World");

    Html(format!("Hello <strong>{name}!!!</strong>"))
}

async fn handler_name(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_name", "HANDLER");

    Html(format!("Hello <strong>{name}!!!</strong>"))
}
// endregion: ---- Handler hello
