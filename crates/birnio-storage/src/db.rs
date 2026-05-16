use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

use crate::StorageResult;

pub type DbPool = SqlitePool;

pub async fn connect(database_url: &str) -> StorageResult<DbPool> {
    Ok(SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?)
}

pub async fn run_migrations(pool: &DbPool) -> StorageResult<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
