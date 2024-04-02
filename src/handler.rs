use axum::{extract::Path, extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::auth::authenticate;

#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct DidWebDocument {
    // Define the structure of the DID document
    // Example:
    // id: String,
    // context: Vec<String>,
    // verification_method: Vec<VerificationMethod>,
    // authentication: Vec<String>,
    // ...
}

pub async fn resolve_did_web(Path(did): Path<String>) -> Result<Json<DidWebDocument>, StatusCode> {
    // Resolve the did:web and return the corresponding DID document
    // Implement the logic based on the did:web specification
    // Example:
    // 1. Parse the did:web identifier
    // 2. Retrieve the DID document from storage (e.g., file system, database)
    // 3. Return the DID document as JSON
    unimplemented!()
}

pub async fn create_did_web(
    State(pool): State<SqlitePool>,
    credentials: Json<Credentials>,
    document: Json<DidWebDocument>,
) -> Result<StatusCode, StatusCode> {
    let authenticated = authenticate(&pool, &credentials.username, &credentials.password)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    if !authenticated {
        return Err(StatusCode::UNAUTHORIZED);
    }
    // Create a new did:web and store the corresponding DID document
    // Implement the logic based on the did:web specification
    // Example:
    // 1. Generate a new did:web identifier
    // 2. Store the DID document in storage (e.g., file system, database)
    // 3. Return a success status code
    unimplemented!()
}

pub async fn update_did_web(
    State(pool): State<SqlitePool>,
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
    // Update the specified did:web with the provided DID document
    // Implement the logic based on the did:web specification
    // Example:
    // 1. Parse the did:web identifier
    // 2. Retrieve the existing DID document from storage
    // 3. Update the DID document with the provided data
    // 4. Store the updated DID document in storage
    // 5. Return a success status code
    unimplemented!()
}

pub async fn delete_did_web(
    State(pool): State<SqlitePool>,
    credentials: Json<Credentials>,
    Path(did): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let authenticated = authenticate(&pool, &credentials.username, &credentials.password)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    if !authenticated {
        return Err(StatusCode::UNAUTHORIZED);
    }
    // Delete the specified did:web and its corresponding DID document
    // Implement the logic based on the did:web specification
    // Example:
    // 1. Parse the did:web identifier
    // 2. Delete the DID document from storage
    // 3. Return a success status code
    unimplemented!()
}