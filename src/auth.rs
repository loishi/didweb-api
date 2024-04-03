use argon2::{self, Config};
use rand::Rng;
use sqlx::SqlitePool;

pub async fn hash_password(password: &str) -> Result<String, argon2::Error> {
    let salt = rand::thread_rng().gen::<[u8; 16]>();
    let config = Config::default();
    let hash = argon2::hash_encoded(password.as_bytes(), &salt, &config)?;
    Ok(hash)
}

pub async fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password.as_bytes())
}

pub async fn authenticate(pool: &SqlitePool, username: &str, password: &str) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!("SELECT password_hash FROM users WHERE username = ?", username)
        .fetch_optional(pool)
        .await?;

    if let Some(row) = result {
        let password_hash = row.password_hash;
        Ok(verify_password(password, &password_hash).await.unwrap_or(false))
    } else {
        Ok(false)
    }
}