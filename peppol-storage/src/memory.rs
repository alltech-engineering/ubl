use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{Storage, StorageError, StoredDocument};

/// In-memory storage backend backed by a `HashMap` protected by a `RwLock`.
///
/// Suitable for development and testing. All operations are async and
/// thread-safe.
#[derive(Debug, Default)]
pub struct InMemoryStore {
    docs: RwLock<HashMap<Uuid, StoredDocument>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            docs: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl Storage for InMemoryStore {
    async fn store(&self, doc: StoredDocument) -> Result<StoredDocument, StorageError> {
        let mut docs = self.docs.write().await;
        docs.insert(doc.id, doc.clone());
        Ok(doc)
    }

    async fn get(&self, id: Uuid) -> Result<Option<StoredDocument>, StorageError> {
        let docs = self.docs.read().await;
        Ok(docs.get(&id).cloned())
    }

    async fn list(
        &self,
        doc_type: Option<&str>,
        limit: usize,
    ) -> Result<Vec<StoredDocument>, StorageError> {
        let docs = self.docs.read().await;
        let results: Vec<StoredDocument> = docs
            .values()
            .filter(|doc| {
                doc_type
                    .map(|dt| doc.document_type == dt)
                    .unwrap_or(true)
            })
            .take(limit)
            .cloned()
            .collect();
        Ok(results)
    }

    async fn delete(&self, id: Uuid) -> Result<(), StorageError> {
        let mut docs = self.docs.write().await;
        docs.remove(&id);
        Ok(())
    }
}
