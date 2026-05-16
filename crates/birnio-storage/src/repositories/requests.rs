use birnio_core::Request;
use sqlx::Row;
use uuid::Uuid;

use crate::{DbPool, StorageResult, models::RequestRecord};

pub async fn upsert(
    pool: &DbPool,
    request: &Request,
    collection_id: Option<Uuid>,
) -> StorageResult<()> {
    let record = RequestRecord::from_domain(request, collection_id)?;

    sqlx::query(
        r#"
        INSERT INTO requests (id, collection_id, name, request_json)
        VALUES (?1, ?2, ?3, ?4)
        ON CONFLICT(id) DO UPDATE SET
            collection_id = excluded.collection_id,
            name = excluded.name,
            request_json = excluded.request_json
        "#,
    )
    .bind(record.id)
    .bind(record.collection_id)
    .bind(record.name)
    .bind(record.request_json)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get(pool: &DbPool, id: Uuid) -> StorageResult<Option<Request>> {
    let row =
        sqlx::query("SELECT id, collection_id, name, request_json FROM requests WHERE id = ?1")
            .bind(id.to_string())
            .fetch_optional(pool)
            .await?;

    row.map(|row| {
        RequestRecord {
            id: row.get("id"),
            collection_id: row.get("collection_id"),
            name: row.get("name"),
            request_json: row.get("request_json"),
        }
        .into_domain()
    })
    .transpose()
}
