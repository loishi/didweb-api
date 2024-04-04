use crate::auth::authenticate;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerificationMethod {
    id: String,
    type_: String,
    controller: String,
    public_key_jwk: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    context: Vec<String>,
    id: String,
    authentication: Option<Vec<VerificationMethod>>,
    assertion_method: Option<Vec<VerificationMethod>>,
    key_agreement: Option<Vec<VerificationMethod>>,
    capability_invocation: Option<Vec<VerificationMethod>>,
    capability_delegation: Option<Vec<VerificationMethod>>,
    service: Option<Vec<serde_json::Value>>,
}

type DidWebStore = Arc<RwLock<std::collections::HashMap<String, Document>>>;

pub async fn resolve_did_web(
    Path(did): Path<String>,
    State(store): State<DidWebStore>,
) -> impl IntoResponse {
    let store = store.read().await;
    let doc = store.get(&did).cloned();
    match doc {
        Some(doc) => (StatusCode::OK, Json(doc)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn create_did_web(
    State(pool): State<SqlitePool>,
    State(store): State<DidWebStore>,
    Json(credentials): Json<Credentials>,
    Json(document): Json<Document>,
) -> impl IntoResponse {
    match authenticate(&pool, &credentials.username, &credentials.password).await {
        Ok(authenticated) if authenticated => {
            let mut store = store.write().await;
            if store.contains_key(&document.id) {
                (StatusCode::CONFLICT, "DID already exists".to_string()).into_response()
            } else {
                store.insert(document.id.clone(), document.clone());
                (StatusCode::CREATED, Json(document)).into_response()
            }
        }
        _ => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()).into_response(),
    }
}

pub async fn update_did_web(
    State(pool): State<SqlitePool>,
    State(store): State<DidWebStore>,
    Path(did): Path<String>,
    Json(credentials): Json<Credentials>,
    Json(document): Json<Document>,
) -> impl IntoResponse {
    match authenticate(&pool, &credentials.username, &credentials.password).await {
        Ok(authenticated) if authenticated => {
            let mut store = store.write().await;
            if !store.contains_key(&did) {
                (StatusCode::NOT_FOUND, "DID not found".to_string()).into_response()
            } else {
                store.insert(did, document);
                StatusCode::NO_CONTENT.into_response()
            }
        }
        _ => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()).into_response(),
    }
}

pub async fn delete_did_web(
    State(pool): State<SqlitePool>,
    State(store): State<DidWebStore>,
    Path(did): Path<String>,
    Json(credentials): Json<Credentials>,
) -> impl IntoResponse {
    match authenticate(&pool, &credentials.username, &credentials.password).await {
        Ok(authenticated) if authenticated => {
            let mut store = store.write().await;
            if !store.contains_key(&did) {
                (StatusCode::NOT_FOUND, "DID not found".to_string()).into_response()
            } else {
                store.remove(&did);
                StatusCode::NO_CONTENT.into_response()
            }
        }
        _ => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()).into_response(),
    }
}