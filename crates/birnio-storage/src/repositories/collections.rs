use birnio_core::Collection;
use sqlx::Row;
use uuid::Uuid;

use crate::{DbPool, StorageResult, models::CollectionRecord};

pub async fn upsert(pool: &DbPool, collection: &Collection) -> StorageResult<()> {
    let record = CollectionRecord::from_domain(collection)?;

    sqlx::query(
        r#"
        INSERT INTO collections (id, name, nodes_json)
        VALUES (?1, ?2, ?3)
        ON CONFLICT(id) DO UPDATE SET
            name = excluded.name,
            nodes_json = excluded.nodes_json
        "#,
    )
    .bind(record.id)
    .bind(record.name)
    .bind(record.nodes_json)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get(pool: &DbPool, id: Uuid) -> StorageResult<Option<Collection>> {
    let row = sqlx::query("SELECT id, name, nodes_json FROM collections WHERE id = ?1")
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?;

    row.map(|row| {
        CollectionRecord {
            id: row.get("id"),
            name: row.get("name"),
            nodes_json: row.get("nodes_json"),
        }
        .into_domain()
    })
    .transpose()
}

pub async fn list(pool: &DbPool) -> StorageResult<Vec<Collection>> {
    let rows = sqlx::query("SELECT id, name, nodes_json FROM collections ORDER BY name")
        .fetch_all(pool)
        .await?;

    rows.into_iter()
        .map(|row| {
            CollectionRecord {
                id: row.get("id"),
                name: row.get("name"),
                nodes_json: row.get("nodes_json"),
            }
            .into_domain()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;

    use super::*;
    use crate::run_migrations;

    #[tokio::test]
    async fn upserts_and_loads_collection() {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("sqlite memory pool");
        run_migrations(&pool).await.expect("migrations");

        let collection = Collection::new("Example API");
        upsert(&pool, &collection).await.expect("upsert collection");

        let loaded = get(&pool, collection.id)
            .await
            .expect("load collection")
            .expect("collection exists");

        assert_eq!(loaded.name, "Example API");
        assert_eq!(list(&pool).await.expect("list collections").len(), 1);
    }
}
