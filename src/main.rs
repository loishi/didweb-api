
use axum::{routing::get, routing::post, Json, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
mod auth;
mod database;
mod handler;

#[tokio::main]
async fn main() {
    let pool = database::create_pool()
        .await
        .expect("Failed to create database pool");
    let store = Arc::new(RwLock::new(std::collections::HashMap::new()));

    let did_web_router = Router::new()
        .route("/", post(handler::create_did_web))
        .route("/:did", get(handler::resolve_did_web))
        .route("/:did", post(handler::update_did_web))
        .route("/:did", axum::routing::delete(handler::delete_did_web));

    let app = Router::new()
        .nest("/did:web", did_web_router)
        .with_state(Arc::clone(&store))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}