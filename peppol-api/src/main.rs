// Peppol BIS REST API Server
//
// Axum server on :3000 exposing Peppol validation rules
// via JSON endpoints.

use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber;

use peppol_api::state::AppState;
use peppol_storage::memory::InMemoryStore;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create the in-memory storage backend
    let store = Arc::new(InMemoryStore::new());

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
