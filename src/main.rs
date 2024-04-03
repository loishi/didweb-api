
use axum::{
    routing::{get, post},
    Router
};
use std::sync::Arc;
use std::net::SocketAddr;
use tokio::sync::RwLock;

mod auth;
mod database;
mod handlers;

#[tokio::main]
async fn main() {
    let pool = database::create_pool().await.expect("Failed to create database pool");
    let store = Arc::new(RwLock::new(std::collections::HashMap::new()));

    let app = Router::new()
    .route("/did:web", post(handlers::create_did_web))
    .route("/did:web/:did", get(handlers::resolve_did_web))
    .route("/did:web/:did", post(handlers::update_did_web))
    .route("/did:web/:did", axum::routing::delete(handlers::delete_did_web))
    .with_state(store)
    .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}