// Peppol API — handler functions
//
// Each handler deserializes JSON, runs Peppol validation rules,
// stores the validated document, and returns the results.

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use chrono::Utc;
use peppol_common::rules::Severity;
use peppol_storage::StoredDocument;
use serde::Serialize;
use uuid::Uuid;

use crate::state::AppState;

// ── Domain types ──
use ubl_documents::billing::{CreditNote, Invoice};
use ubl_documents::catalogue::Catalogue;
use ubl_documents::despatch::DespatchAdvice;
use ubl_documents::ordering::{Order, OrderResponse};
use ubl_documents::status::ApplicationResponse;

// ── Peppol rule engines ──
use peppol_billing::rules::{billing_rules, credit_note_rules};
use peppol_catalogue::rules::catalogue_rules;
use peppol_despatch::rules::despatch_rules;
use peppol_imr::rules::imr_rules;
use peppol_mlr::rules::mlr_rules;
use peppol_ordering::rules::{ordering_response_rules, ordering_rules};

// ── Response types ──

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
}

#[derive(Serialize)]
struct ValidationResponse {
    valid: bool,
    stored_id: String,
    errors: Vec<RuleInfo>,
    warnings: Vec<RuleInfo>,
}

#[derive(Serialize)]
struct RuleInfo {
    rule_id: String,
    message: String,
    severity: String,
}

// ── Health ─────────────────────────────────────────────────────────────

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok", version: env!("CARGO_PKG_VERSION") })
}

// ── Helpers ────────────────────────────────────────────────────────────

fn build_validation(
    outcomes: Vec<peppol_common::rules::RuleOutcome>,
    stored_id: Uuid,
) -> ValidationResponse {
    let mut errors = vec![];
    let mut warnings = vec![];
    for o in outcomes {
        if let Some(ref sev) = o.severity {
            let info = RuleInfo { rule_id: o.rule_id, message: o.message, severity: format!("{:?}", sev) };
            match sev { Severity::Warning => warnings.push(info), _ => errors.push(info) }
        }
    }
    ValidationResponse { valid: errors.is_empty(), stored_id: stored_id.to_string(), errors, warnings }
}

async fn store(
    storage: &(dyn peppol_storage::Storage + Send + Sync),
    doc_type: String, doc_id: String,
    payload: serde_json::Value, outcomes: &[peppol_common::rules::RuleOutcome],
) -> Result<Uuid, peppol_storage::StorageError> {
    let validated = outcomes.iter().filter(|o| o.severity.is_some())
        .all(|o| matches!(o.severity, Some(Severity::Warning)));
    let doc = StoredDocument {
        id: Uuid::new_v4(), document_type: doc_type, document_id: doc_id,
        payload, created_at: Utc::now(), validated,
        validation_errors: serde_json::json!([]),
    };
    let id = doc.id;
    storage.store(doc).await?;
    Ok(id)
}

macro_rules! validate_handler {
    ($name:ident, $ty:ty, $rule_fn:path, $doc_type:expr) => {
        pub async fn $name(
            State(state): State<AppState>,
            Json(doc): Json<$ty>,
        ) -> impl IntoResponse {
            let doc_id = doc.id.value().to_string();
            let payload = serde_json::to_value(&doc).unwrap_or_default();
            let outcomes = $rule_fn(&doc).evaluate_all();
            let stored_id = store(state.storage.as_ref(), $doc_type.into(), doc_id, payload, &outcomes).await.unwrap_or_else(|_| Uuid::nil());
            let response = build_validation(outcomes, stored_id);
            let status = if response.valid { StatusCode::OK } else { StatusCode::UNPROCESSABLE_ENTITY };
            (status, Json(response))
        }
    };
}

validate_handler!(validate_invoice, Invoice, billing_rules, "Invoice");
validate_handler!(validate_credit_note, CreditNote, credit_note_rules, "CreditNote");
validate_handler!(validate_order, Order, ordering_rules, "Order");
validate_handler!(validate_order_response, OrderResponse, ordering_response_rules, "OrderResponse");
validate_handler!(validate_despatch, DespatchAdvice, despatch_rules, "DespatchAdvice");
validate_handler!(validate_mlr, ApplicationResponse, mlr_rules, "MLR");
validate_handler!(validate_imr, ApplicationResponse, imr_rules, "IMR");
validate_handler!(validate_catalogue, Catalogue, catalogue_rules, "Catalogue");

// ── Order form ─────────────────────────────────────────────────────────

/// Serve the HTML order form.
pub async fn order_form() -> Html<&'static str> {
    Html(include_str!("../order_form.html"))
}

// ── Submit order ───────────────────────────────────────────────────────

/// Accept a JSON Order, validate it, store it, and return the result.
pub async fn submit_order(
    State(state): State<AppState>,
    Json(order): Json<Order>,
) -> impl IntoResponse {
    let doc_id = order.id.value().to_string();
    let payload = serde_json::to_value(&order).unwrap_or_default();
    let outcomes = ordering_rules(&order).evaluate_all();
    let stored_id = store(
        state.storage.as_ref(),
        "Order".into(),
        doc_id,
        payload,
        &outcomes,
    )
    .await
    .unwrap_or_else(|_| Uuid::nil());
    let response = build_validation(outcomes, stored_id);
    let status = if response.valid {
        StatusCode::OK
    } else {
        StatusCode::UNPROCESSABLE_ENTITY
    };
    (status, Json(response))
}

// ── Document retrieval ─────────────────────────────────────────────────

pub async fn list_documents(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    state.storage.list(None, 100).await
        .map(|docs| Json(serde_json::json!({ "documents": docs })))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))))
}

pub async fn get_document(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match state.storage.get(id).await {
        Ok(Some(doc)) => Ok(Json(serde_json::to_value(doc).unwrap_or_default())),
        Ok(None) => Err((StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "not found" })))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))),
    }
}
