use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{Storage, StorageError, StoredDocument};

/// PostgreSQL-backed storage using `sqlx::PgPool`.
#[derive(Debug, Clone)]
pub struct PgStore {
    pool: PgPool,
}

impl PgStore {
    /// Create a new `PgStore` and ensure the required table exists.
    pub async fn new(pool: PgPool) -> Result<Self, StorageError> {
        let store = Self { pool };
        store.init_table().await?;
        Ok(store)
    }

    async fn init_table(&self) -> Result<(), StorageError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS peppol_documents (
                id UUID PRIMARY KEY,
                document_type TEXT NOT NULL,
                document_id TEXT NOT NULL,
                payload JSONB NOT NULL,
                created_at TIMESTAMPTZ NOT NULL,
                validated BOOLEAN NOT NULL DEFAULT FALSE,
                validation_errors JSONB NOT NULL DEFAULT '{}'::jsonb
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| StorageError::Internal(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl Storage for PgStore {
    async fn store(&self, doc: StoredDocument) -> Result<StoredDocument, StorageError> {
        sqlx::query(
            r#"
            INSERT INTO peppol_documents (id, document_type, document_id, payload, created_at, validated, validation_errors)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                document_type = EXCLUDED.document_type,
                document_id = EXCLUDED.document_id,
                payload = EXCLUDED.payload,
                created_at = EXCLUDED.created_at,
                validated = EXCLUDED.validated,
                validation_errors = EXCLUDED.validation_errors
            "#,
        )
        .bind(doc.id)
        .bind(&doc.document_type)
        .bind(&doc.document_id)
        .bind(&doc.payload)
        .bind(doc.created_at)
        .bind(doc.validated)
        .bind(&doc.validation_errors)
        .execute(&self.pool)
        .await
        .map_err(|e| StorageError::Internal(e.to_string()))?;

        Ok(doc)
    }

    async fn get(&self, id: Uuid) -> Result<Option<StoredDocument>, StorageError> {
        let row = sqlx::query_as::<_, StoredDocumentRow>(
            "SELECT id, document_type, document_id, payload, created_at, validated, validation_errors FROM peppol_documents WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| StorageError::Internal(e.to_string()))?;

        Ok(row.map(Into::into))
    }

    async fn list(
        &self,
        doc_type: Option<&str>,
        limit: usize,
    ) -> Result<Vec<StoredDocument>, StorageError> {
        let rows = if let Some(dt) = doc_type {
            sqlx::query_as::<_, StoredDocumentRow>(
                "SELECT id, document_type, document_id, payload, created_at, validated, validation_errors FROM peppol_documents WHERE document_type = $1 ORDER BY created_at DESC LIMIT $2",
            )
            .bind(dt)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, StoredDocumentRow>(
                "SELECT id, document_type, document_id, payload, created_at, validated, validation_errors FROM peppol_documents ORDER BY created_at DESC LIMIT $1",
            )
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await
        }
        .map_err(|e| StorageError::Internal(e.to_string()))?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn delete(&self, id: Uuid) -> Result<(), StorageError> {
        let result = sqlx::query("DELETE FROM peppol_documents WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| StorageError::Internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(id.to_string()));
        }

        Ok(())
    }
}

// sqlx query_as row struct — private, not exposed through the trait
#[derive(Debug, sqlx::FromRow)]
struct StoredDocumentRow {
    id: Uuid,
    document_type: String,
    document_id: String,
    payload: serde_json::Value,
    created_at: DateTime<Utc>,
    validated: bool,
    validation_errors: serde_json::Value,
}

impl From<StoredDocumentRow> for StoredDocument {
    fn from(row: StoredDocumentRow) -> Self {
        StoredDocument {
            id: row.id,
            document_type: row.document_type,
            document_id: row.document_id,
            payload: row.payload,
            created_at: row.created_at,
            validated: row.validated,
            validation_errors: row.validation_errors,
        }
    }
}
