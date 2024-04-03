use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use sqlx::SqlitePool;

pub async fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
}

pub async fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
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