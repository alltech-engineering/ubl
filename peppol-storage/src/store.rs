use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredDocument {
    pub id: Uuid,
    pub document_type: String,
    pub document_id: String,
    pub payload: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub validated: bool,
    pub validation_errors: serde_json::Value,
}

#[async_trait]
pub trait Storage: Send + Sync {
    async fn store(&self, doc: StoredDocument) -> Result<StoredDocument, StorageError>;
    async fn get(&self, id: Uuid) -> Result<Option<StoredDocument>, StorageError>;
    async fn list(
        &self,
        doc_type: Option<&str>,
        limit: usize,
    ) -> Result<Vec<StoredDocument>, StorageError>;
    async fn delete(&self, id: Uuid) -> Result<(), StorageError>;
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("storage error: {0}")]
    Internal(String),
}
