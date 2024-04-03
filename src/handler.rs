use axum::{extract::Path, extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::auth::authenticate;

#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DidWebDocument {
    // Define the structure of the DID document
    // Example:
    id: String,
    context: Vec<String>,
    // Add other relevant fields
}

type DidWebStore = Arc<RwLock<std::collections::HashMap<String, DidWebDocument>>>;

pub async fn resolve_did_web(
    Path(did): Path<String>,
    State(store): State<DidWebStore>,
) -> Result<Json<DidWebDocument>, StatusCode> {
    let store = store.read().await;
    let doc = store.get(&did).cloned();
    match doc {
        Some(doc) => Ok(Json(doc)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_did_web(
    State(pool): State<SqlitePool>,
    State(store): State<DidWebStore>,
    credentials: Json<Credentials>,
    document: Json<DidWebDocument>,
) -> Result<StatusCode, StatusCode> {
    let authenticated = authenticate(&pool, &credentials.username, &credentials.password)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    if !authenticated {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let mut store = store.write().await;
    if store.contains_key(&document.id) {
        return Err(StatusCode::CONFLICT);
    }
    store.insert(document.id.clone(), document.into_inner());
    Ok(StatusCode::CREATED)
}

pub async fn update_did_web(
    State(pool): State<SqlitePool>,
    State(store): State<DidWebStore>,
    credentials: Json<Credentials>,
    Path(did): Path<String>,
    document: Json<DidWebDocument>,
) -> Result<StatusCode, StatusCode> {
    let authenticated = authenticate(&pool, &credentials.username, &credentials.password)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    if !authenticated {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let mut store = store.write().await;
    if !store.contains_key(&did) {
        return Err(StatusCode::NOT_FOUND);
    }
    store.insert(did, document.into_inner());
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_did_web(
    State(pool): State<SqlitePool>,
    State(store): State<DidWebStore>,
    credentials: Json<Credentials>,
    Path(did): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let authenticated = authenticate(&pool, &credentials.username, &credentials.password)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    if !authenticated {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let mut store = store.write().await;
    if !store.contains_key(&did) {
        return Err(StatusCode::NOT_FOUND);
    }
    store.remove(&did);
    Ok(StatusCode::NO_CONTENT)
}