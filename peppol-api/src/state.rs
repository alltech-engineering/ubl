// Peppol API — shared application state
//
// Carries the storage backend so handlers can persist
// validated documents.

use peppol_storage::Storage;
use std::sync::Arc;

/// Application state shared across all request handlers.
#[derive(Clone)]
pub struct AppState {
    /// The document storage backend (in-memory or PostgreSQL).
    pub storage: Arc<dyn Storage>,
}

impl AppState {
    /// Create a new `AppState` wrapping a storage backend.
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        Self { storage }
    }
}
