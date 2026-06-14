// Peppol API — route definitions
//
// All POST validation endpoints plus document retrieval and a health check.

use axum::Router;
use axum::routing::{get, post};

use crate::handlers;
use crate::state::AppState;

/// Build the complete Axum router with all Peppol endpoints.
pub fn build_router(state: AppState) -> Router {
    Router::new()
        // Health check
        .route("/api/health", get(handlers::health))
        // Document retrieval
        .route("/api/documents", get(handlers::list_documents))
        .route("/api/documents/{id}", get(handlers::get_document))
        // Billing
        .route("/api/validate/invoice", post(handlers::validate_invoice))
        .route("/api/validate/credit-note", post(handlers::validate_credit_note))
        // Ordering
        .route("/api/validate/order", post(handlers::validate_order))
        .route("/api/validate/order-response", post(handlers::validate_order_response))
        // Despatch
        .route("/api/validate/despatch", post(handlers::validate_despatch))
        // Message Level Response
        .route("/api/validate/mlr", post(handlers::validate_mlr))
        // Invoice Message Response
        .route("/api/validate/imr", post(handlers::validate_imr))
        // Catalogue
        .route("/api/validate/catalogue", post(handlers::validate_catalogue))
        // Attach shared state
        .with_state(state)
}
