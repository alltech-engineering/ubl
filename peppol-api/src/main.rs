// Peppol BIS REST API Server
//
// Axum server on :3000 exposing Peppol validation rules
// via JSON endpoints.

use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber;

use peppol_api::state::AppState;
use peppol_storage::file::FileStore;
use peppol_storage::memory::InMemoryStore;
use peppol_storage::Storage;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Try FileStore first, fall back to InMemoryStore
    let store: Arc<dyn Storage> = match FileStore::new("./orders/").await {
        Ok(fs) => {
            tracing::info!("Using FileStore at ./orders/");
            Arc::new(fs)
        }
        Err(e) => {
            tracing::warn!("FileStore unavailable: {e} — falling back to InMemoryStore");
            Arc::new(InMemoryStore::new())
        }
    };

    // Build shared application state
    let state = AppState::new(store);

    // Build the application router
    let app = peppol_api::routes::build_router(state);

    // Bind to port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Peppol API server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
