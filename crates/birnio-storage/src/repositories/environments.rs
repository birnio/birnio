use birnio_core::Environment;
use sqlx::Row;
use uuid::Uuid;

use crate::{DbPool, StorageResult, models::EnvironmentRecord};

pub async fn upsert(pool: &DbPool, environment: &Environment) -> StorageResult<()> {
    let record = EnvironmentRecord::from_domain(environment)?;

    sqlx::query(
        r#"
        INSERT INTO environments (id, name, variables_json)
        VALUES (?1, ?2, ?3)
        ON CONFLICT(id) DO UPDATE SET
            name = excluded.name,
            variables_json = excluded.variables_json
        "#,
    )
    .bind(record.id)
    .bind(record.name)
    .bind(record.variables_json)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get(pool: &DbPool, id: Uuid) -> StorageResult<Option<Environment>> {
    let row = sqlx::query("SELECT id, name, variables_json FROM environments WHERE id = ?1")
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?;

    row.map(|row| {
        EnvironmentRecord {
            id: row.get("id"),
            name: row.get("name"),
            variables_json: row.get("variables_json"),
        }
        .into_domain()
    })
    .transpose()
}
