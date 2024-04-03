use axum::{routing::get, routing::post, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

mod auth;
mod database;
mod handler;

#[tokio::main]
async fn main() {
    let pool = database::create_pool().await.expect("Failed to create database pool");
    let store = Arc::new(RwLock::new(std::collections::HashMap::new()));

    let app = Router::new()
        .route("/did:web/:did", get(handler::resolve_did_web))
        .route("/did:web", post(handler::create_did_web))
        .route("/did:web/:did", post(handler::update_did_web))
        .route("/did:web/:did", axum::routing::delete(handler::delete_did_web))
        .with_state(pool)
        .with_state(store);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}