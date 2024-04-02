use sqlx::sqlite::SqlitePool;

pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePool::connect("sqlite:did_web.db").await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}