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

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build runtime with larger stack for deep Order clones
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .thread_stack_size(8 * 1024 * 1024) // 8MB stacks
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
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

        let state = AppState::new(store);
        let app = peppol_api::routes::build_router(state);

        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        tracing::info!("Peppol API server listening on {}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });
}
