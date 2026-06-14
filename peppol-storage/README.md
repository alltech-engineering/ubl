# peppol-storage

Clean-architecture document storage for Peppol documents. Defines an async
`Storage` trait with two implementations — in-memory and PostgreSQL.

Used by `peppol-api` for persisting validated Peppol documents.

## Directory Structure

```
src/
├── store.rs     # Storage trait + StoredDocument + StorageError
├── memory.rs    # InMemoryStore: HashMap<RwLock> backend
├── postgres.rs  # PgStore: SQLx + PostgreSQL backend
└── lib.rs       # Public re-exports
```

## Key Types

### `StoredDocument`

```rust
pub struct StoredDocument {
    pub id: Uuid,
    pub document_type: String,       // e.g., "Invoice", "Order"
    pub document_id: String,         // Business identifier
    pub payload: serde_json::Value,  // Full document as JSON
    pub created_at: DateTime<Utc>,
    pub validated: bool,
    pub validation_errors: serde_json::Value,
}
```

### `Storage` Trait

```rust
#[async_trait]
pub trait Storage: Send + Sync {
    async fn store(&self, doc: StoredDocument) -> Result<StoredDocument, StorageError>;
    async fn get(&self, id: Uuid) -> Result<Option<StoredDocument>, StorageError>;
    async fn list(&self, doc_type: Option<&str>, limit: usize)
        -> Result<Vec<StoredDocument>, StorageError>;
    async fn delete(&self, id: Uuid) -> Result<(), StorageError>;
}
```

### Implementations

| Implementation | Backend | Use Case |
|---------------|---------|----------|
| `InMemoryStore` | `HashMap` + `tokio::sync::RwLock` | Development, testing |
| `PgStore` | `sqlx::PgPool` + PostgreSQL | Production |

## Usage

```rust
use peppol_storage::{InMemoryStore, PgStore, Storage, StoredDocument};

// In-memory (dev/test)
let store = InMemoryStore::new();
let saved = store.store(doc).await?;
let docs = store.list(Some("Invoice"), 50).await?;

// PostgreSQL (production)
let pool = sqlx::PgPool::connect("postgres://...").await?;
let store = PgStore::new(pool).await?;  // auto-creates table
store.store(doc).await?;
```

## PostgreSQL Schema

The `PgStore` auto-creates this table on initialization:

```sql
CREATE TABLE IF NOT EXISTS peppol_documents (
    id UUID PRIMARY KEY,
    document_type TEXT NOT NULL,
    document_id TEXT NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    validated BOOLEAN NOT NULL DEFAULT false,
    validation_errors JSONB NOT NULL DEFAULT '[]'
);
```

## Design

- **Trait-based** — swap backends without changing application code
- **Async** throughout (`async_trait` + Tokio)
- `InMemoryStore` for fast development loops, `PgStore` for production
- Documents stored as JSONB for flexible querying
